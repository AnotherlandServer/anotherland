use std::{sync::Arc};

use rsa::{RsaPrivateKey, RsaPublicKey};
use tokio::{net::{UdpSocket, ToSocketAddrs}, io, task::JoinHandle};

use crate::raknet::RakNetListener;

pub struct LoginServer {
    listener: RakNetListener,
}

impl LoginServer {
    pub async fn bind_server<A: ToSocketAddrs>(addr: A) -> io::Result<LoginServer> {
        let mut listener = RakNetListener::bind(addr).await?;

        /*let mut rng = rand::thread_rng();
        let bits = 2048;
        let priv_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
        let pub_key = RsaPublicKey::from(&priv_key);*/

        Ok(Self {listener})
    }
}
