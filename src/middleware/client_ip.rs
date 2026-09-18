//! Haqiqiy klient IP sini aniqlash (reverse proxy ortida ham).
//!
//! Ilova prodda Caddy ortida turadi, ya'ni HAR BIR so'rovning TCP manbasi —
//! Caddy konteyneri. `ConnectInfo` dan olingan IP shu sababli hamma uchun
//! BIR XIL bo'ladi va IP bo'yicha har qanday cheklov butun internetni bitta
//! chelakka soladi ([`crate::middleware::rate_limit`] ga qarang).
//!
//! # Ishonch modeli
//!
//! `X-Forwarded-For` ni istalgan klient o'zi yozib yuborishi mumkin, shuning
//! uchun u FAQAT so'rov ishonchli proksidan kelgan bo'lsa o'qiladi:
//!
//! - TCP manbasi ichki tarmoq (loopback, RFC1918, RFC4193) bo'lsa — proksi
//!   deb hisoblanadi va sarlavha o'qiladi.
//! - TCP manbasi ommaviy IP bo'lsa — demak klient to'g'ridan-to'g'ri ulangan,
//!   sarlavhaga umuman ishonilmaydi.
//!
//! Shu qoida tufayli ilovani proksisiz ochib qo'yish ham xavfsiz qoladi:
//! internetdan kelgan hech kim o'zini boshqa IP deb tanishtira olmaydi.
//!
//! # Nega ro'yxatning OXIRGI elementi
//!
//! Proksilar `X-Forwarded-For` ga o'zi ko'rgan manzilni QO'SHIB boradi:
//!
//! ```text
//!   X-Forwarded-For: <klient yozgan soxta qiymatlar...>, <Caddy ko'rgan haqiqiy IP>
//! ```
//!
//! Ya'ni klient boshiga xohlagancha yolg'on qo'sha oladi, lekin oxirgi qiymatni
//! bizning proksimiz yozadi. Shuning uchun eng O'NGDAGI to'g'ri qiymat olinadi,
//! chapdagi emas — chapdan olish klassik IP-spoofing teshigi.
//!
//! ⚠️ Agar kelajakda oldinga yana bitta proksi qo'yilsa (masalan Cloudflare),
//! oxirgi qiymat o'sha proksining IP si bo'ladi — u holda `CF-Connecting-IP`
//! kabi maxsus sarlavha kerak bo'ladi. Hozir oldinda faqat Caddy turibdi.

use std::net::{IpAddr, SocketAddr};

use axum::{
    extract::{ConnectInfo, Request},
    http::HeaderMap,
};

/// Sarlavha nomi — Caddy `reverse_proxy` uni sukut bo'yicha qo'shadi.
const X_FORWARDED_FOR: &str = "x-forwarded-for";

/// Hech narsa aniqlanmaganda ishlatiladigan zaxira qiymat.
/// (`ConnectInfo` yo'q — masalan testlarda yoki `into_make_service` bilan.)
const UNKNOWN: IpAddr = IpAddr::V4(std::net::Ipv4Addr::UNSPECIFIED);

/// So'rov uchun klientning haqiqiy IP sini qaytaradi.
pub fn resolve(request: &Request) -> IpAddr {
    let peer = peer_ip(request);

    if peer.map(is_trusted_proxy).unwrap_or(false) {
        if let Some(ip) = forwarded_for(request.headers()) {
            return ip;
        }
    }

    peer.unwrap_or(UNKNOWN)
}

/// TCP ulanishning haqiqiy manbasi — uni soxtalashtirib bo'lmaydi.
fn peer_ip(request: &Request) -> Option<IpAddr> {
    request
        .extensions()
        .get::<ConnectInfo<SocketAddr>>()
        .map(|info| info.0.ip())
}

/// Manzil ichki tarmoqqa tegishlimi — ya'ni bizning proksimiz bo'lishi mumkinmi.
fn is_trusted_proxy(ip: IpAddr) -> bool {
    match ip {
        IpAddr::V4(v4) => v4.is_loopback() || v4.is_private() || v4.is_link_local(),
        // `is_unique_local` va `is_unicast_link_local` hali stable emas,
        // shuning uchun qo'lda: fc00::/7 (ULA) va fe80::/10 (link-local).
        IpAddr::V6(v6) => {
            let first = v6.segments()[0];
            v6.is_loopback() || (first & 0xfe00) == 0xfc00 || (first & 0xffc0) == 0xfe80
        }
    }
}

/// `X-Forwarded-For` dagi eng o'ngdagi to'g'ri manzil.
fn forwarded_for(headers: &HeaderMap) -> Option<IpAddr> {
    headers
        .get(X_FORWARDED_FOR)?
        .to_str()
        .ok()?
        .rsplit(',')
        .find_map(|entry| parse_entry(entry.trim()))
}

/// Bitta yozuvni IP ga aylantiradi.
///
/// Ba'zi proksilar port ham qo'shadi (`1.2.3.4:5678`, `[::1]:5678`),
/// shuning uchun avval sof IP, keyin `SocketAddr` sifatida sinaladi.
fn parse_entry(entry: &str) -> Option<IpAddr> {
    if let Ok(ip) = entry.parse::<IpAddr>() {
        return Some(ip);
    }
    if let Ok(addr) = entry.parse::<SocketAddr>() {
        return Some(addr.ip());
    }
    // `[::1]` — portsiz, lekin qavs ichida
    entry
        .strip_prefix('[')
        .and_then(|rest| rest.strip_suffix(']'))
        .and_then(|inner| inner.parse::<IpAddr>().ok())
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::HeaderValue;

    fn headers(value: &str) -> HeaderMap {
        let mut h = HeaderMap::new();
        h.insert(X_FORWARDED_FOR, HeaderValue::from_str(value).unwrap());
        h
    }

    fn ip(s: &str) -> IpAddr {
        s.parse().unwrap()
    }

    #[test]
    fn oxirgi_qiymat_olinadi() {
        // Chapdagilarni klient o'zi yozgan bo'lishi mumkin — faqat oxirgisiga ishonamiz.
        assert_eq!(
            forwarded_for(&headers("1.1.1.1, 2.2.2.2, 93.184.216.34")),
            Some(ip("93.184.216.34"))
        );
    }

    #[test]
    fn bitta_qiymat() {
        assert_eq!(forwarded_for(&headers("93.184.216.34")), Some(ip("93.184.216.34")));
    }

    #[test]
    fn buzuq_qiymatlar_tashlab_ketiladi() {
        assert_eq!(forwarded_for(&headers("1.1.1.1, salom")), Some(ip("1.1.1.1")));
        assert_eq!(forwarded_for(&headers("salom, dunyo")), None);
    }

    #[test]
    fn portli_va_ipv6_yozuvlar() {
        assert_eq!(forwarded_for(&headers("93.184.216.34:443")), Some(ip("93.184.216.34")));
        assert_eq!(forwarded_for(&headers("[2001:db8::1]:443")), Some(ip("2001:db8::1")));
        assert_eq!(forwarded_for(&headers("2001:db8::1")), Some(ip("2001:db8::1")));
    }

    #[test]
    fn ichki_manzillar_ishonchli() {
        for s in ["127.0.0.1", "10.0.0.5", "172.19.0.2", "192.168.1.1", "::1", "fd00::1"] {
            assert!(is_trusted_proxy(ip(s)), "{s} ishonchli bo'lishi kerak");
        }
    }

    #[test]
    fn ommaviy_manzillar_ishonchsiz() {
        for s in ["93.184.216.34", "45.134.39.224", "2001:db8::1"] {
            assert!(!is_trusted_proxy(ip(s)), "{s} ishonchsiz bo'lishi kerak");
        }
    }
}
