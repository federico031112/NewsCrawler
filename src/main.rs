use feed_rs::parser;
use serde::{Deserialize, Serialize};
use tokio;


#[derive(Deserialize)]
struct LoginResponse {
    token: String
}

#[derive(Serialize)]
struct Notizia {
    giornale: String,
    titolo: String,
    comune: String,
    contenuto: String,
    data: String
}

async fn effettua_login(client: &reqwest::Client) -> Result<String, Box<dyn std::error::Error>> {
    let user = serde_json::json!({
        "email": "admin@gmail.com",
        "password": "admin"
    });
    let res = client.post("http://identity_service:8080/api/login").json(&user).send().await?.json::<LoginResponse>().await?;
    Ok(res.token)
}

async fn inserisci_notizia(client: &reqwest::Client, news: Notizia, token: &str) -> Result<(), Box<dyn std::error::Error>> {
    client.post("http://notizie_service:8081/api/notizie")
        .bearer_auth(token)
        .json(notizia)
        .send().await?;
    Ok(())
}

async fn get_news_from_varesenews (client: &reqwest::Client) -> Vec<Notizia> {
    let response = match client.get("https://www.varesenews.it/feed/").send().await {
        Ok(resp) => resp.text().await.unwrap_or_default(),
        Err(_) => return vec![]
    };

    let una_settimana_fa = Utc::now() - Duration::days(7);
    if let Ok(feed) = parser::parse(response.as_bytes()) {
        return feed.entries.into_iter()
            .filter(|entry| {
                if let Some(published) = entry.published {
                    published > una_settimana_fa
                } else {
                    false
                }
            })
            .map(|entry| {
                Notizia {
                    giornale: "VareseNews".to_string(),
                    titolo: entry.title.map(|t| t.content).unwrap_or_default(),
                    comune: "Varese".to_string(),
                    contenuto: entry.summary.map(|s| s.content).unwrap_or_default(),
                    data: entry.published.map(|d| d.to_rfc3339()).unwrap_or_default(),
                }
            }).collect();
    }
    vec![]
}

#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();
    let token = effettua_login(&client).await?;
    let news = get_news_from_varesenews(&client).await;
    for value in news {
        inserisci_notizia(&client, value, token.as_str()).await;
    }

}
