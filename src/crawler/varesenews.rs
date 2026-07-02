use crate::service_management::news::Notizia;
use feed_rs::parser;
use chrono::{Utc, Duration};


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
                let titolo = entry.title.map(|t| t.content).unwrap_or_default();
                let comune = estrai_comune(&titolo);

                Notizia {
                    giornale: "VareseNews".to_string(),
                    titolo: titolo,
                    comune: comune,
                    contenuto: entry.content.map(|s| s.body).unwrap_or_default().unwrap_or_default(),
                    data: entry.published.map(|d| d.to_rfc3339()).unwrap_or_default(),
                }
            }).collect();
    }
    vec![]
}

fn estrai_comune (titolo: &str) -> String {
        if let Some(pos) = titolo.find(" - ") {
        return titolo[..pos].trim().to_string();
    }
    "".to_string()
}