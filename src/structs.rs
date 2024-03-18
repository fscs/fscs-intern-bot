use serde::{Deserialize, Serialize};
use uuid::Uuid;
#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct Antrag {
    pub id: Option<Uuid>,
    pub titel: String,
    pub antragstext: String,
    pub begründung: String,
    pub antragssteller: Option<String>,
}
