use leptos::prelude::*;

use crate::{Language, content};

pub fn activity_page(
    language: Language,
    service: &'static str,
    source_key: Option<&'static str>,
) -> String {
    let (kicker, heading, description, first, second, third, question, answer) = match service {
        "transport" => (
            language.text("TRANSPORT ET LOGISTIQUE", "FREIGHT & LOGISTICS"),
            language.text("Faire circuler vos marchandises", "Moving your goods with care"),
            language.text(
                "Transport de marchandises, conteneurs et équipements pour les secteurs industriels, le BTP, les carrières, le bois et la distribution.",
                "Freight, container and machinery transport for industry, construction, quarries, timber and distribution.",
            ),
            language.text("Transport de marchandises", "Freight transport"),
            language.text("Transport de conteneurs", "Container transport"),
            language.text("Transport d’équipements", "Machinery transport"),
            language.text("Comment préparer une demande de transport ?", "What should I include in a freight request?"),
            language.text(
                "Indiquez la nature du chargement, les lieux de départ et d’arrivée, les dates souhaitées et les contraintes connues.",
                "Share the type of cargo, origin and destination, requested dates, and any known handling or route requirements.",
            ),
        ),
        "equipment" => (
            language.text("LOCATION D’ENGINS", "EQUIPMENT RENTAL"),
            language.text("Les moyens adaptés à vos chantiers", "Equipment for your worksite"),
            language.text(
                "Des engins de chantier pour les travaux publics, les opérations forestières et les activités minières. Les disponibilités et conditions sont à confirmer auprès de SAPTRANS.",
                "Construction equipment for public works, forestry and mining operations. Confirm availability and rental terms with SAPTRANS.",
            ),
            language.text("Besoin ponctuel", "Short-term needs"),
            language.text("Projet de chantier", "Worksite projects"),
            language.text("Conditions adaptées", "Suitable arrangements"),
            language.text("Comment demander un engin ?", "How do I request equipment?"),
            language.text(
                "Décrivez le type d’opération, le lieu, la période et les caractéristiques recherchées. L’équipe confirmera les disponibilités.",
                "Describe the operation, location, rental period and equipment you need. The team will confirm current availability.",
            ),
        ),
        "inspection" => (
            "AUTORELAX",
            language.text("Le contrôle technique de vos véhicules", "Vehicle inspections at AutoRelax"),
            language.text(
                "AutoRelax réalise le contrôle technique des véhicules à l’aide d’équipements et d’un contrôle visuel. Contactez le centre pour confirmer les modalités en vigueur.",
                "AutoRelax combines inspection equipment with a visual vehicle check. Contact your centre to confirm current requirements and opening hours.",
            ),
            language.text("Centre de Douala", "Douala centre"),
            language.text("Centre de Limbé", "Limbe centre"),
            language.text("Information pratique", "Plan your visit"),
            language.text("Comment préparer une visite ?", "How should I prepare for an inspection?"),
            language.text(
                "Contactez le centre concerné pour vérifier les pièces à apporter, les horaires et les modalités en vigueur.",
                "Contact the centre to confirm which documents to bring, opening hours and the current inspection process.",
            ),
        ),
        _ => (
            language.text("IMMOBILIER", "REAL ESTATE"),
            language.text("Parlons de vos projets immobiliers", "Let’s discuss your real-estate plans"),
            language.text(
                "Contactez SAPTRANS pour connaître les biens, projets et services immobiliers actuellement proposés.",
                "Contact SAPTRANS for information about currently available properties, projects and real-estate services.",
            ),
            language.text("Renseignements", "Enquiries"),
            language.text("Offre à documenter", "Offer to be documented"),
            language.text("Interlocuteur dédié", "Dedicated contact"),
            language.text("Comment en savoir plus ?", "How can I find out more?"),
            language.text(
                "Contactez l’équipe SAPTRANS pour obtenir les informations confirmées sur l’activité immobilière.",
                "Contact SAPTRANS for confirmed information about its real-estate activity.",
            ),
        ),
    };

    let media = match service {
        "transport" => view! { <figure class="media-photo"><img src="/media/legacy/4e18e3d465b5b8dc.png" alt="" width="640" height="481" onerror={format!("this.onerror=null;this.src='{}'", "https://saptrans.net/wp-content/uploads/2021/08/transportlogistique_container_640x481.png")}/><figcaption>{language.text("Transport et logistique · SAPTRANS", "Freight and logistics · SAPTRANS")}</figcaption></figure> }.to_html(),
        "equipment" => view! { <figure class="media-photo"><img src="/media/legacy/c91ba6dca26b2637.png" alt="" width="640" height="480" onerror={format!("this.onerror=null;this.src='{}'", "https://saptrans.net/wp-content/uploads/2021/08/location_engins_640x480.png")}/><figcaption>{language.text("Location d’engins · SAPTRANS", "Equipment rental · SAPTRANS")}</figcaption></figure> }.to_html(),
        "inspection" => view! { <figure class="media-photo"><img src="/media/legacy/bb745ce26b67b29c.png" alt="" width="640" height="480" onerror={format!("this.onerror=null;this.src='{}'", "https://saptrans.net/wp-content/uploads/2021/07/visite_technique_service_640x480.png")}/><figcaption>{language.text("Contrôle technique · AutoRelax", "Vehicle inspection · AutoRelax")}</figcaption></figure> }.to_html(),
        _ => view! { <aside class="media-note"><span>"SAPTRANS"</span><p>{language.text("Échangeons sur votre projet immobilier.", "Talk to us about your real-estate enquiry.")}</p></aside> }.to_html(),
    };

    let source_copy = source_key
        .map(|key| {
            if language == Language::En {
                content::paragraphs_en(key)
            } else {
                content::paragraphs(key)
            }
        })
        .unwrap_or_default();
    let source_section = if source_copy.is_empty() {
        String::new()
    } else {
        view! { <section class="container section narrow source-copy"><details class="history-details"><summary>{language.text("En savoir plus sur cette activité", "More about this service")}</summary><p class="source-note">{language.text("Détails issus de l’ancien site SAPTRANS, à actualiser avant publication.", "Details from SAPTRANS’s archived site, to be updated before publication.")}</p>{source_copy.into_iter().map(|paragraph| view! { <p>{paragraph}</p> }).collect_view()}</details></section> }.to_html()
    };
    let offer_section = if matches!(service, "transport" | "inspection") {
        view! { <section class="section section-muted"><div class="container"><p class="eyebrow">{language.text("NOTRE OFFRE", "WHAT WE DO")}</p><h2>{language.text("Les services et centres", "Services and centres")}</h2><div class="cards locations"><article class="card"><h3>{first}</h3></article><article class="card"><h3>{second}</h3></article><article class="card"><h3>{third}</h3></article></div></div></section> }.to_html()
    } else {
        String::new()
    };
    let support_heading = match service {
        "transport" => language.text("Préparer chaque transport", "Planning every shipment"),
        "equipment" => language.text("Préparer votre chantier", "Planning your worksite"),
        "inspection" => language.text("Préparer votre visite", "Planning your visit"),
        _ => language.text(
            "Parlons de votre recherche",
            "Tell us what you are looking for",
        ),
    };
    let contact_href = language.href(match service {
        "transport" => "/contact?service=transport#demande",
        "equipment" => "/contact?service=engins#demande",
        "inspection" => "/contact?service=autorelax#demande",
        _ => "/contact?service=immobilier#demande",
    });

    view! { <main>
        <section class="page-hero"><div class="container"><p class="eyebrow">{kicker}</p><h1>{heading}</h1><p>{description}</p><a class="button" href=contact_href.clone()>{language.text("Demander des renseignements", "Make an enquiry")}</a></div></section>
        <section class="container section split"><div><p class="eyebrow">{language.text("LE SERVICE", "THE SERVICE")}</p><h2>{support_heading}</h2><p>{language.text("Décrivez votre projet, votre localisation et vos contraintes. L’équipe SAPTRANS vous indiquera les modalités disponibles.", "Describe your project, location and requirements. The SAPTRANS team will explain the available options.")}</p></div><div inner_html=media></div></section>
        <div inner_html=source_section></div>
        <div inner_html=offer_section></div>
        <section class="container section narrow"><p class="eyebrow">{language.text("VOTRE DEMANDE", "YOUR ENQUIRY")}</p><h2>{language.text("Un échange simple, étape par étape", "A simple, clear process")}</h2><ol><li>{language.text("Décrivez votre besoin et votre localisation.", "Tell us what you need and where.")}</li><li>{language.text("L’équipe vérifie les informations nécessaires.", "Our team reviews the information provided.")}</li><li>{language.text("Un interlocuteur vous répond avec les modalités disponibles.", "A member of the team will reply with the available options.")}</li></ol><h3>{question}</h3><p>{answer}</p><a class="button" href=contact_href>{language.text("Faire une demande", "Send an enquiry")}</a></section>
    </main> }.to_html()
}
