use serde::Serialize;


#[derive(Serialize)]
pub struct Notizia {
    pub giornale: String,
    pub titolo: String,
    pub comune: String,
    pub contenuto: String,
    pub data: String
}