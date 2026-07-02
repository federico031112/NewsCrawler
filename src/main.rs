mod service_management;
mod crawler;

use service_management::news_service::{effettua_login, inserisci_notizia};
use crawler::varesenews::get_news_from_varesenews;
use tokio;
use chrono::{Datelike, Duration as ChronoDuration, Timelike, Utc}; // AGGIUNTO
use std::time::Duration; // AGGIUNTO
use tokio::time; 

#[tokio::main]
async fn main() {
    println!("🔄 Scheduler avviato - Esecuzione ogni lunedì alle 3:00");
    
    loop {
        let next_run = next_monday_3am();
        let now = Utc::now();
        let wait_seconds = (next_run - now).num_seconds();
        
        if wait_seconds > 0 {
            println!(
                "⏳ Prossima esecuzione: {} (tra {} ore)",
                next_run.format("%Y-%m-%d %H:%M:%S"),
                wait_seconds / 3600
            );
            time::sleep(Duration::from_secs(wait_seconds as u64)).await;
        } else {
            // Se siamo già dopo le 3 del lunedì, aspetta 7 giorni
            time::sleep(Duration::from_secs(7 * 24 * 3600)).await;
            continue;
        }
        
        // Esegui il task settimanale
        execute_weekly_task().await;
    }

}

fn next_monday_3am() -> chrono::DateTime<Utc> {
    let now = Utc::now();
    let days_to_add = match now.weekday().num_days_from_monday() {
        0 => {
            // Oggi è lunedì
            if now.hour() < 3 {
                0 // Prima delle 3, esegui oggi
            } else {
                7 // Dopo le 3, aspetta la prossima settimana
            }
        }
        days => 7 - days, // Giorni mancanti al prossimo lunedì
    };
    
    let next_date = now + ChronoDuration::days(days_to_add as i64);
    next_date
        .with_hour(3)
        .unwrap()
        .with_minute(0)
        .unwrap()
        .with_second(0)
        .unwrap()
}

async fn execute_weekly_task() {
    println!("🔄 Inizio esecuzione task settimanale");
    
    // Crea il client HTTP
    let client = match reqwest::Client::builder()
        .timeout(Duration::from_secs(30))
        .build()
    {
        Ok(c) => c,
        Err(e) => {
            eprintln!("❌ Errore creazione client: {}", e);
            return;
        }
    };
    
    // 1. Autenticazione
    let token = match effettua_login(&client).await {
        Ok(t) => {
            println!("✅ Autenticazione riuscita");
            t
        }
        Err(e) => {
            eprintln!("❌ Errore autenticazione: {}", e);
            return;
        }
    };
    
    // 2. Scarica le notizie da VareseNews
    let notizie = get_news_from_varesenews(&client).await;
    
    if notizie.is_empty() {
        println!("📭 Nessuna notizia da inviare");
        return;
    }
    
    println!("📰 Trovate {} notizie da VareseNews", notizie.len());

    for news in notizie{
    // 3. Invia ogni notizia al news service
        match inserisci_notizia(&client, &news, &token).await {
            Ok(_) => {
                println!("✅ Inviata: {}", news.titolo);
            }
            Err(e) => {
                eprintln!("❌ Errore invio notizia '{}': {}", news.titolo, e);
            }
        }
    }
    println!("✅ Task completato");
}
