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

use std::cmp::max;

macro_rules! encode {
    ($number:expr, $width:expr, $buf:expr) => {{
        let bits = $width - $number.leading_zeros() as usize;
        let len = max(1, (bits as f32 / 7.0).ceil() as usize);
        for (i, byte) in $buf.iter_mut().enumerate().take(len) {
            *byte = ($number >> i * 7) as u8 | 0x80
        }
        $buf[len - 1] &= 0x7F;
        &$buf[0..len]
    }}
}

/// Encode the given `u8` into the given byte array.
///
/// Returns the slice of encoded bytes.
#[inline]
pub fn u8(number: u8, buf: &mut [u8; 2]) -> &[u8] {
    encode!(number, 8, buf)
}

/// Encode the given `u16` into the given byte array.
///
/// Returns the slice of encoded bytes.
#[inline]
pub fn u16(number: u16, buf: &mut [u8; 3]) -> &[u8] {
    encode!(number, 16, buf)
}

/// Encode the given `u32` into the given byte array.
///
/// Returns the slice of encoded bytes.
#[inline]
pub fn u32(number: u32, buf: &mut [u8; 5]) -> &[u8] {
    encode!(number, 32, buf)
}

/// Encode the given `u64` into the given byte array.
///
/// Returns the slice of encoded bytes.
#[inline]
pub fn u64(number: u64, buf: &mut [u8; 10]) -> &[u8] {
    encode!(number, 64, buf)
}

/// Encode the given `u128` into the given byte array.
///
/// Returns the slice of encoded bytes.
#[inline]
pub fn u128(number: u128, buf: &mut [u8; 19]) -> &[u8] {
    encode!(number, 128, buf)
}

/// Encode the given `usize` into the given byte array.
///
/// Returns the slice of encoded bytes.
#[inline]
#[cfg(target_pointer_width = "64")]
pub fn usize(number: usize, buf: &mut [u8; 10]) -> &[u8] {
    u64(number as u64, buf)
}

/// Encode the given `usize` into the given byte array.
///
/// Returns the slice of encoded bytes.
#[inline]
#[cfg(target_pointer_width = "32")]
pub fn usize(number: usize, buf: &mut [u8; 5]) -> &[u8] {
    u32(number as u32, buf)
}
