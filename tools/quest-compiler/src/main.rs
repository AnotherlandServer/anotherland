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

use std::path::PathBuf;

use clap::Parser;
use content::set_content_path;
use futures_util::future;
use log::info;
use realm_api::RealmApi;
use reqwest::Url;
use toolkit::{once_cell::sync::Lazy, print_banner};

mod error;
mod dialogue_importer;
mod quest_importer;

pub use error::*;

use crate::{dialogue_importer::{import_dialogues, watch_dialogue_changes}, quest_importer::{import_quest_templates, watch_quest_template_changes}};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(long, env = "CONTENT_PATH", default_value = "./content")]
    content_path: PathBuf,

    #[arg(long, env = "SERVICE_REALM_API_URL", default_value = "http://127.0.0.1:8001")]
    service_realm_url: Url,

    #[arg(long, default_value_t = false)]
    hot_reload_dialogues: bool,

    #[arg(long, default_value_t = false)]
    hot_reload_quests: bool,
}

static ARGS: Lazy<Cli> = Lazy::new(Cli::parse);

#[tokio::main]
async fn main() -> Result<()> {
    Lazy::force(&ARGS);

    let _ = toolkit::dotenvy::dotenv();
    toolkit::env_logger::Builder::from_env(
        toolkit::env_logger::Env::default()
        .default_filter_or("info")
    ).init();

    print_banner();

    set_content_path(ARGS.content_path.clone())?;

    RealmApi::init(ARGS.service_realm_url.clone());

    import_dialogues().await?;

    if ARGS.hot_reload_dialogues {
        watch_dialogue_changes()?;
    }

    import_quest_templates().await?;

    if ARGS.hot_reload_quests {
        watch_quest_template_changes()?;
    }

    if ARGS.hot_reload_dialogues || ARGS.hot_reload_quests {
        info!("Watching for changes...");

        // Just wait forever since the file watchers run in the background
        future::pending::<()>().await;
    }

    Ok(())
}
