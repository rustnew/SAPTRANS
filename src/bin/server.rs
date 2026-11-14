use actix_session::{SessionMiddleware, storage::CookieSessionStore};
use actix_web::{
    App, HttpRequest, HttpResponse, HttpServer, Responder,
    cookie::{Key, SameSite},
    web,
};
use leptos::prelude::*;
use saptrans::{Language, Page, Site};
use serde::Deserialize;
use sqlx::postgres::PgPoolOptions;
use std::time::{SystemTime, UNIX_EPOCH};
use tracing_subscriber::EnvFilter;

#[derive(Clone)]
struct DevInfo {
    enabled: bool,
    generation: String,
}

#[derive(Deserialize, Default)]
struct SearchParams {
    q: Option<String>,
    reference: Option<String>,
    service: Option<String>,
}

async fn health() -> impl Responder {
    HttpResponse::Ok().body("ok")
}

async fn robots() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/plain; charset=utf-8")
        .body("User-agent: *\nAllow: /\nDisallow: /api/\nDisallow: /admin\nDisallow: /espace\nSitemap: https://saptrans.net/sitemap.xml\n")
}

async fn sitemap() -> impl Responder {
    const ROUTES: &[&str] = &[
        "/",
        "/transport-logistique",
        "/location-engins",
        "/controle-technique",
        "/immobilier",
        "/secteurs",
        "/realisations",
        "/implantations",
        "/actualites",
        "/entreprise",
        "/mentions-legales",
        "/contact",
    ];
    let entries = ROUTES
        .iter()
        .map(|path| {
            format!(
                "<url><loc>https://saptrans.net{path}</loc><xhtml:link rel=\"alternate\" hreflang=\"fr\" href=\"https://saptrans.net{path}?lang=fr\"/><xhtml:link rel=\"alternate\" hreflang=\"en\" href=\"https://saptrans.net{path}?lang=en\"/></url>"
            )
        })
        .collect::<String>();
    HttpResponse::Ok()
        .content_type("application/xml; charset=utf-8")
        .body(format!("<?xml version=\"1.0\" encoding=\"UTF-8\"?><urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\" xmlns:xhtml=\"http://www.w3.org/1999/xhtml\">{entries}</urlset>"))
}

async fn dev_version(dev: web::Data<DevInfo>) -> impl Responder {
    HttpResponse::Ok()
        .insert_header(("Cache-Control", "no-store"))
        .body(dev.generation.clone())
}

async fn legacy_media(filename: web::Path<String>) -> impl Responder {
    let filename = filename.into_inner();
    if !filename
        .chars()
        .all(|ch| ch.is_ascii_alphanumeric() || ch == '.' || ch == '-')
    {
        return HttpResponse::NotFound().finish();
    }
    let media_type = match filename.rsplit_once('.').map(|(_, suffix)| suffix) {
        Some("jpg" | "jpeg") => "image/jpeg",
        Some("png") => "image/png",
        Some("webp") => "image/webp",
        Some("avif") => "image/avif",
        Some("gif") => "image/gif",
        Some("svg") => "image/svg+xml",
        _ => return HttpResponse::NotFound().finish(),
    };
    match std::fs::read(format!("public/media/legacy/{filename}")) {
        Ok(bytes) => HttpResponse::Ok()
            .content_type(media_type)
            .insert_header(("Cache-Control", "public, max-age=31536000, immutable"))
            .body(bytes),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

async fn editorial_media(filename: web::Path<String>) -> impl Responder {
    let filename = filename.into_inner();
    let media_type = match filename.rsplit_once('.').map(|(_, suffix)| suffix) {
        Some("jpg") => "image/jpeg",
        Some("png") => "image/png",
        _ => return HttpResponse::NotFound().finish(),
    };
    if !filename
        .chars()
        .all(|ch| ch.is_ascii_alphanumeric() || ch == '.' || ch == '-')
    {
        return HttpResponse::NotFound().finish();
    }
    match std::fs::read(format!("public/media/editorial/{filename}")) {
        Ok(bytes) => HttpResponse::Ok()
            .content_type(media_type)
            .insert_header(("Cache-Control", "public, max-age=31536000, immutable"))
            .body(bytes),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

async fn font_asset(filename: web::Path<String>) -> impl Responder {
    let bytes: &'static [u8] = match filename.as_str() {
        "manrope-variable.woff2" => include_bytes!("../../public/fonts/manrope-variable.woff2"),
        "barlow-condensed-500.woff2" => {
            include_bytes!("../../public/fonts/barlow-condensed-500.woff2")
        }
        "barlow-condensed-600.woff2" => {
            include_bytes!("../../public/fonts/barlow-condensed-600.woff2")
        }
        "barlow-condensed-700.woff2" => {
            include_bytes!("../../public/fonts/barlow-condensed-700.woff2")
        }
        "barlow-condensed-800.woff2" => {
            include_bytes!("../../public/fonts/barlow-condensed-800.woff2")
        }
        "fira-sans-regular.woff2" => include_bytes!("../../public/fonts/fira-sans-regular.woff2"),
        "fira-sans-medium.woff2" => include_bytes!("../../public/fonts/fira-sans-medium.woff2"),
        "fira-sans-semibold.woff2" => include_bytes!("../../public/fonts/fira-sans-semibold.woff2"),
        _ => return HttpResponse::NotFound().finish(),
    };
    HttpResponse::Ok()
        .content_type("font/woff2")
        .insert_header(("Cache-Control", "public, max-age=31536000, immutable"))
        .body(bytes)
}

async fn partner_logo(filename: web::Path<String>) -> impl Responder {
    let bytes: &'static [u8] = match filename.as_str() {
        "tractafric-equipment.png" => {
            include_bytes!("../../public/brand/partners/tractafric-equipment.png")
        }
        "tractafric-motors.png" => {
            include_bytes!("../../public/brand/partners/tractafric-motors.png")
        }
        "routdaf.png" => include_bytes!("../../public/brand/partners/routdaf.png"),
        "siencam.png" => include_bytes!("../../public/brand/partners/siencam.png"),
        "razel.png" => include_bytes!("../../public/brand/partners/razel.png"),
        "wijma.png" => include_bytes!("../../public/brand/partners/wijma.png"),
        "sogea-satom.png" => include_bytes!("../../public/brand/partners/sogea-satom.png"),
        "guinness.png" => include_bytes!("../../public/brand/partners/guinness.png"),
        "sefeccam.png" => include_bytes!("../../public/brand/partners/sefeccam.png"),
        "boiscam.png" => include_bytes!("../../public/brand/partners/boiscam.png"),
        "arab-contractors.png" => {
            include_bytes!("../../public/brand/partners/arab-contractors.png")
        }
        "alpicam.png" => include_bytes!("../../public/brand/partners/alpicam.png"),
        _ => return HttpResponse::NotFound().finish(),
    };
    HttpResponse::Ok().content_type("image/png").body(bytes)
}

async fn page(req: HttpRequest, dev: web::Data<DevInfo>) -> impl Responder {
    if let Some(destination) = Page::legacy_destination(req.path()) {
        let language = Language::from_query(req.query_string());
        let location = if language == Language::En {
            format!("{destination}?lang=en")
        } else {
            destination.to_string()
        };
        return HttpResponse::PermanentRedirect()
            .insert_header(("Location", location))
            .finish();
    }
    let Some(page) = Page::from_path(req.path()) else {
        return HttpResponse::NotFound().body("Page introuvable");
    };
    let language = Language::from_query(req.query_string());
    let search_query = web::Query::<SearchParams>::from_query(req.query_string())
        .ok()
        .and_then(|params| params.q.clone())
        .unwrap_or_default()
        .chars()
        .take(100)
        .collect::<String>();
    let reference = web::Query::<SearchParams>::from_query(req.query_string())
        .ok()
        .and_then(|params| params.reference.clone())
        .filter(|value| {
            value.len() <= 40 && value.chars().all(|c| c.is_ascii_alphanumeric() || c == '-')
        });
    let selected_service = web::Query::<SearchParams>::from_query(req.query_string())
        .ok()
        .and_then(|params| params.service.clone())
        .filter(|value| {
            matches!(
                value.as_str(),
                "transport" | "engins" | "autorelax" | "immobilier"
            )
        })
        .unwrap_or_default();
    let content =
        view! { <Site page=page language=language query=search_query reference=reference.unwrap_or_default() selected_service=selected_service /> }
            .to_html();
    let reload_script = if dev.enabled {
        format!(
            "<script>const build=\"{}\";setInterval(async()=>{{try{{const r=await fetch('/__dev/version',{{cache:'no-store'}});if(r.ok&&(await r.text())!==build)location.reload()}}catch{{}}}},800)</script>",
            dev.generation
        )
    } else {
        String::new()
    };
    let html = format!(
        "<!doctype html><html lang=\"{}\"><head><meta charset=\"utf-8\"><meta name=\"viewport\" content=\"width=device-width,initial-scale=1\"><meta name=\"description\" content=\"{}\"><meta property=\"og:type\" content=\"website\"><meta property=\"og:site_name\" content=\"SAPTRANS\"><meta property=\"og:title\" content=\"{}\"><meta property=\"og:description\" content=\"{}\"><link rel=\"canonical\" href=\"https://saptrans.net{}\"><link rel=\"alternate\" hreflang=\"fr\" href=\"https://saptrans.net{}?lang=fr\"><link rel=\"alternate\" hreflang=\"en\" href=\"https://saptrans.net{}?lang=en\"><title>{}</title><script>let savedTheme=null;try{{savedTheme=localStorage.getItem('saptrans-theme')}}catch{{}}if(savedTheme==='dark'||savedTheme==='light')document.documentElement.setAttribute('data-theme',savedTheme);else if(matchMedia('(prefers-color-scheme: dark)').matches)document.documentElement.setAttribute('data-theme','dark')</script><link rel=\"preconnect\" href=\"https://saptrans.net\"><link rel=\"preconnect\" href=\"https://images.pexels.com\" crossorigin><link rel=\"stylesheet\" href=\"/style.css\"></head><body>{content}<script>{}</script>{reload_script}</body></html>",
        language.code(),
        page.description(language),
        page.title(language),
        page.description(language),
        page.path(),
        page.path(),
        page.path(),
        page.title(language),
        include_str!("../../public/theme.js")
    );
    let cache = if dev.enabled {
        "no-store"
    } else {
        "public, max-age=60, s-maxage=3600"
    };
    HttpResponse::Ok()
        .insert_header(("Cache-Control", cache))
        .content_type("text/html; charset=utf-8")
        .body(html)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL est requis");
    let key_hex = std::env::var("SAPTRANS_SESSION_KEY_HEX")
        .expect("SAPTRANS_SESSION_KEY_HEX est requis (openssl rand -hex 64)");
    let key_bytes = hex::decode(key_hex).expect("clé de session hexadécimale invalide");
    assert!(
        key_bytes.len() >= 64,
        "la clé de session doit contenir au moins 64 octets"
    );
    let session_key = Key::from(&key_bytes);
    let cookie_secure =
        std::env::var("SAPTRANS_COOKIE_SECURE").map_or(true, |value| value != "false");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("connexion PostgreSQL impossible");
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("migration impossible");
    let addr = std::env::var("LISTEN_ADDR").unwrap_or_else(|_| "0.0.0.0:3000".into());
    let dev = DevInfo {
        enabled: std::env::var("DEV_RELOAD").is_ok_and(|value| value == "1"),
        generation: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("horloge système invalide")
            .as_millis()
            .to_string(),
    };
    tracing::info!(%addr, "SAPTRANS démarré");
    HttpServer::new(move || {
        App::new()
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), session_key.clone())
                    .cookie_name("saptrans_session".into())
                    .cookie_secure(cookie_secure)
                    .cookie_same_site(SameSite::Strict)
                    .cookie_http_only(true)
                    .build(),
            )
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(dev.clone()))
            .route("/health", web::get().to(health))
            .route("/robots.txt", web::get().to(robots))
            .route("/sitemap.xml", web::get().to(sitemap))
            .route("/__dev/version", web::get().to(dev_version))
            .route("/media/legacy/{filename}", web::get().to(legacy_media))
            .route(
                "/media/editorial/{filename}",
                web::get().to(editorial_media),
            )
            .route(
                "/style.css",
                web::get().to(|| async {
                    HttpResponse::Ok()
                        .content_type("text/css; charset=utf-8")
                        .insert_header(("Cache-Control", "no-store"))
                        .body(include_str!("../../public/style.css"))
                }),
            )
            .route(
                "/logo.svg",
                web::get().to(|| async {
                    HttpResponse::Ok()
                        .content_type("image/svg+xml")
                        .body(include_str!("../../public/logo.svg"))
                }),
            )
            .route(
                "/brand/logo-white.png",
                web::get().to(|| async {
                    HttpResponse::Ok()
                        .content_type("image/png")
                        .insert_header(("Cache-Control", "public, max-age=86400"))
                        .body(include_bytes!("../../public/brand/logo-white.png").as_slice())
                }),
            )
            .route(
                "/brand/logo-dark.png",
                web::get().to(|| async {
                    HttpResponse::Ok()
                        .content_type("image/png")
                        .insert_header(("Cache-Control", "public, max-age=86400"))
                        .body(include_bytes!("../../public/brand/logo-dark.png").as_slice())
                }),
            )
            .route("/fonts/{filename}", web::get().to(font_asset))
            .route("/brand/partners/{filename}", web::get().to(partner_logo))
            .service(web::scope("/api").configure(saptrans::contact::routes))
            .configure(saptrans::contact::form_routes)
            .configure(saptrans::admin::routes)
            .default_service(web::get().to(page))
    })
    .bind(addr)?
    .run()
    .await
}
