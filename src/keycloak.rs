use std::collections::HashMap;

use anyhow::Error;
use oauth2::{
    basic::BasicClient,
    http::header::USER_AGENT,
    reqwest::{async_http_client, http_client},
    AuthUrl, ClientId, ClientSecret, RedirectUrl, ResourceOwnerPassword, ResourceOwnerUsername,
    Scope, TokenResponse, TokenUrl,
};
use uuid::Uuid;

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct WebsiteConfig {
    pub url: String,
    pub username: String,
    pub password: String,
    pub client_id: String,
    pub client_secret: String,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
struct WebsiteUser {
    id: Uuid,
    name: String,
}

pub async fn get_token() -> Result<String, Error> {
    let clientid_env = std::env::var("CLIENT_ID").expect("No CLIENT_ID set");
    let clientsecret_env = std::env::var("CLIENT_SECRET").expect("No CLIENT_SECRET set");
    let username_env = std::env::var("USERNAME").expect("No USERNAME set");
    let password_env = std::env::var("PASSWORD").expect("No PASSWORD set");
    let oauth = BasicClient::new(
        ClientId::new(clientid_env),
        Some(ClientSecret::new(clientsecret_env)),
        AuthUrl::new("https://auth.inphima.de/application/o/authorize/".to_string())?,
        Some(TokenUrl::new(
            "https://auth.inphima.de/application/o/token/".to_string(),
        )?),
    );

    let token = oauth
        .exchange_password(
            &ResourceOwnerUsername::new(username_env),
            &ResourceOwnerPassword::new(password_env),
        )
        .add_scope(Scope::new("profile".to_string()))
        .add_scope(Scope::new("openid".to_string()))
        .request_async(async_http_client)
        .await;

    Ok(token.unwrap().access_token().secret().to_string())
}
