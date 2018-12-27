// Copyright 2015-2017 Brian Smith.
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHORS DISCLAIM ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHORS BE LIABLE FOR ANY
// SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION
// OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF OR IN
// CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.

use crate::{error, rand};

pub use self::keys::{KeyPair, PublicKey, Seed};

pub struct Curve {
    pub public_key_len: usize,
    pub elem_scalar_seed_len: usize,

    pub id: CurveID,

    // Precondition: `bytes` is the correct length.
    check_private_key_bytes: fn(bytes: &[u8]) -> Result<(), error::Unspecified>,

    generate_private_key: fn(rng: &rand::SecureRandom, &mut [u8]) -> Result<(), error::Unspecified>,

    public_from_private:
        fn(public_out: &mut [u8], private_key: &Seed) -> Result<(), error::Unspecified>,
}

derive_debug_via_self!(Curve, self.id);

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CurveID {
    Curve25519,
    P256,
    P384,
}

const ELEM_MAX_BITS: usize = 384;
pub const ELEM_MAX_BYTES: usize = (ELEM_MAX_BITS + 7) / 8;

pub const SCALAR_MAX_BYTES: usize = ELEM_MAX_BYTES;
const SEED_MAX_BYTES: usize = ELEM_MAX_BYTES;

/// The maximum length of a PKCS#8 documents generated by *ring* for ECC keys.
///
/// This is NOT the maximum length of a PKCS#8 document that can be consumed by
/// `pkcs8::unwrap_key()`.
///
/// `40` is the length of the P-384 template. It is actually one byte shorter
/// than the P-256 template, but the private key and the public key are much
/// longer.
pub const PKCS8_DOCUMENT_MAX_LEN: usize = 40 + SCALAR_MAX_BYTES + keys::PUBLIC_KEY_MAX_LEN;

pub mod curve25519;
mod keys;
pub mod suite_b;
