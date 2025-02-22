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

use cash_shop_item::import_cash_shop_item;
use cash_shop_item_bundle::import_cash_shop_item_bundles;
use cash_shop_vendor::import_cash_shop_vendors;
use clap::Parser;
use error::SeedRealmResult;
use indicatif::MultiProgress;
use indicatif_log_bridge::LogWrapper;
use log::info;
use object_template::import_object_templates;
use once_cell::sync::Lazy;
use object_placement::import_object_placements;
use realm_api::RealmApi;
use reqwest::Url;
use toolkit::print_banner;
use worlddef::import_worlddef;
use zone::import_zone;

mod error;
mod worlddef;
mod zone;
mod object_placement;
mod object_template;
mod cash_shop_item_bundle;
mod cash_shop_item;
mod cash_shop_vendor;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(long, env = "SERVICE_REALM_API_URL", default_value = "http://127.0.0.1:8001")]
    service_realm_url: Url,

    client_path: PathBuf,
}

static ARGS: Lazy<Cli> = Lazy::new(Cli::parse);
pub static MP: Lazy<MultiProgress> = Lazy::new(MultiProgress::new);

#[tokio::main]
async fn main() -> SeedRealmResult<()> {
    Lazy::force(&ARGS);

    let _ = toolkit::dotenvy::dotenv();
    let logger = toolkit::env_logger::Builder::from_env(
            toolkit::env_logger::Env::default()
            .default_filter_or("info")
        ).build();
    let level = logger.filter();
    Lazy::force(&MP);

    LogWrapper::new(MP.clone(), logger)
        .try_init()
        .unwrap();

    log::set_max_level(level);

    print_banner();

    let realm_api = RealmApi::new(ARGS.service_realm_url.clone());

    import_worlddef(&ARGS.client_path, &realm_api).await?;
    import_zone(&ARGS.client_path, &realm_api).await?;
    import_object_placements(&ARGS.client_path, &realm_api).await?;
    import_object_templates(&ARGS.client_path, &realm_api).await?;
    import_cash_shop_item_bundles(&ARGS.client_path, &realm_api).await?;
    import_cash_shop_item(&ARGS.client_path, &realm_api).await?;
    import_cash_shop_vendors(&ARGS.client_path, &realm_api).await?;

    info!("Import completed!");

    Ok(())
}
