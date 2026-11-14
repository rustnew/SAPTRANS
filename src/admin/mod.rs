mod auth;
mod workspace;

use actix_web::web;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/espace")
            .route("/connexion", web::get().to(auth::login_page))
            .route("/connexion", web::post().to(auth::login))
            .route("/deconnexion", web::post().to(auth::logout))
            .route("", web::get().to(workspace::dashboard))
            .route("/dossiers", web::get().to(workspace::list))
            .route("/dossiers/nouveau", web::get().to(workspace::new_page))
            .route("/dossiers", web::post().to(workspace::create))
            .route("/dossiers/{id}", web::get().to(workspace::detail))
            .route(
                "/dossiers/{id}/statut",
                web::post().to(workspace::change_status),
            ),
    );
}
