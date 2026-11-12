use serde::Deserialize;

#[derive(Deserialize)]
struct Bundle {
    pages: std::collections::HashMap<String, LegacyPage>,
}

#[derive(Deserialize)]
struct LegacyPage {
    key: String,
    paragraphs: Vec<String>,
}

const LEGACY_CONTENT: &str = include_str!("../public/data/legacy-content.json");

/// Return locally bundled copy extracted from the archived French website.
pub fn paragraphs(key: &str) -> Vec<String> {
    serde_json::from_str::<Bundle>(LEGACY_CONTENT)
        .ok()
        .and_then(|bundle| bundle.pages.into_values().find(|page| page.key == key))
        .map(|page| page.paragraphs)
        .unwrap_or_default()
}

/// English copy adapted from the archived French descriptions.
pub fn paragraphs_en(key: &str) -> Vec<String> {
    match key {
        "transport" => vec![
            "SAPTRANS handles industrial freight for construction, quarrying, timber, paper, retail and other sectors, including the transport of machinery on low loaders.".into(),
            "Transport planning is tailored to each shipment, with teams coordinating loading and unloading safely.".into(),
            "The archived service description presents end-to-end transport solutions for industrial, retail and manufactured goods.".into(),
        ],
        "equipment" => vec![
            "Excavators support drainage, ditch clearing, bridge maintenance, earthworks and other site operations. Confirm current equipment availability with SAPTRANS.".into(),
            "Wheel loaders can be used to load trucks with soil, pozzolana and quarry aggregates. Ask the team about current rental terms and equipment.".into(),
        ],
        "inspection" => vec![
            "The vehicle inspection combines readings from inspection equipment with a visual check by a technician.".into(),
            "The archived page lists tyre pressure, wheel alignment and suspension checks among the inspection steps.".into(),
            "It also describes a visual review of the vehicle’s body, interior, dashboard, lights, fluids and filters. Contact the centre to confirm current procedures.".into(),
        ],
        "company" => vec![
            "The archived SAPTRANS website presents industrial freight services for construction, quarrying, timber, paper, retail and manufactured goods, including machinery transport across the CEMAC region.".into(),
            "It also describes the company’s development from freight transport into vehicle inspection services through its AutoRelax centres.".into(),
        ],
        "history" => vec![
            "The archived history states that Sapou Transport SARL was established in Douala in 2007.".into(),
            "It describes a diversification into vehicle inspection services beginning in 2018.".into(),
        ],
        _ => Vec::new(),
    }
}
