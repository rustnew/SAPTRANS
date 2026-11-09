use actix_web::{HttpResponse, web};
use lettre::{AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct ContactRequest {
    #[validate(length(min = 2, max = 120))]
    pub name: String,
    #[validate(email)]
    pub email: String,
    pub service: String,
    #[validate(length(min = 10, max = 4000))]
    pub message: String,
}

#[derive(Serialize)]
struct ApiReply {
    message: &'static str,
}

async fn submit(pool: web::Data<PgPool>, payload: web::Json<ContactRequest>) -> HttpResponse {
    if !valid(&payload) {
        return HttpResponse::BadRequest().json(ApiReply {
            message: "Champs invalides",
        });
    }
    let result = save(pool.get_ref(), &payload).await;
    if result.is_err() {
        tracing::error!("Enregistrement du contact impossible");
        return HttpResponse::InternalServerError().json(ApiReply {
            message: "Erreur temporaire",
        });
    }
    notify(&payload).await;
    HttpResponse::Accepted().json(ApiReply {
        message: "Message reçu",
    })
}

fn valid(payload: &ContactRequest) -> bool {
    payload.validate().is_ok()
        && matches!(
            payload.service.as_str(),
            "transport" | "engins" | "autorelax" | "immobilier"
        )
}

async fn save(pool: &PgPool, payload: &ContactRequest) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO contact_messages (name, email, service, message) VALUES ($1, $2, $3, $4)",
    )
    .bind(payload.name.trim())
    .bind(payload.email.trim())
    .bind(payload.service.as_str())
    .bind(payload.message.trim())
    .execute(pool)
    .await?;
    Ok(())
}

async fn notify(payload: &ContactRequest) {
    if let (Ok(host), Ok(recipient), Ok(sender)) = (
        std::env::var("SMTP_HOST"),
        std::env::var("CONTACT_RECIPIENT"),
        std::env::var("SMTP_FROM"),
    ) {
        if let (Ok(from), Ok(to)) = (sender.parse(), recipient.parse()) {
            if let Ok(mail) = Message::builder()
                .from(from)
                .to(to)
                .subject("Nouveau contact SAPTRANS")
                .body(format!(
                    "Nom : {}\nCourriel : {}\nActivité : {}\n\n{}",
                    payload.name, payload.email, payload.service, payload.message
                ))
            {
                let transport = AsyncSmtpTransport::<Tokio1Executor>::relay(&host);
                if let Ok(transport) = transport {
                    if let Err(error) = transport.build().send(mail).await {
                        tracing::warn!(%error, "Notification SMTP non envoyée");
                    }
                }
            }
        }
    }
}

async fn submit_form(pool: web::Data<PgPool>, payload: web::Form<ContactRequest>) -> HttpResponse {
    if !valid(&payload) {
        return HttpResponse::SeeOther()
            .insert_header(("Location", "/contact/erreur"))
            .finish();
    }
    if save(pool.get_ref(), &payload).await.is_err() {
        return HttpResponse::ServiceUnavailable()
            .content_type("text/plain; charset=utf-8")
            .body("Service temporairement indisponible. Réessayez plus tard.");
    }
    notify(&payload).await;
    HttpResponse::SeeOther()
        .insert_header(("Location", "/contact/merci"))
        .finish()
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/contact", web::post().to(submit));
}

pub fn form_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/contact", web::post().to(submit_form));
}
