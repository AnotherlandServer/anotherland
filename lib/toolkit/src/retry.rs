// Copyright (C) 2026 AnotherlandServer
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

use std::{error::Error, time::Duration};

use anyhow::anyhow;
use database::DatabaseError;
use log::{debug, error};
use mongodb::{ClientSession, Database, error::{RETRYABLE_WRITE_ERROR, TRANSIENT_TRANSACTION_ERROR}, options::{ReadConcern, ReadPreference, SelectionCriteria, TransactionOptions, WriteConcern}};
use tokio::time::sleep;

pub trait GetMongoError {
    fn get_mongo_error(&self) -> Option<&mongodb::error::Error>;
}

impl GetMongoError for mongodb::error::Error {
    fn get_mongo_error(&self) -> Option<&mongodb::error::Error> {
        Some(self)
    }
}

impl GetMongoError for DatabaseError {
    fn get_mongo_error(&self) -> Option<&mongodb::error::Error> {
        self.source().and_then(|e| e.downcast_ref::<mongodb::error::Error>())
    }
}

pub async fn transaction_with_retry<'a, F, R, E, T>(db: Database, fnc: F) -> Result<R, E>
    where 
        R: Send + Sync + 'a,
        T: Future<Output = Result<(ClientSession, R), E>> + Send + 'a,
        F: Fn(ClientSession) -> T,
        E: From<mongodb::error::Error> + From<DatabaseError> + std::fmt::Debug + GetMongoError
{
    
    for n in 0..10 {
        debug!("Starting transaction attempt {}", n + 1);

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

        match async {
            let (mut session, result) = fnc(session).await?;
            session.commit_transaction().await?;

            Ok::<_, E>(result)
        }.await {
            Ok(result) => return Ok(result),
            Err(e) => {
                debug!("Transaction commit failed on attempt {}: {:?}", n + 1, e);

                if 
                    let Some(mongo_err) = e.get_mongo_error() &&
                    (mongo_err.contains_label(TRANSIENT_TRANSACTION_ERROR) || mongo_err.contains_label(RETRYABLE_WRITE_ERROR))
                {
                    let delay = 10 + (rand::random::<u64>() % 50);

                    debug!("Transient transaction error occurred. Attempt {} of 10. Retrying after {}ms", n + 1, delay);
                    sleep(Duration::from_millis(delay)).await;
                    continue;
                } else {
                    return Err(e);
                }
            }
        }
    }
    
    error!("Transaction failed after 10 attempts");

    Err(DatabaseError::Other(anyhow!("Transaction failed too many times!")).into())
}