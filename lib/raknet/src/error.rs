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

use thiserror::Error;

#[derive(Error, Debug)]
pub enum RakNetError {
    #[error("failed to bind address")]
    BindAddressError,
    #[error("failed to read packet buffer")]
    ReadPacketBufferError,
    #[error("invalid message frame")]
    FrameError,
    #[error("unknown packet id")]
    UnknownPacketId,
    #[error("socket is not listening")]
    NotListening,
    #[error("data size is exceeding max mtu")]
    PacketSizeExceedsMTU,
    #[error("connection handshake failed")]
    HandshakeFailed,
    #[error("decryption checksum mismatch")]
    DecryptionFailed,
    #[error("connection was closed")]
    ConnectionClosed,
    #[error("socket error")]
    SocketError,
}

pub type Result<T> = std::result::Result<T, RakNetError>;

// Magic payload to identify offline messages
#[allow(dead_code)]
static OFFLINE_MESSSAGE_DATA_ID: &[u8] = &[
    0x00, 0xFF, 0x00, 0xFF, // 0xFF00FF00u32 in little-endian
    0xFE, 0xFE, 0xFE, 0xFE, // 0xFEFEFEFEu32 in little-endian
    0xFD, 0xFD, 0xFD, 0xFD, // 0xFDFDFDFDu32 in little-endian
    0x78, 0x56, 0x34, 0x12  // 0x12345678u32 in little-endian
];