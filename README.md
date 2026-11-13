# SAPTRANS — refonte Rust

Base de la refonte du site public de SAPTRANS / Sapou Transport SARL. L'extraction HTML fournie a servi de référence documentaire ; [les notes de lecture](docs/source-notes.md) consignent les faits retenus, les sources et les points à confirmer. L'extraction signale des textes de démonstration, des chiffres contradictoires et des éléments à valider avant publication.

## Stack

Rust stable, Leptos 0.8 (HTML rendu côté serveur), Actix Web, PostgreSQL et SQLx. Les composants interactifs pourront être hydratés en WebAssembly à mesure qu'ils seront nécessaires. CSS sur mesure, Serde et validator pour l'API de contact, Lettre pour la notification SMTP, tracing pour les journaux. Docker Compose et Caddy préparent le déploiement Linux.

## Identité visuelle

Le thème utilise actuellement un vert de travail (`#28734F`) pour le mot-symbole local de secours, les liens et les actions. Ce code provient du SVG provisoire du projet ; il n'a pas été mesuré sur le logo officiel. Les grands panneaux emploient un vert profond (`#123D2E`), avec des fonds clairs (`#F6F8F6`) et des séparateurs vert gris (`#DCE5DE`). Les erreurs de formulaire conservent une couleur distincte pour être clairement identifiables.

La typographie utilise Manrope variable pour les grands titres, le texte, la navigation et les formulaires (graisses 400 à 800). Barlow Condensed reste utilisé pour certains accents typographiques et repères de page. Les fichiers WOFF2 sont servis localement depuis `public/fonts/` ; les licences SIL Open Font License sont conservées dans ce dossier. La feuille CSS définit une hiérarchie commune pour les pages, les deux thèmes et les adaptations mobile sans dépendance à un service de polices externe.

La [composition des pages publiques](docs/dpworld-content-patterns.md) décrit la formule éditoriale observée sur DP World et son adaptation aux contenus SAPTRANS.

## Langues et thèmes

Le site est disponible en français et en anglais. Le français est la langue par défaut ; `?lang=en` affiche la version anglaise, et le sélecteur conserve la page lors du changement. Un bouton permet de basculer entre thème clair et sombre. Le site suit d’abord le réglage du système, puis mémorise le choix manuel dans le navigateur.

## Accueil

La page d’accueil présente un hero photographique pleine largeur, quatre accès directs, les activités, les pôles transport et immobilier, les implantations, un espace partenaires et la demande commerciale. Les photographies éditoriales proviennent du Ghana, du Nigeria et du Cameroun ; elles sont servies depuis `public/media/editorial/` et documentées dans [le registre des visuels](docs/media-sources.md). Elles illustrent les activités et ne montrent ni les équipes, ni les biens, ni les opérations de SAPTRANS. Les douze logos de clients et partenaires relevés dans l’ancienne section sont répertoriés dans [le registre des logos](docs/partner-logos.md).

Les pages métiers disposent d'un hero, d'une navigation interne, de prestations, d'informations issues de l'ancien site, d'un parcours de demande et d'une FAQ. L'entreprise, les implantations, les secteurs, les réalisations, les actualités et le contact ont chacun une composition adaptée. [Le plan des pages et des transitions](docs/pages-and-motion.md) décrit les modèles, les interactions et les contenus restant à valider.

## Démarrer

Pour consulter les maquettes sans Docker ni base de données : `./preview.sh`, puis ouvrir `http://localhost:3000`. Le script surveille les fichiers Rust, CSS et SVG, recompile si nécessaire et recharge le navigateur. Cette prévisualisation sert uniquement les pages publiques ; le formulaire ne peut pas enregistrer de message.

1. Installer Rust stable et PostgreSQL, créer une base `saptrans`, puis copier `.env.example` vers `.env` et adapter `DATABASE_URL`.
2. Exporter les variables de `.env` dans le shell, puis lancer `cargo run --bin saptrans-server`.
3. Ouvrir `http://localhost:3000`. Le serveur applique les migrations SQLx au démarrage.

Pour Docker Compose, définir `POSTGRES_PASSWORD` et `SITE_DOMAIN` dans `.env`, puis lancer `docker compose up --build`. Caddy gère HTTPS pour un domaine public résolu vers le serveur.

## Routes

`/`, `/transport-logistique`, `/location-engins`, `/controle-technique`, `/immobilier`, `/secteurs`, `/realisations`, `/implantations`, `/actualites`, `/entreprise`, `/recherche`, `/contact` et `/health` sont les routes publiques. La recherche filtre les rubriques publiques disponibles. Le formulaire fonctionne sans JavaScript et collecte les coordonnées, le service, la marchandise, le départ, la destination, la quantité, la date souhaitée, l'équipement et le message. Les champs complémentaires sont enregistrés dans `contact_messages.details`; une référence SAP est créée après enregistrement. Un champ leurre réduit les soumissions automatiques. L'API `POST /api/contact` accepte les mêmes champs, valide les données et renvoie la référence. `service` vaut `transport`, `engins`, `autorelax` ou `immobilier`. Une notification est tentée si `SMTP_HOST`, `SMTP_FROM` et `CONTACT_RECIPIENT` sont définis. La réponse confirme l'enregistrement, pas la livraison du courriel.

Les pages publiques sont rendues en HTML dès la réponse du serveur, avec descriptions SEO, URL canonique et liens de langue. Un cache partagé de 1 heure est autorisé pour les pages publiques ; les données privées et l'API n'utilisent pas ce cache. L'administration, les sessions sécurisées, Argon2, les catalogues administrables et l'envoi d'images vers S3/CDN restent à construire : aucune interface d'administration n'est exposée actuellement. La table `admin_users` prépare leur schéma, sans créer de compte par défaut.

## Contenu extrait de l’ancien site

Le contenu français utilisé par les pages métier est extrait du fichier archivé `SAPTRANS_Extraction_Ancien_Site_2026-11-09.html`. `python3 scripts/extract_legacy_content.py` régénère `public/data/legacy-content.json` hors ligne ; `./dev.sh` le fait automatiquement avant chaque compilation. Le serveur lit ce fichier local, sans appeler l’ancien site. Seules les pages françaises connues sont importées ; les pages anglaises de démonstration sont écartées. Les chiffres d’effectifs, promesses de délai et inventaires précis ne sont pas repris. Les informations propres à l’immobilier restent à documenter. [L'audit complet de l'archive](docs/legacy-audit.md) décrit les 124 pages, tous les liens et les 80 références de médias. Pour régénérer l'inventaire, installer `beautifulsoup4` puis exécuter `python3 scripts/audit_legacy_archive.py`.

L’en-tête sert les deux variantes transparentes du logo de l’ancien site depuis `public/brand/` : mot-symbole sombre sur le thème clair, mot-symbole blanc sur le thème sombre. Le pied de page sombre utilise la variante blanche. Aucun fond blanc n’est ajouté derrière le logo. Les photographies éditoriales sont locales ; leur provenance et les informations disponibles figurent dans [le registre des visuels](docs/media-sources.md). Sept images de l’ancien site illustrent désormais les prestations documentées. `python3 scripts/fetch_legacy_media.py` permet encore de récupérer les autres médias de l’archive lorsque ce contenu est nécessaire.

## Contenus à confirmer

Coordonnées téléphoniques et horaires, offre immobilière, équipements disponibles, partenaires, chiffres clés, réalisations et autorisations d'utilisation des médias. Les contenus anglais de l'ancien site contiennent des textes du modèle WordPress ; ils ne sont pas repris. Les activités, implantations et rubriques éditoriales sont présentées sans inventaire, résultat, certification ni partenariat inventé.

## Vérification

`cargo fmt --all -- --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`. Les tests navigateur : `npm install`, `npx playwright install chromium`, puis `npm run test:e2e`. Le flux GitHub Actions exécute ces commandes à chaque push et pull request.

## Développement avec backend et frontend

Une seule commande lance le serveur Actix qui rend les pages Leptos SSR, puis surveille Rust, CSS et médias pour recompiler et recharger le navigateur : `./dev.sh`. PostgreSQL doit être installé et démarré localement ; configurez `DATABASE_URL` dans `.env` à partir de `.env.example`. Le script fixe `CARGO_BUILD_JOBS=1` et `RUST_MIN_STACK=33554432` pour éviter le SIGSEGV de compilation signalé sur `js-sys`. Arrêt avec `Ctrl+C`.
