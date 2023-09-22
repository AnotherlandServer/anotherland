use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct ConfLoginServer {
    pub listen_address: String
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct ConfDatabase {
    pub url: String,
    pub namespace: String,
    pub database: String,
    pub username: Option<String>,
    pub password: Option<String>,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct ConfRealmServer {
    pub name: String,
    pub listen_address: String,
    pub advertise_address: String
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct ConfWorldServer {
    pub listen_address: String,
    pub advertise_address: String
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct ConfMain {
    pub login_server: ConfLoginServer,
    pub database: ConfDatabase,
    pub realm: ConfRealmServer,
    pub world: ConfWorldServer,
}

