use actix_session::Session;
use actix_web::{HttpResponse, web};
use argon2::{
    Argon2,
    password_hash::{PasswordVerifier, phc::PasswordHash},
};
use leptos::prelude::*;
use rand::RngCore;
use serde::Deserialize;
use sqlx::{FromRow, PgPool};

#[derive(Clone, FromRow)]
pub(super) struct Member {
    pub id: i64,
    pub display_name: String,
    pub role: String,
    pub center: Option<String>,
}

#[derive(FromRow)]
struct LoginAccount {
    id: i64,
    password_hash: String,
}

#[derive(Deserialize)]
pub(super) struct LoginForm {
    email: String,
    password: String,
    csrf: String,
}

#[derive(Deserialize)]
pub(super) struct CsrfForm {
    pub csrf: String,
}

pub(super) fn new_token() -> String {
    let mut bytes = [0_u8; 32];
    rand::rngs::OsRng.fill_bytes(&mut bytes);
    hex::encode(bytes)
}

pub(super) fn csrf_token(session: &Session) -> Option<String> {
    if let Ok(Some(token)) = session.get::<String>("csrf") {
        return Some(token);
    }
    let token = new_token();
    session.insert("csrf", &token).ok()?;
    Some(token)
}

pub(super) fn valid_csrf(session: &Session, submitted: &str) -> bool {
    session
        .get::<String>("csrf")
        .ok()
        .flatten()
        .is_some_and(|expected| expected.len() == 64 && expected == submitted)
}

pub(super) async fn current_member(session: &Session, pool: &PgPool) -> Option<Member> {
    let id = session.get::<i64>("member_id").ok().flatten()?;
    sqlx::query_as::<_, Member>(
        "SELECT id, display_name, role, center FROM admin_users WHERE id = $1 AND active = TRUE",
    )
    .bind(id)
    .fetch_optional(pool)
    .await
    .ok()
    .flatten()
}

pub(super) fn can_read(member: &Member, kind: &str, center: Option<&str>) -> bool {
    match member.role.as_str() {
        "admin" | "commercial" | "finance" => true,
        "transport" => kind == "transport",
        "equipment" => kind == "engins",
        "inspection" => {
            kind == "autorelax"
                && member
                    .center
                    .as_deref()
                    .is_some_and(|own| Some(own) == center)
        }
        _ => false,
    }
}

pub(super) fn can_edit(member: &Member, kind: &str, center: Option<&str>) -> bool {
    member.role != "finance" && can_read(member, kind, center)
}

pub(super) fn private_response(body: String, title: &str) -> HttpResponse {
    let html = format!(
        "<!doctype html><html lang=\"fr\"><head><meta charset=\"utf-8\"><meta name=\"viewport\" content=\"width=device-width,initial-scale=1\"><meta name=\"robots\" content=\"noindex,nofollow\"><title>{title} — SAPTRANS</title><link rel=\"stylesheet\" href=\"/style.css\"></head><body>{body}</body></html>"
    );
    HttpResponse::Ok()
        .insert_header(("Cache-Control", "no-store, private"))
        .insert_header(("X-Content-Type-Options", "nosniff"))
        .insert_header(("Content-Security-Policy", "default-src 'self'; img-src 'self'; style-src 'self'; form-action 'self'; frame-ancestors 'none'; base-uri 'none'"))
        .content_type("text/html; charset=utf-8")
        .body(html)
}

pub(super) async fn login_page(session: Session) -> HttpResponse {
    render_login(&session, false)
}

fn render_login(session: &Session, failed: bool) -> HttpResponse {
    let Some(csrf) = csrf_token(session) else {
        return HttpResponse::InternalServerError().finish();
    };
    let body = view! {
        <main class="member-login"><div class="member-login-card">
            <a class="member-brand" href="/">"SAPTRANS"</a>
            <p class="eyebrow">"ESPACE ÉQUIPE"</p>
            <h1>"Bienvenue dans votre espace de travail."</h1>
            <p>"Connectez-vous avec votre compte professionnel."</p>
            {failed.then(|| view! { <p class="member-error" role="alert">"Identifiants invalides ou accès temporairement limité."</p> })}
            <form method="post" action="/espace/connexion">
                <input type="hidden" name="csrf" value=csrf />
                <div class="field"><label for="member-email">"Adresse e-mail"</label><input id="member-email" name="email" type="email" required autocomplete="username" maxlength="320" /></div>
                <div class="field"><label for="member-password">"Mot de passe"</label><input id="member-password" name="password" type="password" required autocomplete="current-password" /></div>
                <button class="button" type="submit">"Se connecter"</button>
            </form>
            <a class="text-link" href="/">"Retour au site public ↗"</a>
        </div></main>
    }.to_html();
    private_response(body, "Connexion")
}

pub(super) async fn login(
    session: Session,
    pool: web::Data<PgPool>,
    form: web::Form<LoginForm>,
) -> HttpResponse {
    if !valid_csrf(&session, &form.csrf) {
        return HttpResponse::Forbidden().finish();
    }
    let email = form.email.trim().to_ascii_lowercase();
    if email.len() > 320 || form.password.len() > 1024 {
        return render_login(&session, true);
    }
    let failures: i64 = sqlx::query_scalar(
        "SELECT count(*) FROM member_login_failures WHERE email = $1 AND attempted_at > now() - interval '15 minutes'",
    )
    .bind(&email)
    .fetch_one(pool.get_ref())
    .await
    .unwrap_or(5);
    if failures >= 5 {
        return render_login(&session, true);
    }
    let account = sqlx::query_as::<_, LoginAccount>(
        "SELECT id, password_hash FROM admin_users WHERE lower(email) = $1 AND active = TRUE",
    )
    .bind(&email)
    .fetch_optional(pool.get_ref())
    .await;
    let Ok(account) = account else {
        return HttpResponse::ServiceUnavailable().finish();
    };
    let valid = if let Some(account) = &account {
        let password = form.password.clone();
        let hash = account.password_hash.clone();
        web::block(move || {
            PasswordHash::new(&hash).ok().is_some_and(|parsed| {
                Argon2::default()
                    .verify_password(password.as_bytes(), &parsed)
                    .is_ok()
            })
        })
        .await
        .unwrap_or(false)
    } else {
        false
    };
    if !valid {
        let _ = sqlx::query("INSERT INTO member_login_failures (email) VALUES ($1)")
            .bind(&email)
            .execute(pool.get_ref())
            .await;
        return render_login(&session, true);
    }
    let Some(account) = account else {
        return render_login(&session, true);
    };
    session.renew();
    if session.insert("member_id", account.id).is_err()
        || session.insert("csrf", new_token()).is_err()
    {
        return HttpResponse::InternalServerError().finish();
    }
    let _ = sqlx::query("DELETE FROM member_login_failures WHERE email = $1")
        .bind(&email)
        .execute(pool.get_ref())
        .await;
    HttpResponse::SeeOther()
        .insert_header(("Location", "/espace"))
        .insert_header(("Cache-Control", "no-store"))
        .finish()
}

pub(super) async fn logout(session: Session, form: web::Form<CsrfForm>) -> HttpResponse {
    if !valid_csrf(&session, &form.csrf) {
        return HttpResponse::Forbidden().finish();
    }
    session.purge();
    HttpResponse::SeeOther()
        .insert_header(("Location", "/espace/connexion"))
        .insert_header(("Cache-Control", "no-store"))
        .finish()
}

#[cfg(test)]
mod tests {
    use super::{Member, can_edit, can_read};

    #[test]
    fn inspection_is_limited_to_its_center_and_service() {
        let member = Member {
            id: 1,
            display_name: "Test".into(),
            role: "inspection".into(),
            center: Some("Douala".into()),
        };
        assert!(can_read(&member, "autorelax", Some("Douala")));
        assert!(!can_read(&member, "autorelax", Some("Yaoundé")));
        assert!(!can_read(&member, "transport", Some("Douala")));
        assert!(!can_edit(&member, "transport", Some("Douala")));
    }
}
