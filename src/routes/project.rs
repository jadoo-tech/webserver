use askama_axum::Template;



#[derive(Template)]
#[template(path = "project/exoskeleton.html")]
pub struct ExoskeletonTemplate;

#[derive(Template)]
#[template(path = "project/lithography.html")]
pub struct LithographyTemplate;

#[derive(Template)]
#[template(path = "project/plasma_jet.html")]
pub struct PlasmaJetTemplate;

#[derive(Template)]
#[template(path = "project/energy_storage.html")]
pub struct EnergyStorageTemplate;

#[derive(Template)]
#[template(path = "project/electrowetting.html")]
pub struct ElectrowettingTemplate;

#[derive(Template)]
#[template(path = "project/drone.html")]
pub struct DroneTemplate;

#[derive(Template)]
#[template(path = "project/biosensor.html")]
pub struct BiosensorTemplate;

#[derive(Template)]
#[template(path = "project/analytics.html")]
pub struct AnalyticsTemplate;
