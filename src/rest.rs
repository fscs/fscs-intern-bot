use crate::structs::*;

pub fn create_antrag(antrag: Antrag) {
    let _ = reqwest::Client::new()
        .put("http://localhost/antrag")
        .body(serde_json::to_string(&antrag).unwrap());
}

pub fn edit_antrag(antrag: Antrag) {
    let _ = reqwest::Client::new()
        .patch("http://localhost/antrag")
        .body(serde_json::to_string(&antrag).unwrap());
}
