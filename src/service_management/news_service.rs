use serde::{Deserialize};
use super::news::Notizia;

#[derive(Deserialize)]
struct LoginResponse {
    token: String
}


pub async fn effettua_login(client: &reqwest::Client) -> Result<String, Box<dyn std::error::Error>> {
    let user = serde_json::json!({
        "email": "admin@gmail.com",
        "password": "admin"
    });
    let res = client.post("http://identity_service:8080/api/login").json(&user).send().await?.json::<LoginResponse>().await?;
    Ok(res.token)
}

pub async fn inserisci_notizia(client: &reqwest::Client, news: &Notizia, token: &str) -> Result<(), Box<dyn std::error::Error>> {
    client.post("http://notizie_service:8081/api/notizie")
        .bearer_auth(token)
        .json(&news)
        .send().await?;
    Ok(())
}