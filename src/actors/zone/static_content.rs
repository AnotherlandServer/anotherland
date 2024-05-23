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

use std::collections::HashMap;

use atlas::Uuid;
use once_cell::sync::Lazy;

pub enum DialogChoice {
    Approve(String),
    Reject(String),
    Next(String),
    TellMore(String),
}

pub struct Dialog {
    pub id: i32,
    pub serial: String,
    pub dialog_line_id: u32,
    pub choice_serials: Vec<DialogChoice>,
    pub quest: Option<()>,
}

pub static DIALOGS: Lazy<HashMap<(i32, String), Dialog>> = Lazy::new(|| {
    vec![
        Dialog {
            id: 1355,
            serial: "0".to_string(),
            dialog_line_id: 13502,
            choice_serials: vec![
                DialogChoice::TellMore("1".to_string())
            ],
            quest: None,
        },
        Dialog {
            id: 1355,
            serial: "1".to_string(),
            dialog_line_id: 13503,
            choice_serials: vec![
                DialogChoice::TellMore("2".to_string())
            ],
            quest: None,
        },
        Dialog {
            id: 1355,
            serial: "2".to_string(),
            dialog_line_id: 13504,
            choice_serials: vec![],
            quest: Some(()),
        }
    ]
    .into_iter()
    .map(|v| ((v.id, v.serial.clone()), v))
    .collect()
});
