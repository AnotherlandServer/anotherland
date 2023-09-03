use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct ConfLoginServer {
    pub listen_address: String
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct ConfMain {
    pub login_server: ConfLoginServer
}