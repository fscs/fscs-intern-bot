use serde::Serialize;
#[derive(Debug, Serialize)]
pub struct Antrag {
    pub titel: String,
    pub antragstext: String,
    pub begrundung: String,
    pub antragsteller: String,
}
