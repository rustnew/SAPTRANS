# Numérisation des visites techniques AutoRelax

## Sources et limites

- L'archive SAPTRANS du 9 novembre 2026, conservée dans `data/legacy_archive.json`, décrit l'accueil, l'identification et le paiement, les mesures par équipement, l'examen visuel, puis la documentation et les conseils. Elle emploie les résultats « accepté », « suspendu » et « refusé ». Ces termes et le délai de contre-visite annoncé dans l'archive doivent être vérifiés avant de définir une règle actuelle.
- Les articles 11 à 13 de la [loi camerounaise n° 2022/007 du 27 avril 2022](https://www.prc.cm/files/4b/c2/fc/72f8f94e8b65809f9d500acff2fb1e16.pdf) décrivent la visite périodique et le certificat. Ils renvoient à un texte du ministre chargé des transports pour la périodicité, la procédure, les frais, les éléments contrôlés et les documents. Le rapport [TRANSTAT 2025 du ministère des Transports](https://mintransports.cm/images/news/pdfs/8368812b20a3de079c5a52ba2aea30f1-TRANSTAT-MINT-2025-FR-Ok.pdf) mentionne l'interconnexion de centres et un système de sécurisation des contrôles et vignettes.
- Le présent module constitue un **dossier opérationnel interne**. Un certificat, une vignette ou un procès-verbal officiel exige la procédure approuvée par AutoRelax et, le cas échéant, l'intégration à la plateforme publique en service.

## Parcours et responsabilités

| Étape | Acteur proposé | Données et preuve conservées | Passage à l'étape suivante |
| --- | --- | --- | --- |
| 1. Demande ou arrivée | Client, réception | Centre, créneau souhaité, immatriculation, contact, type de véhicule, référence de demande | La réception confirme la prise en charge. |
| 2. Identification | Réception | Immatriculation, identité du véhicule et du demandeur, documents présentés, date et agent | Le dossier reçoit une référence unique. |
| 3. Paiement, si applicable | Caisse | Référence du reçu et statut de paiement, sans stockage de données bancaires | Le contrôleur peut commencer selon la procédure locale. |
| 4. Mesures | Contrôleur | Résultats bruts, unité, appareil, horodatage, opérateur, éventuel fichier d'origine | Chaque mesure garde sa provenance. |
| 5. Examen visuel | Contrôleur | Liste de points contrôlés, observations et défauts, horodatage | Le contrôleur soumet le dossier à la revue. |
| 6. Revue | Responsable habilité | Décision interne, motifs, auteur, date, version des contrôles examinés | Toute correction produit un nouvel événement traçable. |
| 7. Remise et suite | Réception, client | Document remis, date, destinataire, éventuelle nouvelle visite liée au dossier initial | Le dossier est clos selon la procédure validée. |

Les rôles exacts, la liste des contrôles, les seuils et les documents sont des paramètres métier à confirmer par les responsables AutoRelax. La décision ne doit pas être calculée à partir des anciens textes du site.

## Règles de traçabilité

1. Chaque visite possède une référence stable et rattache un véhicule, un centre et les acteurs intervenus.
2. Tout événement conserve l'auteur, la date, l'action et les données changées. Une modification ajoute une nouvelle version ; elle ne réécrit pas l'historique.
3. Une mesure distingue la saisie manuelle d'un import appareil, et conserve l'identifiant de l'appareil ou du fichier si disponible.
4. Un dossier soumis à la revue est verrouillé pour le contrôleur. Une correction exige un motif et une réouverture par un responsable.
5. Les pièces jointes et les dossiers sont accessibles selon le rôle et le centre. Leur durée de conservation et les droits des personnes doivent être définis au regard de la [loi camerounaise n° 2024/017 sur les données personnelles](https://www.prc.cm/fr/actualites/actes/lois/7588).
6. Les pages publiques ne révèlent ni immatriculation ni résultat. Le suivi du client utilise un accès authentifié ou une communication directe selon le processus choisi.

## Première version à construire dans le site application

- Espace membre Leptos avec comptes individuels, connexion, rôles et accès serveur. Les pages publiques et les demandes restent accessibles sans compte.
- Tableau des visites par centre et statut ; création d'un dossier depuis une demande ou à l'accueil.
- Fiche véhicule et contrôle avec points configurables, mesures, observations et pièces jointes.
- Revue par un responsable, journal d'événements et fiche interne imprimable clairement distincte du certificat officiel.
- Recherche par référence, immatriculation ou date, et indicateurs : visites en attente, dossiers incomplets, reprises et durée de traitement.

## Informations nécessaires pour finaliser les règles

1. Modèles de formulaires, fiches de contrôle, reçus et documents remis aujourd'hui à Douala-Yassa et Limbé.
2. Procédure et plateforme officielles actuellement utilisées, et autorisations d'intégration éventuelles.
3. Marques et modèles des appareils, formats d'export, identifiants et règles d'étalonnage applicables.
4. Rôles réels de la réception, de la caisse, des contrôleurs et du responsable de validation.
5. Créneaux, tarifs, modalités de nouvelle visite et politique de conservation effectivement appliqués.

## Critères de vérification du premier module

- Un membre non connecté ne peut ouvrir aucun dossier ni lire une donnée de véhicule.
- Un agent peut suivre une visite de l'accueil à la revue sans perdre la référence initiale.
- Le système bloque une transition de statut non autorisée et une décision sur un dossier incomplet.
- Une correction laisse visibles la valeur précédente, son auteur et son motif.
- Les tests couvrent les rôles, les transitions, les accès directs et le rendu mobile des écrans Leptos.
