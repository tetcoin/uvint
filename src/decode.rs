// Copyright 2018 Parity Technologies (UK) Ltd.
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of
// this software and associated documentation files (the "Software"), to deal in
// the Software without restriction, including without limitation the rights to
// use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software is furnished to do so,
// subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
// FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS
// OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY,
// WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

quick_error! {
    #[derive(Debug, PartialEq, Eq)]
    pub enum Error {
        /// Not enough input bytes.
        Insufficient {
            display("not enough input bytes")
        }
        /// Input bytes exceed maximum.
        Overflow {
            display("input bytes exceed maximum")
        }
        #[doc(hidden)]
        __Nonexhaustive
    }
}

macro_rules! decode {
    ($buf:expr, $max_bytes:expr, $typ:ident) => {{
        let mut num = 0;
        for (i, byte) in $buf.iter().enumerate() {
            let n = $typ::from(byte & 0x7F);
            num |= n << i * 7;
            if byte & 0x80 == 0 {
                return Ok((num, i + 1))
            }
            if i == $max_bytes {
                return Err(Error::Overflow)
            }
        }
        Err(Error::Insufficient)
    }}
}

/// Decode the given slice as `u8`.
///
/// Returns the value and the start index of the remaining slice.
#[inline]
pub fn u8(buf: &[u8]) -> Result<(u8, usize), Error> {
    decode!(buf, 1, u8)
}

/// Decode the given slice as `u16`.
///
/// Returns the value and the start index of the remaining slice.
#[inline]
pub fn u16(buf: &[u8]) -> Result<(u16, usize), Error> {
    decode!(buf, 2, u16)
}

/// Decode the given slice as `u32`.
///
/// Returns the value and the start index of the remaining slice.
#[inline]
pub fn u32(buf: &[u8]) -> Result<(u32, usize), Error> {
    decode!(buf, 4, u32)
}

/// Decode the given slice as `u64`.
///
/// Returns the value and the start index of the remaining slice.
#[inline]
pub fn u64(buf: &[u8]) -> Result<(u64, usize), Error> {
    decode!(buf, 9, u64)
}

/// Decode the given slice as `u128`.
///
/// Returns the value and the start index of the remaining slice.
#[inline]
pub fn u128(buf: &[u8]) -> Result<(u128, usize), Error> {
    decode!(buf, 18, u128)
}

/// Decode the given slice as `usize`.
///
/// Returns the value and the start index of the remaining slice.
#[inline]
#[cfg(target_pointer_width = "64")]
pub fn usize(buf: &[u8]) -> Result<(usize, usize), Error> {
    u64(buf).map(|(n, i)| (n as usize, i))
}

/// Decode the given slice as `usize`.
///
/// Returns the value and the start index of the remaining slice.
#[inline]
#[cfg(target_pointer_width = "32")]
pub fn usize(buf: &[u8]) -> Result<(usize, usize), Error> {
    u32(buf).map(|(n, i)| (n as usize, i))
}
