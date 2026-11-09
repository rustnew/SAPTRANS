use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, web};
use leptos::prelude::*;
use saptrans::{Page, Site};
use sqlx::postgres::PgPoolOptions;
use tracing_subscriber::EnvFilter;

async fn health() -> impl Responder {
    HttpResponse::Ok().body("ok")
}

async fn page(req: HttpRequest) -> impl Responder {
    let Some(page) = Page::from_path(req.path()) else {
        return HttpResponse::NotFound().body("Page introuvable");
    };
    let content = view! { <Site page=page /> }.to_html();
    let html = format!(
        "<!doctype html><html lang=\"fr\"><head><meta charset=\"utf-8\"><meta name=\"viewport\" content=\"width=device-width,initial-scale=1\"><meta name=\"description\" content=\"SAPTRANS : transport et logistique, location d’engins et contrôle technique au Cameroun.\"><title>{}</title><link rel=\"stylesheet\" href=\"/style.css\"></head><body>{content}</body></html>",
        page.title()
    );
    HttpResponse::Ok()
        .insert_header(("Cache-Control", "public, max-age=60, s-maxage=3600"))
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
    tracing::info!(%addr, "SAPTRANS démarré");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/health", web::get().to(health))
            .route(
                "/style.css",
                web::get().to(|| async {
                    HttpResponse::Ok()
                        .content_type("text/css; charset=utf-8")
                        .body(include_str!("../../public/style.css"))
                }),
            )
            .service(web::scope("/api").configure(saptrans::contact::routes))
            .configure(saptrans::contact::form_routes)
            .default_service(web::get().to(page))
    })
    .bind(addr)?
    .run()
    .await
}
