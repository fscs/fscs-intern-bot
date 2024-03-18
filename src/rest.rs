use crate::structs::*;

pub async fn create_antrag(antrag: Antrag) -> Antrag {
    println!("{:?}", serde_json::to_string(&antrag).unwrap());
    let response = reqwest::Client::new()
        .put("http://localhost/api/topmanager/antrag")
        .header("Content-Type", "application/json")
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
    let respo = reqwest::Client::new()
        .patch("http://localhost/api/topmanager/antrag")
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&antrag).unwrap())
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    println!("{:?}", respo);
}
