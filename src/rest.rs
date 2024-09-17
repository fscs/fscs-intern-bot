use serde_json::json;

use crate::{keycloak, structs::*};

pub async fn get_persons() -> Vec<Person> {
    let url = std::env::var("API_URL").expect("missing API URL");
    let response = reqwest::Client::new()
        .get(url + "/api/persons/")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let persons: Vec<Person> = serde_json::from_str(&response).unwrap();

    persons
}

pub async fn create_antrag(antrag: CreateAntrag) -> EditAntrag {
    let url = std::env::var("API_URL").expect("missing API URL");
    let token = keycloak::get_token().await.unwrap();
    let response = reqwest::Client::new()
        .post(url + "/api/anträge/")
        .header("Content-Type", "application/json")
        .header("Cookie", &format!("access_token={};", token))
        .body(serde_json::to_string(&antrag).unwrap())
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let antrag: EditAntrag = serde_json::from_str(&response).unwrap();

    antrag
}

pub async fn edit_antrag(antrag: EditAntrag) {
    let url = std::env::var("API_URL").expect("missing API URL");
    let token = keycloak::get_token().await.unwrap();
    let _ = reqwest::Client::new()
        .patch(url + &format!("/api/anträge/{}/", antrag.id))
        .header("Content-Type", "application/json")
        .header("Cookie", &format!("access_token={};", token))
        .body(
            serde_json::json!({
        "antragstext": antrag.antragstext,
        "begründung": antrag.begründung,
        "titel": antrag.titel})
            .to_string(),
        )
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
}

pub async fn put_abmeldung(name: String) {
    let url = std::env::var("API_URL").expect("missing API URL");
    let persons = get_persons().await;
    let now = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ");
    let next_sitzung = reqwest::Client::new()
        .get(url.clone() + &format!("/api/sitzungen/first-after/?timestamp={}", now))
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let next_sitzung: Sitzung = serde_json::from_str(&next_sitzung).unwrap();
    let abmeldung = Abmeldung {
        ablaufdatum: next_sitzung.datetime.into(),
        anfangsdatum: next_sitzung.datetime.into(),
        person_id: persons
            .iter()
            .find(|person| person.name == name)
            .expect("Person not found")
            .id,
    };
    let token = keycloak::get_token().await.unwrap();
    let respo = reqwest::Client::new()
        .put(url + &format!("/api/persons/{}/abmeldungen/", abmeldung.person_id))
        .header("Content-Type", "application/json")
        .header("Cookie", &format!("access_token={};", token))
        .body(format!(
            "{{\"start\":\"{}\",\"end\":\"{}\"}}",
            abmeldung.anfangsdatum, abmeldung.ablaufdatum
        ))
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
}
