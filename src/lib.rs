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
    let home = language.href("/");
    let (hero_photo, feature_photo, feature_alt, photo_place) = match page {
        Page::Sectors => (
            "/media/editorial/lagos-road-crew.jpg",
            "/media/editorial/abuja-engineering-team.jpg",
            language.text(
                "Équipe d’ingénieurs équipée autour de plans à Abuja",
                "Equipped engineering team reviewing plans in Abuja",
            ),
            "Abuja, Nigeria",
        ),
        Page::Projects => (
            "/media/editorial/abuja-project-team.jpg",
            "/media/editorial/nigeria-fleet-crew.jpg",
            language.text(
                "Équipe de maintenance en tenue devant un véhicule au Nigeria",
                "Uniformed maintenance team in front of a vehicle in Nigeria",
            ),
            "Nigeria",
        ),
        Page::Locations => (
            "/media/editorial/douala-city.jpg",
            "/media/editorial/douala-construction-workers.jpg",
            language.text(
                "Ouvriers sur un chantier à Douala",
                "Workers at a construction site in Douala",
            ),
            "Douala, Cameroun",
        ),
        Page::News => (
            "/media/editorial/abuja-engineering-team.jpg",
            "/media/editorial/douala-city.jpg",
            language.text(
                "Vue de Douala au coucher du soleil",
                "View of Douala at sunset",
            ),
            "Douala, Cameroun",
        ),
        _ => (
            "/media/editorial/douala-city.jpg",
            "/media/editorial/douala-city.jpg",
            language.text(
                "Vue de Douala au coucher du soleil",
                "View of Douala at sunset",
            ),
            "Douala, Cameroun",
        ),
    };
    let content = match page {
        Page::Sectors => view! { <section class="section container directory-content"><div class="directory-lead"><p class="eyebrow">{language.text("DOMAINES D’INTERVENTION", "INDUSTRIES SERVED")}</p><h2>{language.text("Des contraintes différentes. Une préparation adaptée.", "Different requirements. Thoughtful planning.")}</h2><p>{language.text("Les secteurs ci-dessous figurent dans les prestations présentées sur l’ancien site SAPTRANS.", "The sectors below appear in the services presented on SAPTRANS’s archived website.")}</p></div><div class="directory-list">{points.into_iter().enumerate().map(|(i, point)| view! { <article><span>{format!("{:02}", i + 1)}</span><h3>{point}</h3><p>{language.text("Parlons des marchandises, du site et des contraintes propres à votre activité.", "Tell us about the cargo, site and requirements specific to your work.")}</p></article> }).collect_view()}</div></section> }.to_html(),
        Page::Locations => view! { <section class="section container directory-content"><div class="directory-lead"><p class="eyebrow">{language.text("SUR LE TERRAIN", "ON THE GROUND")}</p><h2>{language.text("Trois lieux mentionnés par SAPTRANS.", "Three locations listed by SAPTRANS.")}</h2><p>{language.text("Les lieux ci-dessous proviennent de l’ancien site. Les adresses précises et horaires doivent être confirmés avant un déplacement.", "These locations come from the archived website. Confirm exact addresses and opening hours before travelling.")}</p></div><div class="location-directory"><article><span class="eyebrow">{language.text("DIRECTION GÉNÉRALE", "HEAD OFFICE")}</span><h3>"Douala-Nyalla"</h3><p>{language.text("Nyalla Pariso · BP 191 Douala", "Nyalla Pariso · PO Box 191, Douala")}</p><a class="text-link" href=language.href("/contact")>{language.text("Contacter l’équipe", "Contact the team")} " ↗"</a></article><article><span class="eyebrow">"AUTORELAX"</span><h3>"Douala-Yassa"</h3><p>{language.text("Centre de contrôle technique · informations pratiques à confirmer.", "Vehicle inspection centre · confirm practical details.")}</p><a class="text-link" href=language.href("/controle-technique")>{language.text("Voir le service", "View the service")} " ↗"</a></article><article><span class="eyebrow">"AUTORELAX"</span><h3>{language.text("Limbé", "Limbe")}</h3><p>{language.text("Centre de contrôle technique · informations pratiques à confirmer.", "Vehicle inspection centre · confirm practical details.")}</p><a class="text-link" href=language.href("/controle-technique")>{language.text("Voir le service", "View the service")} " ↗"</a></article></div></section> }.to_html(),
        Page::Projects => view! { <section class="section container directory-content"><div class="directory-lead"><p class="eyebrow">{language.text("RÉALISATIONS", "PROJECTS")}</p><h2>{language.text("Des projets à présenter avec leurs preuves.", "Projects presented with their evidence.")}</h2><p>{language.text("Cette rubrique accueillera les opérations dont le contexte, les photographies, le lieu et les résultats peuvent être publiés avec l’accord des clients.", "This section will feature operations whose context, photos, location and outcomes can be published with client approval.")}</p></div><div class="empty-editorial"><span>"SAPTRANS / 01"</span><p>{language.text("Les études de cas vérifiées seront publiées ici.", "Verified case studies will appear here.")}</p><a class="text-link" href=language.href("/transport-logistique")>{language.text("Découvrir nos activités", "Explore our services")} " ↗"</a></div></section> }.to_html(),
        Page::News => view! { <section class="section container directory-content"><div class="directory-lead"><p class="eyebrow">{language.text("PUBLICATIONS", "PUBLICATIONS")}</p><h2>{language.text("Des nouvelles datées et vérifiées.", "Dated and verified updates.")}</h2><p>{language.text("Les articles seront publiés après validation par SAPTRANS. Aucun événement ni annonce n’est ajouté sans source.", "Updates will be published after SAPTRANS approval. No event or announcement is added without a source.")}</p></div><div class="empty-editorial"><span>"SAPTRANS / JOURNAL"</span><p>{language.text("Les prochaines actualités de l’entreprise prendront place ici.", "The company’s next updates will appear here.")}</p><a class="text-link" href=language.href("/entreprise")>{language.text("Découvrir l’entreprise", "Explore the company")} " ↗"</a></div></section> }.to_html(),
        _ => view! { <section class="section container directory-content"><div class="directory-lead"><p class="eyebrow">{language.text("INFORMATIONS À VALIDER", "INFORMATION TO CONFIRM")}</p><h2>{language.text("Une publication fondée sur des données exactes.", "Publishing with accurate information.")}</h2><p>{language.text("Ces éléments doivent être complétés et approuvés par SAPTRANS avant la mise en ligne définitive.", "These details must be completed and approved by SAPTRANS before final publication.")}</p></div><div class="directory-list">{points.into_iter().enumerate().map(|(i, point)| view! { <article><span>{format!("{:02}", i + 1)}</span><h3>{point}</h3></article> }).collect_view()}</div></section> }.to_html(),
    };
    view! { <main class="directory-page">
        <section class="page-hero directory-hero directory-hero-photo"><img class="directory-hero-image" src=hero_photo alt="" fetchpriority="high"/><div class="container"><nav class="hero-breadcrumbs" aria-label=language.text("Fil d’Ariane", "Breadcrumb")><a href=home>{language.text("Accueil", "Home")}</a><span aria-hidden="true">"›"</span><span aria-current="page">{label}</span></nav><p class="eyebrow">{label}</p><h1>{title}</h1><p>{intro}</p><a class="button button-light" href=contact.clone()>{language.text("Contacter SAPTRANS", "Contact SAPTRANS")}</a></div></section>
        <nav class="page-anchor-nav" aria-label=language.text("Sur cette page", "On this page")><div class="container"><a href="#explorer">{language.text("Explorer", "Explore")}</a><a href="#parlons">{language.text("Contact", "Contact")}</a></div></nav>
        <div id="explorer" inner_html=content></div>
        <figure class="container directory-feature-photo"><img src=feature_photo alt=feature_alt loading="lazy"/><figcaption>{language.text("Photo d’illustration prise en Afrique · ", "Illustrative photo taken in Africa · ")}{photo_place}</figcaption></figure>
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
    view! { <main class="search-page"><section class="page-hero directory-hero directory-hero-photo"><img class="directory-hero-image" src="/media/editorial/douala-city.jpg" alt="" fetchpriority="high"/><div class="container"><nav class="hero-breadcrumbs" aria-label=language.text("Fil d’Ariane", "Breadcrumb")><a href=language.href("/")>{language.text("Accueil", "Home")}</a><span aria-hidden="true">"›"</span><span aria-current="page">{language.text("Recherche", "Search")}</span></nav><p class="eyebrow">{language.text("RECHERCHE SAPTRANS", "SAPTRANS SEARCH")}</p><h1>{language.text("Que recherchez-vous ?", "What are you looking for?")}</h1><p>{language.text("Retrouvez une activité, un service ou une implantation.", "Find a service, activity or location.")}</p></div></section><section class="container section"><form class="search-form" method="get" action="/recherche"><input type="hidden" name="lang" value=language.code()/><label for="q">{language.text("Activité, service ou lieu", "Service, activity or location")}</label><div class="search-control"><input id="q" name="q" type="search" value=query placeholder=language.text("Ex. transport, engins, Douala…", "E.g. freight, equipment, Douala…")/><button class="button" type="submit">{language.text("Rechercher", "Search")}</button></div></form><p class="search-count">{if normalized.is_empty() { language.text("Parcourez les principales rubriques.", "Browse the main sections.") } else if count == 0 { language.text("Aucun résultat. Essayez un autre terme ou contactez SAPTRANS.", "No results. Try another term or contact SAPTRANS.") } else if language == Language::Fr { "Résultats dans les rubriques publiées :" } else { "Results in published sections:" }}</p><div class="cards search-results">{results.into_iter().map(|(title, description, path, _)| { let href = language.href(path); view! { <article class="card"><h2>{title}</h2><p>{description}</p><a href=href>{language.text("Consulter", "View section")} "→"</a></article> } }).collect_view()}</div></section></main> }.to_html()
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
fn ClosingVisual(language: Language, page: Page) -> impl IntoView {
    let (section_id, title, description, action, href) = match page {
        Page::Contact => (
            "contact-fin",
            language.text("Votre demande commence ici.", "Your enquiry starts here."),
            language.text("Décrivez votre besoin dans le formulaire pour joindre l’équipe concernée.", "Describe your needs in the form to reach the right team."),
            language.text("Accéder au formulaire", "Go to the form"),
            language.href("/contact#demande"),
        ),
        Page::ContactSuccess => (
            "contact-fin",
            language.text("Découvrez nos activités.", "Explore our services."),
            language.text("Merci pour votre demande. Parcourez les activités de SAPTRANS pendant que notre équipe prépare sa réponse.", "Thank you for your enquiry. Explore SAPTRANS services while our team prepares a response."),
            language.text("Voir les activités", "View our services"),
            language.href("/#services"),
        ),
        Page::ContactError => (
            "contact-fin",
            language.text("Reprenons votre demande.", "Let's revisit your enquiry."),
            language.text("Vérifiez les champs du formulaire et envoyez votre demande à nouveau.", "Check the form fields and send your enquiry again."),
            language.text("Revenir au formulaire", "Return to the form"),
            language.href("/contact#demande"),
        ),
        _ => (
            "contact-service",
            language.text("Parlons de votre prochain projet.", "Let’s discuss your next project."),
            language.text("Une demande précise nous aide à vous orienter vers le bon service.", "A clear enquiry helps us connect you with the right team."),
            language.text("Demander un devis", "Request a quote"),
            language.href("/contact#demande"),
        ),
    };
    let section_id = match page {
        Page::Company => "contact-entreprise",
        Page::Sectors | Page::Projects | Page::Locations | Page::News | Page::Legal => "parlons",
        _ => section_id,
    };
    let (photo, photo_credit) = match page {
        Page::Equipment => (
            "/media/editorial/lagos-road-crew.jpg",
            language.text(
                "Photo d’illustration · Lagos, Nigeria",
                "Illustrative photo · Lagos, Nigeria",
            ),
        ),
        Page::Inspection => (
            "/media/editorial/ghana-inspection-bay.png",
            language.text("Photo d’illustration · Ghana", "Illustrative photo · Ghana"),
        ),
        Page::RealEstate => (
            "/media/editorial/lagos-residential-building.jpg",
            language.text(
                "Photo d’illustration · Lagos, Nigeria",
                "Illustrative photo · Lagos, Nigeria",
            ),
        ),
        _ => (
            "/media/editorial/douala-city.jpg",
            language.text(
                "Photo d’illustration · Douala, Cameroun",
                "Illustrative photo · Douala, Cameroon",
            ),
        ),
    };
    view! { <section class="home-contact-visual" id=section_id aria-labelledby="closing-title"><img class="home-contact-image" src=photo alt="" loading="lazy" decoding="async"/><div class="container home-contact-content"><p class="eyebrow">{language.text("CONTACT SAPTRANS", "CONTACT SAPTRANS")}</p><h2 id="closing-title">{title}</h2><p>{description}</p><a class="button button-light" href=href>{action}<span aria-hidden="true">" ↗"</span></a></div><small class="home-contact-credit">{photo_credit}</small></section> }
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
                    <img src="/media/editorial/abuja-engineering-team.jpg" alt="" width="1600" height="1200" fetchpriority="high" decoding="async"/>
                </picture>
                <div class="container hero-inner"><div class="hero-copy">
                    <p class="eyebrow">{language.text("SAPTRANS — Votre partenaire de confiance", "SAPTRANS — Your trusted partner")}</p>
                    <h1 id="home-title">{language.text("Le mouvement au service de vos ambitions.", "Moving your ambitions forward.")}</h1>
                    <p>{language.text("Transport, logistique, immobilier et services spécialisés au Cameroun et en Afrique centrale.", "Freight, logistics, real estate and specialist services in Cameroon and Central Africa.")}</p>
                    <div class="actions"><a class="button button-light" href="#services">{language.text("Découvrir nos activités", "Explore our services")}</a><a class="button button-outline-light" href=contact_form.clone()>{language.text("Demander un devis", "Request a quote")}</a></div>
                </div></div><small class="hero-credit">{language.text("Photo d’illustration : Harrun Muhammad / Pexels · Abuja", "Illustrative photo: Harrun Muhammad / Pexels · Abuja")}</small>
            </section>
            <section class="partners-section" id="partenaires" aria-labelledby="partners-title"><div class="container partners-inner"><div class="partners-heading"><p class="eyebrow">{language.text("NOS CLIENTS ET PARTENAIRES", "OUR CLIENTS AND PARTNERS")}</p><h2 id="partners-title">{language.text("Clients qui font confiance à nos services", "Clients who trust our services")}</h2></div><div class="partners-copy"><h3>{language.text("Nous établissons la norme en matière de qualité de service et du matériel.", "Setting the standard for service and equipment quality.")}</h3><p>{language.text("Chez SAPTRANS, nous sommes fiers de notre réputation auprès de nos clients et partenaires et de notre présence dans plusieurs secteurs d’activité.", "At SAPTRANS, we are proud of our reputation with clients and partners and our work across several sectors.")}</p><p>{language.text("Avec SAPTRANS, vous avez un partenaire de confiance !", "With SAPTRANS, you have a trusted partner.")}</p><a class="text-link" href=contact.clone()>{language.text("Contacter notre équipe", "Contact our team")} "→"</a></div><div class="partner-logos" aria-label=language.text("Entreprises citées sur l’ancien site", "Companies featured on the former website")><figure><img src="/brand/partners/tractafric-equipment.png" alt="Tractafric Equipment" loading="lazy"/></figure><figure><img src="/brand/partners/tractafric-motors.png" alt="Tractafric Motors" loading="lazy"/></figure><figure><img src="/brand/partners/razel.png" alt="Razel" loading="lazy"/></figure><figure><img src="/brand/partners/routdaf.png" alt="Routdaf" loading="lazy"/></figure><figure><img src="/brand/partners/wijma.png" alt="Wijma" loading="lazy"/></figure><figure><img src="/brand/partners/sogea-satom.png" alt="Sogea-Satom" loading="lazy"/></figure><figure><img src="/brand/partners/siencam.png" alt="Siencam" loading="lazy"/></figure><figure><img src="/brand/partners/guinness.png" alt="Guinness" loading="lazy"/></figure><figure><img src="/brand/partners/sefeccam.png" alt="Sefeccam" loading="lazy"/></figure><figure><img src="/brand/partners/boiscam.png" alt="Boiscam" loading="lazy"/></figure><figure><img src="/brand/partners/arab-contractors.png" alt="Arab Contractors" loading="lazy"/></figure><figure><img src="/brand/partners/alpicam.png" alt="Alpicam" loading="lazy"/></figure></div></div></section>
            <section class="section home-promise" aria-labelledby="intro-title"><div class="container editorial-intro"><div><p class="eyebrow">{language.text("POURQUOI SAPTRANS ?", "WHY SAPTRANS?")}</p><h2 id="intro-title">{language.text("Un partenaire. Plusieurs solutions.", "One partner. Several solutions.")}</h2></div><div><p>{language.text("Du transport de marchandises aux opérations de chantier, du contrôle technique à l’immobilier, SAPTRANS réunit des métiers complémentaires au Cameroun. Présentez votre besoin : nous vous orientons vers l’activité concernée.", "From freight transport and worksite operations to vehicle inspection and real estate, SAPTRANS brings complementary services together in Cameroon. Tell us what you need and we will direct you to the right service.")}</p><a class="text-link" href=company.clone()>{language.text("Découvrir l’entreprise", "Discover the company")} "→"</a></div></div></section>
            <section class="container section services-section" aria-labelledby="services"><div class="section-heading-row"><div><p class="eyebrow">{language.text("NOS ACTIVITÉS", "OUR SERVICES")}</p><h2 id="services">{language.text("Quatre domaines. Des réponses concrètes.", "Four areas of expertise. Clear ways forward.")}</h2></div></div><div class="cards service-cards">
                <article class="card service-card"><figure class="service-image"><img src="/media/editorial/nigeria-fleet-crew.jpg" alt={language.text("Équipe de maintenance en tenue devant un véhicule au Nigeria, photo d’illustration", "Uniformed maintenance team in front of a vehicle in Nigeria, illustrative photo")} loading="lazy"/></figure><div class="service-card-body"><h3>{language.text("Transport & logistique", "Freight & logistics")}</h3><p>{language.text("Marchandises, conteneurs, bois et équipements.", "Freight, containers, timber and equipment.")}</p><a href=transport.clone()>{language.text("Explorer le transport", "Explore transport")} "→"</a></div></article>
                <article class="card service-card"><figure class="service-image"><img src="/media/editorial/lagos-road-crew.jpg" alt={language.text("Équipe équipée et engins sur un chantier routier à Lagos, photo d’illustration", "Equipped crew and machinery at a roadwork site in Lagos, illustrative photo")} loading="lazy"/></figure><div class="service-card-body"><h3>{language.text("Location d’engins", "Equipment rental")}</h3><p>{language.text("Des équipements pour vos opérations de chantier.", "Equipment for worksite operations.")}</p><a href=equipment.clone()>{language.text("Voir l’activité", "View the service")} "→"</a></div></article>
                <article class="card service-card"><figure class="service-image"><img src="/media/editorial/ghana-inspection-bay.png" alt={language.text("Techniciens en tenue dans un centre de contrôle au Ghana, photo d’illustration", "Uniformed technicians in a vehicle inspection centre in Ghana, illustrative photo")} loading="lazy"/></figure><div class="service-card-body"><h3>{language.text("Contrôle technique", "Vehicle inspection")}</h3><p>{language.text("AutoRelax à Douala et Limbé.", "AutoRelax in Douala and Limbe.")}</p><a href=inspection.clone()>{language.text("Découvrir AutoRelax", "Discover AutoRelax")} "→"</a></div></article>
                <article class="card service-card"><figure class="service-image"><img src="/media/editorial/abuja-project-team.jpg" alt={language.text("Équipe de projet immobilier en tenue à Abuja, photo d’illustration", "Property project team in safety gear in Abuja, illustrative photo")} loading="lazy"/></figure><div class="service-card-body"><h3>{language.text("Immobilier", "Real estate")}</h3><p>{language.text("Biens et projets : informations sur demande.", "Properties and projects: information on request.")}</p><a href=real_estate.clone()>{language.text("Explorer l’immobilier", "Explore real estate")} "→"</a></div></article>
            </div></section>
            <section class="sector-strip" aria-label=language.text("Secteurs desservis", "Industries served")><div class="container sector-strip-inner"><p class="eyebrow">{language.text("NOS SECTEURS", "OUR SECTORS")}</p><div class="sector-list"><span>{language.text("BTP et travaux publics", "Construction")}</span><span>{language.text("Carrières", "Quarries")}</span><span>{language.text("Bois", "Timber")}</span><span>{language.text("Industrie et distribution", "Industry and distribution")}</span><a href=sectors.clone()>{language.text("Explorer les secteurs", "Explore sectors")} " ↗"</a></div></div></section>
            <section class="focus-section" aria-labelledby="transport-title"><div class="container focus-grid"><figure class="media-photo focus-photo"><img src="/media/editorial/nigeria-fleet-crew.jpg" alt={language.text("Équipe de maintenance en tenue devant un véhicule au Nigeria, photo d’illustration", "Uniformed maintenance team in front of a vehicle in Nigeria, illustrative photo")} loading="lazy"/></figure><div class="focus-copy"><p class="eyebrow">{language.text("TRANSPORT & LOGISTIQUE", "FREIGHT & LOGISTICS")}</p><h2 id="transport-title">{language.text("Le terrain donne sa mesure à chaque transport.", "Every shipment starts with the realities on the ground.")}</h2><p>{language.text("Marchandises, conteneurs, bois, équipements et agrégats : SAPTRANS étudie le chargement, le trajet et les contraintes de livraison avec vous.", "Freight, containers, timber, equipment and aggregates: SAPTRANS reviews the cargo, route and delivery requirements with you.")}</p><a class="button button-light" href=transport.clone()>{language.text("Découvrir nos solutions", "Explore our solutions")}</a></div></div></section>
            <section class="estate-feature" aria-labelledby="estate-title"><div class="container estate-grid"><div class="estate-copy"><p class="eyebrow">{language.text("IMMOBILIER", "REAL ESTATE")}</p><h2 id="estate-title">{language.text("Un interlocuteur pour vos projets immobiliers.", "A contact for your real-estate plans.")}</h2><p>{language.text("Décrivez votre recherche. L’équipe SAPTRANS vous renseignera sur les biens et projets actuellement proposés.", "Tell us what you are looking for. The SAPTRANS team can advise you on currently available properties and projects.")}</p><a class="text-link" href=real_estate.clone()>{language.text("Explorer l’espace immobilier", "Explore real estate")} "→"</a></div><figure class="estate-visual estate-visual-photo"><img src="/media/editorial/lagos-residential-building.jpg" alt={language.text("Immeuble résidentiel à Lagos, photo d’illustration", "Residential building in Lagos, illustrative photo")} loading="lazy"/><figcaption>{language.text("Photo d’illustration · Lagos, Nigeria", "Illustrative photo · Lagos, Nigeria")}</figcaption></figure></div></section>
            <section class="section home-method" aria-labelledby="method-title"><div class="container"><div class="home-method-heading"><p class="eyebrow">{language.text("NOTRE APPROCHE", "OUR APPROACH")}</p><h2 id="method-title">{language.text("Le sérieux se mesure dans la préparation.", "Good work begins with good preparation.")}</h2><p>{language.text("SAPTRANS présente des prestations qui demandent une connaissance du terrain, des moyens adaptés et des contrôles précis.", "SAPTRANS describes services that call for local understanding, suitable equipment and careful checks.")}</p></div><div class="home-method-grid"><article><span>"01"</span><h3>{language.text("Planifier le transport", "Plan the movement")}</h3><p>{language.text("Comprendre le chargement, le trajet et les conditions de livraison.", "Understand the cargo, route and delivery conditions.")}</p></article><article><span>"02"</span><h3>{language.text("Adapter les moyens", "Match the equipment")}</h3><p>{language.text("Étudier l’engin et les modalités selon le chantier.", "Review the machine and terms for the worksite.")}</p></article><article><span>"03"</span><h3>{language.text("Contrôler le véhicule", "Check the vehicle")}</h3><p>{language.text("Associer mesures par équipements et examen visuel chez AutoRelax.", "Combine equipment measurements and a visual check at AutoRelax.")}</p></article></div></div></section>
            <section class="container section locations-section" aria-labelledby="locations-title"><div class="section-heading-row"><div><p class="eyebrow">{language.text("IMPLANTATIONS", "LOCATIONS")}</p><h2 id="locations-title">{language.text("Au plus près de vos opérations.", "Close to your operations.")}</h2></div><a class="text-link" href=locations.clone()>{language.text("Voir les implantations", "View locations")} "→"</a></div><div class="location-list"><a href=company.clone()><strong>"Douala-Nyalla"</strong><span>{language.text("Direction générale", "Head office")}</span><span aria-hidden="true">"↗"</span></a><a href=inspection.clone()><strong>"Douala-Yassa"</strong><span>{language.text("AutoRelax · contrôle technique", "AutoRelax · vehicle inspection")}</span><span aria-hidden="true">"↗"</span></a><a href=inspection.clone()><strong>{language.text("Limbé", "Limbe")}</strong><span>{language.text("AutoRelax · contrôle technique", "AutoRelax · vehicle inspection")}</span><span aria-hidden="true">"↗"</span></a></div></section>

            <section class="home-finale" aria-labelledby="finale-title"><div class="container">
                <div class="home-finale-heading"><p class="eyebrow">{language.text("SUR LE TERRAIN", "ON THE GROUND")}</p><h2 id="finale-title">{language.text("Des métiers qui répondent à vos projets.", "Services shaped around your projects.")}</h2><p>{language.text("Du transport au chantier et au contrôle technique, découvrez le service qui correspond à votre demande.", "From freight to worksites and vehicle inspection, find the service that fits your enquiry.")}</p></div>
                <div class="home-finale-grid">
                    <a class="home-finale-card" href=transport.clone()><img src="/media/editorial/nigeria-fleet-crew.jpg" alt="" loading="lazy" decoding="async"/><span class="home-finale-card-copy"><small>"01 / "{language.text("TRANSPORT", "FREIGHT")}</small><strong>{language.text("Faire avancer vos marchandises.", "Move your freight forward.")}</strong><span>{language.text("Découvrir le transport", "Explore freight")} " ↗"</span></span></a>
                    <a class="home-finale-card" href=equipment.clone()><img src="/media/editorial/lagos-road-crew.jpg" alt="" loading="lazy" decoding="async"/><span class="home-finale-card-copy"><small>"02 / "{language.text("CHANTIER", "WORKSITES")}</small><strong>{language.text("Préparer les opérations de chantier.", "Prepare your worksite operations.")}</strong><span>{language.text("Découvrir les engins", "Explore equipment")} " ↗"</span></span></a>
                    <a class="home-finale-card" href=inspection.clone()><img src="/media/editorial/ghana-inspection-bay.png" alt="" loading="lazy" decoding="async"/><span class="home-finale-card-copy"><small>"03 / AUTORELAX"</small><strong>{language.text("Contrôler les véhicules.", "Inspect your vehicles.")}</strong><span>{language.text("Découvrir AutoRelax", "Explore AutoRelax")} " ↗"</span></span></a>
                </div>
                <p class="home-finale-note">{language.text("Photographies d’illustration prises en Afrique ; elles ne représentent pas les équipes SAPTRANS.", "Illustrative photographs taken in Africa; they do not depict SAPTRANS staff.")}</p>
            </div></section>
        </main> }.to_html(),
        Page::Transport => activities::activity_page(language, "transport", Some("transport")),
        Page::Equipment => activities::activity_page(language, "equipment", Some("equipment")),
        Page::Inspection => activities::activity_page(language, "inspection", Some("inspection")),
        Page::RealEstate => activities::activity_page(language, "real-estate", None),
        Page::Sectors | Page::Projects | Page::Locations | Page::News | Page::Legal => directory_page(language, page),
        Page::Search => search_page(language, &query),
        Page::Company => {
            let company_copy = if language == Language::En { content::paragraphs_en("company") } else { content::paragraphs("company") };
            view! { <main class="company-page">
                <section class="page-hero directory-hero directory-hero-photo"><img class="directory-hero-image" src="/media/editorial/abuja-engineering-team.jpg" alt="" fetchpriority="high"/><div class="container"><nav class="hero-breadcrumbs" aria-label=language.text("Fil d’Ariane", "Breadcrumb")><a href=home.clone()>{language.text("Accueil", "Home")}</a><span aria-hidden="true">"›"</span><span aria-current="page">{language.text("Entreprise", "Company")}</span></nav><p class="eyebrow">{language.text("L’ENTREPRISE", "ABOUT SAPTRANS")}</p><h1>{language.text("Une entreprise ancrée dans le mouvement.", "A company built around movement.")}</h1><p>{language.text("Sapou Transport SARL développe au Cameroun des activités de transport, de location d’engins et de contrôle technique AutoRelax.", "Sapou Transport SARL operates in Cameroon across freight transport, equipment rental and AutoRelax vehicle inspections.")}</p></div></section>
                <nav class="page-anchor-nav" aria-label=language.text("Sur cette page", "On this page")><div class="container"><a href="#presentation">{language.text("Présentation", "Overview")}</a><a href="#histoire">{language.text("Histoire", "History")}</a><a href="#equipe">{language.text("Équipe", "Team")}</a><a href="#activites">{language.text("Activités", "Services")}</a><a href="#implantations-entreprise">{language.text("Implantations", "Locations")}</a><a href="#contact-entreprise">{language.text("Contact", "Contact")}</a></div></nav>
                <section class="section container editorial-split" id="presentation"><div class="editorial-copy"><p class="eyebrow">{language.text("QUI NOUS SOMMES", "WHO WE ARE")}</p><h2>{language.text("Des métiers complémentaires, une même exigence de terrain.", "Connected services, one commitment to the work on the ground.")}</h2><p class="company-summary">{language.text("Depuis 2007, SAPTRANS développe au Cameroun le transport de marchandises, la location d’engins et le contrôle technique AutoRelax. Chaque métier répond à des besoins de terrain précis.", "Since 2007, SAPTRANS has developed freight transport, equipment rental and AutoRelax vehicle inspections in Cameroon. Each service responds to specific needs on the ground.")}</p><details class="company-archive"><summary>{language.text("Lire la présentation de l’ancien site", "Read the archived company overview")}</summary><div class="source-prose">{company_copy.into_iter().map(|paragraph| view! { <p>{paragraph}</p> }).collect_view()}</div><p class="source-note">{language.text("Présentation issue de l’ancien site SAPTRANS, à actualiser avant publication.", "Overview from SAPTRANS’s archived website; details require updating before publication.")}</p></details></div><figure class="editorial-media"><img src="/media/editorial/archive-autorelax-inspection.png" alt=language.text("Techniciens AutoRelax et véhicule dans un centre de contrôle, visuel de l’ancien site", "AutoRelax technicians and a vehicle at an inspection centre, image from the former site") loading="lazy"/><figcaption>{language.text("Visuel de l’ancien site SAPTRANS · AutoRelax", "Image from the former SAPTRANS site · AutoRelax")}</figcaption></figure></section>
                <section class="section source-section" id="histoire"><div class="container"><p class="eyebrow">{language.text("NOTRE HISTOIRE", "OUR HISTORY")}</p><h2>{language.text("Les étapes de SAPTRANS.", "SAPTRANS through the years.")}</h2><div class="history-timeline">
                    <article><span>"2007"</span><div><h3>{language.text("Création à Douala", "Founded in Douala")}</h3><p>{language.text("Sapou Kana Etienne fonde Sapou Transport SARL, dite SAPTRANS, à Ndogbong-Bassa, derrière la direction commerciale de CAMLAIT.", "Sapou Kana Etienne founds Sapou Transport SARL, known as SAPTRANS, in Ndogbong-Bassa, near CAMLAIT’s commercial office.")}</p></div></article>
                    <article><span>{language.text("Début des années 2010", "Early 2010s")}</span><div><h3>{language.text("Extension à Yassa", "Expansion to Yassa")}</h3><p>{language.text("SAPTRANS acquiert un terrain de plus de 6 000 m² à Yassa, derrière le SYNAFOC et en face de l’usine AZUR. L’entreprise y construit des bureaux de direction et de logistique, un garage pour les camions et des magasins de stockage.", "SAPTRANS acquires a site of more than 6,000 m² in Yassa, behind SYNAFOC and opposite the AZUR factory. The company builds management and logistics offices, a truck repair garage and storage facilities there.")}</p></div></article>
                    <article><span>"2018"</span><div><h3>{language.text("Diversification vers le contrôle technique", "Diversification into vehicle inspection")}</h3><p>{language.text("La direction engage le développement de centres de contrôle technique automobile au Cameroun, avec l’objectif annoncé de contribuer à la réduction des accidents routiers.", "Management begins developing vehicle inspection centres in Cameroon, with the stated aim of helping reduce road accidents.")}</p></div></article>
                    <article><span>{language.text("Juillet 2019", "July 2019")}</span><div><h3>{language.text("Premier centre à Limbé", "First centre in Limbe")}</h3><p>{language.text("Après l’obtention d’un agrément ministériel mentionné sur l’ancien site, le premier centre ouvre à Mile 2, près de la station BOCOM.", "Following ministerial approval mentioned on the former website, the first centre opens in Mile 2, near the BOCOM station.")}</p></div></article>
                </div><p class="source-note">{language.text("Chronologie issue de l’ancien site SAPTRANS. Les lieux et installations décrivent leur situation historique et ne constituent pas des coordonnées actuelles.", "Timeline from the former SAPTRANS website. Locations and facilities describe historical circumstances and are not current contact details.")}</p></div></section>
                <section class="section company-team" id="equipe"><div class="container"><p class="eyebrow">{language.text("L’ÉQUIPE", "THE TEAM")}</p><h2>{language.text("Les professionnels présentés par SAPTRANS.", "The people introduced by SAPTRANS.")}</h2><p class="team-intro">{language.text("La confiance accordée à chaque employé nourrit son dynamisme et son engagement envers les clients. SAPTRANS décrit une équipe sélectionnée et formée avec soin, professionnelle, accueillante et responsable.", "Trust in each employee supports their commitment to clients. SAPTRANS describes a carefully selected and trained team that works professionally and responsibly.")}</p><div class="team-grid">
                    <article class="team-card"><img src="/media/editorial/team-sapou-kana-etienne.png" alt="Sapou Kana Etienne" loading="lazy"/><div><h3>"Sapou Kana Etienne"</h3><p>{language.text("Président-directeur général", "Chairman and CEO")}</p></div></article>
                    <article class="team-card"><img src="/media/editorial/team-fokam-zephyrin.png" alt="Fokam Zephyrin" loading="lazy"/><div><h3>"Fokam Zephyrin"</h3><p>{language.text("Directeur général", "General Manager")}</p></div></article>
                    <article class="team-card"><img src="/media/editorial/team-kapche-josiane.png" alt="Kapche Josiane" loading="lazy"/><div><h3>"Kapche Josiane"</h3><p>{language.text("Cheffe comptable", "Chief Accountant")}</p></div></article>
                    <article class="team-card"><img src="/media/editorial/team-foko-justin.png" alt="Foko Justin" loading="lazy"/><div><h3>"Foko Justin"</h3><p>{language.text("Responsable transport et logistique", "Transport and Logistics Manager")}</p></div></article>
                    <article class="team-card"><img src="/media/editorial/team-nantia-merlin.png" alt="Nantia Merlin" loading="lazy"/><div><h3>"Nantia Merlin"</h3><p>{language.text("Responsable location d’engins", "Equipment Rental Manager")}</p></div></article>
                    <article class="team-card"><img src="/media/editorial/team-nana-serges.png" alt="Nana Serges" loading="lazy"/><div><h3>"Nana Serges"</h3><p>{language.text("Contrôleur en chef, visite technique", "Chief Vehicle Inspection Officer")}</p></div></article>
                </div><p class="source-note">{language.text("Noms, fonctions et portraits issus de l’ancien site SAPTRANS ; fonctions actuelles à confirmer.", "Names, roles and portraits from the former SAPTRANS website; current roles should be confirmed.")}</p></div></section>
                <section class="section service-offers" id="activites"><div class="container"><p class="eyebrow">{language.text("NOS ACTIVITÉS", "OUR SERVICES")}</p><h2>{language.text("Quatre façons de vous accompagner.", "Four ways to work with us.")}</h2><div class="company-divisions"><a href=transport.clone()><span>"01"</span><strong>{language.text("Transport & logistique", "Freight & logistics")}</strong><span aria-hidden="true">"↗"</span></a><a href=equipment.clone()><span>"02"</span><strong>{language.text("Location d’engins", "Equipment rental")}</strong><span aria-hidden="true">"↗"</span></a><a href=inspection.clone()><span>"03"</span><strong>"AutoRelax"</strong><span aria-hidden="true">"↗"</span></a><a href=real_estate.clone()><span>"04"</span><strong>{language.text("Immobilier", "Real estate")}</strong><span aria-hidden="true">"↗"</span></a></div></div></section>
                <section class="section company-network" id="implantations-entreprise"><div class="container"><div class="section-heading-row"><div><p class="eyebrow">{language.text("PRÉSENCE LOCALE", "LOCAL PRESENCE")}</p><h2>{language.text("Des implantations à connaître.", "Locations to know.")}</h2></div><p>{language.text("L'ancien site mentionne la direction générale à Douala-Nyalla et deux centres AutoRelax. Confirmez les coordonnées avant tout déplacement.", "The archived site lists the head office in Douala-Nyalla and two AutoRelax centres. Confirm details before visiting.")}</p></div><div class="situation-grid"><article class="situation-card"><span>"01"</span><h3>"Douala-Nyalla"</h3><p>{language.text("Direction générale · Nyalla Pariso, BP 191 Douala", "Head office · Nyalla Pariso, PO Box 191, Douala")}</p></article><article class="situation-card"><span>"02"</span><h3>"Douala-Yassa"</h3><p>{language.text("Centre AutoRelax · détails pratiques à confirmer", "AutoRelax centre · practical details to confirm")}</p></article><article class="situation-card"><span>"03"</span><h3>{language.text("Limbé", "Limbe")}</h3><p>{language.text("Centre AutoRelax · détails pratiques à confirmer", "AutoRelax centre · practical details to confirm")}</p></article></div><a class="text-link" href=locations.clone()>{language.text("Voir les implantations", "View locations")} " ↗"</a></div></section>
            </main> }.to_html()
        }
        Page::Contact => view! { <main class="contact-page"><section class="page-hero directory-hero directory-hero-photo"><img class="directory-hero-image" src="/media/editorial/douala-city.jpg" alt="" fetchpriority="high"/><div class="container"><nav class="hero-breadcrumbs" aria-label=language.text("Fil d’Ariane", "Breadcrumb")><a href=home.clone()>{language.text("Accueil", "Home")}</a><span aria-hidden="true">"›"</span><span aria-current="page">{language.text("Contact", "Contact")}</span></nav><p class="eyebrow">{language.text("CONTACT", "CONTACT")}</p><h1>{language.text("Parlons de votre projet.", "Let’s discuss your project.")}</h1><p>{language.text("Choisissez votre activité, décrivez votre besoin et nous transmettrons votre demande au service concerné.", "Choose a service, describe your needs and we will direct your enquiry to the relevant team.")}</p></div></section><section class="container section"><div class="directory-lead"><p class="eyebrow">{language.text("VOTRE INTERLOCUTEUR", "YOUR POINT OF CONTACT")}</p><h2>{language.text("Le bon service, dès le premier échange.", "The right team from the first conversation.")}</h2></div><div class="contact-paths"><a href=language.href("/contact?service=transport#demande")>{language.text("Transport & logistique", "Freight & logistics")} " ↗"</a><a href=language.href("/contact?service=engins#demande")>{language.text("Location d’engins", "Equipment rental")} " ↗"</a><a href=language.href("/contact?service=autorelax#demande")>"AutoRelax ↗"</a><a href=language.href("/contact?service=immobilier#demande")>{language.text("Immobilier", "Real estate")} " ↗"</a></div><div class="contact-grid"><section class="card" id="demande"><p class="eyebrow">{language.text("FORMULAIRE", "ENQUIRY FORM")}</p><h2>{language.text("Faire une demande", "Send an enquiry")}</h2><form method="post" action=contact.clone()>
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
                </form></section><div class="contact-details"><section class="card"><h2>"SAPTRANS"</h2><p>{language.text("Direction générale · Douala-Nyalla Pariso, BP 191 Douala", "Head office · Nyalla Pariso, PO Box 191, Douala")}</p><p class="source-note">{language.text("Téléphone à confirmer auprès de SAPTRANS.", "Phone number to be confirmed with SAPTRANS.")}</p><p><a href="mailto:infos.sapou@groupesapou.com">"infos.sapou@groupesapou.com"</a></p></section><section class="card"><h2>"AutoRelax"</h2><p>{language.text("Coordonnées et horaires à confirmer auprès du centre.", "Contact details and opening hours to be confirmed with the centre.")}</p></section></div></div></section>
                <section class="section contact-locations"><div class="container"><div class="section-heading-row"><div><p class="eyebrow">{language.text("NOUS TROUVER", "FIND US")}</p><h2>{language.text("Des points de contact au Cameroun.", "Contact points in Cameroon.")}</h2></div><p>{language.text("Les lieux ci-dessous proviennent de l'ancien site. Confirmez les horaires et coordonnées avant tout déplacement.", "The locations below come from the archived site. Confirm hours and contact details before visiting.")}</p></div><div class="situation-grid"><article class="situation-card"><span>"01"</span><h3>"Douala-Nyalla"</h3><p>{language.text("Direction générale SAPTRANS", "SAPTRANS head office")}</p></article><article class="situation-card"><span>"02"</span><h3>"Douala-Yassa"</h3><p>{language.text("Centre de contrôle AutoRelax", "AutoRelax inspection centre")}</p></article><article class="situation-card"><span>"03"</span><h3>{language.text("Limbé", "Limbe")}</h3><p>{language.text("Centre de contrôle AutoRelax", "AutoRelax inspection centre")}</p></article></div><a class="text-link" href=locations.clone()>{language.text("Toutes les implantations", "All locations")} " ↗"</a></div></section>
                <section class="section faq-section container"><div><p class="eyebrow">{language.text("QUESTIONS FRÉQUENTES", "FREQUENT QUESTIONS")}</p><h2>{language.text("Avant d'envoyer votre demande.", "Before sending your enquiry.")}</h2></div><div class="faq-list"><details><summary>{language.text("Quel service choisir ?", "Which service should I choose?")}</summary><p>{language.text("Choisissez l'activité la plus proche de votre besoin et décrivez le contexte dans votre message.", "Choose the service closest to your needs and describe the context in your message.")}</p></details><details><summary>{language.text("Puis-je obtenir une réponse locale ?", "Can I get local assistance?")}</summary><p>{language.text("Indiquez votre ville et le lieu concerné. Les coordonnées et horaires des centres doivent être confirmés par SAPTRANS.", "Share your city and relevant location. SAPTRANS must confirm current centre details and hours.")}</p></details></div></section></main> }.to_html(),
        Page::ContactSuccess => view! { <main class="status-page"><section class="page-hero directory-hero"><div class="container"><p class="eyebrow">{language.text("CONTACT", "CONTACT")}</p><h1>{language.text("Votre demande a été reçue.", "Your enquiry has been received.")}</h1><p>{language.text("Merci. L’équipe SAPTRANS pourra vous répondre à l’adresse indiquée.", "Thank you. The SAPTRANS team will reply to the email address you provided.")}</p></div></section><section class="container section narrow"><p class="request-reference"><strong>{language.text("Référence de votre demande : ", "Your enquiry reference: ")}</strong><span>{if request_reference.is_empty() { language.text("non disponible", "unavailable") } else { request_reference.as_str() }}</span></p><a class="button" href=home.clone()>{language.text("Retour à l’accueil", "Back to home")}</a></section></main> }.to_html(),
        Page::ContactError => view! { <main class="status-page"><section class="page-hero directory-hero"><div class="container"><p class="eyebrow">{language.text("CONTACT", "CONTACT")}</p><h1>{language.text("Vérifiez votre demande.", "Please check your enquiry.")}</h1><p>{language.text("Certains champs doivent être corrigés avant l’envoi.", "Some fields need to be corrected before sending.")}</p></div></section><section class="container section narrow"><p class="intro">{language.text("Le nom, l’adresse e-mail, l’activité et le message doivent être renseignés correctement. Si le problème persiste, écrivez-nous à l’adresse indiquée sur la page contact.", "Please provide a valid name, email address, service and message. If the problem persists, email us using the address on the contact page.")}</p><a class="button" href=contact_form.clone()>{language.text("Revenir au formulaire", "Return to the form")}</a></section></main> }.to_html(),
    };

    let theme_label = language.text("Clair", "Light");
    let language_aria = language.text("View this page in English", "Voir cette page en français");
    let (shell_class, header_class) = if matches!(page, Page::Home) {
        ("site-shell home-shell", "header header-home")
    } else {
        ("site-shell", "header")
    };
    view! { <div class=shell_class><a class="skip-link" href="#content">{language.text("Aller au contenu", "Skip to content")}</a><header class=header_class><div class="container nav"><a class="brand" href=home.clone() aria-label="SAPTRANS"><img class="brand-logo logo-for-light" src="/brand/logo-dark.png" alt="SAPTRANS" width="225" height="68" onerror="this.hidden=true;this.parentElement.querySelector('.brand-wordmark').hidden=false"/><img class="brand-logo logo-for-dark" src="/brand/logo-white.png" alt="" width="225" height="68" onerror="this.hidden=true;this.parentElement.querySelector('.brand-wordmark').hidden=false"/><span class="brand-wordmark" hidden>SAPTRANS</span></a><nav class="desktop-nav" aria-label=language.text("Navigation principale", "Main navigation")>
        <details class="dropdown mega-menu solutions-menu"><summary>{language.text("Solutions", "Solutions")}</summary><div class="dropdown-panel mega-panel solutions-panel">
          <div class="mega-column mega-choices"><p class="dropdown-intro">{language.text("CHOISIR UNE ACTIVITÉ", "CHOOSE A SERVICE")}</p>
            <button type="button" data-mega-target="transport" aria-pressed="true">{language.text("Transport & logistique", "Freight & logistics")} <span aria-hidden="true">"→"</span></button>
            <button type="button" data-mega-target="equipment" aria-pressed="false">{language.text("Location d’engins", "Equipment rental")} <span aria-hidden="true">"→"</span></button>
            <button type="button" data-mega-target="inspection" aria-pressed="false">{language.text("Contrôle technique", "Vehicle inspection")} <span aria-hidden="true">"→"</span></button>
            <button type="button" data-mega-target="estate" aria-pressed="false">{language.text("Immobilier", "Real estate")} <span aria-hidden="true">"→"</span></button>
          </div>
          <div class="mega-column mega-content">
            <div data-mega-content="transport"><p class="dropdown-intro">{language.text("TRANSPORT & LOGISTIQUE", "FREIGHT & LOGISTICS")}</p><a href=transport.clone()>{language.text("Découvrir le transport", "Explore freight transport")} <span aria-hidden="true">"↗"</span></a><a href=language.href("/contact?service=transport#demande")>{language.text("Organiser un transport", "Arrange a shipment")} <span aria-hidden="true">"↗"</span></a></div>
            <div data-mega-content="equipment" hidden><p class="dropdown-intro">{language.text("LOCATION D’ENGINS", "EQUIPMENT RENTAL")}</p><a href=equipment.clone()>{language.text("Découvrir les engins", "Explore equipment")} <span aria-hidden="true">"↗"</span></a><a href=language.href("/contact?service=engins#demande")>{language.text("Demander un engin", "Request equipment")} <span aria-hidden="true">"↗"</span></a></div>
            <div data-mega-content="inspection" hidden><p class="dropdown-intro">{language.text("CONTRÔLE TECHNIQUE", "VEHICLE INSPECTION")}</p><a href=inspection.clone()>{language.text("Découvrir AutoRelax", "Explore AutoRelax")} <span aria-hidden="true">"↗"</span></a><a href=language.href("/contact?service=autorelax#demande")>{language.text("Contacter un centre", "Contact a centre")} <span aria-hidden="true">"↗"</span></a></div>
            <div data-mega-content="estate" hidden><p class="dropdown-intro">{language.text("IMMOBILIER", "REAL ESTATE")}</p><a href=real_estate.clone()>{language.text("Découvrir l’immobilier", "Explore real estate")} <span aria-hidden="true">"↗"</span></a><a href=language.href("/contact?service=immobilier#demande")>{language.text("Présenter votre projet", "Discuss your project")} <span aria-hidden="true">"↗"</span></a></div>
          </div>
          <div class="mega-column mega-context"><p class="dropdown-intro">{language.text("POUR ALLER PLUS LOIN", "GO FURTHER")}</p><a href=sectors.clone()>{language.text("Secteurs desservis", "Industries served")} <span aria-hidden="true">"↗"</span></a><a href=locations.clone()>{language.text("Implantations", "Locations")} <span aria-hidden="true">"↗"</span></a><a href=company.clone()>{language.text("Notre entreprise", "Our company")} <span aria-hidden="true">"↗"</span></a></div>
          <a class="mega-feature" href=contact.clone()><img src="/media/editorial/nigeria-fleet-crew.jpg" alt="" loading="lazy"/><strong>{language.text("Parlons de votre prochain projet.", "Let’s discuss your next project.")}</strong><span>{language.text("Contacter SAPTRANS", "Contact SAPTRANS")} " →"</span></a>
        </div></details>
        <details class="dropdown mega-menu"><summary>{language.text("Services", "Services")}</summary><div class="dropdown-panel mega-panel">
          <div class="mega-column"><p class="dropdown-intro">{language.text("NOS SERVICES", "OUR SERVICES")}</p><a href=transport.clone()>{language.text("Transport & logistique", "Freight & logistics")} <span aria-hidden="true">"↗"</span></a><a href=equipment.clone()>{language.text("Location d’engins", "Equipment rental")} <span aria-hidden="true">"↗"</span></a><a href=inspection.clone()>{language.text("Contrôle technique", "Vehicle inspection")} <span aria-hidden="true">"↗"</span></a><a href=real_estate.clone()>{language.text("Immobilier", "Real estate")} <span aria-hidden="true">"↗"</span></a></div>
          <div class="mega-column"><p class="dropdown-intro">{language.text("FAIRE UNE DEMANDE", "SEND AN ENQUIRY")}</p><a href=language.href("/contact?service=transport#demande")>{language.text("Demander un transport", "Request transport")} <span aria-hidden="true">"↗"</span></a><a href=language.href("/contact?service=engins#demande")>{language.text("Demander un engin", "Request equipment")} <span aria-hidden="true">"↗"</span></a><a href=language.href("/contact?service=autorelax#demande")>{language.text("Contacter AutoRelax", "Contact AutoRelax")} <span aria-hidden="true">"↗"</span></a><a href=language.href("/contact?service=immobilier#demande")>{language.text("Projet immobilier", "Property enquiry")} <span aria-hidden="true">"↗"</span></a></div>
          <div class="mega-column"><p class="dropdown-intro">{language.text("EXPLORER", "EXPLORE")}</p><a href=language.href("/#services")>{language.text("Tous les services", "All services")} <span aria-hidden="true">"↗"</span></a><a href=sectors.clone()>{language.text("Secteurs desservis", "Industries served")} <span aria-hidden="true">"↗"</span></a><a href=locations.clone()>{language.text("Implantations", "Locations")} <span aria-hidden="true">"↗"</span></a></div>
          <a class="mega-feature" href=transport.clone()><img src="/media/editorial/nigeria-fleet-crew.jpg" alt="" loading="lazy"/><strong>{language.text("Des services pour vos opérations.", "Services for your operations.")}</strong><span>{language.text("Explorer le transport", "Explore freight transport")} " →"</span></a>
        </div></details>
        <details class="dropdown mega-menu"><summary>{language.text("Secteurs & lieux", "Sectors & locations")}</summary><div class="dropdown-panel mega-panel">
          <div class="mega-column"><p class="dropdown-intro">{language.text("SECTEURS", "INDUSTRIES")}</p><a href=sectors.clone()>{language.text("Tous les secteurs", "All industries")} <span aria-hidden="true">"↗"</span></a><a href=sectors.clone()>{language.text("BTP et travaux publics", "Construction")} <span aria-hidden="true">"↗"</span></a><a href=sectors.clone()>{language.text("Bois et carrières", "Timber and quarries")} <span aria-hidden="true">"↗"</span></a></div>
          <div class="mega-column"><p class="dropdown-intro">{language.text("IMPLANTATIONS", "LOCATIONS")}</p><a href=locations.clone()>{language.text("Nos implantations", "Our locations")} <span aria-hidden="true">"↗"</span></a><a href=locations.clone()>{language.text("Douala", "Douala")} <span aria-hidden="true">"↗"</span></a><a href=locations.clone()>{language.text("Limbé", "Limbe")} <span aria-hidden="true">"↗"</span></a></div>
          <div class="mega-column"><p class="dropdown-intro">{language.text("SUR LE TERRAIN", "ON THE GROUND")}</p><a href=projects.clone()>{language.text("Réalisations", "Projects")} <span aria-hidden="true">"↗"</span></a><a href=company.clone()>{language.text("Notre entreprise", "Our company")} <span aria-hidden="true">"↗"</span></a><a href=contact.clone()>{language.text("Nous contacter", "Contact us")} <span aria-hidden="true">"↗"</span></a></div>
          <a class="mega-feature" href=locations.clone()><img src="/media/editorial/douala-city.jpg" alt="" loading="lazy"/><strong>{language.text("Présents au Cameroun.", "Working across Cameroon.")}</strong><span>{language.text("Voir les implantations", "Explore locations")} " →"</span></a>
        </div></details>
        <details class="dropdown mega-menu"><summary>{language.text("À découvrir", "Explore")}</summary><div class="dropdown-panel mega-panel">
          <div class="mega-column"><p class="dropdown-intro">{language.text("L’ENTREPRISE", "OUR COMPANY")}</p><a href=company.clone()>{language.text("Présentation", "Overview")} <span aria-hidden="true">"↗"</span></a><a href=language.href("/entreprise#histoire")>{language.text("Historique", "History")} <span aria-hidden="true">"↗"</span></a><a href=language.href("/entreprise#equipe")>{language.text("L’équipe", "Our team")} <span aria-hidden="true">"↗"</span></a></div>
          <div class="mega-column"><p class="dropdown-intro">{language.text("ACTUALITÉS", "NEWS")}</p><a href=news.clone()>{language.text("Toutes les actualités", "All news")} <span aria-hidden="true">"↗"</span></a><a href=projects.clone()>{language.text("Réalisations", "Projects")} <span aria-hidden="true">"↗"</span></a></div>
          <div class="mega-column"><p class="dropdown-intro">{language.text("INFORMATIONS", "INFORMATION")}</p><a href=search.clone()>{language.text("Recherche", "Search")} <span aria-hidden="true">"↗"</span></a><a href=legal.clone()>{language.text("Mentions légales", "Legal notice")} <span aria-hidden="true">"↗"</span></a><a href=contact.clone()>{language.text("Contact", "Contact")} <span aria-hidden="true">"↗"</span></a></div>
          <a class="mega-feature" href=company.clone()><img src="/media/editorial/abuja-project-team.jpg" alt="" loading="lazy"/><strong>{language.text("Découvrez SAPTRANS.", "Get to know SAPTRANS.")}</strong><span>{language.text("Voir l’entreprise", "About the company")} " →"</span></a>
        </div></details>
        <a href=contact.clone()>{language.text("Contact", "Contact")}</a>
        </nav><div class="header-controls"><a class="search-trigger" href=search.clone() aria-label=language.text("Rechercher sur le site", "Search the site")>"⌕"</a><a class="language-switch" href=language_switch aria-label=language_aria>{language_switch_label}</a><button class="theme-toggle" type="button" attr:data-theme-toggle="" attr:data-light-label=language.text("Activer le thème clair", "Switch to light theme") attr:data-dark-label=language.text("Activer le thème sombre", "Switch to dark theme") attr:data-light-mode-label=language.text("Clair", "Light") attr:data-dark-mode-label=language.text("Sombre", "Dark") aria-label=language.text("Activer le thème sombre", "Switch to dark theme") aria-pressed="false"><span class="theme-icon" aria-hidden="true">"◐"</span><span class="theme-label">{theme_label}</span></button></div><a class="button header-cta" href=contact_form.clone()>{language.text("Demander un devis", "Request a quote")}</a><details class="mobile-menu"><summary>{language.text("Menu", "Menu")}</summary><nav aria-label=language.text("Navigation mobile", "Mobile navigation")><a href=home.clone()>{language.text("Accueil", "Home")}</a><a href=language.href("/#services")>{language.text("Services", "Services")}</a><span>{language.text("Solutions et services", "Solutions and services")}</span><a href=transport.clone()>{language.text("Transport & logistique", "Freight & logistics")}</a><a href=real_estate.clone()>{language.text("Immobilier", "Real estate")}</a><a href=equipment.clone()>{language.text("Location d’engins", "Equipment rental")}</a><a href=inspection.clone()>"AutoRelax"</a><a href=sectors.clone()>{language.text("Nos secteurs", "Our sectors")}</a><a href=projects.clone()>{language.text("Réalisations", "Projects")}</a><a href=locations.clone()>{language.text("Nos implantations", "Locations")}</a><a href=company.clone()>{language.text("L’entreprise", "Company")}</a><a href=news.clone()>{language.text("Actualités", "News")}</a><a href=contact.clone()>{language.text("Contact", "Contact")}</a><a class="button" href=contact_form.clone()>{language.text("Demander un devis", "Request a quote")}</a></nav></details></div></header><div id="content" inner_html=body></div><ClosingVisual language=language page=page/><footer class="footer"><div class="container footer-main">
      <div class="footer-identity"><a class="footer-brand" href=home.clone() aria-label="SAPTRANS — accueil"><img class="footer-logo" src="/brand/logo-white.png" alt="SAPTRANS" width="225" height="68" onerror="this.hidden=true;this.nextElementSibling.hidden=false"/><span class="footer-wordmark" hidden>SAPTRANS</span></a></div>
      <nav class="footer-links" aria-label=language.text("Navigation de pied de page", "Footer navigation")>
        <div><h2>{language.text("Solutions", "Solutions")}</h2><a href=transport.clone()>{language.text("Transport & logistique", "Freight & logistics")}</a><a href=equipment.clone()>{language.text("Location d’engins", "Equipment rental")}</a><a href=inspection.clone()>{language.text("Contrôle technique", "Vehicle inspection")}</a><a href=real_estate.clone()>{language.text("Immobilier", "Real estate")}</a></div>
        <div><h2>{language.text("Secteurs", "Industries")}</h2><a href=sectors.clone()>{language.text("BTP et travaux publics", "Construction")}</a><a href=sectors.clone()>{language.text("Carrières", "Quarries")}</a><a href=sectors.clone()>{language.text("Bois", "Timber")}</a><a href=sectors.clone()>{language.text("Industrie et distribution", "Industry and distribution")}</a></div>
        <div><h2>{language.text("Liens utiles", "Quick links")}</h2><a href=projects.clone()>{language.text("Réalisations", "Projects")}</a><a href=news.clone()>{language.text("Actualités", "News")}</a><a href=search.clone()>{language.text("Recherche", "Search")}</a><a href=contact.clone()>{language.text("Nous contacter", "Contact us")}</a></div>
        <div><h2>{language.text("Entreprise", "Company")}</h2><a href=company.clone()>{language.text("Présentation", "Overview")}</a><a href=language.href("/entreprise#histoire")>{language.text("Historique", "History")}</a><a href=language.href("/entreprise#equipe")>{language.text("L’équipe", "Our team")}</a><a href=locations.clone()>{language.text("Implantations", "Locations")}</a><a href=language.href("/#partenaires")>{language.text("Partenaires", "Partners")}</a></div>
        <div><h2>{language.text("Coordonnées", "Contact details")}</h2><p>{language.text("Sapou Transport SARL", "Sapou Transport SARL")}</p><p>{language.text("Douala, Cameroun", "Douala, Cameroon")}</p><a href="mailto:infos.sapou@groupesapou.com">"infos.sapou@groupesapou.com"</a><a href=contact.clone()>{language.text("Faire une demande", "Send an enquiry")}</a></div>
      </nav>
      <nav class="footer-social" aria-label=language.text("Réseaux sociaux", "Social media")><span>{language.text("Réseaux sociaux", "Social media")}</span><a href="http://www.facebook.com" target="_blank" rel="noopener noreferrer">"Facebook"</a><a href="http://www.twitter.com" target="_blank" rel="noopener noreferrer">"X / Twitter"</a><a href="http://www.instagram.com" target="_blank" rel="noopener noreferrer">"Instagram"</a><a href="http://www.linkedin.com" target="_blank" rel="noopener noreferrer">"LinkedIn"</a></nav>
      <div class="footer-bottom"><nav aria-label=language.text("Informations légales", "Legal information")><a href=legal.clone()>{language.text("Mentions légales et confidentialité", "Legal notice and privacy")}</a><a href="/sitemap.xml">{language.text("Plan du site", "Sitemap")}</a><a href=contact.clone()>{language.text("Contact", "Contact")}</a></nav><span>"© SAPTRANS · Sapou Transport SARL"</span></div>
    </div></footer></div> }
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
    fn page_families_render_their_specific_sections_in_both_languages() {
        for language in [Language::Fr, Language::En] {
            let home = view! { <Site page=Page::Home language=language /> }.to_html();
            for section in [
                "hero",
                "partners-section",
                "services-section",
                "sector-strip",
                "home-method",
                "locations-section",
            ] {
                assert!(home.contains(section), "home is missing {section}");
            }
            for page in [
                Page::Transport,
                Page::Equipment,
                Page::Inspection,
                Page::RealEstate,
            ] {
                let html = view! { <Site page=page language=language /> }.to_html();
                assert!(
                    html.contains("class=\"page-anchor-nav\""),
                    "{}",
                    page.path()
                );
                assert!(html.contains("id=\"prestations\""), "{}", page.path());
                assert!(html.contains("id=\"questions\""), "{}", page.path());
            }
            let locations = view! { <Site page=Page::Locations language=language /> }.to_html();
            assert!(locations.contains("Douala-Yassa"));
            assert!(locations.contains("location-directory"));
            let contact = view! { <Site page=Page::Contact language=language /> }.to_html();
            assert!(contact.contains("contact-paths"));
            assert!(contact.contains("id=\"demande\""));
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
                            Page::from_path(&path).is_some() || path == "/sitemap.xml",
                            "broken link on {}: {href}",
                            page.path()
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn page_section_navigation_points_to_rendered_sections() {
        for page in [
            Page::Transport,
            Page::Equipment,
            Page::Inspection,
            Page::RealEstate,
            Page::Sectors,
            Page::Projects,
            Page::Locations,
            Page::News,
            Page::Legal,
            Page::Company,
        ] {
            let html = view! { <Site page=page language=Language::Fr /> }.to_html();
            for link in html.split("href=\"#").skip(1) {
                let target = link.split('"').next().unwrap_or_default();
                assert!(
                    html.contains(&format!("id=\"{target}\"")),
                    "section #{target} is missing on {}",
                    page.path()
                );
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
