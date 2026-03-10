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

use std::time::Duration;

use anyhow::anyhow;
use mongodb::{ClientSession, Database, error::TRANSIENT_TRANSACTION_ERROR, options::{ReadConcern, ReadPreference, SelectionCriteria, TransactionOptions, WriteConcern}};
use tokio::time::sleep;

use crate::DatabaseError;

pub async fn transaction_with_retry<'a, F, R, E, T>(db: Database, fnc: F) -> Result<R, E>
    where 
        R: Send + Sync + 'a,
        T: Future<Output = Result<(ClientSession, R), E>> + Send + 'a,
        F: Fn(ClientSession) -> T,
        E: From<mongodb::error::Error> + From<DatabaseError>
{
    for _ in 0..10 {
        let mut session = db.client()
            .start_session()
            .default_transaction_options(TransactionOptions::builder()
                .read_concern(ReadConcern::majority())
                .write_concern(WriteConcern::majority())
                .selection_criteria(SelectionCriteria::ReadPreference(ReadPreference::Primary))
                .build()
            )
            .causal_consistency(true)      
            .await?;

        session.start_transaction().await?;

        let (mut session, result) = fnc(session).await?;

        match session.commit_transaction().await {
            Ok(_) => return Ok(result),
            Err(e) => {
                if e.contains_label(TRANSIENT_TRANSACTION_ERROR) {
                    session.abort_transaction().await?;
                    sleep(Duration::from_millis(10 + (rand::random::<u64>() % 50))).await;
                    continue;
                } else {
                    return Err(e.into());
                }
            }
        }
    }
    
    Err(DatabaseError::Other(anyhow!("Transaction failed too many times!")).into())
}