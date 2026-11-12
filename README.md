# SAPTRANS — refonte Rust

Base de la refonte du site public de SAPTRANS / Sapou Transport SARL. L'extraction HTML fournie a servi de référence documentaire ; [les notes de lecture](docs/source-notes.md) consignent les faits retenus, les sources et les points à confirmer. L'extraction signale des textes de démonstration, des chiffres contradictoires et des éléments à valider avant publication.

## Stack

Rust stable, Leptos 0.8 (HTML rendu côté serveur), Actix Web, PostgreSQL et SQLx. Les composants interactifs pourront être hydratés en WebAssembly à mesure qu'ils seront nécessaires. CSS sur mesure, Serde et validator pour l'API de contact, Lettre pour la notification SMTP, tracing pour les journaux. Docker Compose et Caddy préparent le déploiement Linux.

## Identité visuelle

Le thème utilise actuellement un vert de travail (`#28734F`) pour le mot-symbole local de secours, les liens et les actions. Ce code provient du SVG provisoire du projet ; il n'a pas été mesuré sur le logo officiel. Les grands panneaux emploient un vert profond (`#123D2E`), avec des fonds clairs (`#F6F8F6`) et des séparateurs vert gris (`#DCE5DE`). Les erreurs de formulaire conservent une couleur distincte pour être clairement identifiables.

La typographie utilise Fira Sans en trois graisses (400, 500 et 600), servie localement en WOFF2 depuis `public/fonts/`. Les fichiers sont sous licence SIL Open Font License, conservée dans ce dossier. La feuille CSS unique définit les composants, les espaces, les thèmes et les adaptations mobile sans dépendance à un service de polices externe.

## Langues et thèmes

Le site est disponible en français et en anglais. Le français est la langue par défaut ; `?lang=en` affiche la version anglaise, et le sélecteur conserve la page lors du changement. Un bouton permet de basculer entre thème clair et sombre. Le site suit d’abord le réglage du système, puis mémorise le choix manuel dans le navigateur.

## Accueil

La page d’accueil présente un hero photographique pleine largeur, quatre accès directs, les activités, les pôles transport et immobilier, les implantations, un espace partenaires et la demande commerciale. Le hero utilise provisoirement la [photographie originale d’Erik Mclean sur Pexels](https://www.pexels.com/photo/aerial-photography-of-a-container-truck-on-the-road-near-by-the-forest-9195241/) (4 000 × 3 000 px, licence Pexels), sans voile ni filtre CSS. Il ne s’agit pas d’un véhicule SAPTRANS ; remplacer ce visuel par une photographie authentique de l’entreprise lorsqu’elle sera disponible. Les autres photographies proviennent des pages métier de l’ancien site. Si leurs fichiers ne chargent pas, la mise en page conserve un fond lisible sans afficher d’image cassée. Les logos de partenaires ne sont pas publiés tant que les collaborations et droits d’usage ne sont pas confirmés.

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

L'en-tête et le pied de page appellent actuellement la variante sombre du logo officiel depuis l'ancien domaine. Si ce chargement échoue, le navigateur affiche le mot-symbole local de secours. L'ancien site référence aussi une variante blanche, mais les deux fichiers PNG manquent dans l'archive locale : leurs couleurs exactes ne sont pas encore vérifiables. Le thème sombre présente le logo sur un fond blanc pour préserver sa lisibilité. Les photographies de l'accueil utilisent des fichiers locaux lorsqu'ils sont disponibles et l'ancien site en repli temporaire ; les pages métiers chargent provisoirement les URL de l'ancien site. `python3 scripts/fetch_legacy_media.py` récupère les images depuis leurs URL inventoriées sur une machine connectée. Les droits d'utilisation doivent être confirmés avant publication.

## Contenus à confirmer

Coordonnées téléphoniques et horaires, offre immobilière, équipements disponibles, partenaires, chiffres clés, réalisations et autorisations d'utilisation des médias. Les contenus anglais de l'ancien site contiennent des textes du modèle WordPress ; ils ne sont pas repris. Les activités, implantations et rubriques éditoriales sont présentées sans inventaire, résultat, certification ni partenariat inventé.

## Vérification

`cargo fmt --all -- --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`. Les tests navigateur : `npm install`, `npx playwright install chromium`, puis `npm run test:e2e`. Le flux GitHub Actions exécute ces commandes à chaque push et pull request.

## Développement avec backend et frontend

Une seule commande lance le serveur Actix qui rend les pages Leptos SSR, puis surveille Rust, CSS et médias pour recompiler et recharger le navigateur : `./dev.sh`. PostgreSQL doit être installé et démarré localement ; configurez `DATABASE_URL` dans `.env` à partir de `.env.example`. Le script fixe `CARGO_BUILD_JOBS=1` et `RUST_MIN_STACK=33554432` pour éviter le SIGSEGV de compilation signalé sur `js-sys`. Arrêt avec `Ctrl+C`.
