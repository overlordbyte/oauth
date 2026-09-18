# oauth.uz — reja

## Nima qurilyapti

**Keycloak o'rnini bosadigan, o'zbek bozoriga mo'ljallangan multi-tenant identity
va access management (IAM) platformasi.** Uchinchi tomon saytlari o'z auth
tizimini qurish o'rniga oauth.uz ni ulaydi va foydalanuvchilar, rollar hamda
ruxsatlarni shu yerdan boshqaradi.

Asosiy ssenariy:

1. Kimdir oauth.uz ga ro'yxatdan o'tadi
2. O'zi tashkilot (organization) yaratadi: nomi, websayti
3. Admin panelda redirect URL larini sozlaydi, `client_id` / `client_secret` oladi
4. **O'z tashkilotining rollarini yaratadi** (masalan `editor`, `moderator`, `premium`)
5. Foydalanuvchilarga o'sha rollarni biriktiradi
6. `example.uz` oauth.uz orqali kirishni yoqadi va tokendan rollarni o'qib,
   o'z ichida authorization qiladi

**Muhim qaror:** tashkilot yaratish to'liq self-service, super admin tasdiqlashi
shart emas. Super admin faqat nazorat qiladi va kerak bo'lganda aralashadi.
(Sabab: qo'lda tasdiqlash o'sishni to'xtatadi, amalda esa farqi yo'q.)

---

## Asosiy tushunchalar

Keycloak atamalari bilan solishtirma — hujjatlarni o'qiyotganda foydali:

| oauth.uz | Keycloak | Izoh |
|---|---|---|
| Organization | Realm | Izolyatsiya chegarasi: o'z foydalanuvchilari, rollari, clientlari |
| Client | Client | Ulanayotgan ilova (`example.uz`) |
| Organization role | Realm role | Tashkilot o'zi yaratadigan rol |
| Client role | Client role | Bitta client doirasidagi rol |
| Group | Group | Rollarni guruhga berib, foydalanuvchini guruhga qo'shish |
| Platform role | — | oauth.uz ning o'z rollari (super admin va h.k.) |

**Muhim farq:** Keycloak'da foydalanuvchi bitta realm'ga tegishli. oauth.uz'da
foydalanuvchi **bitta**, lekin bir nechta tashkilotga a'zo bo'lishi mumkin va
har birida turli rollarga ega. Ya'ni `ali@example.com` ham `example.uz` da
`editor`, ham `shop.uz` da `admin` bo'la oladi. Bu integratsiyani ancha
soddalashtiradi.

---

## Rollar: ikki qatlam

### 1. Platforma rollari (oauth.uz ning o'zini boshqarish)

| Rol | Kim | Nima qila oladi |
|---|---|---|
| **Super admin** | Bitta, platforma egasi | Hammasini: tashkilotlar, foydalanuvchilar, to'lovlar, bloklash, impersonatsiya |
| **Organization admin** | Tashkilot yaratgan yoki taklif qilingan | O'z tashkiloti: clientlar, rollar, a'zolar, sozlamalar |
| **User** | Oddiy foydalanuvchi | O'z profili, seanslari, ruxsat bergan saytlari |

### 2. Tashkilot rollari (oauth.uz ning asosiy qiymati)

Har bir tashkilot **o'z rollarini o'zi yaratadi va boshqaradi**. oauth.uz ularni
saqlaydi, biriktiradi va tokenga joylaydi — `example.uz` hech qanday rol jadvali
yuritmaydi.

Nima kerak:

- [ ] Tashkilot rol yaratadi: nomi, tavsifi, rangi/belgisi
- [ ] Rolni tahrirlaydi va o'chiradi (o'chirilsa — barcha biriktirishlar ham)
- [ ] Rolni istalgan a'zoga biriktiradi va olib tashlaydi
- [ ] Bitta foydalanuvchida bir nechta rol bo'lishi mumkin
- [ ] **Default rol** — yangi a'zoga avtomatik beriladi (masalan `member`)
- [ ] **Composite rol** — rol ichida boshqa rollar (`admin` → `editor` + `viewer`)
- [ ] **Client rol** — faqat bitta client doirasida amal qiladigan rol
- [ ] **Guruhlar** — rollarni guruhga berib, foydalanuvchini guruhga qo'shish
- [ ] Guruhlar ierarxiyasi (ota guruh rollari meros qoladi)
- [ ] Rollar tokenga qanday joylanishi sozlanadi (claim nomi, yassi yoki client bo'yicha)
- [ ] Rol biriktirish muddati (vaqtinchalik rol, avtomat tugaydi)
- [ ] Rol o'zgarishlari audit logga tushadi

Tokendagi ko'rinishi (Keycloak formatiga yaqin, moslashtirsa bo'ladi):

```json
{
  "sub": "user-uuid",
  "org": "example-uz",
  "realm_access": { "roles": ["member", "editor"] },
  "resource_access": {
    "example-web": { "roles": ["admin"] }
  }
}
```

### Rol boshqaruvi API

- [ ] `GET/POST /api/v1/organizations/{org}/roles`
- [ ] `GET/PATCH/DELETE /api/v1/organizations/{org}/roles/{role}`
- [ ] `GET/PUT/DELETE /api/v1/organizations/{org}/members/{user}/roles`
- [ ] `GET /api/v1/organizations/{org}/roles/{role}/members` — rolga ega a'zolar
- [ ] `POST /api/v1/organizations/{org}/roles/{role}/composites` — ichki rollar
- [ ] `GET/POST /api/v1/organizations/{org}/groups` va guruh rollari
- [ ] Ommaviy (bulk) biriktirish va olib tashlash

---

## Ma'lumotlar modeli

Hammasi bitta bazada — foydalanuvchilar, tashkilotlar, rollar va biriktirishlar.

**Identity**
- `users` — `id`, `first_name`, `last_name`, `email`, `password_hash`, `email_verified`, `status`, `created_at`
- `user_attributes` — ixtiyoriy qo'shimcha maydonlar (key/value)
- `identities` — tashqi provayder hisoblari (`provider`, `provider_user_id`, `user_id`)
- `credentials` — parol, TOTP sirlari, WebAuthn kalitlari (turlar bo'yicha)
- `email_verification_codes` — kod, muddati, urinishlar soni
- `password_reset_tokens`

**Tashkilotlar**
- `organizations` — `id`, `slug`, `name`, `website`, `owner_id`, `status`, `created_at`
- `organization_members` — `org_id`, `user_id`, `status`, `joined_at`
- `organization_invitations` — email bo'yicha taklif, muddati bilan
- `organization_domains` — domen bo'yicha avtomatik a'zolik (ixtiyoriy)
- `organization_settings` — parol siyosati, 2FA majburiyligi, seans muddati

**Rollar**
- `roles` — `id`, `org_id`, `client_id` (NULL bo'lsa tashkilot roli), `name`, `description`, `is_default`
- `role_composites` — `role_id`, `child_role_id`
- `member_roles` — `member_id`, `role_id`, `expires_at`
- `groups` — `id`, `org_id`, `parent_id`, `name`
- `group_roles` — `group_id`, `role_id`
- `group_members` — `group_id`, `member_id`

**OAuth / OIDC**
- `clients` — `client_id`, `client_secret_hash`, `org_id`, `name`, `type` (public/confidential), `flows`
- `client_redirect_uris`
- `client_scopes` va `scope_mappers` — tokenga qanday claimlar tushishi
- `authorization_codes` — code, PKCE challenge, muddati
- `refresh_tokens` — rotatsiya va qayta ishlatishni aniqlash uchun
- `sessions` — faol seanslar
- `consents` — foydalanuvchi qaysi clientga qaysi scope'ga ruxsat bergani
- `signing_keys` — kalit rotatsiyasi

**Audit**
- `login_events` — kirish urinishlari, IP, user agent, natija
- `admin_events` — kim nimani o'zgartirdi
- `brute_force_state` — bloklangan hisoblar

---

## Arxitektura qarorlari

- **Faqat REST.** gRPC butunlay olib tashlandi — integratsiya qiluvchilar uchun
  murakkab edi. Barcha endpointlar HTTP + JSON.
- Backend: Axum + Diesel (async) + PostgreSQL
- Frontend: Dioxus (wasm SPA), `dx components` dizayn tizimi
- Xatolar bitta shaklda: `{"message": "...", "errors": [{"field": "...", "message": "..."}]}`
- Sahifalash: keyset (cursor), offset emas
- Token imzosi: **RS256** (hozir HS256 — Faza 7 da almashtiriladi). Uchinchi
  tomon saytlari imzoni ochiq kalit bilan tekshirishi shart.

---

## Bajarilgan

- [x] Clean architecture qatlamlari: domain / usecase / store / controller
- [x] PostgreSQL, `users` jadvali, keyset sahifalash
- [x] Argon2 parol hash, JWT servisi
- [x] IP bo'yicha rate limiting (auth: 5/daq, umumiy: 120/daq)
- [x] CORS
- [x] Frontend: login sahifasi (Google / GitHub / GitLab / Telegram + email)
- [x] Frontend: ikki qadamli register (forma → email kodini tasdiqlash)
- [x] `PROJECT_NAME` va `PROJECT_DOMAIN` env dan olinadi
- [x] gRPC olib tashlandi, `GET /api/v1/users` va `/users/{id}` REST ga o'tkazildi

---

## Faza 1 — Auth asoslari

Frontend allaqachon shu endpointlarni chaqiradi, backendda hali yo'q.

- [ ] `POST /api/v1/auth/register` — `first_name`, `last_name`, `email`, `password`
- [ ] `POST /api/v1/auth/verify-email` — `email`, `code` → token juftligi
- [ ] `POST /api/v1/auth/resend-code`
- [ ] `POST /api/v1/auth/login`
- [ ] `POST /api/v1/auth/refresh`
- [ ] `POST /api/v1/auth/logout`
- [ ] `GET /api/v1/me`
- [ ] Email yuborish servisi (SMTP yoki provayder)
- [ ] JWT middleware: `Authorization: Bearer` → `UserPrincipal`
- [ ] Parolni tiklash: `forgot-password` / `reset-password`

**Migratsiya kerak:** `users` da bitta `name` ustuni bor, frontend esa
`first_name`/`last_name` yuboradi.

**Kelishish kerak:** tasdiqlash kodi uzunligi (frontendda 6 deb belgilangan).

## Faza 2 — Tashkilotlar va rollar

Bu oauth.uz ning asosiy qiymati. Yuqoridagi "Tashkilot rollari" bo'limi to'liq
shu fazaga kiradi.

- [ ] Tashkilot CRUD (self-service yaratish)
- [ ] A'zolar: taklif qilish, qabul qilish, chiqarish
- [ ] Rol CRUD, composite rollar, default rol
- [ ] Rol biriktirish / olib tashlash (yakka va ommaviy)
- [ ] Guruhlar va guruh rollari
- [ ] Tashkilot sozlamalari (parol siyosati, 2FA majburiyligi)
- [ ] Har bir so'rovda tegishlilikni tekshirish (authorization guard)

## Faza 3 — OAuth2 / OIDC provider

Keycloak vazifasini bajaradigan asosiy qism.

- [ ] Client CRUD, `client_secret` generatsiya va rotatsiya
- [ ] Redirect URI larni qat'iy tekshirish (ochiq redirect zaifligiga yo'l qo'ymaslik)
- [ ] `GET /oauth/authorize` — authorization code flow
- [ ] **PKCE** (S256) — public clientlar uchun majburiy
- [ ] `POST /oauth/token` — code, refresh_token, client_credentials grantlari
- [ ] `POST /oauth/introspect` — token tekshirish (RFC 7662)
- [ ] `POST /oauth/revoke` (RFC 7009)
- [ ] `GET /oauth/userinfo`
- [ ] `GET /.well-known/openid-configuration` — discovery
- [ ] `GET /.well-known/jwks.json`
- [ ] RP-initiated logout, back-channel va front-channel logout
- [ ] Consent ekrani va consent'ni bekor qilish
- [ ] Scope'lar: `openid`, `profile`, `email`, `roles` + maxsus scope'lar
- [ ] **Protocol mapper'lar** — tokenga qanday claim tushishini sozlash
- [ ] Device authorization flow (RFC 8628) — TV va CLI uchun
- [ ] Token exchange (RFC 8693)
- [ ] Dynamic client registration (RFC 7591)

## Faza 4 — Tashqi provayderlar (identity brokering)

Hozir login tugmalari faqat redirect qiladi, oqim tugallanmagan.

- [ ] Google — to'liq oqim
- [ ] GitHub — to'liq oqim
- [ ] GitLab
- [ ] Telegram Login Widget (hash tekshiruvi — boshqacha mexanizm)
- [ ] Umumiy OIDC provayder (tashkilot o'zi qo'shadi)
- [ ] SAML 2.0 provayder (korporativ mijozlar uchun)
- [ ] Hisoblarni bog'lash (email bo'yicha va qo'lda)
- [ ] Birinchi kirishda hisobni avtomatik yaratish va default rol berish
- [ ] Tashqi provayderdan kelgan claimlarni rolga o'girish (mapper)

## Faza 5 — Autentifikatsiya kuchaytirish

- [ ] TOTP (2FA) — QR kod, tiklash kodlari
- [ ] WebAuthn / passkeys
- [ ] Brute force himoyasi: urinishlar soni, vaqtinchalik bloklash
- [ ] Parol siyosati: uzunlik, murakkablik, takrorlanmaslik, muddat
- [ ] "Required actions" — birinchi kirishda parol almashtirish, 2FA sozlash
- [ ] Sozlanadigan autentifikatsiya oqimlari (browser / direct grant / reset)
- [ ] "Meni eslab qol" va seans muddati sozlamalari
- [ ] Shubhali kirish haqida email xabarnoma

## Faza 6 — Admin panel (frontend)

- [ ] Tashkilotlar ro'yxati va yaratish
- [ ] Tashkilot sozlamalari: nomi, websayti, redirect URL lar
- [ ] Client boshqaruvi, `client_secret` ko'rsatish va qayta generatsiya
- [ ] **Rollar sahifasi**: yaratish, tahrirlash, o'chirish, composite sozlash
- [ ] **A'zolar sahifasi**: rol biriktirish/olib tashlash, ommaviy amallar
- [ ] Guruhlar sahifasi
- [ ] Foydalanuvchi qidiruvi va profili
- [ ] Faol seanslar va ularni to'xtatish
- [ ] Audit log ko'rinishi
- [ ] Login sahifasini brendlash (logo, rang) — tashkilot bo'yicha

## Faza 7 — Foydalanuvchi kabineti (account console)

- [ ] Profilni tahrirlash
- [ ] Parolni almashtirish
- [ ] 2FA sozlash va tiklash kodlari
- [ ] Bog'langan tashqi hisoblar
- [ ] Faol seanslar va qurilmalar, ularni to'xtatish
- [ ] Qaysi saytlarga ruxsat berilgani va uni bekor qilish
- [ ] Men a'zo bo'lgan tashkilotlar va ulardagi rollarim
- [ ] Hisobni o'chirish

## Faza 8 — Super admin

- [ ] Barcha tashkilotlar ro'yxati, qidiruv, filtrlash
- [ ] Tashkilotni bloklash / faolsizlantirish
- [ ] Barcha foydalanuvchilar, bloklash
- [ ] Impersonatsiya (nomidan kirish) — audit bilan
- [ ] Platforma statistikasi va dashboard
- [ ] Super admin birinchi marta qanday yaratiladi (seed yoki CLI buyrug'i)
- [ ] To'lov va tarif rejalari (hozircha qo'lda)

## Faza 9 — Ilg'or authorization (Keycloak Authorization Services analogi)

Rollardan chuqurroq kerak bo'lganda. Faza 2 yetarli bo'lsa — keyinga qoldirsa bo'ladi.

- [ ] Resurslar va scope'lar
- [ ] Siyosatlar: rol bo'yicha, foydalanuvchi bo'yicha, vaqt bo'yicha, agregat
- [ ] Ruxsatlar (permissions) va ularni baholash
- [ ] Siyosatni sinab ko'rish vositasi (policy evaluation)
- [ ] UMA 2.0 (ehtimol kerak emas — talab bo'lsa)

## Faza 10 — Production

- [ ] RS256 + kalit rotatsiyasi (hozir HS256, bitta secret)
- [ ] `JWT_SECRET` ni prod uchun almashtirish (hozir dev qiymat)
- [ ] Refresh token rotatsiyasi va qayta ishlatishni aniqlash
- [ ] Audit log to'liq yoqilishi
- [ ] Docker + docker-compose
- [ ] CI: `cargo clippy`, `cargo test`, frontend build
- [ ] Integratsiya testlari (auth va OAuth oqimlari uchun)
- [ ] **OpenAPI hujjati** — integratsiya qiluvchilar uchun eng muhimi
- [ ] Ulash qo'llanmasi: "example.uz ni oauth.uz ga qanday ulash kerak"
- [ ] SDK yoki namuna loyihalar (JS, PHP, Python, Rust)
- [ ] Monitoring, metrikalar, xatolarni kuzatish
- [ ] Zaxira nusxa va tiklash tartibi

---

## Keycloak funksional qamrovi

Qaysi fazada qoplanishi. Keyinroq "buni qilganmizmi?" degan savolga javob berish uchun.

| Keycloak imkoniyati | Faza | Izoh |
|---|---|---|
| Realms (multi-tenancy) | 2 | Bizda organization |
| Clients (public/confidential) | 3 | |
| Client credentials / service account | 3 | |
| Users CRUD, attributes | 1, 6 | |
| Realm roles | 2 | Tashkilot roli |
| Client roles | 2 | |
| Composite roles | 2 | |
| Default roles | 2 | |
| Groups + ierarxiya | 2 | |
| Role mapping (user, group) | 2 | |
| OIDC: authorization code + PKCE | 3 | |
| OIDC: client credentials | 3 | |
| OIDC: device flow | 3 | |
| OIDC: token exchange | 3 | |
| Discovery, JWKS | 3 | |
| Token introspection, revocation | 3 | |
| Logout (RP-initiated, back/front-channel) | 3 | |
| Protocol mappers / client scopes | 3 | |
| Consent | 3 | |
| Dynamic client registration | 3 | |
| SAML 2.0 provider | — | **Qamrab olinmaydi** — talab bo'lsa keyinroq |
| Identity brokering (social) | 4 | |
| Identity brokering (OIDC, SAML) | 4 | |
| Account linking | 4 | |
| User federation (LDAP / AD) | — | **Qamrab olinmaydi** — korporativ talab bo'lsa |
| TOTP / 2FA | 5 | |
| WebAuthn / passkeys | 5 | |
| Brute force detection | 5 | |
| Password policies | 5 | |
| Required actions | 5 | |
| Authentication flows (sozlanadigan) | 5 | |
| Admin console | 6 | |
| Admin REST API | 2, 3, 6 | Har bir faza o'z API sini beradi |
| Account console | 7 | |
| Sessions (ko'rish, to'xtatish) | 6, 7 | |
| Offline sessions | 10 | |
| Impersonation | 8 | |
| Login/admin events (audit) | 6, 10 | |
| Email (SMTP, shablonlar) | 1 | |
| Themes / brendlash | 6 | Keycloak darajasida emas — logo va rang |
| Localization (i18n) | 10 | O'zbek / rus / ingliz |
| Key rotation | 10 | |
| Realm import/export | 10 | Tashkilot sozlamalarini JSON qilib |
| Authorization services (UMA) | 9 | |
| Terms & conditions | 5 | Required action sifatida |
| User profile (declarative) | 6 | `user_attributes` orqali |

**Ataylab qamrab olinmaydi:** SAML provider va LDAP/AD federation. Ikkalasi ham
katta ish va faqat korporativ mijozlar so'raydi. Talab paydo bo'lsa qo'shiladi.

---

## oauth.uz ning Keycloak'da yo'q imkoniyatlari

Bular bizning ustunligimiz — rejadan tushib qolmasin:

- [ ] **Bitta foydalanuvchi, ko'p tashkilot.** Keycloak'da foydalanuvchi bitta
      realm'ga qamalgan. Bizda `ali@example.com` bir nechta tashkilotda turli
      rollar bilan bo'ladi.
- [ ] **Telegram Login** — mintaqa uchun muhim, Keycloak'da yo'q
- [ ] **O'zbek tilida interfeys va hujjatlar** — asosiy farqlovchi
- [ ] Soddalashtirilgan admin panel — Keycloak konsoli haddan tashqari murakkab
- [ ] Ulash uchun tayyor SDK va copy-paste namunalar
- [ ] Bir nechta tashkilotga a'zo foydalanuvchi uchun tashkilot almashtirish (switcher)

---

## Texnik qarzlar

- `frontend/src/components/card/component.rs` — `merge_attributes` ishlatmaydi, ya'ni
  tashqaridan berilgan `class` `dx-card` ni almashtirib yuboradi. `Input` da tuzatildi,
  `Card` da hali yo'q (hozircha wrapper `div` bilan chetlab o'tilgan).
- `frontend/src/ui/pages/home.rs` — hali inline `style:` ishlatadi.
- `dx components add <nom>` dan keyin yangi komponent `#[css_module]` bilan keladi —
  uni darhol oddiy CSS + `static_css!` ga o'tkazish kerak, aks holda sahifa stilsiz
  ko'rinib ketadi (FOUC). Sabab: `css_module` `<link>` ni render paytida qo'shadi.
- `frontend/Dioxus.toml` dagi `title` literal `oauth.uz` — build konfiguratsiyasi env
  o'qiy olmaydi. Wasm yuklangach `document::Title` uni `PROJECT_NAME` dan qayta yozadi.
- `users.id` hozir `SERIAL` (i32). Ommaviy IAM uchun UUID afzalroq — id sanab
  chiqib bo'lmasligi kerak. Faza 1 migratsiyasida hal qilish arzon, keyin qimmat.
