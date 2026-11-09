use leptos::prelude::*;

pub fn activity_page(
    kicker: &'static str,
    heading: &'static str,
    description: &'static str,
) -> String {
    let (first, second, third, question, answer) = match kicker {
        "TRANSPORT ET LOGISTIQUE" => (
            "Transport de marchandises",
            "Transport de conteneurs",
            "Transport d’équipements",
            "Comment préparer une demande de transport ?",
            "Indiquez la nature du chargement, les lieux de départ et d’arrivée, les dates souhaitées et les contraintes connues.",
        ),
        "LOCATION D’ENGINS" => (
            "Besoin ponctuel",
            "Projet de chantier",
            "Conditions adaptées",
            "Comment demander un engin ?",
            "Décrivez le type d’opération, le lieu, la période et les caractéristiques recherchées. L’équipe confirmera les disponibilités.",
        ),
        "AUTORELAX" => (
            "Centre de Douala",
            "Centre de Limbé",
            "Information pratique",
            "Comment préparer une visite ?",
            "Contactez le centre concerné pour vérifier les pièces à apporter, les horaires et les modalités en vigueur.",
        ),
        _ => (
            "Renseignements",
            "Offre à documenter",
            "Interlocuteur dédié",
            "Comment en savoir plus ?",
            "Contactez l’équipe SAPTRANS pour obtenir les informations confirmées sur l’activité immobilière.",
        ),
    };
    view! { <main>
        <section class="page-hero"><div class="container"><p class="eyebrow">{kicker}</p><h1>{heading}</h1><p>{description}</p><a class="button" href="/contact#demande">"Demander des renseignements"</a></div></section>
        <section class="container section split"><div><p class="eyebrow">"LE SERVICE"</p><h2>"Une réponse adaptée à votre besoin"</h2><p>{description}</p><p>"Notre équipe prend connaissance de votre demande avant de vous orienter vers la solution et le bon interlocuteur."</p></div><div class="media-placeholder media-light" role="img" aria-label="Emplacement pour une photographie réelle du service"><span>"Photographie du service à intégrer"</span></div></section>
        <section class="section section-muted"><div class="container"><p class="eyebrow">"NOTRE OFFRE"</p><h2>"Les points à explorer ensemble"</h2><div class="cards locations"><article class="card"><span class="card-number">"01"</span><h3>{first}</h3></article><article class="card"><span class="card-number">"02"</span><h3>{second}</h3></article><article class="card"><span class="card-number">"03"</span><h3>{third}</h3></article></div></div></section>
        <section class="container section narrow"><p class="eyebrow">"VOTRE DEMANDE"</p><h2>"Un échange simple, étape par étape"</h2><ol><li>"Décrivez votre besoin et votre localisation."</li><li>"L’équipe vérifie les informations nécessaires."</li><li>"Un interlocuteur vous répond avec les modalités disponibles."</li></ol><h3>{question}</h3><p>{answer}</p><a class="button" href="/contact#demande">"Faire une demande"</a></section>
    </main> }.to_html()
}
