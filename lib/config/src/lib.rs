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

mod types;

use config_crate::File;
use once_cell::sync::Lazy;
use glob::glob;

pub use types::*;

pub static CLUSTER_CONF: Lazy<ConfClusterConfig> = Lazy::new(|| {
    type Config = ::config_crate::Config;
    
    let mut builder = Config::builder()
        .add_source(
            glob("conf/*.toml")
                .unwrap()
                .map(|path| File::from(path.unwrap()))
                .collect::<Vec<_>>(),
        );

    if cfg!(unix) {
        builder = builder.add_source(
            glob("/etc/anotherland/*.toml")
                .unwrap()
                .map(|path| File::from(path.unwrap()))
                .collect::<Vec<_>>(),
        );
    }
        
    builder
        .build()
        .unwrap()
        .try_deserialize::<ConfClusterConfig>()
        .expect("Failed to parse config")
});

pub static REALM_CONF: Lazy<ConfRealmMain> = Lazy::new(|| {
    type Config = ::config_crate::Config;
    
    let mut builder = Config::builder()
        .add_source(
            glob("conf/*.toml")
                .unwrap()
                .map(|path| File::from(path.unwrap()))
                .collect::<Vec<_>>(),
        );

    if cfg!(unix) {
        builder = builder.add_source(
            glob("/etc/anotherland/*.toml")
                .unwrap()
                .map(|path| File::from(path.unwrap()))
                .collect::<Vec<_>>(),
        );
    }
        
    builder
        .build()
        .unwrap()
        .try_deserialize::<ConfRealmMain>()
        .expect("Failed to parse config")
});