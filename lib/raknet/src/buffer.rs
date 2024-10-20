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

use std::io::{Cursor, Read};
use bytes::Buf;

use crate::error::{RakNetError, Result};

pub struct RakNetReader<'a> {
    buf: Cursor<&'a [u8]>,
    bit_offset: usize,
}

impl <'a>RakNetReader<'a> {
    pub fn new(buf: &'a [u8]) -> Self {
        Self {
            buf: Cursor::new(buf),
            bit_offset: 0,
        }
    }

    #[allow(dead_code)]
    pub fn remaining(&self) -> usize {
        (self.buf.remaining() * 8usize - self.bit_offset) / 8usize
    }

    pub fn byte_align(&mut self) {
        if self.bit_offset != 0 {
            self.skip_bits(u8::BITS as usize - self.bit_offset);
        }
    }

    pub fn bits_remaining(&self) -> usize {
        if self.bit_offset > 0 {
            self.buf.remaining() * u8::BITS as usize - self.bit_offset
        } else {
            self.buf.remaining() * u8::BITS as usize
        }
    }

    pub fn read(&mut self, buf: &mut [u8]) -> Result<()> {
        if self.bit_offset == 0 {
            self.buf.read_exact(buf)
                .map_err(|_| RakNetError::ReadPacketBufferError)?;
        } else {
            self.read_bits(buf, buf.len() * u8::BITS as usize)?;
        }

        Ok(())
    }

    pub fn read_bits(&mut self, buf: &mut [u8], mut bits: usize) -> Result<()> {
        if self.bits_remaining() < bits {
            return Err(RakNetError::ReadPacketBufferError);
        }

        let mut out_offset = 0;

        while bits > 0 {
            buf[out_offset] |= self.buf.chunk()[0] << self.bit_offset; // first half of partial byte

            // read remaining bits from second byte
            if self.bit_offset > 0 && bits > 8 - self.bit_offset {
                buf[out_offset] |= self.buf.chunk()[1] >> (8 - self.bit_offset); // second half of partial byte
            }

            if bits >= 8 {
                bits -= 8;
                self.skip_bits(8);
                out_offset += 1;
            } else {
                let neg = bits as isize - u8::BITS as isize;
                if neg < 0 {
                    buf[out_offset] >>= -neg;

                    self.skip_bits((8 + neg) as usize);
                } else {
                    self.skip_bits(8);
                }

                bits = 0;
            }
        }

        Ok(())
    }

    pub fn read_compressed(&mut self, buf: &mut [u8], bits: usize, unsigned_data: bool) -> Result<()> {
        let byte_match;
        let half_byte_match;
        let mut current_byte = (bits >> 3) - 1;

        if unsigned_data {
            byte_match = 0;
            half_byte_match = 0;
        } else {
            byte_match = 0xFF;
            half_byte_match = 0xF0;
        }

        // Upper bytes are specified with a single 1 if they match byteMatch
	    // From high byte to low byte, if high byte is a byteMatch then write a 1 bit. Otherwise write a 0 bit and then write the remaining bytes
        while current_byte > 0 {
            // If we read a 1 then the data is byte_match.
            if self.read_bit()? {
                buf[current_byte] = byte_match;
                current_byte -= 1;
            } else {
                // Read the rest of the bytes
                self.read_bits(buf, (current_byte + 1) << 3)?;
                return Ok(());
            }
        }

        // All but the first bytes are byteMatch. If the upper half of the last byte is a 0 (positive) or 16 (negative) then what we read will be a 1 and the remaining 4 bits.
	    // Otherwise we read a 0 and the 8 bytes
        if self.read_bit()? {
            self.read_bits(buf, 4)?;

            buf[0] |= half_byte_match;
        } else {
            self.read_bits(buf, 8)?;
        }

        Ok(())
    }

    pub fn read_bit(&mut self) -> Result<bool> {
        if self.bits_remaining() < 1 {
            return Err(RakNetError::ReadPacketBufferError);
        }

        let val = (self.buf.chunk()[0] & (0x80 >> self.bit_offset)) != 0;

        self.skip_bits(1);
        Ok(val)
    }

    pub fn read_u8(&mut self) -> Result<u8> {
        if self.bits_remaining() < u8::BITS as usize {
            return Err(RakNetError::ReadPacketBufferError);
        }

        let mut buf = [0; 1];
        self.read(&mut buf)?;

        Ok(buf[0])
    }

    #[allow(dead_code)]
    pub fn read_u8_compressed(&mut self) -> Result<u8> {
        if self.bits_remaining() < u8::BITS as usize {
            return Err(RakNetError::ReadPacketBufferError);
        }

        let mut buf = [0; 1];
        self.read_compressed(&mut buf, u8::BITS as usize, true)?;

        Ok(buf[0])
    }

    pub fn read_u16(&mut self) -> Result<u16> {
        if self.bits_remaining() < u8::BITS as usize {
            return Err(RakNetError::ReadPacketBufferError);
        }

        let mut buf = [0; 2];
        self.read(&mut buf)?;

        Ok(u16::from_le_bytes(buf))
    }

    pub fn read_u16_compressed(&mut self) -> Result<u16> {
        if self.bits_remaining() < u8::BITS as usize {
            return Err(RakNetError::ReadPacketBufferError);
        }

        let mut buf = [0; 2];
        self.read_compressed(&mut buf, u16::BITS as usize, true)?;

        Ok(u16::from_le_bytes(buf))
    }

    pub fn read_u32(&mut self) -> Result<u32> {
        if self.bits_remaining() < u8::BITS as usize {
            return Err(RakNetError::ReadPacketBufferError);
        }

        let mut buf = [0; 4];
        self.read(&mut buf)?;

        Ok(u32::from_le_bytes(buf))
    }

    pub fn read_u32_compressed(&mut self) -> Result<u32> {
        if self.bits_remaining() < u8::BITS as usize {
            return Err(RakNetError::ReadPacketBufferError);
        }

        let mut buf = [0; 4];
        self.read_compressed(&mut buf, u32::BITS as usize, true)?;

        Ok(u32::from_le_bytes(buf))
    }

    #[allow(dead_code)]
    pub fn read_u64(&mut self) -> Result<u64> {
        if self.bits_remaining() < u8::BITS as usize {
            return Err(RakNetError::ReadPacketBufferError);
        }

        let mut buf = [0; 8];
        self.read(&mut buf)?;

        Ok(u64::from_le_bytes(buf))
    }

    #[allow(dead_code)]
    pub fn read_u64_compressed(&mut self) -> Result<u64> {
        if self.bits_remaining() < u8::BITS as usize {
            return Err(RakNetError::ReadPacketBufferError);
        }

        let mut buf = [0; 8];
        self.read_compressed(&mut buf, u64::BITS as usize, true)?;

        Ok(u64::from_le_bytes(buf))
    }

    #[allow(dead_code)]
    pub fn read_i64(&mut self) -> Result<i64> {
        if self.bits_remaining() < u8::BITS as usize {
            return Err(RakNetError::ReadPacketBufferError);
        }

        let mut buf = [0; 8];
        self.read(&mut buf)?;

        Ok(i64::from_le_bytes(buf))
    }

    #[allow(dead_code)]
    pub fn read_i64_compressed(&mut self) -> Result<i64> {
        if self.bits_remaining() < u8::BITS as usize {
            return Err(RakNetError::ReadPacketBufferError);
        }

        let mut buf = [0; 8];
        self.read_compressed(&mut buf, i64::BITS as usize, false)?;

        Ok(i64::from_le_bytes(buf))
    }

    #[allow(dead_code)]
    pub fn read_string(&mut self) -> Result<String> {
        let len = self.read_u16()?;
        let mut buf = vec![0u8; len as usize].into_boxed_slice();

        self.read(&mut buf)?;

        String::from_utf8(buf.into_vec())
            .map_err(|_| RakNetError::ReadPacketBufferError)
    }

    pub fn skip_bits(&mut self, bits: usize) {
        self.bit_offset += bits;

        self.buf.advance(self.bit_offset / 8);
        self.bit_offset &= 7;
    }

    #[allow(dead_code)]
    pub fn bit_pos(&self) -> usize {
        self.buf.position() as usize * u8::BITS as usize + self.bit_offset
    }
}

pub struct RakNetWriter {
    buf: Vec<u8>,
    next_bit_offset: usize,
}

impl RakNetWriter {
    pub fn new() -> Self {
        Self {
            buf: Vec::new(),
            next_bit_offset: 0,
        }
    }

    fn add_bits(&mut self, bits_to_add: usize) {
        let bit_size = self.bits_used() + bits_to_add;

        if self.bits_capacity() < self.bits_used() + bits_to_add {
            self.buf.reserve((bit_size + 7) / 8 );
        }
    }

    pub fn bits_used(&self) -> usize {
        if self.next_bit_offset == 0 {
            self.buf.len() * u8::BITS as usize
        } else {
            (self.buf.len() - 1) * u8::BITS as usize + self.next_bit_offset - 1
        }
    }

    fn bits_capacity(&self) -> usize {
        if self.next_bit_offset == 0 {
            self.buf.capacity() * u8::BITS as usize
        } else {
            (self.buf.capacity() - 1) * u8::BITS as usize + (u8::BITS as usize - self.next_bit_offset)
        }
    }

    pub fn write(&mut self, data: &[u8]) {
        self.add_bits(data.len() * u8::BITS as usize);
        self.write_bits(data.len() * 8, data);
    }

    pub fn write_bits(&mut self, mut bits_to_write: usize, data: &[u8]) {
        let mut offset = 0;

        while bits_to_write > 0 {
            let mut data = data[offset];

            if bits_to_write < 8 {
                data <<= 8 - bits_to_write;
            }

            if self.next_bit_offset == 0 {
                self.buf.push(data);
            } else {
                *self.buf.last_mut().unwrap() |= data.overflowing_shr(self.next_bit_offset as u32).0;
                if (8 - self.next_bit_offset) < 8 && 8 - self.next_bit_offset < bits_to_write {
                    self.buf.push(data.overflowing_shl((8 - self.next_bit_offset) as u32).0);
                }
            }

            if bits_to_write < 8 {
                self.next_bit_offset += bits_to_write;
                self.next_bit_offset %= 8;
            }

            if bits_to_write >= 8 {
                bits_to_write -= 8;
            } else {
                bits_to_write = 0;
            }

            offset += 1;
        }
    }

    pub fn write_compressed(&mut self, unsigned: bool, data: &[u8]) {
        let byte_match = if unsigned { 0 } else { 0xFF };

        for b in (1..data.len()).rev() {
            if data[b] == byte_match {
                self.write_bit(true);
            } else {
                self.write_bit(false);
                self.write(&data[0..b+1]);

                return;
            }
        }

        if (data[0] & 0xF0) == (byte_match & 0xF0) {
            self.write_bit(true);
            self.write_bits(4, &[data[0] & 0x0F]);
        } else {
            self.write_bit(false);
            self.write_bits(8, &[data[0]]);
        }
    }

    pub fn write_aligned(&mut self, data: &[u8]) {
        self.next_bit_offset = 0;
        self.write(data);
    }

    pub fn write_bit(&mut self, val: bool) {
        if self.next_bit_offset == 0 {
            self.buf.push(0);
        }

        if val {
            *self.buf.last_mut().unwrap() |= 0x80 >> self.next_bit_offset;
        }

        self.next_bit_offset += 1;
        self.next_bit_offset &= 7;
    }

    pub fn write_u8(&mut self, val: u8) {
        self.add_bits(8);

        if self.next_bit_offset == 0 {
            self.buf.push(val);
        } else {
            self.write_bits(8, &[val]);
        }
    }

    pub fn write_u16(&mut self, val: u16) {
        self.write(&val.to_le_bytes());
    }

    pub fn write_u16_compressed(&mut self, val: u16) {
        self.write_compressed(true, &val.to_le_bytes());
    }

    pub fn write_u32(&mut self, val: u32) {
        self.write(&val.to_le_bytes());
    }

    pub fn write_u32_compressed(&mut self, val: u32) {
        self.write_compressed(true, &val.to_le_bytes());
    }

    #[allow(dead_code)]
    pub fn write_u64(&mut self, val: u64) {
        self.write(&val.to_le_bytes());
    }

    #[allow(dead_code)]
    pub fn write_i64(&mut self, val: i64) {
        self.write(&val.to_le_bytes());
    }

    #[allow(dead_code)]
    pub fn write_string(&mut self, val: &str) {
        self.write_u16(val.len() as u16);
        self.write(val.as_bytes());
    }

    pub fn take_buffer(self) -> Vec<u8> {
        self.buf
    }

    pub fn buffer(&self) -> &[u8] {
        &self.buf
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_u8_aligned() {
        let data = [0b11010101u8];
        let mut reader = RakNetReader::new(&data);
        let value = reader.read_u8().unwrap();
        assert_eq!(value, 0b11010101u8);
        assert_eq!(reader.bits_remaining(), 0);
    }

    #[test]
    fn test_read_u8_unaligned() {
        let data = [0b11010101u8, 0b01110010u8];
        let mut reader = RakNetReader::new(&data);
        reader.skip_bits(3); // Skip first 3 bits (MSB to LSB)

        let mut buf = [0u8];
        reader.read_bits(&mut buf, 8).unwrap();

        assert_eq!(buf[0], 0b10101011u8);
        assert_eq!(reader.bits_remaining(), 5); // 16 bits total - 3 skipped - 8 read = 5 bits remaining
    }

    #[test]
    fn test_read_u16_aligned() {
        let data = [0xAAu8, 0x55u8]; // 0b10101010, 0b01010101
        let mut reader = RakNetReader::new(&data);
        let value = reader.read_u16().unwrap();
        // Little-endian: least significant byte first
        // So value = 0x55AA
        assert_eq!(value, 0x55AAu16);
        assert_eq!(reader.bits_remaining(), 0);
    }

    #[test]
    fn test_read_u16_unaligned() {
        let data = [0xF0u8, 0x0Fu8, 0xAAu8];
        let mut reader = RakNetReader::new(&data);
        reader.skip_bits(4); // Unaligned read

        let value = reader.read_u16().unwrap();

        assert_eq!(value, 0xFA00u16);
    }

    #[test]
    fn test_read_bits_various_lengths() {
        let data = [0b10101100u8, 0b11010010u8, 0b00110101u8];
        let mut reader = RakNetReader::new(&data);

        // Read first 5 bits
        let mut buf = [0u8];
        reader.read_bits(&mut buf, 5).unwrap();

        assert_eq!(buf[0], 0b10101u8); // Shift right to align bits to the right

        // Read next 10 bits
        let mut buf = [0u8; 2];
        reader.read_bits(&mut buf, 10).unwrap();

        assert_eq!(buf[0], 0b10011010u8);
        assert_eq!(buf[1] & 0b00000011u8, 0b00000001u8); // Check first two bits of buf[1]
    }

    #[test]
    fn test_read_u32_aligned() {
        let data = [0x12u8, 0x34u8, 0x56u8, 0x78u8];
        let mut reader = RakNetReader::new(&data);
        let value = reader.read_u32().unwrap();
        // Little-endian: 0x78563412
        assert_eq!(value, 0x78563412u32);
        assert_eq!(reader.bits_remaining(), 0);
    }

    #[test]
    fn test_read_u32_unaligned() {
        let data = [0xAAu8, 0xBBu8, 0xCCu8, 0xDDu8, 0xEEu8];
        let mut reader = RakNetReader::new(&data);
        reader.skip_bits(7); // Unaligned read

        let value = reader.read_u32().unwrap();

        let expected_value = 0xF76EE65D; // Placeholder value for demonstration
        assert_eq!(value, expected_value);
    }

    #[test]
    fn test_read_bit_by_bit() {
        let data = [0b11100010u8];
        let mut reader = RakNetReader::new(&data);
        for i in 0..8 {
            let bit = reader.read_bit().unwrap();
            // Since bits are read from MSB to LSB, expected_bit is:
            let expected_bit = ((data[0] >> (7 - i)) & 1) == 1;
            assert_eq!(bit, expected_bit);
        }
        assert_eq!(reader.bits_remaining(), 0);
    }

    #[test]
    fn test_byte_align() {
        let data = [0b11110000u8, 0b10101010u8];
        let mut reader = RakNetReader::new(&data);
        reader.skip_bits(3); // Unaligned
        reader.byte_align(); // Align to next byte
        assert_eq!(reader.bit_pos() % 8, 0); // Should be byte-aligned
        let value = reader.read_u8().unwrap();
        assert_eq!(value, 0b10101010u8);
    }

    #[test]
    fn test_bits_remaining() {
        let data = [0xFFu8; 2]; // 16 bits
        let mut reader = RakNetReader::new(&data);
        assert_eq!(reader.bits_remaining(), 16);
        reader.skip_bits(5);
        assert_eq!(reader.bits_remaining(), 11);
    }

    #[test]
    fn test_read_u64_aligned() {
        let data = [0x01u8, 0x23u8, 0x45u8, 0x67u8, 0x89u8, 0xABu8, 0xCDu8, 0xEFu8];
        let mut reader = RakNetReader::new(&data);
        let value = reader.read_u64().unwrap();
        // Little-endian: 0xEFCDAB8967452301
        assert_eq!(value, 0xEFCDAB8967452301u64);
        assert_eq!(reader.bits_remaining(), 0);
    }

    #[test]
    fn test_read_i64() {
        let data = [0xFFu8; 8]; // All bits set
        let mut reader = RakNetReader::new(&data);
        let value = reader.read_i64().unwrap();
        // Should be -1 in two's complement
        assert_eq!(value, -1i64);
        assert_eq!(reader.bits_remaining(), 0);
    }

    #[test]
    fn test_skip_bits() {
        let data = [0b10101010u8, 0b11001100u8];
        let mut reader = RakNetReader::new(&data);
        reader.skip_bits(4);
        let value = reader.read_u8().unwrap();

        assert_eq!(value, 0b10101100u8);
    }

    #[test]
    fn test_bit_pos() {
        let data = [0x00u8; 2]; // Provide enough data to avoid buffer overrun
        let mut reader = RakNetReader::new(&data);
        assert_eq!(reader.bit_pos(), 0);
        reader.skip_bits(3);
        assert_eq!(reader.bit_pos(), 3);
        reader.read_u8().unwrap();
        assert_eq!(reader.bit_pos(), 11); // 3 skipped + 8 read
    }

    #[test]
    fn compressed() {
        let mut buf = RakNetWriter::new();
        buf.write_u16_compressed(12345);
        
        let buf = buf.take_buffer();
        let mut read = RakNetReader::new(&buf);
        assert_eq!(read.read_u16_compressed().unwrap(), 12345);

        let mut buf = RakNetWriter::new();
        buf.write_u16_compressed(1);
        
        let buf = buf.take_buffer();
        let mut read = RakNetReader::new(&buf);
        assert_eq!(read.read_u16_compressed().unwrap(), 1);
    }
}