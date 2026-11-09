use leptos::prelude::*;

pub mod activities;
#[cfg(feature = "ssr")]
pub mod admin;
#[cfg(feature = "ssr")]
pub mod contact;

#[derive(Clone, Copy)]
pub enum Page {
    Home,
    Transport,
    Equipment,
    Inspection,
    RealEstate,
    Company,
    Contact,
    ContactSuccess,
    ContactError,
}

impl Page {
    pub fn from_path(path: &str) -> Option<Self> {
        match path.trim_end_matches('/') {
            "" => Some(Self::Home),
            "/transport-logistique" => Some(Self::Transport),
            "/location-engins" => Some(Self::Equipment),
            "/controle-technique" => Some(Self::Inspection),
            "/immobilier" => Some(Self::RealEstate),
            "/entreprise" => Some(Self::Company),
            "/contact" => Some(Self::Contact),
            "/contact/merci" => Some(Self::ContactSuccess),
            "/contact/erreur" => Some(Self::ContactError),
            _ => None,
        }
    }

    pub fn title(self) -> &'static str {
        match self {
            Self::Home => "SAPTRANS — Transport, engins et contrôle technique",
            Self::Transport => "Transport et logistique — SAPTRANS",
            Self::Equipment => "Location d’engins — SAPTRANS",
            Self::Inspection => "Contrôle technique AutoRelax — SAPTRANS",
            Self::RealEstate => "Immobilier — SAPTRANS",
            Self::Company => "L’entreprise — SAPTRANS",
            Self::Contact => "Contact — SAPTRANS",
            Self::ContactSuccess => "Demande reçue — SAPTRANS",
            Self::ContactError => "Demande à vérifier — SAPTRANS",
        }
    }
}

#[component]
pub fn Site(page: Page) -> impl IntoView {
    let body = match page {
        Page::Home => view! { <main>
            <section class="hero"><div class="container hero-grid"><div><p class="eyebrow">"SAPOU TRANSPORT SARL · CAMEROUN"</p><h1>"Des solutions pour vous faire avancer."</h1><p>"Transport et logistique, location d’engins et contrôle technique automobile à vos côtés."</p><div class="actions"><a class="button" href="/contact#demande">"Demander un devis"</a><a class="secondary-link" href="#services">"Explorer nos activités →"</a></div></div><div class="media-placeholder" role="img" aria-label="Emplacement pour une photographie réelle des activités SAPTRANS"><span>"Photographie SAPTRANS à intégrer"</span></div></div></section>
            <section class="container section" aria-labelledby="services"><p class="eyebrow">"NOS ACTIVITÉS"</p><h2 id="services">"Un partenaire pour vos opérations"</h2><div class="cards">
                <article class="card"><span class="card-number">"01"</span><h3>"Transport et logistique"</h3><p>"Des solutions de transport adaptées à vos marchandises et à vos chantiers."</p><a href="/transport-logistique">"Découvrir →"</a></article>
                <article class="card"><span class="card-number">"02"</span><h3>"Location d’engins"</h3><p>"Des équipements pour accompagner vos travaux et vos besoins de terrain."</p><a href="/location-engins">"Découvrir →"</a></article>
                <article class="card"><span class="card-number">"03"</span><h3>"Contrôle technique"</h3><p>"AutoRelax vous accueille dans ses centres de Douala et Limbé."</p><a href="/controle-technique">"Découvrir →"</a></article>
                <article class="card"><span class="card-number">"04"</span><h3>"Immobilier"</h3><p>"Une activité du groupe en cours de documentation."</p><a href="/immobilier">"En savoir plus →"</a></article>
            </div></section>
            <section class="container section split"><div class="media-placeholder media-light" role="img" aria-label="Emplacement pour une photographie réelle de l’équipe ou d’un site SAPTRANS"><span>"Photographie d’équipe ou de site à intégrer"</span></div><div><p class="eyebrow">"L’ENTREPRISE"</p><h2>"Ancrés à Douala, engagés sur le terrain."</h2><p>"SAPTRANS réunit plusieurs métiers autour des besoins de transport, de chantier et de mobilité. Ses équipes sont implantées au Cameroun."</p><a class="text-link" href="/entreprise">"Découvrir l’entreprise →"</a></div></section>
            <section class="section section-muted"><div class="container"><p class="eyebrow">"MOYENS ET RÉALISATIONS"</p><h2>"Des opérations à montrer, des faits à vérifier."</h2><p class="intro">"Les équipements, projets et références seront présentés ici avec des photographies et des informations validées par SAPTRANS."</p><div class="proof-grid"><div class="media-placeholder media-light" role="img" aria-label="Emplacement pour une photographie d’un véhicule SAPTRANS"><span>"Véhicule ou opération à documenter"</span></div><div class="media-placeholder media-light" role="img" aria-label="Emplacement pour une photographie d’un centre AutoRelax"><span>"Centre ou équipement à documenter"</span></div></div></div></section>
            <section class="container section"><p class="eyebrow">"IMPLANTATIONS"</p><h2>"Vos interlocuteurs au Cameroun"</h2><div class="cards locations"><article class="card"><h3>"Douala-Nyalla"</h3><p>"Direction générale · Nyalla Pariso, BP 191 Douala"</p><a href="/contact">"Coordonnées →"</a></article><article class="card"><h3>"AutoRelax Douala"</h3><p>"Centre de contrôle technique à Douala-Yassa"</p><a href="/contact">"Coordonnées →"</a></article><article class="card"><h3>"AutoRelax Limbé"</h3><p>"Centre de contrôle technique à Limbé"</p><a href="/contact">"Coordonnées →"</a></article></div></section>
            <section class="band"><div class="container band-inner"><div><p class="eyebrow">"CONTACT"</p><h2>"Parlons de votre projet."</h2></div><p>"Choisissez votre activité et décrivez votre besoin. Nous vous orienterons vers le bon interlocuteur."</p><a class="button button-light" href="/contact#demande">"Faire une demande"</a></div></section>
        </main> }.to_html(),
        Page::Transport => activities::activity_page("TRANSPORT ET LOGISTIQUE", "Faire circuler vos marchandises", "Nous accompagnons les besoins de transport de marchandises, de conteneurs et d’équipements, avec une attention portée à la planification et à la sécurité des opérations."),
        Page::Equipment => activities::activity_page("LOCATION D’ENGINS", "Les moyens adaptés à vos chantiers", "La location d’engins répond aux besoins ponctuels ou réguliers des professionnels. Contactez l’équipe pour préciser l’équipement recherché et les conditions de mise à disposition."),
        Page::Inspection => activities::activity_page("AUTORELAX", "Le contrôle technique de vos véhicules", "AutoRelax accueille les automobilistes dans ses centres de Douala et Limbé. Contactez le centre pour connaître les modalités et horaires en vigueur."),
        Page::RealEstate => activities::activity_page("IMMOBILIER", "Une activité à découvrir", "Les informations sur l’offre immobilière SAPTRANS sont en cours de préparation. Notre équipe peut vous orienter vers le bon interlocuteur."),
        Page::Company => view! { <main><section class="page-hero"><div class="container"><p class="eyebrow">"L’ENTREPRISE"</p><h1>"SAPTRANS, des métiers complémentaires."</h1><p>"Sapou Transport SARL développe ses activités au Cameroun autour du transport, des engins et du contrôle technique AutoRelax."</p></div></section><section class="container section narrow"><h2>"Notre ancrage"</h2><p>"L’ancien site situe les débuts de l’entreprise à Douala et évoque une extension à Yassa. Les dates, l’équipe et les moyens seront détaillés après validation des informations actuelles."</p><a class="text-link" href="/contact">"Contacter l’équipe →"</a></section></main> }.to_html(),
        Page::Contact => view! { <main class="container section contact-page"><p class="eyebrow">"CONTACT"</p><h1>"Échangeons sur votre besoin"</h1><p class="intro">"Précisez le service souhaité ou contactez directement l’équipe concernée."</p><div class="contact-grid"><section class="card" id="demande"><h2>"Faire une demande"</h2><form method="post" action="/contact"><div class="field"><label for="name">"Nom complet"</label><input id="name" name="name" required minlength="2" maxlength="120" autocomplete="name"/></div><div class="field"><label for="email">"Adresse e-mail"</label><input id="email" name="email" type="email" required autocomplete="email"/></div><div class="field"><label for="service">"Activité concernée"</label><select id="service" name="service" required><option value="">"Choisir une activité"</option><option value="transport">"Transport et logistique"</option><option value="engins">"Location d’engins"</option><option value="autorelax">"AutoRelax"</option><option value="immobilier">"Immobilier"</option></select></div><div class="field"><label for="message">"Votre besoin"</label><textarea id="message" name="message" rows="5" required minlength="10" maxlength="4000"></textarea></div><button class="button" type="submit">"Envoyer la demande"</button></form></section><div class="contact-details"><section class="card"><h2>"SAPTRANS"</h2><p>"Direction générale · Douala-Nyalla Pariso, BP 191 Douala"</p><p><a href="tel:+237243059022">"+237 243 05 90 22"</a></p><p><a href="mailto:infos.sapou@groupesapou.com">"infos.sapou@groupesapou.com"</a></p></section><section class="card"><h2>"AutoRelax"</h2><p>"Douala : " <a href="tel:+237691921634">"+237 691 92 16 34"</a></p><p>"Limbé : " <a href="tel:+237691939163">"+237 691 93 91 63"</a></p></section></div></div><p class="note">"Coordonnées reprises de l’ancien site, à confirmer auprès de l’entreprise."</p></main> }.to_html(),
        Page::ContactSuccess => view! { <main class="container section narrow"><p class="eyebrow">"CONTACT"</p><h1>"Votre demande a été reçue."</h1><p class="intro">"Merci. L’équipe SAPTRANS pourra vous répondre à l’adresse indiquée."</p><a class="button" href="/">"Retour à l’accueil"</a></main> }.to_html(),
        Page::ContactError => view! { <main class="container section narrow"><p class="eyebrow">"CONTACT"</p><h1>"Vérifiez votre demande."</h1><p class="intro">"Le nom, l’adresse e-mail, l’activité et le message doivent être renseignés correctement. Si le problème persiste, contactez-nous par téléphone."</p><a class="button" href="/contact#demande">"Revenir au formulaire"</a></main> }.to_html(),
    };
    view! { <div class="site-shell"><a class="skip-link" href="#content">"Aller au contenu"</a><header class="header"><div class="container nav"><a class="brand" href="/" aria-label="SAPTRANS, accueil"><img src="https://saptrans.net/wp-content/uploads/2021/06/logo_redisign_dunkel_224x67.png" alt="SAPTRANS" width="224" height="67"/></a><nav class="desktop-nav" aria-label="Navigation principale"><a href="/">"Accueil"</a><details class="dropdown"><summary>"Nos activités"</summary><div class="dropdown-panel"><a href="/transport-logistique">"Transport et logistique"</a><a href="/location-engins">"Location d’engins"</a><a href="/controle-technique">"AutoRelax"</a><a href="/immobilier">"Immobilier"</a></div></details><a href="/entreprise">"L’entreprise"</a><a href="/contact">"Contact"</a></nav><a class="button header-cta" href="/contact#demande">"Demander un devis"</a><details class="mobile-menu"><summary>"Menu"</summary><nav aria-label="Navigation mobile"><a href="/">"Accueil"</a><span>"Nos activités"</span><a href="/transport-logistique">"Transport et logistique"</a><a href="/location-engins">"Location d’engins"</a><a href="/controle-technique">"AutoRelax"</a><a href="/immobilier">"Immobilier"</a><a href="/entreprise">"L’entreprise"</a><a href="/contact">"Contact"</a><a class="button" href="/contact#demande">"Demander un devis"</a></nav></details></div></header><div id="content" inner_html=body></div><footer class="footer"><div class="container footer-inner"><div><img class="footer-logo" src="https://saptrans.net/wp-content/uploads/2021/06/logo_redisign_white_224x67.png" alt="SAPTRANS" width="224" height="67"/><p>"Sapou Transport SARL · Douala, Cameroun"</p></div><div><strong>"Activités"</strong><p><a href="/transport-logistique">"Transport"</a><br/><a href="/location-engins">"Location d’engins"</a><br/><a href="/controle-technique">"AutoRelax"</a><br/><a href="/immobilier">"Immobilier"</a></p></div><div><strong>"Contact"</strong><p><a href="mailto:infos.sapou@groupesapou.com">"infos.sapou@groupesapou.com"</a><br/><a href="tel:+237243059022">"+237 243 05 90 22"</a></p></div></div></footer></div> }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn public_routes_are_explicit() {
        assert!(Page::from_path("/immobilier/").is_some());
        assert!(Page::from_path("/admin").is_none());
    }

    #[test]
    fn key_pages_render_html_with_official_logo() {
        for page in [Page::Home, Page::Transport, Page::Contact] {
            let html = view! { <Site page=page /> }.to_html();
            assert!(html.contains("<main"));
            assert!(html.contains("logo_redisign_dunkel_224x67.png"));
        }
    }
}
