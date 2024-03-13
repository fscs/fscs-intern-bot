use crate::structs::*;

pub async fn create_antrag(antrag: Antrag) -> String {
    println!("{:?}", serde_json::to_string(&antrag).unwrap());
    let response = reqwest::Client::new()
        .put("http://localhost/topmanager/antrag")
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&antrag).unwrap())
        .send()
        .await
        .unwrap();

    response.text().await.unwrap()
}

pub fn edit_antrag(antrag: Antrag) {
    let _ = reqwest::Client::new()
        .patch("http://localhost/topmanager/antrag")
        .body(serde_json::to_string(&antrag).unwrap());
}
