# Pages publiques et mouvement

La structure éditoriale s'inspire des parcours observés sur [l'accueil DP World](https://www.dpworld.com/en), [une page de service](https://www.dpworld.com/en/supply-chain-solutions/contract-logistics), [la présentation du groupe](https://www.dpworld.com/en/about-us) et [la page de contact](https://www.dpworld.com/en/contact-us). Le contenu, les couleurs, les destinations et les médias restent propres à SAPTRANS.

| Famille | Pages SAPTRANS | Structure |
|---|---|---|
| Accueil | `/` | Hero, accès directs, activités, focus métier, implantations, contact |
| Activités | `/transport-logistique`, `/location-engins`, `/controle-technique`, `/immobilier` | Hero, sommaire ancré, vue d'ensemble, prestations, informations publiées, parcours de demande, FAQ |
| Institutionnel | `/entreprise` | Présentation, histoire, divisions, contact |
| Répertoires | `/secteurs`, `/implantations`, `/realisations`, `/actualites` | Introduction et contenu adapté à la rubrique |
| Conversion | `/contact` | Choix du service, formulaire, coordonnées |
| Utilitaires | `/recherche`, `/contact/merci`, `/contact/erreur` | En-tête éditorial, résultat ou confirmation lisible |

Les images d'activités proviennent des URL présentes dans l'extraction de l'ancien site. Le site reste lisible si ces images sont indisponibles. Les biens immobiliers, études de cas, actualités, chiffres et partenaires ne sont pas créés sans validation de SAPTRANS.

## Vérification de l'accueil face à la référence

| Structure observée chez DP World | Adaptation SAPTRANS |
|---|---|
| Ouverture forte et proposition de valeur | Hero photographique et deux actions |
| Accès aux solutions | Quatre accès directs et quatre domaines d'activité |
| Présentation institutionnelle | Introduction de l'entreprise et deux focus métiers |
| Secteurs et méthode | Bande des secteurs et section sur la préparation des opérations |
| Réseau et contact | Implantations publiées et demande commerciale |

La structure est comparable, mais la page n'est pas identique visuellement. DP World publie des données mondiales, des initiatives et des études de cas qui n'ont pas d'équivalent SAPTRANS validé. La comparaison pixel par pixel nécessite aussi une capture navigateur des deux sites ; elle n'a pas été possible dans l'environnement de développement actuel.

Le mouvement est défini dans `public/style.css` et `public/theme.js` : entrée du hero, en-tête compact au défilement, ouverture des menus, survol des cartes, révélation des sections. Le HTML public est rendu côté serveur et reste visible sans JavaScript. Les animations sont désactivées par `prefers-reduced-motion: reduce`.

Avant publication, vérifier les coordonnées, les horaires, les droits des photographies issues de l'ancien site et les informations commerciales de chaque activité. Ajouter ensuite les contenus validés dans les modèles existants.
