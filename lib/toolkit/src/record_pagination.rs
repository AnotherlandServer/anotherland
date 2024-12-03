// Copyright (C) 2024 AnotherlandServer
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

use std::fmt::Display;

use async_graphql::{connection::{query, Connection, CursorType, Edge, EmptyFields}, Error, ErrorExtensions, ObjectType};
use database::DatabaseRecord;
use futures_util::{StreamExt, TryStreamExt};
use mongodb::{bson::{doc, Document}, Database};

pub type RecordConnection<T> = Connection<i64, T, EmptyFields, EmptyFields>;

pub async fn record_query<R: DatabaseRecord, T>(
    db: Database,
    filter: Option<Document>,
    after: Option<String>, 
    before: Option<String>, 
    first: Option<i32>, 
    last: Option<i32>
) -> Result<RecordConnection<T>, Error> 
where 
    <R as DatabaseRecord>::PrimaryKey: CursorType,
    T: TryFrom<R> + ObjectType,
    <T as TryFrom<R>>::Error: 'static + Display + Send + Sync
{
    query(after, before, first, last, |after, before, first, last| async move {
        let collection = R::collection(&db);
        let filter = filter.unwrap_or_default();
        let total= collection.aggregate(vec! [
            doc! { "$match": filter.clone() },
            doc! { "$count": "total" }
        ]).await?
        .try_next().await?.unwrap_or_default()
        .get("total").unwrap_or(&mongodb::bson::Bson::Int32(0))
        .as_i32().unwrap() as i64;
        
        let mut start = after.map(|after| after + 1).unwrap_or(0);
        let mut end = before.unwrap_or(total);

        if let Some(first) = first {
            end = (start + first as i64).min(end);
        }

        if let Some(last) = last {
            start = if last as i64 > end - start {
                end
            } else {
                end - last as i64
            };
        }
    
        let mut cursor = collection
            .find(filter)
            .sort(doc!{ R::key_name(): 1 })
            .skip(start as u64)
            .limit(end - start)
            .await?;

        let mut records = vec![];

        while let Some(record) = cursor.try_next().await? {
            records.push(T::try_from(record)?);
        }

        let mut connection = Connection::new(start > 0, end < total);
        connection.edges.extend(
            records.into_iter().enumerate().map(|(i, r)| 
                Edge::new(i as i64 + start, r)
            )
        );
    
        Ok::<_, Error>(connection)
    }).await
}