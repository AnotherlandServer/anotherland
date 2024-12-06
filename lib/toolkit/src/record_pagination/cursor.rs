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

use std::{collections::VecDeque, future::Future, pin::Pin, sync::Arc, task::{Context, Poll}};

use futures::{future::BoxFuture, lock::Mutex, Stream};

pub struct RecordPage<T> {
    pub records: Vec<T>,
    pub at_end: bool,
    pub last_cursor: Option<String>
}

pub trait RecordQuery {
    type Record;
    type Error;

    fn query_next(&mut self, after: Option<String>, limit: usize) -> impl Future<Output = Result<RecordPage<Self::Record>, Self::Error>> + Send;
}

pub struct RecordCursor<T: RecordQuery> {
    query: Arc<Mutex<T>>,
    records: VecDeque<T::Record>,
    at_end: bool,
    cursor: Option<String>,
    pending_query: Option<BoxFuture<'static, Result<RecordPage<T::Record>, T::Error>>>,
}

impl <T: RecordQuery> RecordCursor<T> {
    pub fn new(query: T) -> Self {
        Self {
            query: Arc::new(Mutex::new(query)),
            records: VecDeque::with_capacity(10),
            at_end: false,
            cursor: None,
            pending_query: None,
        }
    }
}

impl <T: RecordQuery> RecordCursor<T> 
    where
        T: 'static + Send,
{
    fn query_more(&mut self, cx: &mut Context<'_>) -> Poll<Result<RecordPage<T::Record>, T::Error>> {
        // If we have a query running, process that
        if let Some(query) = &mut self.pending_query {
            query.as_mut().poll(cx)
        } else {
            let query = self.query.clone();
            let cursor = self.cursor.clone();

            // Start new query if buffer ran dry and we are not at end
            self.pending_query = Some(Box::pin(async move {
                let mut query = query.lock().await;
                query.query_next(cursor, 10).await
            }));

            // Poll query for the first time
            self.pending_query.as_mut().unwrap().as_mut().poll(cx)
        }
    }
}

impl <T: RecordQuery> Stream for RecordCursor<T> 
    where
        T: 'static + Send,
        T::Error: Send,
        T::Record: Unpin + Send,
{
    type Item = Result<T::Record, T::Error>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let buf = self.get_mut();

        // Return buffered rows
        if let Some(record) = buf.records.pop_front() {
            Poll::Ready(Some(Ok(record)))
        } else if buf.at_end {
            Poll::Ready(None)
        } else {
            match buf.query_more(cx) {
                Poll::Ready(Ok(page)) => {
                    buf.pending_query = None;
                    buf.at_end = page.at_end;
                    buf.cursor = page.last_cursor;
                    buf.records = page.records.into();

                    if let Some(record) = buf.records.pop_front() {
                        Poll::Ready(Some(Ok(record)))
                    } else {
                        Poll::Ready(None)
                    }
                },
                Poll::Ready(Err(err)) => {
                    buf.at_end = true;
                    buf.pending_query = None;
                    Poll::Ready(Some(Err(err)))
                },
                Poll::Pending => {
                    Poll::Pending
                }
            }
        }
    }
}