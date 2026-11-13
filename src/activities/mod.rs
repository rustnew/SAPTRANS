use crate::{Language, content};
use leptos::prelude::*;

pub fn activity_page(
    language: Language,
    service: &'static str,
    source_key: Option<&'static str>,
) -> String {
    let (kicker, title, intro, lead, explanation, hero_image, detail_image, image_alt, photo_location) = match service {
        "transport" => (
            language.text("TRANSPORT & LOGISTIQUE", "FREIGHT & LOGISTICS"),
            language.text("Le transport pensé pour le terrain.", "Freight planned for the realities on the ground."),
            language.text("Marchandises, conteneurs, bois et équipements : nous étudions votre opération avec ses contraintes de chargement, de trajet et de livraison.", "Freight, containers, timber and equipment: we review your operation against its loading, route and delivery requirements."),
            language.text("Chaque opération commence par une bonne compréhension du chargement.", "Every operation begins with a clear understanding of the cargo."),
            language.text("SAPTRANS présente des solutions de transport industriel, de conteneurs, de grumes et de matériel. Les modalités sont définies selon la marchandise, les sites et le calendrier.", "SAPTRANS presents industrial freight, container, timber and equipment transport. The terms depend on the cargo, sites and schedule."),
            Some("/media/editorial/nigeria-fleet-crew.jpg"),
            Some("/media/editorial/nigeria-fleet-crew.jpg"),
            language.text("Équipe de maintenance en tenue devant un véhicule au Nigeria", "Uniformed maintenance team in front of a vehicle in Nigeria"),
            language.text("Nigeria", "Nigeria"),
        ),
        "equipment" => (
            language.text("LOCATION D’ENGINS", "EQUIPMENT RENTAL"),
            language.text("Les moyens adaptés à votre chantier.", "Equipment suited to your worksite."),
            language.text("SAPTRANS présente des engins pour les travaux publics, les opérations forestières et les activités minières. Disponibilités et conditions sur demande.", "SAPTRANS presents equipment for public works, forestry and mining. Availability and terms are confirmed on request."),
            language.text("Le bon engin dépend du travail à réaliser.", "The right machine depends on the job."),
            language.text("Décrivez l’opération, le terrain, la période et la localisation. L’équipe vous orientera vers les équipements et les conditions disponibles.", "Describe the task, terrain, period and location. Our team can advise on equipment and terms currently available."),
            Some("/media/editorial/lagos-road-crew.jpg"),
            Some("/media/editorial/douala-construction-workers.jpg"),
            language.text("Ouvriers sur un chantier à Douala", "Workers at a construction site in Douala"),
            language.text("Douala, Cameroun", "Douala, Cameroon"),
        ),
        "inspection" => (
            "AUTORELAX",
            language.text("Le contrôle technique, avec une information claire.", "Vehicle inspection with clear practical information."),
            language.text("AutoRelax présente le contrôle technique automobile dans ses centres de Douala et Limbé. Contactez le centre pour confirmer les modalités en vigueur.", "AutoRelax presents vehicle inspections at its Douala and Limbe centres. Contact the centre to confirm current requirements."),
            language.text("Préparer la visite, comprendre le contrôle.", "Prepare your visit and understand the inspection."),
            language.text("Le contrôle associe des résultats obtenus avec les équipements et un examen visuel du véhicule. Les documents et horaires doivent être confirmés auprès du centre.", "The inspection combines equipment results with a visual vehicle check. Confirm documents and opening hours with the centre."),
            Some("/media/editorial/ghana-inspection-bay.png"),
            Some("/media/editorial/ghana-inspection-bay.png"),
            language.text("Techniciens en tenue inspectant un véhicule au Ghana", "Uniformed technicians inspecting a vehicle in Ghana"),
            language.text("Ghana", "Ghana"),
        ),
        _ => (
            language.text("IMMOBILIER", "REAL ESTATE"),
            language.text("Un échange précis pour votre projet immobilier.", "A clear conversation about your property plans."),
            language.text("L’équipe SAPTRANS peut vous renseigner sur les biens, projets et services immobiliers effectivement proposés. Les offres publiables seront ajoutées après validation.", "SAPTRANS can advise on properties, projects and real-estate services actually offered. Publishable listings will be added after approval."),
            language.text("Une recherche immobilière mérite des informations à jour.", "Property decisions need current information."),
            language.text("Décrivez la localisation et le type de bien recherchés. L’équipe vous indiquera les informations confirmées et le bon interlocuteur.", "Tell us the location and type of property you seek. Our team will share confirmed information and connect you with the right person."),
            Some("/media/editorial/abuja-project-team.jpg"),
            Some("/media/editorial/lagos-residential-building.jpg"),
            language.text("Immeuble résidentiel à Lagos, photo d’illustration", "Residential building in Lagos, illustrative photo"),
            language.text("Lagos, Nigeria", "Lagos, Nigeria"),
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
    let situations = match service {
        "transport" => [
            (language.text("Marchandises industrielles", "Industrial freight"), language.text("Précisez la nature du chargement, son volume et les contraintes de manutention.", "Describe the cargo, its volume and any handling requirements.")),
            (language.text("Conteneurs et bois", "Containers and timber"), language.text("Indiquez les lieux de chargement et de livraison, ainsi que les accès disponibles.", "Share the loading and delivery sites and available access.")),
            (language.text("Engins et matériel", "Machinery and equipment"), language.text("Communiquez les dimensions, le poids et les dates souhaitées.", "Provide dimensions, weight and preferred dates.")),
        ],
        "equipment" => [
            (language.text("Terrassement", "Earthworks"), language.text("Décrivez la surface, le terrain et la durée prévue.", "Describe the area, ground conditions and expected duration.")),
            (language.text("Chargement", "Loading"), language.text("Précisez le matériau, le volume et les conditions d'accès au site.", "Specify the material, volume and site access.")),
            (language.text("Interventions spécialisées", "Specialist work"), language.text("Présentez l'opération pour vérifier les moyens réellement disponibles.", "Explain the task so available equipment can be confirmed.")),
        ],
        "inspection" => [
            (language.text("Préparer la visite", "Prepare a visit"), language.text("Confirmez les pièces nécessaires et les horaires auprès du centre.", "Confirm the required documents and opening hours with the centre.")),
            (language.text("Choisir un centre", "Choose a centre"), language.text("Les implantations mentionnées sont Douala-Yassa et Limbé.", "The listed locations are Douala-Yassa and Limbe.")),
            (language.text("Comprendre le contrôle", "Understand the inspection"), language.text("Consultez les étapes décrites par AutoRelax, puis vérifiez la procédure actuelle.", "Read the steps described by AutoRelax, then confirm the current procedure.")),
        ],
        _ => [
            (language.text("Décrire un bien", "Describe a property"), language.text("Indiquez le type de bien et la localisation recherchés.", "Share the property type and location you seek.")),
            (language.text("Préciser votre calendrier", "Share your timing"), language.text("Mentionnez votre échéance et les critères essentiels.", "Tell us your timeframe and essential criteria.")),
            (language.text("Vérifier les offres", "Confirm availability"), language.text("Demandez les informations validées avant toute décision.", "Request confirmed details before making a decision.")),
        ],
    };
    let contact_href = language.href(match service {
        "transport" => "/contact?service=transport#demande",
        "equipment" => "/contact?service=engins#demande",
        "inspection" => "/contact?service=autorelax#demande",
        _ => "/contact?service=immobilier#demande",
    });
    let home_href = language.href("/");
    let services_href = language.href("/#services");
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
        view! { <section class="section source-section" id="informations"><div class="container editorial-split"><div><p class="eyebrow">{language.text("INFORMATIONS UTILES", "USEFUL INFORMATION")}</p><h2>{language.text("Des détails confirmés avant de vous engager.", "Confirmed details before you decide.")}</h2></div><div class="source-prose"><p>{language.text("Les offres immobilières et leurs conditions ne sont pas encore documentées dans les sources disponibles. L'équipe SAPTRANS pourra vous répondre selon votre demande.", "Property listings and terms are not yet documented in the available sources. The SAPTRANS team can respond to your specific enquiry.")}</p><a class="text-link" href=contact_href.clone()>{language.text("Poser une question", "Ask a question")} " ↗"</a></div></div></section> }.to_html()
    } else {
        view! { <section class="section source-section" id="informations"><div class="container editorial-split"><div><p class="eyebrow">{language.text("INFORMATIONS UTILES", "USEFUL INFORMATION")}</p><h2>{language.text("Les informations publiées par SAPTRANS.", "Information published by SAPTRANS.")}</h2><p class="source-note">{language.text("Extrait de l’ancien site, à revérifier avant publication.", "From the archived website; details require verification before publication.")}</p></div><div class="source-prose">{source_copy.into_iter().map(|paragraph| view! { <p>{paragraph}</p> }).collect_view()}</div></div></section> }.to_html()
    };
    let hero_media = hero_image.map(|url| view! { <img src=url alt="" decoding="async" fetchpriority="high" onerror="this.hidden=true"/> }.to_html()).unwrap_or_default();
    let editorial_media = detail_image.map(|url| view! { <figure class="editorial-media"><img src=url alt=image_alt loading="lazy" decoding="async" onerror="this.hidden=true;this.parentElement.classList.add('media-unavailable')"/><figcaption>{language.text("Photo d’illustration prise en ", "Illustrative photo taken in ")}{photo_location}</figcaption></figure> }.to_html()).unwrap_or_default();
    let hero_class = if hero_image.is_some() {
        "page-hero activity-hero has-photo"
    } else {
        "page-hero activity-hero"
    };

    view! { <main class="activity-page">
        <section class=hero_class><div class="activity-hero-media" inner_html=hero_media></div><div class="container activity-hero-content"><nav class="hero-breadcrumbs" aria-label=language.text("Fil d’Ariane", "Breadcrumb")><a href=home_href>{language.text("Accueil", "Home")}</a><span aria-hidden="true">"›"</span><a href=services_href>{language.text("Solutions", "Solutions")}</a><span aria-hidden="true">"›"</span><span aria-current="page">{kicker}</span></nav><p class="eyebrow">{kicker}</p><h1>{title}</h1><p>{intro}</p><div class="actions"><a class="button" href=contact_href.clone()>{language.text("Faire une demande", "Send an enquiry")} " →"</a><a class="button button-quiet" href="#apercu">{language.text("Découvrir le service", "Explore the service")}</a></div></div></section>
        <nav class="page-anchor-nav" aria-label=language.text("Sur cette page", "On this page")><div class="container"><a href="#apercu">{language.text("Vue d’ensemble", "Overview")}</a><a href="#prestations">{language.text("Prestations", "Services")}</a><a href="#situations">{language.text("Applications", "Applications")}</a><a href="#demarche">{language.text("Notre méthode", "Our approach")}</a><a href="#informations">{language.text("Informations", "Information")}</a><a href="#questions">{language.text("Questions", "FAQs")}</a><a href="#contact-service">{language.text("Contact", "Contact")}</a></div></nav>
        <section class="section editorial-split container" id="apercu"><div class="editorial-copy"><p class="eyebrow">{language.text("VUE D’ENSEMBLE", "OVERVIEW")}</p><h2>{lead}</h2><p>{explanation}</p><a class="text-link" href=contact_href.clone()>{language.text("Parler à notre équipe", "Talk to our team")} " ↗"</a></div><div inner_html=editorial_media></div></section>
        <section class="section service-offers" id="prestations"><div class="container"><div class="section-heading-row"><div><p class="eyebrow">{language.text("NOS RÉPONSES", "WHAT WE OFFER")}</p><h2>{language.text("Un service adapté à votre besoin.", "A service shaped around your needs.")}</h2></div><p>{language.text("Les modalités précises sont confirmées avec l’équipe avant toute prestation.", "The team confirms the precise terms before any service.")}</p></div><div class="offer-grid">{offers.into_iter().enumerate().map(|(index, (name, description))| view! { <article class="offer-card"><span class="card-number">{format!("{:02}", index + 1)}</span><h3>{name}</h3><p>{description}</p></article> }).collect_view()}</div></div></section>
        <section class="section situations-section" id="situations"><div class="container"><div class="section-heading-row"><div><p class="eyebrow">{language.text("APPLICATIONS", "APPLICATIONS")}</p><h2>{language.text("Une réponse qui part de votre situation.", "Start with your specific situation.")}</h2></div><p>{language.text("Ces exemples vous aident à préparer votre demande. Les prestations disponibles sont confirmées directement par SAPTRANS.", "These examples help you prepare an enquiry. SAPTRANS confirms which services are available.")}</p></div><div class="situation-grid">{situations.into_iter().enumerate().map(|(index, (title, description))| view! { <article class="situation-card"><span>{format!("0{}", index + 1)}</span><h3>{title}</h3><p>{description}</p></article> }).collect_view()}</div><a class="text-link" href=language.href("/secteurs")>{language.text("Explorer les secteurs", "Explore sectors")} " ↗"</a></div></section>
        <section class="section process-section" id="demarche"><div class="container"><p class="eyebrow">{language.text("VOTRE DEMANDE", "YOUR ENQUIRY")}</p><h2>{language.text("Un parcours simple, de la demande à la réponse.", "A clear path from enquiry to response.")}</h2><div class="process-grid"><div><span>"01"</span><h3>{language.text("Décrivez", "Describe")}</h3><p>{language.text("Indiquez votre besoin, la localisation et les contraintes connues.", "Share your needs, location and known requirements.")}</p></div><div><span>"02"</span><h3>{language.text("Nous étudions", "We review")}</h3><p>{language.text("L’équipe vérifie les informations nécessaires à une réponse pertinente.", "The team checks the information needed for a useful response.")}</p></div><div><span>"03"</span><h3>{language.text("Nous répondons", "We respond")}</h3><p>{language.text("Un interlocuteur vous indique les modalités disponibles.", "A contact explains the options currently available.")}</p></div></div></div></section>
        <div inner_html=source_section></div>
        <section class="section faq-section container" id="questions"><div><p class="eyebrow">{language.text("QUESTIONS FRÉQUENTES", "FREQUENT QUESTIONS")}</p><h2>{language.text("Avant de nous contacter.", "Before you get in touch.")}</h2></div><div class="faq-list"><details><summary>{language.text("Quelles informations fournir ?", "What information should I provide?")}</summary><p>{language.text("Précisez le service recherché, le lieu, les dates souhaitées et toute contrainte utile. Nous vous demanderons les compléments nécessaires.", "Share the service, location, preferred dates and any useful requirements. We will ask for anything else we need.")}</p></details><details><summary>{language.text("Comment connaître les disponibilités ?", "How can I check availability?")}</summary><p>{language.text("Envoyez une demande au service concerné. Les disponibilités, horaires et conditions sont confirmés directement par SAPTRANS.", "Send an enquiry to the relevant team. SAPTRANS confirms availability, hours and terms directly.")}</p></details></div></section>
        <section class="band" id="contact-service"><div class="container band-inner"><div><p class="eyebrow">{language.text("PARLONS DE VOTRE PROJET", "LET’S DISCUSS YOUR PROJECT")}</p><h2>{language.text("La bonne réponse commence ici.", "The right answer starts here.")}</h2></div><p>{language.text("Décrivez votre demande pour être orienté vers le bon service.", "Describe your enquiry so we can connect you with the right team.")}</p><a class="button button-light" href=contact_href>{language.text("Faire une demande", "Send an enquiry")}</a></div></section>
    </main> }.to_html()
}
