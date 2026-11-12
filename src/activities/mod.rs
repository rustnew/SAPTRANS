use crate::{Language, content};
use leptos::prelude::*;

pub fn activity_page(
    language: Language,
    service: &'static str,
    source_key: Option<&'static str>,
) -> String {
    let (kicker, title, intro, lead, explanation, hero_image, detail_image, image_alt) = match service {
        "transport" => (
            language.text("TRANSPORT & LOGISTIQUE", "FREIGHT & LOGISTICS"),
            language.text("Le transport pensé pour le terrain.", "Freight planned for the realities on the ground."),
            language.text("Marchandises, conteneurs, bois et équipements : nous étudions votre opération avec ses contraintes de chargement, de trajet et de livraison.", "Freight, containers, timber and equipment: we review your operation against its loading, route and delivery requirements."),
            language.text("Chaque opération commence par une bonne compréhension du chargement.", "Every operation begins with a clear understanding of the cargo."),
            language.text("SAPTRANS présente des solutions de transport industriel, de conteneurs, de grumes et de matériel. Les modalités sont définies selon la marchandise, les sites et le calendrier.", "SAPTRANS presents industrial freight, container, timber and equipment transport. The terms depend on the cargo, sites and schedule."),
            Some("https://saptrans.net/wp-content/uploads/2021/08/transportlogistique_container_640x481.png"),
            Some("https://saptrans.net/wp-content/uploads/2021/07/transportlogistique_grume_640x480-600x377.png"),
            language.text("Transport de bois SAPTRANS", "SAPTRANS timber transport"),
        ),
        "equipment" => (
            language.text("LOCATION D’ENGINS", "EQUIPMENT RENTAL"),
            language.text("Les moyens adaptés à votre chantier.", "Equipment suited to your worksite."),
            language.text("SAPTRANS présente des engins pour les travaux publics, les opérations forestières et les activités minières. Disponibilités et conditions sur demande.", "SAPTRANS presents equipment for public works, forestry and mining. Availability and terms are confirmed on request."),
            language.text("Le bon engin dépend du travail à réaliser.", "The right machine depends on the job."),
            language.text("Décrivez l’opération, le terrain, la période et la localisation. L’équipe vous orientera vers les équipements et les conditions disponibles.", "Describe the task, terrain, period and location. Our team can advise on equipment and terms currently available."),
            Some("https://saptrans.net/wp-content/uploads/2021/10/1B8A1500_pelleuse_w1920-1024x698.jpg"),
            Some("https://saptrans.net/wp-content/uploads/2021/08/pelle_chargeuse_1-640x480.jpeg"),
            language.text("Pelle chargeuse présentée par SAPTRANS", "Wheel loader presented by SAPTRANS"),
        ),
        "inspection" => (
            "AUTORELAX",
            language.text("Le contrôle technique, avec une information claire.", "Vehicle inspection with clear practical information."),
            language.text("AutoRelax présente le contrôle technique automobile dans ses centres de Douala et Limbé. Contactez le centre pour confirmer les modalités en vigueur.", "AutoRelax presents vehicle inspections at its Douala and Limbe centres. Contact the centre to confirm current requirements."),
            language.text("Préparer la visite, comprendre le contrôle.", "Prepare your visit and understand the inspection."),
            language.text("Le contrôle associe des résultats obtenus avec les équipements et un examen visuel du véhicule. Les documents et horaires doivent être confirmés auprès du centre.", "The inspection combines equipment results with a visual vehicle check. Confirm documents and opening hours with the centre."),
            Some("https://saptrans.net/wp-content/uploads/2021/07/visite_technique_service_640x480.png"),
            Some("https://saptrans.net/wp-content/uploads/2021/08/visite_technique_distinction-400x300.jpg"),
            language.text("Contrôle technique AutoRelax", "AutoRelax vehicle inspection"),
        ),
        _ => (
            language.text("IMMOBILIER", "REAL ESTATE"),
            language.text("Un échange précis pour votre projet immobilier.", "A clear conversation about your property plans."),
            language.text("L’équipe SAPTRANS peut vous renseigner sur les biens, projets et services immobiliers effectivement proposés. Les offres publiables seront ajoutées après validation.", "SAPTRANS can advise on properties, projects and real-estate services actually offered. Publishable listings will be added after approval."),
            language.text("Une recherche immobilière mérite des informations à jour.", "Property decisions need current information."),
            language.text("Décrivez la localisation et le type de bien recherchés. L’équipe vous indiquera les informations confirmées et le bon interlocuteur.", "Tell us the location and type of property you seek. Our team will share confirmed information and connect you with the right person."),
            None, None, language.text("Immobilier SAPTRANS", "SAPTRANS real estate"),
        ),
    };
    let offers = match service {
        "transport" => [
            (language.text("Marchandises et conteneurs", "Freight and containers"), language.text("Chargement, départ, destination et contraintes de manutention.", "Cargo, origin, destination and handling requirements.")),
            (language.text("Bois et grumes", "Timber and logs"), language.text("Volumes, accès et conditions propres à l’opération.", "Volumes, site access and operation requirements.")),
            (language.text("Engins et matériel", "Equipment and machinery"), language.text("Déplacement selon les dimensions et les moyens nécessaires.", "Transport based on dimensions and required equipment.")),
        ],
        "equipment" => [
            (language.text("Pelles excavatrices", "Excavators"), language.text("Pour les opérations de creusement et de préparation de terrain décrites par SAPTRANS.", "For excavation and site preparation described by SAPTRANS.")),
            (language.text("Pelles chargeuses", "Wheel loaders"), language.text("Pour le chargement de terre, de pouzzolane et d’agrégats.", "For loading earth, pozzolana and aggregates.")),
            (language.text("Autres engins", "Other equipment"), language.text("Catégories et disponibilités à confirmer auprès de l’équipe.", "Confirm categories and availability with our team.")),
        ],
        "inspection" => [
            (language.text("Contrôles avec équipements", "Equipment checks"), language.text("Pneus, parallélisme et suspension sont cités sur l’ancien site.", "Tyres, alignment and suspension are listed on the archived site.")),
            (language.text("Examen visuel", "Visual inspection"), language.text("L’état du véhicule est également examiné par le contrôleur.", "The inspector also examines the vehicle visually.")),
            (language.text("Centres AutoRelax", "AutoRelax centres"), language.text("Douala-Yassa et Limbé : confirmez horaires et documents avant la visite.", "Douala-Yassa and Limbe: confirm hours and documents before visiting.")),
        ],
        _ => [
            (language.text("Décrire votre recherche", "Describe your search"), language.text("Précisez la ville, le type de bien et vos critères importants.", "Share the city, property type and key criteria.")),
            (language.text("Recevoir des informations", "Get information"), language.text("L’équipe indique les biens ou projets dont les détails sont confirmés.", "Our team shares confirmed property or project details.")),
            (language.text("Échanger avec SAPTRANS", "Speak with SAPTRANS"), language.text("Un interlocuteur confirmera les prochaines étapes selon votre demande.", "A contact will confirm the next steps for your enquiry.")),
        ],
    };
    let contact_href = language.href(match service {
        "transport" => "/contact?service=transport#demande",
        "equipment" => "/contact?service=engins#demande",
        "inspection" => "/contact?service=autorelax#demande",
        _ => "/contact?service=immobilier#demande",
    });
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
        view! { <section class="section source-section"><div class="container editorial-split"><div><p class="eyebrow">{language.text("LE SAVOIR-FAIRE SAPTRANS", "SAPTRANS EXPERIENCE")}</p><h2>{language.text("Les informations publiées par SAPTRANS.", "Information published by SAPTRANS.")}</h2><p class="source-note">{language.text("Extrait de l’ancien site, à revérifier avant publication.", "From the archived website; details require verification before publication.")}</p></div><div class="source-prose">{source_copy.into_iter().map(|paragraph| view! { <p>{paragraph}</p> }).collect_view()}</div></div></section> }.to_html()
    };
    let hero_media = hero_image.map(|url| view! { <img src=url alt="" decoding="async" fetchpriority="high" onerror="this.hidden=true"/> }.to_html()).unwrap_or_default();
    let editorial_media = detail_image.map(|url| view! { <figure class="editorial-media"><img src=url alt=image_alt loading="lazy" decoding="async" onerror="this.hidden=true;this.parentElement.classList.add('media-unavailable')"/><figcaption>{language.text("Photographie issue de l’ancien site SAPTRANS", "Photo from SAPTRANS’s archived website")}</figcaption></figure> }.to_html()).unwrap_or_else(|| view! { <div class="editorial-media editorial-media-empty"><span>"SAPTRANS"</span><strong>{language.text("Immobilier", "Real estate")}</strong></div> }.to_html());
    let hero_class = if hero_image.is_some() {
        "page-hero activity-hero has-photo"
    } else {
        "page-hero activity-hero"
    };

    view! { <main class="activity-page">
        <section class=hero_class><div class="activity-hero-media" inner_html=hero_media></div><div class="container activity-hero-content"><p class="eyebrow">{kicker}</p><h1>{title}</h1><p>{intro}</p><div class="actions"><a class="button button-light" href=contact_href.clone()>{language.text("Faire une demande", "Send an enquiry")}</a><a class="button button-outline-light" href="#apercu">{language.text("Découvrir le service", "Explore the service")}</a></div></div></section>
        <nav class="page-anchor-nav" aria-label=language.text("Sur cette page", "On this page")><div class="container"><a href="#apercu">{language.text("Vue d’ensemble", "Overview")}</a><a href="#prestations">{language.text("Prestations", "Services")}</a><a href="#demarche">{language.text("Votre demande", "Your enquiry")}</a><a href="#questions">{language.text("Questions", "Questions")}</a></div></nav>
        <section class="section editorial-split container" id="apercu"><div class="editorial-copy"><p class="eyebrow">{language.text("VUE D’ENSEMBLE", "OVERVIEW")}</p><h2>{lead}</h2><p>{explanation}</p><a class="text-link" href=contact_href.clone()>{language.text("Parler à notre équipe", "Talk to our team")} " ↗"</a></div><div inner_html=editorial_media></div></section>
        <section class="section service-offers" id="prestations"><div class="container"><div class="section-heading-row"><div><p class="eyebrow">{language.text("NOS RÉPONSES", "WHAT WE OFFER")}</p><h2>{language.text("Un service adapté à votre besoin.", "A service shaped around your needs.")}</h2></div><p>{language.text("Les modalités précises sont confirmées avec l’équipe avant toute prestation.", "The team confirms the precise terms before any service.")}</p></div><div class="offer-grid">{offers.into_iter().enumerate().map(|(index, (name, description))| view! { <article class="offer-card"><span class="card-number">{format!("{:02}", index + 1)}</span><h3>{name}</h3><p>{description}</p></article> }).collect_view()}</div></div></section>
        <div inner_html=source_section></div>
        <section class="section process-section" id="demarche"><div class="container"><p class="eyebrow">{language.text("VOTRE DEMANDE", "YOUR ENQUIRY")}</p><h2>{language.text("Un parcours simple, de la demande à la réponse.", "A clear path from enquiry to response.")}</h2><div class="process-grid"><div><span>"01"</span><h3>{language.text("Décrivez", "Describe")}</h3><p>{language.text("Indiquez votre besoin, la localisation et les contraintes connues.", "Share your needs, location and known requirements.")}</p></div><div><span>"02"</span><h3>{language.text("Nous étudions", "We review")}</h3><p>{language.text("L’équipe vérifie les informations nécessaires à une réponse pertinente.", "The team checks the information needed for a useful response.")}</p></div><div><span>"03"</span><h3>{language.text("Nous répondons", "We respond")}</h3><p>{language.text("Un interlocuteur vous indique les modalités disponibles.", "A contact explains the options currently available.")}</p></div></div></div></section>
        <section class="section faq-section container" id="questions"><div><p class="eyebrow">{language.text("QUESTIONS FRÉQUENTES", "FREQUENT QUESTIONS")}</p><h2>{language.text("Avant de nous contacter.", "Before you get in touch.")}</h2></div><div class="faq-list"><details><summary>{language.text("Quelles informations fournir ?", "What information should I provide?")}</summary><p>{language.text("Précisez le service recherché, le lieu, les dates souhaitées et toute contrainte utile. Nous vous demanderons les compléments nécessaires.", "Share the service, location, preferred dates and any useful requirements. We will ask for anything else we need.")}</p></details><details><summary>{language.text("Comment connaître les disponibilités ?", "How can I check availability?")}</summary><p>{language.text("Envoyez une demande au service concerné. Les disponibilités, horaires et conditions sont confirmés directement par SAPTRANS.", "Send an enquiry to the relevant team. SAPTRANS confirms availability, hours and terms directly.")}</p></details></div></section>
        <section class="band"><div class="container band-inner"><div><p class="eyebrow">{language.text("PARLONS DE VOTRE PROJET", "LET’S DISCUSS YOUR PROJECT")}</p><h2>{language.text("La bonne réponse commence ici.", "The right answer starts here.")}</h2></div><p>{language.text("Décrivez votre demande pour être orienté vers le bon service.", "Describe your enquiry so we can connect you with the right team.")}</p><a class="button button-light" href=contact_href>{language.text("Faire une demande", "Send an enquiry")}</a></div></section>
    </main> }.to_html()
}
