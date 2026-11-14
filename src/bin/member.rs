use argon2::{Argon2, PasswordHasher};
use sqlx::postgres::PgPoolOptions;
use std::io::{self, Write};

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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
    print!("Mot de passe (20 caractères minimum) : ");
    io::stdout().flush()?;
    let mut password = String::new();
    io::stdin().read_line(&mut password)?;
    let password = password.trim_end_matches(['\r', '\n']);
    if password.chars().count() < 20 || password.len() > 1024 {
        return Err("Le mot de passe doit contenir de 20 à 1024 caractères".into());
    }
    let hash = Argon2::default()
        .hash_password(password.as_bytes())?
        .to_string();
    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(&std::env::var("DATABASE_URL")?)
        .await?;
    sqlx::migrate!().run(&pool).await?;
    sqlx::query("INSERT INTO admin_users (email, password_hash, display_name, role, center) VALUES ($1, $2, $3, $4, $5)")
        .bind(email).bind(hash).bind(name).bind(role).bind(center).execute(&pool).await?;
    println!("Compte créé.");
    Ok(())
}
