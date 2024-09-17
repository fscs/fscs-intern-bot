use serde_json::json;

use crate::{keycloak, structs::*};

pub async fn get_persons() -> Vec<Person> {
    let url = "https://fscs.hhu.de/api/persons/";
    let response = reqwest::Client::new()
        .get(url)
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
    let url = "https://fscs.hhu.de";
    let token = keycloak::get_token().await.unwrap();

    println!("{:?}", serde_json::to_string(&antrag).unwrap());

    let client = reqwest::Client::new();

    let antrag_response = client
        .post(format!("{}/api/anträge/", url))
        .header("Content-Type", "application/json")
        .header("Cookie", &format!("access_token={};", token))
        .body(serde_json::to_string(&antrag).unwrap())
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let antrag: EditAntrag = serde_json::from_str(&antrag_response).unwrap();

    let local = chrono::Local::now();

    let sitzung_response = client
        .post(
            format!(
                "{}/api/sitzungen/first-after/?timestamp={}",
                url,
                local.format("%Y-%m-%dT%H:%M:%SZ")
            )
        )
        .header("Content-Type", "application/json")
        .header("Cookie", &format!("access_token={};", token))
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let sitzung: Sitzung = serde_json::from_str(&sitzung_response).unwrap();

    let top_response = client
        .post(format!("{}/api/sitzungen/{}/tops/", url, sitzung.id))
        .header("Content-Type", "application/json")
        .header("Cookie", &format!("access_token={};", token))
        .body(
            serde_json::json!({
                "kind": "normal",
                "name": antrag.titel,
            })
            .to_string(),
        )
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let top: Top = serde_json::from_str(&top_response).unwrap();

    client
        .patch(format!("{},/api/sitzungen/{}/tops/{}/assoc/", url, sitzung.id, top.id))
        .header("Content-Type", "application/json")
        .header("Cookie", &format!("access_token={};", token))
        .body(
            serde_json::json!({
                "antrag_id": antrag.id
            })
            .to_string(),
        )
        .send()
        .await
        .unwrap();

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

    println!("{:?}", abmeldung);
    let token = keycloak::get_token().await.unwrap();
    let respo = reqwest::Client::new()
        .put(url + &format!("/api/persons/{}/abmeldungen/", abmeldung.person_id))
        .header("Content-Type", "application/json")
        .header("Cookie", &format!("access_token={};", token))
        .body(format!(
            "{{\"start\":\"{}\",\"end\":\"{}\"}}",
            abmeldung.anfangsdatum.date_naive(),
            abmeldung.ablaufdatum.date_naive()
        ))
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    println!("{respo}");
}
