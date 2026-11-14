use actix_web::{HttpRequest, HttpResponse, web};
use lettre::{AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct ContactRequest {
    #[validate(length(min = 2, max = 120))]
    pub name: String,
    #[validate(length(max = 160))]
    pub company: Option<String>,
    #[validate(length(max = 32))]
    pub phone: Option<String>,
    #[validate(email)]
    pub email: String,
    pub service: String,
    #[validate(length(min = 10, max = 4000))]
    pub message: String,
    #[validate(length(max = 160))]
    pub cargo: Option<String>,
    #[validate(length(max = 160))]
    pub origin: Option<String>,
    #[validate(length(max = 160))]
    pub destination: Option<String>,
    #[validate(length(max = 120))]
    pub quantity: Option<String>,
    #[validate(length(max = 80))]
    pub desired_date: Option<String>,
    #[validate(length(max = 160))]
    pub equipment: Option<String>,
    #[serde(default)]
    #[validate(length(max = 200))]
    pub website: Option<String>,
}

#[derive(Serialize)]
struct ApiReply {
    message: &'static str,
    reference: Option<String>,
}

async fn submit(
    request: HttpRequest,
    pool: web::Data<PgPool>,
    payload: web::Json<ContactRequest>,
) -> HttpResponse {
    let english = request
        .query_string()
        .split('&')
        .any(|part| part.eq_ignore_ascii_case("lang=en"));
    if !valid(&payload) {
        return HttpResponse::BadRequest().json(ApiReply {
            message: if english {
                "Invalid fields"
            } else {
                "Champs invalides"
            },
            reference: None,
        });
    }
    let reference = match save(pool.get_ref(), &payload).await {
        Ok(reference) => reference,
        Err(error) => {
            tracing::error!(%error, "Enregistrement du contact impossible");
            return HttpResponse::InternalServerError().json(ApiReply {
                message: if english {
                    "Temporary error"
                } else {
                    "Erreur temporaire"
                },
                reference: None,
            });
        }
    };
    notify(&payload).await;
    HttpResponse::Accepted().json(ApiReply {
        message: if english {
            "Message received"
        } else {
            "Message reçu"
        },
        reference: Some(reference),
    })
}

fn valid(payload: &ContactRequest) -> bool {
    payload.validate().is_ok()
        && matches!(
            payload.service.as_str(),
            "transport" | "engins" | "autorelax" | "immobilier"
        )
        && clean_option(payload.website.as_deref()).is_none()
}

async fn save(pool: &PgPool, payload: &ContactRequest) -> Result<String, sqlx::Error> {
    let mut transaction = pool.begin().await?;
    let details = serde_json::json!({
        "company": clean_option(payload.company.as_deref()),
        "phone": clean_option(payload.phone.as_deref()),
        "cargo": clean_option(payload.cargo.as_deref()),
        "origin": clean_option(payload.origin.as_deref()),
        "destination": clean_option(payload.destination.as_deref()),
        "quantity": clean_option(payload.quantity.as_deref()),
        "desired_date": clean_option(payload.desired_date.as_deref()),
        "equipment": clean_option(payload.equipment.as_deref()),
    });
    let (id,): (i64,) = sqlx::query_as(
        "INSERT INTO contact_messages (name, email, service, message, details) VALUES ($1, $2, $3, $4, $5) RETURNING id",
    )
    .bind(payload.name.trim())
    .bind(payload.email.trim())
    .bind(payload.service.as_str())
    .bind(payload.message.trim())
    .bind(details.to_string())
    .fetch_one(&mut *transaction)
    .await?;
    let reference = format!("SAP-{id:06}");
    let (work_id,): (i64,) = sqlx::query_as(
        "INSERT INTO work_items (source_contact_id, kind, title, customer_name, customer_email, customer_phone, description, details) VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING id",
    )
    .bind(id)
    .bind(&payload.service)
    .bind(format!("Demande {} {reference}", payload.service))
    .bind(payload.name.trim())
    .bind(payload.email.trim())
    .bind(clean_option(payload.phone.as_deref()))
    .bind(payload.message.trim())
    .bind(details)
    .fetch_one(&mut *transaction)
    .await?;
    sqlx::query("INSERT INTO work_events (work_item_id, action, new_status) VALUES ($1, 'demande_recue', 'nouveau')")
        .bind(work_id)
        .execute(&mut *transaction)
        .await?;
    transaction.commit().await?;
    Ok(reference)
}

fn clean_option(value: Option<&str>) -> Option<&str> {
    value.map(str::trim).filter(|value| !value.is_empty())
}

async fn notify(payload: &ContactRequest) {
    let (Ok(host), Ok(recipient), Ok(sender)) = (
        std::env::var("SMTP_HOST"),
        std::env::var("CONTACT_RECIPIENT"),
        std::env::var("SMTP_FROM"),
    ) else {
        return;
    };
    let (Ok(from), Ok(to)) = (sender.parse(), recipient.parse()) else {
        tracing::warn!("Adresses SMTP invalides ; notification non envoyée");
        return;
    };
    let mail = Message::builder()
        .from(from)
        .to(to)
        .subject("Nouveau contact SAPTRANS")
        .body(format!(
            "Nom : {}\nEntreprise : {}\nTéléphone : {}\nCourriel : {}\nActivité : {}\nMarchandise : {}\nDépart : {}\nDestination : {}\nQuantité : {}\nDate souhaitée : {}\nÉquipement : {}\n\n{}",
            payload.name,
            payload.company.as_deref().unwrap_or("—"),
            payload.phone.as_deref().unwrap_or("—"),
            payload.email,
            payload.service,
            payload.cargo.as_deref().unwrap_or("—"),
            payload.origin.as_deref().unwrap_or("—"),
            payload.destination.as_deref().unwrap_or("—"),
            payload.quantity.as_deref().unwrap_or("—"),
            payload.desired_date.as_deref().unwrap_or("—"),
            payload.equipment.as_deref().unwrap_or("—"),
            payload.message
        ));
    let Ok(mail) = mail else {
        tracing::warn!("Message SMTP invalide ; notification non envoyée");
        return;
    };
    let Ok(relay) = AsyncSmtpTransport::<Tokio1Executor>::relay(&host) else {
        tracing::warn!("Serveur SMTP invalide ; notification non envoyée");
        return;
    };
    if let Err(error) = relay.build().send(mail).await {
        tracing::warn!(%error, "Notification SMTP non envoyée");
    }
}

async fn submit_form(
    request: HttpRequest,
    pool: web::Data<PgPool>,
    payload: web::Form<ContactRequest>,
) -> HttpResponse {
    let is_english = request
        .query_string()
        .split('&')
        .any(|part| part.eq_ignore_ascii_case("lang=en"));
    let language_query = if is_english { "?lang=en" } else { "" };
    let language_suffix = if is_english { "&lang=en" } else { "" };
    if !valid(&payload) {
        return HttpResponse::SeeOther()
            .insert_header(("Location", format!("/contact/erreur{language_query}")))
            .finish();
    }
    let reference = match save(pool.get_ref(), &payload).await {
        Ok(reference) => reference,
        Err(error) => {
            tracing::error!(%error, "Enregistrement du formulaire impossible");
            return HttpResponse::ServiceUnavailable()
                .content_type("text/plain; charset=utf-8")
                .body(if is_english {
                    "Service temporarily unavailable. Please try again later."
                } else {
                    "Service temporairement indisponible. Réessayez plus tard."
                });
        }
    };
    notify(&payload).await;
    HttpResponse::SeeOther()
        .insert_header((
            "Location",
            format!("/contact/merci?ref={reference}{language_suffix}"),
        ))
        .finish()
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/contact", web::post().to(submit));
}

pub fn form_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/contact", web::post().to(submit_form));
}
