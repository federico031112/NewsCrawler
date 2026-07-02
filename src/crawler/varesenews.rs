use serde::Serialize;
use crate::service_management::news::Notizia;


pub async fn get_news_from_varesenews (client: &reqwest::Client) -> Vec<Notizia> {
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