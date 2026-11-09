# SAPTRANS — refonte Rust

Base de la refonte du site public de SAPTRANS / Sapou Transport SARL. L'extraction HTML fournie a servi de référence documentaire ; [les notes de lecture](docs/source-notes.md) consignent les faits retenus, les sources et les points à confirmer. L'extraction signale des textes de démonstration, des chiffres contradictoires et des éléments à valider avant publication.

## Stack

Rust stable, Leptos 0.8 (HTML rendu côté serveur), Actix Web, PostgreSQL et SQLx. Les composants interactifs pourront être hydratés en WebAssembly à mesure qu'ils seront nécessaires. CSS sur mesure, Serde et validator pour l'API de contact, Lettre pour la notification SMTP, tracing pour les journaux. Docker Compose et Caddy préparent le déploiement Linux.

## Démarrer

Pour consulter les maquettes sans Docker ni base de données : `./preview.sh`, puis ouvrir `http://localhost:3000`. Cette prévisualisation sert uniquement les pages publiques ; le formulaire ne peut pas enregistrer de message.

1. Installer Rust stable et PostgreSQL, créer une base `saptrans`, puis copier `.env.example` vers `.env` et adapter `DATABASE_URL`.
2. Exporter les variables de `.env` dans le shell, puis lancer `cargo run --bin saptrans-server`.
3. Ouvrir `http://localhost:3000`. Le serveur applique les migrations SQLx au démarrage.

Pour Docker Compose, définir `POSTGRES_PASSWORD` et `SITE_DOMAIN` dans `.env`, puis lancer `docker compose up --build`. Caddy gère HTTPS pour un domaine public résolu vers le serveur.

## Routes

`/`, `/transport-logistique`, `/location-engins`, `/controle-technique`, `/immobilier`, `/entreprise`, `/contact`, `/health`. Le formulaire de contact fonctionne sans JavaScript. L'API `POST /api/contact` accepte `{ "name": "...", "email": "...", "service": "transport", "message": "..." }`, valide les données et les enregistre en base. `service` vaut `transport`, `engins`, `autorelax` ou `immobilier`. Une notification est tentée si `SMTP_HOST`, `SMTP_FROM` et `CONTACT_RECIPIENT` sont définis. La réponse `202` confirme l'enregistrement, pas la livraison du courriel.

Les pages publiques sont rendues en HTML dès la réponse du serveur. Un cache partagé de 1 heure est autorisé pour ces pages ; les données privées et l'API n'utilisent pas ce cache. L'administration, les sessions sécurisées, Argon2 et l'envoi de fichiers vers S3/CDN sont les prochains modules : aucune interface d'administration n'est exposée actuellement. La table `admin_users` prépare leur schéma, sans créer de compte par défaut.

Le logo d'en-tête et sa variante blanche pointent actuellement vers les fichiers du site historique. Ils devront être intégrés comme médias locaux/CDN après récupération des originaux et validation de leur usage. Les cadres photographiques visibles dans la maquette marquent les emplacements de visuels réels à fournir.

## Contenus à confirmer

Coordonnées et horaires, offre immobilière, équipements disponibles, partenaires, autorisations d'utilisation des médias. Les contenus anglais de l'ancien site contiennent des textes du modèle WordPress ; ils ne sont pas repris. Aucun média de l'ancien site n'est recopié avant vérification des droits.

## Vérification

`cargo fmt --all -- --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`. Les tests navigateur : `npm install`, `npx playwright install chromium`, puis `npm run test:e2e`. Le flux GitHub Actions exécute ces commandes à chaque push et pull request.
