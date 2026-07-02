mod service_management;
mod crawler;

use service_management::news_service::{effettua_login, inserisci_notizia};
use service_management::news::Notizia;
use crawler::varesenews::get_news_from_varesenews;

use feed_rs::parser;
use serde::{Deserialize, Serialize};
use tokio;

#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();
    let token = effettua_login(&client).await?;
    let news = get_news_from_varesenews(&client).await;
    for value in news {
        inserisci_notizia(&client, value, token.as_str()).await;
    }

}
