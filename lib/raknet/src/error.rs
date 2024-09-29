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

#[derive(Debug)]
pub enum RakNetError {
    BindAddressError,
    ReadPacketBufferError,
    ParserMismatchError,
    FrameError,
    UnknownPacketId,
    NotListening,
}

pub type Result<T> = std::result::Result<T, RakNetError>;

// Magic payload to identify offline messages
static OFFLINE_MESSSAGE_DATA_ID: &[u8] = &[
    0x00, 0xFF, 0x00, 0xFF, // 0xFF00FF00u32 in little-endian
    0xFE, 0xFE, 0xFE, 0xFE, // 0xFEFEFEFEu32 in little-endian
    0xFD, 0xFD, 0xFD, 0xFD, // 0xFDFDFDFDu32 in little-endian
    0x78, 0x56, 0x34, 0x12  // 0x12345678u32 in little-endian
];