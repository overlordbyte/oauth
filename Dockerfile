# syntax=docker/dockerfile:1.7
# ============================================================
# oauth.uz — bitta Dockerfile, ikkita mustaqil target:
#
#   --target backend    axum API (Rust), 3000-port
#   --target frontend   Dioxus wasm SPA, nginx orqali, 80-port
#
# Ikkalasini ham docker-compose.yml quradi. Qo'lda:
#   docker build --target backend -t oauth-backend:latest .
# ============================================================

# ── 1) Backend build ─────────────────────────────────────────
FROM rust:1.98-slim-bookworm AS backend-build
WORKDIR /app

# diesel ning postgres backendi libpq ga linklanadi; git — git-dependency uchun
RUN apt-get update \
 && apt-get install -y --no-install-recommends libpq-dev pkg-config git \
 && rm -rf /var/lib/apt/lists/*

COPY . .

# Cache mount registry va target ni keyingi build'ga saqlaydi, lekin ular image
# ichida QOLMAYDI — shuning uchun binar aynan shu RUN ichida cache'dan tashqariga
# ko'chiriladi, aks holda keyingi qatlamda topilmaydi.
RUN --mount=type=cache,target=/usr/local/cargo/registry,sharing=locked \
    --mount=type=cache,target=/app/target,sharing=locked \
    cargo build --release --package oauth \
 && cp target/release/oauth /usr/local/bin/oauth

# ── 2) Backend runtime ───────────────────────────────────────
FROM debian:bookworm-slim AS backend
RUN apt-get update \
 && apt-get install -y --no-install-recommends libpq5 ca-certificates curl \
 && rm -rf /var/lib/apt/lists/* \
 && useradd --uid 10002 --user-group --no-create-home --shell /usr/sbin/nologin oauth

COPY --from=backend-build /usr/local/bin/oauth /usr/local/bin/oauth

USER 10002
EXPOSE 3000
# main.rs `.env.$APP_ENV` faylini qidiradi — konteynerda u YO'Q va kerak emas:
# barcha qiymatlar compose `env_file` orqali muhitga beriladi (dotenvy .ok() qiladi).
ENV APP_ENV=production
CMD ["/usr/local/bin/oauth"]

# ── 3) Frontend build (wasm) ─────────────────────────────────
# ⚠️ trixie, bookworm EMAS: rasmiy `dx` binari GLIBC 2.39 talab qiladi,
# bookworm'da esa 2.36 — binar "version `GLIBC_2.39' not found" bilan yiqiladi.
# Bu bosqich faqat statik fayl chiqaradi, shuning uchun distro farqi zararsiz.
FROM rust:1.98-slim-trixie AS frontend-build
WORKDIR /app
ARG DX_VERSION=0.7.10
# Bu ikki qiymatni frontend/build.rs kompilyatsiya vaqtida wasm ichiga yozadi —
# brauzerda muhit o'zgaruvchisi yo'q, shuning uchun aynan build ARG bo'lishi shart.
ARG PROJECT_NAME
ARG PROJECT_DOMAIN
ENV PROJECT_NAME=${PROJECT_NAME} PROJECT_DOMAIN=${PROJECT_DOMAIN}

RUN apt-get update \
 && apt-get install -y --no-install-recommends curl ca-certificates git \
 && rm -rf /var/lib/apt/lists/*
RUN rustup target add wasm32-unknown-unknown

# dx ni manbadan qurish ~15 daqiqa — rasmiy relizdagi tayyor binar olinadi.
RUN mkdir -p /tmp/dx \
 && curl -sSfL "https://github.com/DioxusLabs/dioxus/releases/download/v${DX_VERSION}/dx-x86_64-unknown-linux-gnu.tar.gz" \
    | tar -xz -C /tmp/dx \
 && mv "$(find /tmp/dx -type f -name dx | head -n1)" /usr/local/bin/dx \
 && chmod +x /usr/local/bin/dx \
 && rm -rf /tmp/dx \
 && dx --version

COPY . .

# Natija (`.../web/public`) ham cache mount ichida tug'iladi — o'sha RUN da /out ga olinadi.
RUN --mount=type=cache,target=/usr/local/cargo/registry,sharing=locked \
    --mount=type=cache,target=/app/target,sharing=locked \
    cd frontend && dx build --release --platform web \
 && cp -r "$(find /app/target/dx -type d -name public | head -n1)" /out

# ── 4) Frontend runtime ──────────────────────────────────────
FROM nginx:alpine AS frontend
COPY frontend/nginx.conf /etc/nginx/conf.d/default.conf
COPY --from=frontend-build /out /usr/share/nginx/html
EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]
