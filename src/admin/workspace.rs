use super::auth::{
    Member, can_edit, can_read, csrf_token, current_member, private_response, valid_csrf,
};
use actix_session::Session;
use actix_web::{HttpResponse, web};
use leptos::prelude::*;
use serde::Deserialize;
use sqlx::{FromRow, PgPool};

#[derive(FromRow)]
struct Item {
    id: i64,
    kind: String,
    status: String,
    title: String,
    customer_name: String,
    customer_email: Option<String>,
    customer_phone: Option<String>,
    description: String,
    details: String,
    center: Option<String>,
    created_at: String,
}

#[derive(FromRow)]
struct Event {
    action: String,
    note: String,
    old_status: Option<String>,
    new_status: Option<String>,
    actor: Option<String>,
    recorded_at: String,
}

#[derive(Deserialize)]
pub(super) struct Filter {
    kind: Option<String>,
    status: Option<String>,
}

#[derive(Deserialize)]
pub(super) struct NewItem {
    csrf: String,
    kind: String,
    title: String,
    customer_name: String,
    customer_email: String,
    customer_phone: String,
    description: String,
    center: String,
}

#[derive(Deserialize)]
pub(super) struct StatusForm {
    csrf: String,
    status: String,
    note: String,
}

const KINDS: &[(&str, &str)] = &[
    ("transport", "Transport"),
    ("engins", "Location d’engins"),
    ("autorelax", "Contrôle technique AutoRelax"),
    ("immobilier", "Immobilier"),
];
const STATUSES: &[(&str, &str)] = &[
    ("nouveau", "Nouveau"),
    ("qualifie", "Qualifié"),
    ("planifie", "Planifié"),
    ("en_cours", "En cours"),
    ("en_revue", "En revue"),
    ("termine", "Terminé"),
    ("annule", "Annulé"),
];

fn label<'a>(options: &'a [(&str, &str)], value: &str) -> &'a str {
    options
        .iter()
        .find(|(key, _)| *key == value)
        .map_or("—", |(_, name)| name)
}

fn allowed_kind(member: &Member) -> Option<&'static str> {
    match member.role.as_str() {
        "transport" => Some("transport"),
        "equipment" => Some("engins"),
        "inspection" => Some("autorelax"),
        _ => None,
    }
}

fn redirect_login() -> HttpResponse {
    HttpResponse::SeeOther()
        .insert_header(("Location", "/espace/connexion"))
        .insert_header(("Cache-Control", "no-store"))
        .finish()
}

fn shell(member: &Member, csrf: String, title: &str, content: String) -> HttpResponse {
    let name = member.display_name.clone();
    let role = member.role.clone();
    let body = view! {
        <div class="workspace">
            <aside class="workspace-sidebar">
                <a class="workspace-logo" href="/espace">"SAPTRANS"<small>"ESPACE ÉQUIPE"</small></a>
                <nav aria-label="Navigation interne">
                    <a href="/espace">"Vue d’ensemble"</a>
                    <a href="/espace/dossiers">"Tous les dossiers"</a>
                    <a href="/espace/dossiers?kind=transport">"Transport"</a>
                    <a href="/espace/dossiers?kind=engins">"Location d’engins"</a>
                    <a href="/espace/dossiers?kind=autorelax">"AutoRelax"</a>
                    <a href="/espace/dossiers?kind=immobilier">"Immobilier"</a>
                </nav>
                <div class="workspace-user"><strong>{name}</strong><span>{role}</span>
                    <form method="post" action="/espace/deconnexion"><input type="hidden" name="csrf" value=csrf /><button type="submit">"Se déconnecter"</button></form>
                    <a href="/">"Site public ↗"</a>
                </div>
            </aside>
            <div class="workspace-main"><header class="workspace-top"><span>"Espace interne"</span><span>"SAPTRANS • Cameroun"</span></header><main inner_html=content></main></div>
        </div>
    }.to_html();
    private_response(body, title)
}

async fn items(
    pool: &PgPool,
    member: &Member,
    kind: Option<&str>,
    status: Option<&str>,
    limit: i64,
) -> Result<Vec<Item>, sqlx::Error> {
    if member.role == "viewer" {
        return Ok(Vec::new());
    }
    sqlx::query_as::<_, Item>(
        "SELECT id, kind, status, title, customer_name, customer_email, customer_phone, description, details::text AS details, center, to_char(created_at AT TIME ZONE 'Africa/Douala', 'DD/MM/YYYY HH24:MI') AS created_at FROM work_items WHERE ($1::text IS NULL OR kind = $1) AND ($2::text IS NULL OR status = $2) AND ($3::text IS NULL OR kind = $3) AND ($4::text IS NULL OR center = $4) ORDER BY created_at DESC LIMIT $5"
    ).bind(kind).bind(status).bind(allowed_kind(member)).bind(if member.role == "inspection" { member.center.as_deref() } else { None }).bind(limit).fetch_all(pool).await
}

fn can_open(member: &Member) -> bool {
    matches!(
        member.role.as_str(),
        "admin" | "commercial" | "transport" | "equipment" | "inspection"
    )
}

pub(super) async fn dashboard(session: Session, pool: web::Data<PgPool>) -> HttpResponse {
    let Some(member) = current_member(&session, &pool).await else {
        return redirect_login();
    };
    let Some(csrf) = csrf_token(&session) else {
        return HttpResponse::InternalServerError().finish();
    };
    let Ok(recent) = items(&pool, &member, None, None, 8).await else {
        return HttpResponse::ServiceUnavailable().finish();
    };
    let total = recent.len();
    let open = recent
        .iter()
        .filter(|item| item.status != "termine" && item.status != "annule")
        .count();
    let recent_html = if recent.is_empty() {
        view! { <p>"Aucun dossier pour le moment."</p> }.to_html()
    } else {
        item_list(&recent)
    };
    let content = view! {
        <section class="workspace-heading"><p class="eyebrow">"VUE D’ENSEMBLE"</p><h1>"Bonjour, "{member.display_name.clone()}"."</h1><p>"Retrouvez les demandes et missions à traiter par votre équipe."</p></section>
        <section class="workspace-stats"><div><strong>{open}</strong><span>"Dossiers ouverts parmi les 8 plus récents"</span></div><div><strong>{total}</strong><span>"Dossiers récents accessibles"</span></div></section>
        <section class="workspace-panel"><div class="workspace-section-head"><h2>"Activité récente"</h2><a href="/espace/dossiers">"Voir tous les dossiers →"</a></div>
            <div inner_html=recent_html></div>
        </section>
    }.to_html();
    shell(&member, csrf, "Vue d’ensemble", content)
}

fn item_list(items: &[Item]) -> String {
    view! { <div class="workspace-list">{items.iter().map(|item| {
        let href = format!("/espace/dossiers/{}", item.id);
        let kind = label(KINDS, &item.kind);
        let status = label(STATUSES, &item.status);
        let title = item.title.clone();
        let name = item.customer_name.clone();
        let date = item.created_at.clone();
        view! { <a class="workspace-list-row" href=href><span><small>{kind}</small><strong>{title}</strong><em>{name}</em></span><span class="workspace-list-meta"><span class="workspace-pill">{status}</span><small>{date}</small></span></a> }
    }).collect_view()}</div> }.to_html()
}

pub(super) async fn list(
    session: Session,
    pool: web::Data<PgPool>,
    query: web::Query<Filter>,
) -> HttpResponse {
    let Some(member) = current_member(&session, &pool).await else {
        return redirect_login();
    };
    let Some(csrf) = csrf_token(&session) else {
        return HttpResponse::InternalServerError().finish();
    };
    let kind = query
        .kind
        .as_deref()
        .filter(|value| KINDS.iter().any(|(key, _)| *key == *value));
    let status = query
        .status
        .as_deref()
        .filter(|value| STATUSES.iter().any(|(key, _)| *key == *value));
    let Ok(rows) = items(&pool, &member, kind, status, 100).await else {
        return HttpResponse::ServiceUnavailable().finish();
    };
    let rows_html = if rows.is_empty() {
        view! { <p>"Aucun dossier correspondant."</p> }.to_html()
    } else {
        item_list(&rows)
    };
    let content = view! {
        <section class="workspace-heading"><p class="eyebrow">"SUIVI"</p><h1>"Dossiers"</h1><p>"Demandes reçues, interventions et missions suivies au même endroit."</p></section>
        <section class="workspace-panel"><div class="workspace-section-head"><h2>{format!("{} dossiers", rows.len())}</h2>{can_open(&member).then(|| view! { <a class="workspace-action" href="/espace/dossiers/nouveau">"+ Nouveau dossier"</a> })}</div>
            <form class="workspace-filters" method="get" action="/espace/dossiers"><label>"Activité"<select name="kind"><option value="">"Toutes"</option>{KINDS.iter().map(|(key, value)| view! { <option value=*key selected=kind==Some(*key)>{*value}</option> }).collect_view()}</select></label><label>"Statut"<select name="status"><option value="">"Tous"</option>{STATUSES.iter().map(|(key, value)| view! { <option value=*key selected=status==Some(*key)>{*value}</option> }).collect_view()}</select></label><button type="submit">"Filtrer"</button></form>
            <div inner_html=rows_html></div>
        </section>
    }.to_html();
    shell(&member, csrf, "Dossiers", content)
}

pub(super) async fn new_page(session: Session, pool: web::Data<PgPool>) -> HttpResponse {
    let Some(member) = current_member(&session, &pool).await else {
        return redirect_login();
    };
    if !can_open(&member) {
        return HttpResponse::Forbidden().finish();
    }
    let Some(csrf) = csrf_token(&session) else {
        return HttpResponse::InternalServerError().finish();
    };
    let permitted: Vec<_> = KINDS
        .iter()
        .filter(|(kind, _)| can_edit(&member, kind, member.center.as_deref()))
        .collect();
    let content = view! {
        <section class="workspace-heading"><a href="/espace/dossiers">"← Retour aux dossiers"</a><h1>"Nouveau dossier"</h1><p>"Enregistrez une demande ou une mission. Les changements resteront traçables."</p></section>
        <section class="workspace-panel"><form class="workspace-form" method="post" action="/espace/dossiers"><input type="hidden" name="csrf" value=csrf.clone() />
            <label>"Activité"<select name="kind" required>{permitted.iter().map(|(key, value)| view! { <option value=*key>{*value}</option> }).collect_view()}</select></label>
            <label>"Titre du dossier"<input name="title" required maxlength="180" /></label>
            <label>"Nom du client"<input name="customer_name" required maxlength="120" /></label>
            <div class="workspace-form-grid"><label>"E-mail"<input name="customer_email" type="email" maxlength="320" /></label><label>"Téléphone"<input name="customer_phone" type="tel" maxlength="32" /></label></div>
            <label>"Centre ou agence"<input name="center" maxlength="40" value=member.center.clone().unwrap_or_default() readonly=member.role=="inspection" /></label>
            <label>"Description et informations utiles"<textarea name="description" rows="6" maxlength="10000"></textarea></label>
            <button class="workspace-action" type="submit">"Créer le dossier"</button>
        </form></section>
    }.to_html();
    shell(&member, csrf, "Nouveau dossier", content)
}

pub(super) async fn create(
    session: Session,
    pool: web::Data<PgPool>,
    form: web::Form<NewItem>,
) -> HttpResponse {
    let Some(member) = current_member(&session, &pool).await else {
        return HttpResponse::Unauthorized().finish();
    };
    if !valid_csrf(&session, &form.csrf) {
        return HttpResponse::Forbidden().finish();
    }
    let kind = form.kind.as_str();
    let center = if member.role == "inspection" {
        member.center.as_deref().unwrap_or("")
    } else {
        form.center.trim()
    };
    if !can_edit(&member, kind, Some(center))
        || !KINDS.iter().any(|(key, _)| *key == kind)
        || form.title.trim().is_empty()
        || form.title.len() > 180
        || form.customer_name.trim().is_empty()
        || form.customer_name.len() > 120
        || form.customer_email.len() > 320
        || form.customer_phone.len() > 32
        || form.description.len() > 10000
        || center.len() > 40
    {
        return HttpResponse::BadRequest().finish();
    }
    let Ok(mut tx) = pool.begin().await else {
        return HttpResponse::ServiceUnavailable().finish();
    };
    let result: Result<i64, _> = sqlx::query_scalar("INSERT INTO work_items (kind, title, customer_name, customer_email, customer_phone, description, center, created_by) VALUES ($1, $2, $3, nullif($4, ''), nullif($5, ''), $6, nullif($7, ''), $8) RETURNING id")
        .bind(kind).bind(form.title.trim()).bind(form.customer_name.trim()).bind(form.customer_email.trim()).bind(form.customer_phone.trim()).bind(form.description.trim()).bind(center).bind(member.id).fetch_one(&mut *tx).await;
    let Ok(id) = result else {
        return HttpResponse::ServiceUnavailable().finish();
    };
    if sqlx::query("INSERT INTO work_events (work_item_id, actor_id, action, note, new_status) VALUES ($1, $2, 'creation', 'Dossier créé par un membre', 'nouveau')")
        .bind(id).bind(member.id).execute(&mut *tx).await.is_err() || tx.commit().await.is_err() { return HttpResponse::ServiceUnavailable().finish() }
    HttpResponse::SeeOther()
        .insert_header(("Location", format!("/espace/dossiers/{id}")))
        .insert_header(("Cache-Control", "no-store"))
        .finish()
}

async fn one(pool: &PgPool, id: i64) -> Result<Option<Item>, sqlx::Error> {
    sqlx::query_as::<_, Item>("SELECT id, kind, status, title, customer_name, customer_email, customer_phone, description, details::text AS details, center, to_char(created_at AT TIME ZONE 'Africa/Douala', 'DD/MM/YYYY HH24:MI') AS created_at FROM work_items WHERE id = $1")
        .bind(id).fetch_optional(pool).await
}

pub(super) async fn detail(
    session: Session,
    pool: web::Data<PgPool>,
    path: web::Path<i64>,
) -> HttpResponse {
    let Some(member) = current_member(&session, &pool).await else {
        return redirect_login();
    };
    let Some(csrf) = csrf_token(&session) else {
        return HttpResponse::InternalServerError().finish();
    };
    let Ok(Some(item)) = one(&pool, path.into_inner()).await else {
        return HttpResponse::NotFound().finish();
    };
    if !can_read(&member, &item.kind, item.center.as_deref()) {
        return HttpResponse::Forbidden().finish();
    }
    let Ok(events) = sqlx::query_as::<_, Event>("SELECT e.action, e.note, e.old_status, e.new_status, u.display_name AS actor, to_char(e.recorded_at AT TIME ZONE 'Africa/Douala', 'DD/MM/YYYY HH24:MI') AS recorded_at FROM work_events e LEFT JOIN admin_users u ON u.id = e.actor_id WHERE e.work_item_id = $1 ORDER BY e.recorded_at DESC, e.id DESC")
        .bind(item.id).fetch_all(pool.get_ref()).await else { return HttpResponse::ServiceUnavailable().finish() };
    let status_options: Vec<_> = STATUSES
        .iter()
        .filter(|(next, _)| allowed_transition(&item.status, next))
        .collect();
    let action = format!("/espace/dossiers/{}/statut", item.id);
    let kind_label = label(KINDS, &item.kind);
    let status_label = label(STATUSES, &item.status);
    let fields = detail_fields(&item.details);
    let form_html = if can_edit(&member, &item.kind, item.center.as_deref())
        && !status_options.is_empty()
    {
        view! { <form class="workspace-form" method="post" action=action><input type="hidden" name="csrf" value=csrf.clone() /><label>"Passer au statut"<select name="status">{status_options.iter().map(|(key, value)| view! { <option value=*key>{*value}</option> }).collect_view()}</select></label><label>"Compte rendu du changement"<textarea name="note" rows="3" maxlength="2000" required></textarea></label><button class="workspace-action" type="submit">"Enregistrer le changement"</button></form> }.to_html()
    } else {
        String::new()
    };
    let content = view! {
        <section class="workspace-heading"><a href="/espace/dossiers">"← Retour aux dossiers"</a><p class="eyebrow">{kind_label}" · Dossier "{item.id}</p><h1>{item.title.clone()}</h1><span class="workspace-pill">{status_label}</span></section>
        <div class="workspace-columns"><section class="workspace-panel"><h2>"Informations"</h2><dl class="workspace-details"><dt>"Client"</dt><dd>{item.customer_name.clone()}</dd><dt>"E-mail"</dt><dd>{item.customer_email.clone().unwrap_or_else(|| "—".into())}</dd><dt>"Téléphone"</dt><dd>{item.customer_phone.clone().unwrap_or_else(|| "—".into())}</dd><dt>"Centre"</dt><dd>{item.center.clone().unwrap_or_else(|| "—".into())}</dd><dt>"Ouvert le"</dt><dd>{item.created_at.clone()}</dd>{fields.iter().map(|(key, value)| view! { <dt>{*key}</dt><dd>{value.clone()}</dd> }).collect_view()}</dl><h3>"Description"</h3><p class="workspace-description">{item.description.clone()}</p></section>
            <section class="workspace-panel"><h2>"Suivi du dossier"</h2>
                <div inner_html=form_html></div>
                <h3>"Historique"</h3><ol class="workspace-timeline">{events.iter().map(|event| { let action = event.action.clone(); let note = event.note.clone(); let date = event.recorded_at.clone(); let actor = event.actor.clone().unwrap_or_else(|| "Site public".into()); let change = match (&event.old_status, &event.new_status) { (Some(old), Some(new)) => format!("{} → {}", label(STATUSES, old), label(STATUSES, new)), (_, Some(new)) => label(STATUSES, new).to_string(), _ => String::new() }; view! { <li><strong>{action}</strong><span>{change}</span><p>{note}</p><small>{date}" · "{actor}</small></li> } }).collect_view()}</ol>
            </section></div>
    }.to_html();
    shell(&member, csrf, "Dossier", content)
}

fn allowed_transition(current: &str, next: &str) -> bool {
    matches!(
        (current, next),
        ("nouveau", "qualifie" | "annule")
            | ("qualifie", "planifie" | "annule")
            | ("planifie", "en_cours" | "annule")
            | ("en_cours", "en_revue" | "termine" | "annule")
            | ("en_revue", "en_cours" | "termine" | "annule")
    )
}

fn detail_fields(details: &str) -> Vec<(&'static str, String)> {
    let Ok(value) = serde_json::from_str::<serde_json::Value>(details) else {
        return Vec::new();
    };
    let value = value
        .get("contact_details")
        .and_then(|value| value.as_str())
        .and_then(|text| serde_json::from_str::<serde_json::Value>(text).ok())
        .unwrap_or(value);
    [
        ("Entreprise", "company"),
        ("Marchandise", "cargo"),
        ("Départ", "origin"),
        ("Destination", "destination"),
        ("Quantité", "quantity"),
        ("Date souhaitée", "desired_date"),
        ("Engin", "equipment"),
    ]
    .into_iter()
    .filter_map(|(label, key)| {
        value
            .get(key)
            .and_then(|value| value.as_str())
            .filter(|value| !value.is_empty())
            .map(|value| (label, value.to_string()))
    })
    .collect()
}

pub(super) async fn change_status(
    session: Session,
    pool: web::Data<PgPool>,
    path: web::Path<i64>,
    form: web::Form<StatusForm>,
) -> HttpResponse {
    let Some(member) = current_member(&session, &pool).await else {
        return HttpResponse::Unauthorized().finish();
    };
    if !valid_csrf(&session, &form.csrf) {
        return HttpResponse::Forbidden().finish();
    }
    if form.note.trim().is_empty() || form.note.len() > 2000 {
        return HttpResponse::BadRequest().finish();
    }
    let id = path.into_inner();
    let Ok(mut tx) = pool.begin().await else {
        return HttpResponse::ServiceUnavailable().finish();
    };
    let Ok(Some(row)) = sqlx::query_as::<_, (String, String, Option<String>)>(
        "SELECT kind, status, center FROM work_items WHERE id = $1 FOR UPDATE",
    )
    .bind(id)
    .fetch_optional(&mut *tx)
    .await
    else {
        return HttpResponse::NotFound().finish();
    };
    let (kind, old, center) = row;
    if !can_edit(&member, &kind, center.as_deref()) {
        return HttpResponse::Forbidden().finish();
    }
    if !allowed_transition(&old, &form.status) {
        return HttpResponse::BadRequest().finish();
    }
    if sqlx::query("UPDATE work_items SET status = $1, updated_at = now(), closed_at = CASE WHEN $1 IN ('termine', 'annule') THEN now() ELSE NULL END WHERE id = $2")
        .bind(&form.status).bind(id).execute(&mut *tx).await.is_err() { return HttpResponse::ServiceUnavailable().finish() }
    if sqlx::query("INSERT INTO work_events (work_item_id, actor_id, action, note, old_status, new_status) VALUES ($1, $2, 'changement_statut', $3, $4, $5)")
        .bind(id).bind(member.id).bind(form.note.trim()).bind(&old).bind(&form.status).execute(&mut *tx).await.is_err() || tx.commit().await.is_err() { return HttpResponse::ServiceUnavailable().finish() }
    HttpResponse::SeeOther()
        .insert_header(("Location", format!("/espace/dossiers/{id}")))
        .insert_header(("Cache-Control", "no-store"))
        .finish()
}

#[cfg(test)]
mod tests {
    use super::{Item, allowed_transition, detail_fields, item_list};

    #[test]
    fn workflow_rejects_skips_and_reopening_closed_cases() {
        assert!(allowed_transition("nouveau", "qualifie"));
        assert!(!allowed_transition("nouveau", "termine"));
        assert!(allowed_transition("en_revue", "en_cours"));
        assert!(!allowed_transition("termine", "en_cours"));
    }

    #[test]
    fn dossier_list_renders_links_and_escapes_customer_content() {
        let item = Item {
            id: 7,
            kind: "transport".into(),
            status: "nouveau".into(),
            title: "<script>alert(1)</script>".into(),
            customer_name: "Client".into(),
            customer_email: None,
            customer_phone: None,
            description: String::new(),
            details: "{}".into(),
            center: None,
            created_at: "14/11/2026".into(),
        };
        let html = item_list(&[item]);
        assert!(html.contains("href=\"/espace/dossiers/7\""));
        assert!(!html.contains("<script>"));
        assert!(html.contains("&lt;script&gt;"));
    }

    #[test]
    fn public_request_details_are_visible_to_staff() {
        let fields =
            detail_fields(r#"{"origin":"Douala","destination":"Yaoundé","equipment":null}"#);
        assert_eq!(
            fields,
            vec![
                ("Départ", "Douala".into()),
                ("Destination", "Yaoundé".into())
            ]
        );
    }
}
