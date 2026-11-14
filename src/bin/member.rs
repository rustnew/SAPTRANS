use argon2::{Argon2, PasswordHasher};
use sqlx::postgres::PgPoolOptions;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Le même fichier de configuration que ./dev.sh est chargé sans écraser
    // les variables déjà fournies par le shell ou le conteneur.
    let _ = dotenvy::dotenv();
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 5 || args[1] != "create" {
        eprintln!("Usage: cargo run --bin saptrans-member -- create EMAIL ROLE NOM CENTRE");
        eprintln!("Rôles: admin, commercial, transport, equipment, inspection, finance, viewer");
        std::process::exit(2);
    }
    let email = args[2].trim().to_ascii_lowercase();
    let role = args[3].as_str();
    let name = args[4].trim();
    let center = args
        .get(5)
        .map(|value| value.trim())
        .filter(|value| !value.is_empty());
    if !email.contains('@')
        || email.len() > 320
        || name.is_empty()
        || name.len() > 120
        || center.is_some_and(|value| value.len() > 40)
        || !matches!(
            role,
            "admin"
                | "commercial"
                | "transport"
                | "equipment"
                | "inspection"
                | "finance"
                | "viewer"
        )
        || (role == "inspection" && center.is_none())
    {
        return Err("Paramètres de compte invalides".into());
    }
    let database_url = std::env::var("DATABASE_URL")
        .map_err(|_| "DATABASE_URL manque. Configurez .env à la racine du projet.")?;
    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(&database_url)
        .await?;
    sqlx::migrate!().run(&pool).await?;
    let password = loop {
        let password = rpassword::prompt_password("Mot de passe (20 caractères minimum) : ")?;
        if password.chars().count() >= 20 && password.len() <= 1024 {
            break password;
        }
        eprintln!("Le mot de passe doit contenir de 20 à 1024 caractères. Réessayez.");
    };
    let hash = Argon2::default()
        .hash_password(password.as_bytes())?
        .to_string();
    sqlx::query("INSERT INTO admin_users (email, password_hash, display_name, role, center) VALUES ($1, $2, $3, $4, $5)")
        .bind(email).bind(hash).bind(name).bind(role).bind(center).execute(&pool).await?;
    println!("Compte créé. Connectez-vous sur /espace/connexion.");
    Ok(())
}
