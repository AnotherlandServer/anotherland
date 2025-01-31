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

use std::{net::IpAddr, time::{Duration, SystemTime}};

pub fn cur_timestamp(reference_time: SystemTime) -> Duration {
    SystemTime::now()
        .duration_since(reference_time)
        .unwrap()
}

pub trait BinaryAddress {
    fn to_bytes(&self) -> Vec<u8>;
}

impl BinaryAddress for IpAddr {
    fn to_bytes(&self) -> Vec<u8> {
        match self {
            IpAddr::V4(ip) => ip.octets().to_vec(),
            IpAddr::V6(ip) => ip.to_ipv4()
                .map(|ip| ip.octets().to_vec())
                .unwrap_or(ip.octets()[12..].to_vec()) 
                    // This is not okay, but this version of RakNet has method
                    // for dealing with IPv6.
        }
    }
}
