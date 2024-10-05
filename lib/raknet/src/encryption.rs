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

use std::net::SocketAddr;

use aes::{cipher::{generic_array::GenericArray, BlockDecrypt, KeyInit}, Aes128};
use log::debug;
use rand::{rngs::OsRng, RngCore};
use rsa::RsaPrivateKey;
use sha1::{Digest, Sha1};

use crate::error::{RakNetError, Result};

pub struct EncryptionHanshakeContext {
    addr: SocketAddr,
    rsa_key: RsaPrivateKey,
    syn_cookie: [u8; 20],
}

impl EncryptionHanshakeContext {
    pub fn new(addr: SocketAddr, rsa_key: RsaPrivateKey) -> Self {
        let mut ret = Self {
            addr,
            rsa_key,
            syn_cookie: [0u8; 20],
        };

        ret.create_syn_cookie();

        ret
    }

    pub fn rsa_key(&self) -> &RsaPrivateKey {
        &self.rsa_key
    }

    pub fn syn_cookie(&self) -> &[u8] {
        &self.syn_cookie
    }

    pub fn create_syn_cookie(&mut self) {
        let mut random_number = [0u8; 20];
        OsRng.fill_bytes(&mut random_number);

        let mut hasher = Sha1::new();
        hasher.update(self.addr.ip().to_string());
        hasher.update(self.addr.port().to_le_bytes());
        hasher.update(random_number);

        self.syn_cookie.copy_from_slice(hasher.finalize().as_slice());
    }
}

pub struct Checksum {
    r: u16,
    c1: u16,
    c2: u16,
    sum: u32,
}

impl Checksum {
    pub fn new() -> Self {
        Self {
            r: 55665,
            c1: 52845,
            c2: 22719,
            sum: 0,
        }
    }

    pub fn finish(&self) -> u32 {
        self.sum
    }

    pub fn write(&mut self, bytes: &[u8]) {
        for b in bytes {
            let cipher = b ^ (self.r >> 8) as u8;
            self.r = (cipher as u16)
                .wrapping_add(self.r)
                .wrapping_mul(self.c1)
                .wrapping_add(self.c2);
            self.sum = self.sum.wrapping_add(cipher as u32);
        }
    }
}

pub fn aes_decrypt(key: u128, message: &mut Vec<u8>) -> Result<()> {
    if message.len() % 16 != 0 {
        return Err(RakNetError::DecryptionFailed);
    }

    // initialize decryption
    let mut blocks: Vec<&mut [u8]> = message.chunks_mut(16).collect();
    let cipher = Aes128::new(GenericArray::from_slice(&key.to_le_bytes()));

    // decrypt blocks following the first one
    for index in 1..blocks.len() {
        cipher.decrypt_block(GenericArray::from_mut_slice(blocks[index]));

        for byte_index in 0..16 {
            if index == blocks.len() - 1 {
                blocks[index][byte_index] ^= blocks[0][byte_index];
            } else {
                blocks[index][byte_index] ^= blocks[index + 1][byte_index];
            }
        }
    }

    // decrypt first block
    cipher.decrypt_block(GenericArray::from_mut_slice(blocks[0]));

    // read size of padding
    let paddingbytes = (message[5] & 0x0F) as usize;

    // compute original message length
    let message_len = message.len() - 6 - paddingbytes;

    // validate checksum
    let mut checksum = Checksum::new();
    checksum.write(&message[4..]);

    if u32::from_le_bytes(message[..4].to_owned().try_into().unwrap()) != checksum.finish() {
        debug!("Expected: {}", u32::from_le_bytes(message[..4].to_owned().try_into().unwrap()));
        debug!("Computed: {}", checksum.finish());
        return Err(RakNetError::DecryptionFailed);
    }

    // move decrypted message to the front of the buffer
    message.copy_within(6 + paddingbytes..6 + paddingbytes + message_len, 0);

    // truncate message buffer
    message.truncate(message_len);

    Ok(())
}