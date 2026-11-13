# Audit de l'ancien site SAPTRANS

Source : `SAPTRANS_Extraction_Ancien_Site_2026-11-09.html` (archive du 9 novembre 2026, retirée du dépôt). Après avoir remis cette archive à la racine, `python3 scripts/audit_legacy_archive.py` régénère `data/legacy_archive.json` sans accéder à Internet. Le JSON conserve le texte, les métadonnées, les champs de formulaire et les liens de chaque page, ainsi que les URL des médias, leurs textes alternatifs et leurs pages sources. Il contient **124 pages**, **136 URL couvertes**, **179 destinations relevées dans les pages**, **80 références de médias** et **265 URL distinctes dans l'ensemble des liens HTML de l'archive**. Les 136 URL comprennent des fichiers techniques en plus des pages. Cet inventaire est documentaire et n'est pas publié tel quel dans le site public.

## Utilisation sur le nouveau site

| Ancienne URL | Destination |
| --- | --- |
| `/nos-services/`, `/nos-services/apercu-services/`, `/services/` | `/` |
| `/nos-services/transport-et-logistique/`, `/services/transport-et-logistique/` | `/transport-logistique` |
| `/nos-services/location-engins/`, `/services/location-dengins/` | `/location-engins` |
| `/nos-services/visite-technique/`, `/services/visite-technique/` | `/controle-technique` |
| `/entreprise/historique/`, `/qui-sommes-nous/` | `/entreprise` |
| `/contacts/` | `/contact` |

Les autres liens sont **conservés dans l'inventaire**, mais ne sont pas automatiquement publiés ou redirigés : certaines URL concernent du contenu de démonstration WordPress, des profils personnels, des partenaires non confirmés ou des réseaux sociaux sans compte officiel vérifié. Les anciennes pages `/entreprise/`, `/contact/` et `/actualites/` sont servies par les nouvelles routes de même nom. Les pages métier exploitent déjà une sélection de textes français extraits par `scripts/extract_legacy_content.py` ; l'immobilier n'a pas de catalogue vérifiable dans l'archive. Les chiffres, témoignages, personnes, coordonnées contradictoires et offres datées attendent une validation de SAPTRANS.

## Médias

L'archive HTML contient **des URL d'images, pas les fichiers images**. Les photos et le logo sont référencés localement en priorité dans les pages actuelles. Tant que les fichiers ne sont pas récupérés, le navigateur tente l'URL de l'ancien site ; le logo dispose aussi d'un mot-symbole SVG local de repli. Le site n'est donc pas encore autonome pour ses photographies.

Sur une machine ayant accès à `saptrans.net` :

```sh
python3 scripts/fetch_legacy_media.py
```

Le script n'accepte que les hôtes SAPTRANS et les types d'image attendus, limite chaque fichier à 20 Mio et écrit `data/legacy_media_download.json` avec le statut, la taille et le SHA-256 de chaque média. Il peut être relancé pour reprendre les échecs. Vérifier les droits de publication, la qualité et le contenu des images avant leur mise en production ; les 80 références incluent des médias WordPress de démonstration qui ne doivent pas être affichés sans contrôle.
Après récupération des fichiers, relancer la prévisualisation ou reconstruire l'image Docker pour que le serveur distribue les médias locaux.

## Limites vérifiées

- L'archive présente des coordonnées et des effectifs contradictoires ; les coordonnées complètes doivent être confirmées.
- Les liens vers des profils et des plateformes externes ne prouvent pas un partenariat ou un compte officiel.
- La version anglaise de l'ancien site comporte des textes de modèle ; les traductions du nouveau site doivent être revues par SAPTRANS.
- Les 80 références de médias ne correspondent pas nécessairement à 80 photographies SAPTRANS originales : il existe des variantes, des logos et du contenu de démonstration.
