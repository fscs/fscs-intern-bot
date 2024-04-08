use serde_json::json;

use crate::{keycloak, structs::*};

pub async fn get_persons() -> Vec<Person> {
    let url = std::env::var("API_URL").expect("missing API URL");
    let response = reqwest::Client::new()
        .get(url + "/api/person/")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    println!("{:?}", response);

    let persons: Vec<Person> = serde_json::from_str(&response).unwrap();

    persons
}

pub async fn create_antrag(antrag: Antrag) -> Antrag {
    let url = std::env::var("API_URL").expect("missing API URL");
    let token = keycloak::get_token().await.unwrap();
    println!("{:?}", serde_json::to_string(&antrag).unwrap());
    let response = reqwest::Client::new()
        .put(url + "/api/topmanager/antrag/")
        .header("Content-Type", "application/json")
        .header("Cookie", &format!("access_token={}", token))
        .body(serde_json::to_string(&antrag).unwrap())
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    println!("{:?}", response);

    let antrag: Antrag = serde_json::from_str(&response).unwrap();

    antrag
}

pub async fn edit_antrag(antrag: Antrag) {
    let url = std::env::var("API_URL").expect("missing API URL");
    let token = keycloak::get_token().await.unwrap();
    let respo = reqwest::Client::new()
        .patch(url + "/api/topmanager/antrag/")
        .header("Content-Type", "application/json")
        .header("Cookie", &format!("access_token={}", token))
        .body(serde_json::to_string(&antrag).unwrap())
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    println!("{:?}", antrag);

    println!("{:?}", respo);
}

pub async fn put_abmeldung(name: String) {
    let url = std::env::var("API_URL").expect("missing API URL");
    let persons = get_persons().await;
    let abmeldung = Abmeldung {
        ablaufdatum: chrono::Local::today().naive_local(),
        anfangsdatum: chrono::Local::today().naive_local(),
        person_id: persons
            .iter()
            .find(|person| person.name == name)
            .expect("Person not found")
            .id,
    };
    let respo = reqwest::Client::new()
        .put(url + "/api/topmanager/abmeldungen/")
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&abmeldung).unwrap())
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    println!("{:?}", respo);
}
