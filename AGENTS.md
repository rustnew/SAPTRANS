# SAPTRANS agent instructions

## Work in this repository

- Follow the project guidance in `README.md` and `docs/`. Preserve existing behavior and user changes. For code changes, inspect callers, run the relevant entry point and focused checks, then report observed results and concrete blockers.
- The root crate is the PostgreSQL-backed Leptos/Actix application. `preview/` is a separate Cargo crate for public-page preview and browser tests. Run commands below from the repository root.

## Setup and entry points

- Use Rust stable **1.88 or newer** with `rustfmt` and `clippy` (root `Cargo.toml` requires 1.88), Python 3, and, for browser tests, Node.js/npm and Playwright Chromium. CI uses Node 22. Run `npm install` and `npx playwright install chromium` for local browser tests; CI installs Chromium with `--with-deps`.
- Public preview without a database: `./preview.sh`, then open `http://localhost:3000` and exercise the public routes and `?lang=en`. The preview does not persist contact submissions. `npm run test:e2e` starts the preview automatically through `playwright.config.ts` if port 3000 is free.
- Full application: install and start PostgreSQL, create the `saptrans` database, copy `.env.example` to `.env`, and set `DATABASE_URL`. Run `./dev.sh`, then open `http://localhost:3000` and check `/health`, public pages, and the contact flow as relevant. The script loads `.env`, regenerates `public/data/legacy-content.json`, and rebuilds on changes. `cargo run --bin saptrans-server` also starts the server when the environment variables are already exported. The server applies SQLx migrations at startup. Stop with Ctrl+C. For deployment, `docker compose up --build` requires `POSTGRES_PASSWORD` in `.env`; set `SITE_DOMAIN` for the public domain.
- `python3 scripts/extract_legacy_content.py` regenerates public legacy content offline. `python3 scripts/audit_legacy_archive.py` needs `beautifulsoup4` and regenerates the documentary archive inventory. These scripts write generated files; run them only when that content is relevant.

## Verification

- Focus a Rust unit test with `cargo test <test-name-filter>`; tests are in `src/lib.rs`. Focus a browser test with `npx playwright test tests/public-pages.spec.ts --grep '<test-title>'`. The browser suite covers public pages, theme, language, and search against the preview service.
- Required CI checks: `cargo fmt --all -- --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`, and `npm run test:e2e`. For the browser check, install npm dependencies and Chromium first. The Playwright configuration runs desktop and mobile Chromium projects.
- No benchmark command or benchmark target is defined in the current manifests or CI. For future performance work, establish and compare the same workload before documenting a benchmark command.

## Command evidence (2026-11-12 initialization)

- **Verified in this checkout:** `cargo fmt --all -- --check` passed.
- **Found in scripts, manifests, README, or CI; not executed during initialization:** `./preview.sh`, `./dev.sh`, `cargo run --bin saptrans-server`, `docker compose up --build`, `npm install`, `npx playwright install chromium`, `cargo test <test-name-filter>`, `npx playwright test tests/public-pages.spec.ts --grep '<test-title>'`, `cargo clippy --all-targets -- -D warnings`, `cargo test`, `npm run test:e2e`, and the Python scripts above. Treat their results as unverified until run in the current environment.
