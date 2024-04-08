use chrono::NaiveDate;
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

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow, Clone)]
pub struct Person {
    pub id: Uuid,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct Abmeldung {
    pub ablaufdatum: NaiveDate,
    pub anfangsdatum: NaiveDate,
    pub person_id: Uuid,
}
