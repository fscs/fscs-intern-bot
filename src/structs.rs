use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct CreateAntrag {
    pub titel: String,
    pub antragstext: String,
    pub begründung: String,
    pub antragssteller: Vec<Uuid>,
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct EditAntrag {
    pub id: Uuid,
    pub titel: String,
    pub antragstext: String,
    pub begründung: String,
    pub creators: Vec<Uuid>,
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow, Clone)]
pub struct Person {
    pub id: Uuid,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow, Clone)]
pub struct Sitzung {
    pub id: Uuid,
    pub datetime: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow, Clone)]
pub struct Top {
    pub id: Uuid,
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct Abmeldung {
    pub ablaufdatum: DateTime<Utc>,
    pub anfangsdatum: DateTime<Utc>,
    pub person_id: Uuid,
}
