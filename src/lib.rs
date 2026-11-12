use leptos::prelude::*;

pub mod activities;
#[cfg(feature = "ssr")]
pub mod admin;
#[cfg(feature = "ssr")]
pub mod contact;
pub mod content;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Language {
    Fr,
    En,
}

impl Language {
    pub fn from_query(query: &str) -> Self {
        if query
            .split('&')
            .any(|part| part.eq_ignore_ascii_case("lang=en"))
        {
            Self::En
        } else {
            Self::Fr
        }
    }

    pub fn code(self) -> &'static str {
        match self {
            Self::Fr => "fr",
            Self::En => "en",
        }
    }

    pub fn text(self, french: &'static str, english: &'static str) -> &'static str {
        match self {
            Self::Fr => french,
            Self::En => english,
        }
    }

    /// Add the selected language while preserving any in-page anchor.
    pub fn href(self, path: &str) -> String {
        let (base, fragment) = path.split_once('#').unwrap_or((path, ""));
        let separator = if base.contains('?') { '&' } else { '?' };
        if fragment.is_empty() {
            format!("{base}{separator}lang={}", self.code())
        } else {
            format!("{base}{separator}lang={}#{fragment}", self.code())
        }
    }
}

#[derive(Clone, Copy)]
pub enum Page {
    Home,
    Transport,
    Equipment,
    Inspection,
    RealEstate,
    Sectors,
    Projects,
    Locations,
    News,
    Search,
    Legal,
    Company,
    Contact,
    ContactSuccess,
    ContactError,
}

fn directory_page(language: Language, page: Page) -> String {
    let (label, title, intro, points) = match page {
        Page::Sectors => (
            language.text("NOS SECTEURS", "OUR SECTORS"),
            language.text("Des réponses pensées pour vos réalités métier.", "Solutions shaped around your industry."),
            language.text("Les contenus de l’ancien site présentent des interventions liées au BTP, aux carrières, au bois, à la distribution et aux opérations industrielles. Chaque demande est étudiée selon ses contraintes réelles.", "SAPTRANS’s archived website describes work related to construction, quarries, timber, distribution and industrial operations. Each request is reviewed against its actual requirements."),
            [language.text("BTP et travaux publics", "Construction and public works"), language.text("Carrières et agrégats", "Quarries and aggregates"), language.text("Bois et grumes", "Timber and logs"), language.text("Industrie et distribution", "Industry and distribution")],
        ),
        Page::Projects => (
            language.text("RÉALISATIONS", "PROJECTS"),
            language.text("Des opérations documentées, bientôt réunies ici.", "Documented operations, gathered here soon."),
            language.text("Les réalisations seront publiées avec l’accord des clients et des éléments vérifiables : contexte, intervention, lieu et résultats. Aucun cas client n’est inventé pour cette maquette.", "Projects will be published with client approval and verifiable details: context, work performed, location and outcomes. No client case study has been invented for this preview."),
            [language.text("Transport & logistique", "Freight & logistics"), language.text("Location d’engins", "Equipment rental"), language.text("Contrôle technique", "Vehicle inspection"), language.text("Projets immobiliers", "Real-estate projects")],
        ),
        Page::Locations => (
            language.text("NOS IMPLANTATIONS", "OUR LOCATIONS"),
            language.text("Des contacts locaux pour vos demandes.", "Local contacts for your enquiries."),
            language.text("Les coordonnées et horaires doivent être confirmés avant publication. Les informations disponibles dans l’archive distinguent la direction à Douala-Nyalla et les centres AutoRelax à Douala-Yassa et Limbé.", "Addresses and opening hours must be confirmed before publication. Archived information distinguishes the head office in Douala-Nyalla and AutoRelax centres in Douala-Yassa and Limbe."),
            [language.text("Direction générale · Douala-Nyalla", "Head office · Douala-Nyalla"), language.text("AutoRelax · Douala-Yassa", "AutoRelax · Douala-Yassa"), language.text("AutoRelax · Limbé", "AutoRelax · Limbe"), language.text("Zones desservies à confirmer selon la prestation", "Service areas depend on the service and must be confirmed")],
        ),
        Page::Legal => (
            language.text("INFORMATIONS LÉGALES", "LEGAL INFORMATION"),
            language.text("Mentions légales et confidentialité.", "Legal notice and privacy."),
            language.text("Les mentions légales complètes, l’identité de l’hébergeur et les informations de confidentialité doivent être confirmées par SAPTRANS avant la mise en production. Aucun responsable de publication ni numéro d’immatriculation n’est inventé.", "The full legal notice, hosting provider details and privacy information must be confirmed by SAPTRANS before launch. No publisher details or registration numbers have been invented."),
            [language.text("Éditeur et responsable de publication à confirmer", "Publisher and editor to be confirmed"), language.text("Hébergeur à renseigner", "Hosting provider to be listed"), language.text("Politique de confidentialité à valider", "Privacy policy to be approved"), language.text("Droits des médias à confirmer", "Media rights to be confirmed")],
        ),
        _ => (
            language.text("ACTUALITÉS", "NEWS"),
            language.text("Les nouvelles de SAPTRANS.", "The latest from SAPTRANS."),
            language.text("Les actualités seront publiées lorsqu’une information datée et validée sera disponible. Cette rubrique est prête à accueillir les opérations, équipements, projets et recrutements confirmés.", "News will be published when dated and verified information is available. This section is ready for confirmed operations, equipment, projects and recruitment updates."),
            [language.text("Opérations", "Operations"), language.text("Équipements", "Equipment"), language.text("Projets", "Projects"), language.text("Vie de l’entreprise", "Company updates")],
        ),
    };
    let contact = language.href("/contact#demande");
    view! { <main>
        <section class="page-hero"><div class="container"><p class="eyebrow">{label}</p><h1>{title}</h1><p>{intro}</p><a class="button" href=contact.clone()>{language.text("Contacter SAPTRANS", "Contact SAPTRANS")}</a></div></section>
        <section class="container section"><p class="eyebrow">{language.text("INFORMATIONS DISPONIBLES", "AVAILABLE INFORMATION")}</p><h2>{language.text("Une présentation fiable commence par des faits.", "A reliable presentation starts with facts.")}</h2><div class="cards directory-cards">{points.into_iter().enumerate().map(|(i, point)| view! { <article class="card"><span class="card-number">{format!("{:02}", i + 1)}</span><h3>{point}</h3></article> }).collect_view()}</div></section>
        <section class="band"><div class="container band-inner"><div><p class="eyebrow">{language.text("VOTRE PROJET", "YOUR PROJECT")}</p><h2>{language.text("Parlons de votre besoin.", "Let’s discuss your needs.")}</h2></div><p>{language.text("L’équipe SAPTRANS vous orientera vers le service concerné.", "The SAPTRANS team will connect you with the relevant service.")}</p><a class="button button-light" href=contact>{language.text("Prendre contact", "Get in touch")}</a></div></section>
    </main> }.to_html()
}

fn search_page(language: Language, query: &str) -> String {
    let entries = [
        (
            language.text("Transport & logistique", "Freight & logistics"),
            language.text(
                "Transport de marchandises, conteneurs, bois et équipements.",
                "Freight, container, timber and equipment transport.",
            ),
            "/transport-logistique",
            language.text(
                "transport logistique marchandises conteneur grumes bois camion industrie",
                "freight logistics goods container logs timber truck industry",
            ),
        ),
        (
            language.text("Location d’engins", "Equipment rental"),
            language.text(
                "Informations et demandes de location pour les opérations de chantier.",
                "Information and rental enquiries for worksite operations.",
            ),
            "/location-engins",
            language.text(
                "location engins chantier équipement pelle bulldozer",
                "rental equipment construction excavator bulldozer",
            ),
        ),
        (
            language.text(
                "Contrôle technique AutoRelax",
                "AutoRelax vehicle inspection",
            ),
            language.text(
                "Informations sur les centres et la préparation d’une visite.",
                "Information about centres and preparing for an inspection.",
            ),
            "/controle-technique",
            language.text(
                "contrôle technique voiture automobile centre douala limbè yassa",
                "vehicle inspection automotive car centre douala limbe yassa",
            ),
        ),
        (
            language.text("Immobilier", "Real estate"),
            language.text(
                "Contactez SAPTRANS pour connaître les biens et projets disponibles.",
                "Contact SAPTRANS for information about available properties and projects.",
            ),
            "/immobilier",
            language.text(
                "immobilier bien terrain projet logement vente location",
                "real estate property land project housing sale rental",
            ),
        ),
        (
            language.text("L’entreprise", "Company"),
            language.text(
                "Présentation et histoire de Sapou Transport SARL.",
                "Company overview and history of Sapou Transport SARL.",
            ),
            "/entreprise",
            language.text(
                "entreprise histoire sapou transport cameroun saptrans",
                "company history sapou transport cameroon saptrans",
            ),
        ),
        (
            language.text("Secteurs", "Sectors"),
            language.text(
                "BTP, carrières, bois, industrie et distribution.",
                "Construction, quarries, timber, industry and distribution.",
            ),
            "/secteurs",
            language.text(
                "secteur btp carrière agrégat bois industrie distribution",
                "sectors construction quarry aggregate timber industry distribution",
            ),
        ),
        (
            language.text("Implantations", "Locations"),
            language.text(
                "Direction générale et centres mentionnés au Cameroun.",
                "Head office and centres listed in Cameroon.",
            ),
            "/implantations",
            language.text(
                "implantation agence centre douala nyalla yassa limbe limbé",
                "locations office centre douala nyalla yassa limbe",
            ),
        ),
        (
            language.text("Réalisations", "Projects"),
            language.text(
                "Études de cas vérifiées, dès validation de leur publication.",
                "Verified case studies, once publication is approved.",
            ),
            "/realisations",
            language.text(
                "réalisation projet client opération référence",
                "projects client operations case study",
            ),
        ),
    ];
    let normalized = query.trim().to_lowercase();
    let results = entries
        .into_iter()
        .filter(|(_, _, _, terms)| {
            normalized.is_empty() || terms.to_lowercase().contains(&normalized)
        })
        .collect::<Vec<_>>();
    let count = results.len();
    view! { <main class="container section search-page"><p class="eyebrow">{language.text("RECHERCHE SAPTRANS", "SAPTRANS SEARCH")}</p><h1>{language.text("Que recherchez-vous ?", "What are you looking for?")}</h1><form class="search-form" method="get" action="/recherche"><input type="hidden" name="lang" value=language.code()/><label for="q">{language.text("Activité, service ou lieu", "Service, activity or location")}</label><div class="search-control"><input id="q" name="q" type="search" value=query placeholder=language.text("Ex. transport, engins, Douala…", "E.g. freight, equipment, Douala…")/><button class="button" type="submit">{language.text("Rechercher", "Search")}</button></div></form><p class="search-count">{if normalized.is_empty() { language.text("Parcourez les principales rubriques.", "Browse the main sections.") } else if count == 0 { language.text("Aucun résultat. Essayez un autre terme ou contactez SAPTRANS.", "No results. Try another term or contact SAPTRANS.") } else if language == Language::Fr { "Résultats dans les rubriques publiées :" } else { "Results in published sections:" }}</p><div class="cards search-results">{results.into_iter().map(|(title, description, path, _)| { let href = language.href(path); view! { <article class="card"><h2>{title}</h2><p>{description}</p><a href=href>{language.text("Consulter", "View section")} "→"</a></article> } }).collect_view()}</div></main> }.to_html()
}

impl Page {
    /// Canonical destinations for verified legacy pages retained in the new site.
    pub fn legacy_destination(path: &str) -> Option<&'static str> {
        match path.trim_end_matches('/') {
            "/nos-services" | "/nos-services/apercu-services" | "/services" => Some("/"),
            "/nos-services/transport-et-logistique" | "/services/transport-et-logistique" => {
                Some("/transport-logistique")
            }
            "/nos-services/location-engins" | "/services/location-dengins" => {
                Some("/location-engins")
            }
            "/nos-services/visite-technique" | "/services/visite-technique" => {
                Some("/controle-technique")
            }
            "/entreprise/historique" | "/qui-sommes-nous" => Some("/entreprise"),
            "/contacts" => Some("/contact"),
            _ => None,
        }
    }

    pub fn from_path(path: &str) -> Option<Self> {
        match path.trim_end_matches('/') {
            "" => Some(Self::Home),
            "/transport-logistique" => Some(Self::Transport),
            "/location-engins" => Some(Self::Equipment),
            "/controle-technique" => Some(Self::Inspection),
            "/immobilier" => Some(Self::RealEstate),
            "/secteurs" => Some(Self::Sectors),
            "/realisations" => Some(Self::Projects),
            "/implantations" => Some(Self::Locations),
            "/actualites" => Some(Self::News),
            "/recherche" => Some(Self::Search),
            "/mentions-legales" => Some(Self::Legal),
            "/entreprise" => Some(Self::Company),
            "/contact" => Some(Self::Contact),
            "/contact/merci" => Some(Self::ContactSuccess),
            "/contact/erreur" => Some(Self::ContactError),
            _ => None,
        }
    }

    pub fn path(self) -> &'static str {
        match self {
            Self::Home => "/",
            Self::Transport => "/transport-logistique",
            Self::Equipment => "/location-engins",
            Self::Inspection => "/controle-technique",
            Self::RealEstate => "/immobilier",
            Self::Sectors => "/secteurs",
            Self::Projects => "/realisations",
            Self::Locations => "/implantations",
            Self::News => "/actualites",
            Self::Search => "/recherche",
            Self::Legal => "/mentions-legales",
            Self::Company => "/entreprise",
            Self::Contact => "/contact",
            Self::ContactSuccess => "/contact/merci",
            Self::ContactError => "/contact/erreur",
        }
    }

    pub fn title(self, language: Language) -> &'static str {
        match (self, language) {
            (Self::Home, Language::Fr) => {
                "SAPTRANS — Transport, immobilier et services spécialisés"
            }
            (Self::Home, Language::En) => "SAPTRANS — Freight, real estate and specialist services",
            (Self::Transport, Language::Fr) => "Transport et logistique — SAPTRANS",
            (Self::Transport, Language::En) => "Freight and logistics — SAPTRANS",
            (Self::Equipment, Language::Fr) => "Location d’engins — SAPTRANS",
            (Self::Equipment, Language::En) => "Equipment rental — SAPTRANS",
            (Self::Inspection, Language::Fr) => "Contrôle technique AutoRelax — SAPTRANS",
            (Self::Inspection, Language::En) => "AutoRelax vehicle inspection — SAPTRANS",
            (Self::RealEstate, Language::Fr) => "Immobilier — SAPTRANS",
            (Self::RealEstate, Language::En) => "Real estate — SAPTRANS",
            (Self::Sectors, Language::Fr) => "Nos secteurs — SAPTRANS",
            (Self::Sectors, Language::En) => "Our sectors — SAPTRANS",
            (Self::Projects, Language::Fr) => "Réalisations — SAPTRANS",
            (Self::Projects, Language::En) => "Projects — SAPTRANS",
            (Self::Locations, Language::Fr) => "Nos implantations — SAPTRANS",
            (Self::Locations, Language::En) => "Our locations — SAPTRANS",
            (Self::News, Language::Fr) => "Actualités — SAPTRANS",
            (Self::News, Language::En) => "News — SAPTRANS",
            (Self::Search, Language::Fr) => "Recherche — SAPTRANS",
            (Self::Search, Language::En) => "Search — SAPTRANS",
            (Self::Legal, Language::Fr) => "Mentions légales et confidentialité — SAPTRANS",
            (Self::Legal, Language::En) => "Legal notice and privacy — SAPTRANS",
            (Self::Company, Language::Fr) => "L’entreprise — SAPTRANS",
            (Self::Company, Language::En) => "About SAPTRANS",
            (Self::Contact, Language::Fr) => "Contact — SAPTRANS",
            (Self::Contact, Language::En) => "Contact — SAPTRANS",
            (Self::ContactSuccess, Language::Fr) => "Demande reçue — SAPTRANS",
            (Self::ContactSuccess, Language::En) => "Enquiry received — SAPTRANS",
            (Self::ContactError, Language::Fr) => "Demande à vérifier — SAPTRANS",
            (Self::ContactError, Language::En) => "Check your enquiry — SAPTRANS",
        }
    }

    pub fn description(self, language: Language) -> &'static str {
        match (self, language) {
            (Self::Home, Language::Fr) => {
                "Découvrez les activités de SAPTRANS : transport et logistique, immobilier, location d’engins et contrôle technique AutoRelax au Cameroun."
            }
            (Self::Home, Language::En) => {
                "Explore SAPTRANS services: freight and logistics, real estate, equipment rental and AutoRelax vehicle inspections in Cameroon."
            }
            (Self::Transport, Language::Fr) => {
                "Solutions de transport et logistique SAPTRANS pour marchandises, conteneurs, bois et équipements. Décrivez votre besoin."
            }
            (Self::Transport, Language::En) => {
                "SAPTRANS freight and logistics for goods, containers, timber and equipment. Tell us what you need."
            }
            (Self::RealEstate, Language::Fr) => {
                "Contactez SAPTRANS pour obtenir des informations confirmées sur son activité immobilière."
            }
            (Self::RealEstate, Language::En) => {
                "Contact SAPTRANS for confirmed information about its real-estate activity."
            }
            (Self::Equipment, Language::Fr) => {
                "Renseignez-vous auprès de SAPTRANS sur la location d’engins, les disponibilités et les conditions."
            }
            (Self::Equipment, Language::En) => {
                "Ask SAPTRANS about equipment rental, availability and terms."
            }
            (Self::Inspection, Language::Fr) => {
                "Informations sur AutoRelax et ses centres de contrôle technique mentionnés au Cameroun."
            }
            (Self::Inspection, Language::En) => {
                "Information about AutoRelax and its vehicle inspection centres in Cameroon."
            }
            (Self::Company, Language::Fr) => {
                "SAPTRANS, Sapou Transport SARL : activités et informations institutionnelles issues des sources de l’entreprise."
            }
            (Self::Company, Language::En) => {
                "SAPTRANS, Sapou Transport SARL: company activities and institutional information."
            }
            (Self::Contact, Language::Fr) => {
                "Contactez SAPTRANS et adressez votre demande au service concerné."
            }
            (Self::Contact, Language::En) => {
                "Contact SAPTRANS and direct your enquiry to the relevant team."
            }
            (Self::Sectors, Language::Fr) => {
                "Découvrez les secteurs associés aux activités SAPTRANS et contactez l’équipe pour confirmer votre besoin."
            }
            (Self::Sectors, Language::En) => {
                "Explore industries related to SAPTRANS services and contact the team to confirm your requirements."
            }
            (Self::Projects, Language::Fr) => {
                "Réalisations SAPTRANS documentées et vérifiées, publiées avec l’accord des clients."
            }
            (Self::Projects, Language::En) => {
                "Verified SAPTRANS projects, published with client approval."
            }
            (Self::Locations, Language::Fr) => {
                "Implantations SAPTRANS et centres AutoRelax mentionnés au Cameroun. Coordonnées à confirmer."
            }
            (Self::Locations, Language::En) => {
                "SAPTRANS locations and AutoRelax centres listed in Cameroon. Contact details to be confirmed."
            }
            (Self::News, Language::Fr) => "Actualités et informations vérifiées de SAPTRANS.",
            (Self::News, Language::En) => "Verified news and updates from SAPTRANS.",
            (Self::Search, Language::Fr) => {
                "Recherchez les activités, pages et informations publiées par SAPTRANS."
            }
            (Self::Search, Language::En) => {
                "Search SAPTRANS activities, pages and published information."
            }
            (Self::Legal, Language::Fr) => {
                "Mentions légales et informations de confidentialité de SAPTRANS."
            }
            (Self::Legal, Language::En) => "SAPTRANS legal notice and privacy information.",
            (Self::ContactSuccess, Language::Fr) => {
                "Votre demande de contact a bien été reçue par SAPTRANS."
            }
            (Self::ContactSuccess, Language::En) => "Your enquiry has been received by SAPTRANS.",
            (Self::ContactError, Language::Fr) => "Vérifiez votre demande de contact SAPTRANS.",
            (Self::ContactError, Language::En) => "Please check your SAPTRANS enquiry.",
        }
    }
}

#[component]
pub fn Site(
    page: Page,
    language: Language,
    #[prop(optional)] query: String,
    #[prop(optional)] reference: String,
    #[prop(optional)] selected_service: String,
) -> impl IntoView {
    let home = language.href("/");
    let transport = language.href("/transport-logistique");
    let equipment = language.href("/location-engins");
    let inspection = language.href("/controle-technique");
    let real_estate = language.href("/immobilier");
    let company = language.href("/entreprise");
    let contact = language.href("/contact");
    let sectors = language.href("/secteurs");
    let projects = language.href("/realisations");
    let locations = language.href("/implantations");
    let news = language.href("/actualites");
    let legal = language.href("/mentions-legales");
    let search = language.href("/recherche");
    let contact_form = language.href("/contact#demande");
    let request_reference = reference;
    let language_switch = match language {
        Language::Fr => Language::En.href(page.path()),
        Language::En => Language::Fr.href(page.path()),
    };
    let language_switch_label = match language {
        Language::Fr => "EN",
        Language::En => "FR",
    };

    let body = match page {
        Page::Home => view! { <main>
            <section class="hero" aria-labelledby="home-title">
                <picture class="hero-media">
                    <img src="/media/legacy/d697a1526c31005b.jpg" alt="" fetchpriority="high" onerror={format!("this.onerror=function(){{this.hidden=true}};this.src='{}'", "https://saptrans.net/wp-content/uploads/2021/06/mobile_slide.jpg")}/>
                </picture>
                <div class="container hero-inner"><div class="hero-copy">
                    <p class="eyebrow">{language.text("SAPTRANS — Votre partenaire de confiance", "SAPTRANS — Your trusted partner")}</p>
                    <h1 id="home-title">{language.text("Le mouvement au service de vos ambitions.", "Moving your ambitions forward.")}</h1>
                    <p>{language.text("Transport, logistique, immobilier et services spécialisés au Cameroun et en Afrique centrale.", "Freight, logistics, real estate and specialist services in Cameroon and Central Africa.")}</p>
                    <div class="actions"><a class="button button-light" href="#services">{language.text("Découvrir nos activités", "Explore our services")}</a><a class="button button-outline-light" href=contact_form.clone()>{language.text("Demander un devis", "Request a quote")}</a></div>
                </div></div>
            </section>
            <section class="quick-actions" aria-labelledby="quick-title"><div class="container">
                <div class="quick-heading"><p class="eyebrow">{language.text("UN ACCÈS DIRECT", "QUICK ACCESS")}</p><h2 id="quick-title">{language.text("Comment pouvons-nous vous accompagner ?", "How can we help you?")}</h2></div>
                <div class="quick-grid">
                    <a href=language.href("/contact?service=transport#demande")><strong>{language.text("Transport & logistique", "Freight & logistics")}</strong><span>{language.text("Demander un transport", "Request transport")} "↗"</span></a>
                    <a href=real_estate.clone()><strong>{language.text("Immobilier", "Real estate")}</strong><span>{language.text("Explorer les biens et projets", "Explore properties and projects")} "↗"</span></a>
                    <a href=equipment.clone()><strong>{language.text("Location d’engins", "Equipment rental")}</strong><span>{language.text("Consulter les équipements", "Explore equipment")} "↗"</span></a>
                    <a href=inspection.clone()><strong>{language.text("Contrôle technique", "Vehicle inspection")}</strong><span>{language.text("Découvrir AutoRelax", "Discover AutoRelax")} "↗"</span></a>
                </div>
            </div></section>
            <section class="container section editorial-intro" aria-labelledby="intro-title"><div><p class="eyebrow">{language.text("SAPTRANS AU CAMEROUN", "SAPTRANS IN CAMEROON")}</p><h2 id="intro-title">{language.text("Des métiers complémentaires. Un même engagement de terrain.", "Connected services. One commitment to the work on the ground.")}</h2></div><div><p>{language.text("SAPTRANS réunit le transport de marchandises, la location d’engins, le contrôle technique AutoRelax et l’immobilier. Chaque activité répond à des besoins précis et dispose de son propre parcours de contact.", "SAPTRANS brings together freight transport, equipment rental, AutoRelax vehicle inspection and real estate. Each activity has a dedicated way to get in touch.")}</p><a class="text-link" href=company.clone()>{language.text("Découvrir l’entreprise", "Discover the company")} "→"</a></div></section>
            <section class="container section services-section" aria-labelledby="services"><div class="section-heading-row"><div><p class="eyebrow">{language.text("NOS ACTIVITÉS", "OUR SERVICES")}</p><h2 id="services">{language.text("Quatre domaines. Des réponses concrètes.", "Four areas of expertise. Clear ways forward.")}</h2></div></div><div class="cards service-cards">
                <article class="card service-card"><figure class="service-image"><img src="/media/legacy/4e18e3d465b5b8dc.png" alt={language.text("Conteneur pris en charge dans les opérations SAPTRANS", "Container handled in SAPTRANS operations")} loading="lazy" width="640" height="481" onerror={format!("this.onerror=function(){{this.hidden=true}};this.src='{}'", "https://saptrans.net/wp-content/uploads/2021/08/transportlogistique_container_640x481.png")}/></figure><div class="service-card-body"><h3>{language.text("Transport & logistique", "Freight & logistics")}</h3><p>{language.text("Marchandises, conteneurs, bois et équipements.", "Freight, containers, timber and equipment.")}</p><a href=transport.clone()>{language.text("Explorer le transport", "Explore transport")} "→"</a></div></article>
                <article class="card service-card"><figure class="service-image"><img src="/media/legacy/c91ba6dca26b2637.png" alt={language.text("Engin de chantier présenté par SAPTRANS", "Worksite equipment presented by SAPTRANS")} loading="lazy" width="640" height="480" onerror={format!("this.onerror=function(){{this.hidden=true}};this.src='{}'", "https://saptrans.net/wp-content/uploads/2021/08/location_engins_640x480.png")}/></figure><div class="service-card-body"><h3>{language.text("Location d’engins", "Equipment rental")}</h3><p>{language.text("Des équipements pour vos opérations de chantier.", "Equipment for worksite operations.")}</p><a href=equipment.clone()>{language.text("Voir l’activité", "View the service")} "→"</a></div></article>
                <article class="card service-card"><figure class="service-image"><img src="/media/legacy/bb745ce26b67b29c.png" alt={language.text("Contrôle technique automobile AutoRelax", "AutoRelax vehicle inspection")} loading="lazy" width="640" height="480" onerror={format!("this.onerror=function(){{this.hidden=true}};this.src='{}'", "https://saptrans.net/wp-content/uploads/2021/07/visite_technique_service_640x480.png")}/></figure><div class="service-card-body"><h3>{language.text("Contrôle technique", "Vehicle inspection")}</h3><p>{language.text("AutoRelax à Douala et Limbé.", "AutoRelax in Douala and Limbe.")}</p><a href=inspection.clone()>{language.text("Découvrir AutoRelax", "Discover AutoRelax")} "→"</a></div></article>
                <article class="card service-card"><div class="service-image service-image-placeholder" role="img" aria-label=language.text("Espace immobilier SAPTRANS, visuel de projet à confirmer", "SAPTRANS real estate, project image to be confirmed")><span>{language.text("Immobilier", "Real estate")}</span></div><div class="service-card-body"><h3>{language.text("Immobilier", "Real estate")}</h3><p>{language.text("Biens et projets : informations sur demande.", "Properties and projects: information on request.")}</p><a href=real_estate.clone()>{language.text("Explorer l’immobilier", "Explore real estate")} "→"</a></div></article>
            </div></section>
            <section class="focus-section" aria-labelledby="transport-title"><div class="container focus-grid"><figure class="media-photo focus-photo"><img src="/media/legacy/249c267c19828cf1.png" alt={language.text("Camion de transport de bois SAPTRANS", "SAPTRANS timber transport truck")} loading="lazy" width="600" height="377" onerror={format!("this.onerror=function(){{this.hidden=true}};this.src='{}'", "https://saptrans.net/wp-content/uploads/2021/07/transportlogistique_grume_640x480-600x377.png")}/></figure><div class="focus-copy"><p class="eyebrow">{language.text("TRANSPORT & LOGISTIQUE", "FREIGHT & LOGISTICS")}</p><h2 id="transport-title">{language.text("Le terrain donne sa mesure à chaque transport.", "Every shipment starts with the realities on the ground.")}</h2><p>{language.text("Marchandises, conteneurs, bois, équipements et agrégats : SAPTRANS étudie le chargement, le trajet et les contraintes de livraison avec vous.", "Freight, containers, timber, equipment and aggregates: SAPTRANS reviews the cargo, route and delivery requirements with you.")}</p><a class="button button-light" href=transport.clone()>{language.text("Découvrir nos solutions", "Explore our solutions")}</a></div></div></section>
            <section class="estate-feature" aria-labelledby="estate-title"><div class="container estate-grid"><div class="estate-copy"><p class="eyebrow">{language.text("IMMOBILIER", "REAL ESTATE")}</p><h2 id="estate-title">{language.text("Un interlocuteur pour vos projets immobiliers.", "A contact for your real-estate plans.")}</h2><p>{language.text("Décrivez votre recherche. L’équipe SAPTRANS vous renseignera sur les biens et projets actuellement proposés.", "Tell us what you are looking for. The SAPTRANS team can advise you on currently available properties and projects.")}</p><a class="text-link" href=real_estate.clone()>{language.text("Explorer l’espace immobilier", "Explore real estate")} "→"</a></div><div class="estate-visual" aria-hidden="true"><span>"SAPTRANS"</span><strong>{language.text("Immobilier", "Real estate")}</strong></div></div></section>
            <section class="container section locations-section" aria-labelledby="locations-title"><div class="section-heading-row"><div><p class="eyebrow">{language.text("IMPLANTATIONS", "LOCATIONS")}</p><h2 id="locations-title">{language.text("Au plus près de vos opérations.", "Close to your operations.")}</h2></div><a class="text-link" href=locations.clone()>{language.text("Voir les implantations", "View locations")} "→"</a></div><div class="location-list"><a href=company.clone()><strong>"Douala-Nyalla"</strong><span>{language.text("Direction générale", "Head office")}</span><span aria-hidden="true">"↗"</span></a><a href=inspection.clone()><strong>"Douala-Yassa"</strong><span>{language.text("AutoRelax · contrôle technique", "AutoRelax · vehicle inspection")}</span><span aria-hidden="true">"↗"</span></a><a href=inspection.clone()><strong>{language.text("Limbé", "Limbe")}</strong><span>{language.text("AutoRelax · contrôle technique", "AutoRelax · vehicle inspection")}</span><span aria-hidden="true">"↗"</span></a></div></section>
            <section class="partners-section" aria-labelledby="partners-title"><div class="container partners-inner"><div class="partners-heading"><p class="eyebrow">{language.text("PARTENAIRES", "PARTNERS")}</p><h2 id="partners-title">{language.text("Avancer ensemble, sur le terrain.", "Working together, on the ground.")}</h2></div><div class="partners-copy"><p>{language.text("Chaque opération demande une coordination claire entre clients, équipes et partenaires techniques. Parlons des besoins de votre projet.", "Each operation calls for clear coordination between clients, teams and technical partners. Tell us what your project needs.")}</p><a class="text-link" href=contact.clone()>{language.text("Contacter notre équipe", "Contact our team")} "→"</a></div></div></section>
            <section class="band"><div class="container band-inner"><div><p class="eyebrow">{language.text("CONTACT COMMERCIAL", "BUSINESS ENQUIRIES")}</p><h2>{language.text("Parlons de votre prochain projet.", "Let’s discuss your next project.")}</h2></div><p>{language.text("Une demande précise nous aide à vous orienter vers le bon service.", "A clear enquiry helps us connect you with the right team.")}</p><a class="button button-light" href=contact_form.clone()>{language.text("Demander un devis", "Request a quote")}</a></div></section>
        </main> }.to_html(),
        Page::Transport => activities::activity_page(language, "transport", Some("transport")),
        Page::Equipment => activities::activity_page(language, "equipment", Some("equipment")),
        Page::Inspection => activities::activity_page(language, "inspection", Some("inspection")),
        Page::RealEstate => activities::activity_page(language, "real-estate", None),
        Page::Sectors | Page::Projects | Page::Locations | Page::News | Page::Legal => directory_page(language, page),
        Page::Search => search_page(language, &query),
        Page::Company => {
            let company_copy = if language == Language::En { content::paragraphs_en("company") } else { content::paragraphs("company") };
            let history_copy = if language == Language::En { content::paragraphs_en("history") } else { content::paragraphs("history") };
            view! { <main><section class="page-hero"><div class="container"><p class="eyebrow">{language.text("L’ENTREPRISE", "ABOUT SAPTRANS")}</p><h1>{language.text("SAPTRANS, des métiers complémentaires.", "SAPTRANS brings complementary services together.")}</h1><p>{language.text("Sapou Transport SARL développe ses activités au Cameroun autour du transport, des engins et du contrôle technique AutoRelax.", "Sapou Transport SARL operates in Cameroon across freight transport, equipment rental and AutoRelax vehicle inspections.")}</p></div></section><section class="container section narrow"><h2>{language.text("Les activités présentées par SAPTRANS", "Services presented by SAPTRANS")}</h2><p class="source-note">{language.text("Présentation adaptée du site SAPTRANS archivé le 9 novembre 2026.", "Copy adapted from SAPTRANS’s archived website, captured on 9 November 2026.")}</p>{company_copy.into_iter().map(|paragraph| view! { <p>{paragraph}</p> }).collect_view()}<details class="history-details"><summary>{language.text("Repères historiques publiés sur l’ancien site", "History published on the archived website")}</summary>{history_copy.into_iter().map(|paragraph| view! { <p>{paragraph}</p> }).collect_view()}</details><a class="text-link" href=contact.clone()>{language.text("Contacter l’équipe", "Contact our team")}" →"</a></section></main> }.to_html()
        }
        Page::Contact => view! { <main class="container section contact-page"><p class="eyebrow">{language.text("CONTACT", "CONTACT")}</p><h1>{language.text("Échangeons sur votre besoin", "Tell us what you need")}</h1><p class="intro">{language.text("Précisez le service souhaité ou contactez directement l’équipe concernée.", "Choose a service or contact the relevant team directly.")}</p><div class="contact-grid"><section class="card" id="demande"><h2>{language.text("Faire une demande", "Send an enquiry")}</h2><form method="post" action=contact.clone()>
                <div class="spam-trap" aria-hidden="true"><label for="website">Website</label><input id="website" name="website" tabindex="-1" autocomplete="off"/></div>
                <div class="form-grid">
                <div class="field"><label for="name">{language.text("Nom complet", "Full name")}</label><input id="name" name="name" required minlength="2" maxlength="120" autocomplete="name"/></div>
                <div class="field"><label for="company">{language.text("Entreprise (si applicable)", "Company (if applicable)")}</label><input id="company" name="company" maxlength="160" autocomplete="organization"/></div>
                <div class="field"><label for="phone">{language.text("Téléphone", "Phone")}</label><input id="phone" name="phone" type="tel" maxlength="32" autocomplete="tel"/></div>
                <div class="field"><label for="email">{language.text("Adresse e-mail", "Email address")}</label><input id="email" name="email" type="email" required autocomplete="email"/></div>
                <div class="field"><label for="service">{language.text("Activité concernée", "Service")}</label><select id="service" name="service" required><option value="" selected=selected_service.is_empty()>{language.text("Choisir une activité", "Select a service")}</option><option value="transport" selected=selected_service == "transport">{language.text("Transport et logistique", "Freight and logistics")}</option><option value="engins" selected=selected_service == "engins">{language.text("Location d’engins", "Equipment rental")}</option><option value="autorelax" selected=selected_service == "autorelax">"AutoRelax"</option><option value="immobilier" selected=selected_service == "immobilier">{language.text("Immobilier", "Real estate")}</option></select></div>
                <details class="form-extra field-wide"><summary>{language.text("Précisions sur la prestation (facultatif)", "Service details (optional)")}</summary><div class="form-grid"><div class="field"><label for="cargo">{language.text("Marchandise ou besoin", "Cargo or requirement")}</label><input id="cargo" name="cargo" maxlength="160"/></div>
                <div class="field"><label for="origin">{language.text("Lieu de départ", "Origin")}</label><input id="origin" name="origin" maxlength="160"/></div>
                <div class="field"><label for="destination">{language.text("Destination / localisation", "Destination / location")}</label><input id="destination" name="destination" maxlength="160"/></div>
                <div class="field"><label for="quantity">{language.text("Quantité, poids ou dimensions", "Quantity, weight or dimensions")}</label><input id="quantity" name="quantity" maxlength="120"/></div>
                <div class="field"><label for="desired_date">{language.text("Date souhaitée", "Requested date")}</label><input id="desired_date" name="desired_date" type="date"/></div>
                <div class="field"><label for="equipment">{language.text("Équipement particulier", "Specific equipment")}</label><input id="equipment" name="equipment" maxlength="160"/></div></div></details>
                <div class="field field-wide"><label for="message">{language.text("Votre demande et informations complémentaires", "Your request and additional information")}</label><textarea id="message" name="message" rows="5" required minlength="10" maxlength="4000"></textarea></div>
                </div>
                <p class="form-note">{language.text("Ne joignez pas de documents confidentiels à ce formulaire. L’équipe SAPTRANS vous contactera pour toute pièce nécessaire.", "Do not submit confidential documents through this form. The SAPTRANS team will contact you if documents are needed.")}</p>
                <button class="button" type="submit">{language.text("Envoyer la demande", "Send enquiry")}</button>
                </form></section><div class="contact-details"><section class="card"><h2>"SAPTRANS"</h2><p>{language.text("Direction générale · Douala-Nyalla Pariso, BP 191 Douala", "Head office · Nyalla Pariso, PO Box 191, Douala")}</p><p class="source-note">{language.text("Téléphone à confirmer auprès de SAPTRANS.", "Phone number to be confirmed with SAPTRANS.")}</p><p><a href="mailto:infos.sapou@groupesapou.com">"infos.sapou@groupesapou.com"</a></p></section><section class="card"><h2>"AutoRelax"</h2><p>{language.text("Coordonnées et horaires à confirmer auprès du centre.", "Contact details and opening hours to be confirmed with the centre.")}</p></section></div></div></main> }.to_html(),
        Page::ContactSuccess => view! { <main class="container section narrow"><p class="eyebrow">{language.text("CONTACT", "CONTACT")}</p><h1>{language.text("Votre demande a été reçue.", "Your enquiry has been received.")}</h1><p class="intro">{language.text("Merci. L’équipe SAPTRANS pourra vous répondre à l’adresse indiquée.", "Thank you. The SAPTRANS team will reply to the email address you provided.")}</p><p class="request-reference"><strong>{language.text("Référence de votre demande : ", "Your enquiry reference: ")}</strong><span>{if request_reference.is_empty() { language.text("non disponible", "unavailable") } else { request_reference.as_str() }}</span></p><a class="button" href=home.clone()>{language.text("Retour à l’accueil", "Back to home")}</a></main> }.to_html(),
        Page::ContactError => view! { <main class="container section narrow"><p class="eyebrow">{language.text("CONTACT", "CONTACT")}</p><h1>{language.text("Vérifiez votre demande.", "Please check your enquiry.")}</h1><p class="intro">{language.text("Le nom, l’adresse e-mail, l’activité et le message doivent être renseignés correctement. Si le problème persiste, contactez-nous par téléphone.", "Please provide a valid name, email address, service and message. If the problem persists, contact us by phone.")}</p><a class="button" href=contact_form.clone()>{language.text("Revenir au formulaire", "Return to the form")}</a></main> }.to_html(),
    };

    let theme_label = language.text("Clair", "Light");
    let language_aria = language.text("View this page in English", "Voir cette page en français");
    view! { <div class="site-shell"><a class="skip-link" href="#content">{language.text("Aller au contenu", "Skip to content")}</a><header class="header"><div class="container nav"><a class="brand" href=home.clone() aria-label="SAPTRANS"><img class="brand-logo" src="https://saptrans.net/wp-content/uploads/2021/06/logo_redisign_dunkel_224x67.png" alt="SAPTRANS" width="224" height="67" onerror="this.hidden=true;this.nextElementSibling.hidden=false"/><span class="brand-wordmark" hidden>SAPTRANS</span></a><nav class="desktop-nav" aria-label=language.text("Navigation principale", "Main navigation")>
    <a href=home.clone()>{language.text("Accueil", "Home")}</a>
    <details class="dropdown"><summary>{language.text("Nos activités", "Our services")}</summary><div class="dropdown-panel"><p class="dropdown-intro">{language.text("Nos solutions", "Our solutions")}</p><a class="dropdown-item" href=transport.clone()><strong>{language.text("Transport & logistique", "Freight & logistics")}</strong><span>{language.text("Marchandises, conteneurs et équipements", "Freight, containers and machinery")}</span></a><a class="dropdown-item" href=real_estate.clone()><strong>{language.text("Immobilier", "Real estate")}</strong><span>{language.text("Biens et projets · informations à confirmer", "Properties and projects · details to be confirmed")}</span></a><a class="dropdown-item" href=equipment.clone()><strong>{language.text("Location d’engins", "Equipment rental")}</strong><span>{language.text("Des moyens pour les chantiers", "Equipment for worksite operations")}</span></a><a class="dropdown-item" href=inspection.clone()><strong>"AutoRelax · contrôle technique"</strong><span>{language.text("Centres à Douala et Limbé", "Centres in Douala and Limbe")}</span></a></div></details>
    <details class="dropdown"><summary>{language.text("Le groupe", "The company")}</summary><div class="dropdown-panel company-panel"><p class="dropdown-intro">{language.text("Découvrir SAPTRANS", "Discover SAPTRANS")}</p><a class="dropdown-item" href=company.clone()><strong>{language.text("L’entreprise", "About SAPTRANS")}</strong><span>{language.text("Présentation et histoire", "Overview and history")}</span></a><a class="dropdown-item" href=sectors.clone()><strong>{language.text("Nos secteurs", "Our sectors")}</strong><span>{language.text("Domaines d’intervention", "Industries served")}</span></a><a class="dropdown-item" href=projects.clone()><strong>{language.text("Réalisations", "Projects")}</strong><span>{language.text("Opérations documentées", "Documented operations")}</span></a><a class="dropdown-item" href=locations.clone()><strong>{language.text("Implantations", "Locations")}</strong><span>{language.text("Sites mentionnés au Cameroun", "Locations listed in Cameroon")}</span></a><a class="dropdown-item" href=news.clone()><strong>{language.text("Actualités", "News")}</strong><span>{language.text("Informations SAPTRANS", "SAPTRANS updates")}</span></a></div></details>
    <a href=projects.clone()>{language.text("Réalisations", "Projects")}</a><a href=locations.clone()>{language.text("Implantations", "Locations")}</a><a href=contact.clone()>{language.text("Contact", "Contact")}</a>
    </nav><div class="header-controls"><a class="search-trigger" href=search.clone() aria-label=language.text("Rechercher sur le site", "Search the site")>"⌕"</a><a class="language-switch" href=language_switch aria-label=language_aria>{language_switch_label}</a><button class="theme-toggle" type="button" attr:data-theme-toggle="" attr:data-light-label=language.text("Activer le thème clair", "Switch to light theme") attr:data-dark-label=language.text("Activer le thème sombre", "Switch to dark theme") attr:data-light-mode-label=language.text("Clair", "Light") attr:data-dark-mode-label=language.text("Sombre", "Dark") aria-label=language.text("Activer le thème sombre", "Switch to dark theme") aria-pressed="false"><span class="theme-icon" aria-hidden="true">"◐"</span><span class="theme-label">{theme_label}</span></button></div><a class="button header-cta" href=contact_form.clone()>{language.text("Demander un devis", "Request a quote")}</a><details class="mobile-menu"><summary>{language.text("Menu", "Menu")}</summary><nav aria-label=language.text("Navigation mobile", "Mobile navigation")><a href=home.clone()>{language.text("Accueil", "Home")}</a><span>{language.text("Nos activités", "Our services")}</span><a href=transport.clone()>{language.text("Transport & logistique", "Freight & logistics")}</a><a href=real_estate.clone()>{language.text("Immobilier", "Real estate")}</a><a href=equipment.clone()>{language.text("Location d’engins", "Equipment rental")}</a><a href=inspection.clone()>"AutoRelax"</a><a href=sectors.clone()>{language.text("Nos secteurs", "Our sectors")}</a><a href=projects.clone()>{language.text("Réalisations", "Projects")}</a><a href=locations.clone()>{language.text("Nos implantations", "Locations")}</a><a href=company.clone()>{language.text("L’entreprise", "Company")}</a><a href=news.clone()>{language.text("Actualités", "News")}</a><a href=contact.clone()>{language.text("Contact", "Contact")}</a><a class="button" href=contact_form.clone()>{language.text("Demander un devis", "Request a quote")}</a></nav></details></div></header><div id="content" inner_html=body></div><footer class="footer"><div class="container footer-inner"><div><a class="footer-brand" href=home.clone() aria-label="SAPTRANS"><img class="footer-logo" src="https://saptrans.net/wp-content/uploads/2021/06/logo_redisign_dunkel_224x67.png" alt="SAPTRANS" width="224" height="67" onerror="this.hidden=true;this.nextElementSibling.hidden=false"/><span class="footer-wordmark" hidden>SAPTRANS</span></a><p>{language.text("Sapou Transport SARL · Douala, Cameroun", "Sapou Transport SARL · Douala, Cameroon")}</p></div><div><strong>{language.text("Activités", "Services")}</strong><p><a href=transport.clone()>{language.text("Transport & logistique", "Freight & logistics")}</a><br/><a href=real_estate.clone()>{language.text("Immobilier", "Real estate")}</a><br/><a href=equipment.clone()>{language.text("Location d’engins", "Equipment rental")}</a><br/><a href=inspection.clone()>"AutoRelax"</a></p></div><div><strong>{language.text("Découvrir", "Discover")}</strong><p><a href=sectors.clone()>{language.text("Nos secteurs", "Our sectors")}</a><br/><a href=projects.clone()>{language.text("Réalisations", "Projects")}</a><br/><a href=locations.clone()>{language.text("Implantations", "Locations")}</a><br/><a href=news.clone()>{language.text("Actualités", "News")}</a><br/><a href=company.clone()>{language.text("L’entreprise", "Company")}</a></p></div><div><strong>{language.text("Contact", "Contact")}</strong><p><a href="mailto:infos.sapou@groupesapou.com">"infos.sapou@groupesapou.com"</a></p></div></div><div class="container footer-bottom"><span>"© SAPTRANS · Sapou Transport SARL"</span><a href=legal>{language.text("Mentions légales et confidentialité", "Legal notice and privacy")}</a></div></footer></div> }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn public_routes_are_explicit() {
        assert!(Page::from_path("/immobilier/").is_some());
        assert!(Page::from_path("/recherche").is_some());
        assert!(Page::from_path("/implantations").is_some());
        assert!(Page::from_path("/mentions-legales").is_some());
        assert!(Page::from_path("/admin").is_none());
        assert_eq!(
            Page::legacy_destination("/nos-services/transport-et-logistique/"),
            Some("/transport-logistique")
        );
        assert_eq!(
            Page::legacy_destination("/services/location-dengins/"),
            Some("/location-engins")
        );
        assert_eq!(
            Page::legacy_destination("/services/visite-technique/"),
            Some("/controle-technique")
        );
        assert_eq!(Page::legacy_destination("/contacts/"), Some("/contact"));
        assert_eq!(Page::legacy_destination("/staff/sapou-kana-etienne/"), None);
    }

    #[test]
    fn key_pages_render_html_with_brand_wordmark() {
        for page in [Page::Home, Page::Transport, Page::Contact, Page::Search] {
            let html = view! { <Site page=page language=Language::Fr /> }.to_html();
            assert!(html.contains("<main"));
            assert!(html.contains("brand-wordmark"));
            assert!(html.contains("onerror=\""));
        }
    }

    #[test]
    fn all_rendered_internal_links_have_a_destination() {
        let pages = [
            Page::Home,
            Page::Transport,
            Page::Equipment,
            Page::Inspection,
            Page::RealEstate,
            Page::Sectors,
            Page::Projects,
            Page::Locations,
            Page::News,
            Page::Search,
            Page::Legal,
            Page::Company,
            Page::Contact,
            Page::ContactSuccess,
            Page::ContactError,
        ];
        for language in [Language::Fr, Language::En] {
            for page in pages {
                let html = view! { <Site page=page language=language /> }.to_html();
                for link in html.split("href=\"").skip(1) {
                    let href = link.split('"').next().unwrap_or_default();
                    if let Some(path) = href.strip_prefix('/') {
                        let path =
                            format!("/{}", path.split(['?', '#']).next().unwrap_or_default());
                        assert!(
                            Page::from_path(&path).is_some(),
                            "broken link on {}: {href}",
                            page.path()
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn search_filters_to_published_sections() {
        let html =
            view! { <Site page=Page::Search language=Language::En query="freight".to_owned() /> }
                .to_html();
        assert!(html.contains("Freight &amp; logistics") || html.contains("Freight & logistics"));
    }
}
