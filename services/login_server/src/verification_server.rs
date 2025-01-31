// Copyright (C) 2025 AnotherlandServer
// 
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
// 
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
// 
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

use core_api::CoreApi;
use log::{error, info};
use steamworks::SteamId;
use tokio::{io::{self, AsyncWriteExt}, net::{TcpListener, ToSocketAddrs}};

use crate::error::AppResult;

async fn verify_account(auth_api: &CoreApi, id: &SteamId) -> AppResult<()> {
    if auth_api.find_account(core_api::AccountQuery::SteamId(*id)).await?.is_none() {
        auth_api.register_steam_account(*id).await?;
    }
    
    Ok(())
}

pub async fn start_verification_server(auth_api: CoreApi, bind_addr: impl ToSocketAddrs) -> AppResult<()> {
    let listener = TcpListener::bind(bind_addr).await.unwrap();

    tokio::spawn(async move {
        loop {
            let auth_api = auth_api.clone();
            let (mut client, peer_addr) = listener.accept().await.unwrap();
            tokio::spawn(async move {
                info!("Connected: {}", peer_addr);
                loop {
                    let mut msg = vec![0; 1024];

                    client.readable().await.unwrap();

                    match client.try_read(&mut msg) {
                        Ok(p) => {
                            // client closing
                            if p == 0 {
                                let _ = client.shutdown().await;
                                return;
                            }

                            let size = u16::from_le_bytes(msg[0..2].try_into().unwrap());
                            let cmd = u16::from_le_bytes(msg[2..4].try_into().unwrap());
                            info!("S: {} CMD: {} Data: {:?}", size, cmd, &msg[4..p]);

                            let raw_steam_id = &msg[4..p];
                            if raw_steam_id.len() != 8 {
                                let _ = client.shutdown().await;
                                return;
                            }

                            let steam_id = SteamId::from_raw(u64::from_le_bytes(raw_steam_id.try_into().unwrap()));
                            match verify_account(&auth_api, &steam_id).await {
                                Ok(_) => {
                                    let _ = client.write_all(&[0x5,0x0,0x5,0x0,0x0]).await;
                                },
                                Err(e) => {
                                    error!("Failed to verify steam id {}: {:#?}", steam_id.steamid32(), e);
                                    let _ = client.shutdown().await;
                                    return;
                                }
                            }

                            // 3 - Register
                            // 4 - Wait for E-Mail
                            // 5 - Okay

                            //let _ = client.write_all(&[0x5,0x0,0x3,0x0,0x0]).await;

                            //let _ = client.write_all(&[0x5,0x0,0x4,0x0,0x0]).await;
                            //sleep(Duration::from_secs(2)).await;

                        
                        }
                        Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                            continue;
                        }
                        Err(e) => {
                            error!("Error: {}", e);
                            return;
                        }
                    }
                }
            });
        }
    });

    Ok(())
}