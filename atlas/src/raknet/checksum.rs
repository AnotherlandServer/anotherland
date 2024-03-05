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

/*
Just like the original RakNet implementation, this checksum algorithm is based on the algorithm
as outlined here: http://www.flounder.com/checksum.htm
*/

pub(crate) struct Checksum {
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
