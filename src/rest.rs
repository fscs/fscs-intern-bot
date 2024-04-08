use crate::{keycloak, structs::*};

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

    println!("{:?}", respo);
}
