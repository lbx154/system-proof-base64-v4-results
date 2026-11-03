//! Source-grounded checked exact-prefix consumer; conditional on D-ENC scalar and D-DEC.
//! Recipe follows encode_slice's truncate-to-reported-prefix doctest and
//! decode_slice's decoded_len_estimate sizing documentation.
use vstd::prelude::*;
use crate::{Engine as _, engine::general_purpose::STANDARD, encoded_len, decoded_len_estimate};
use crate::roundtrip_spec::*;
verus! {
/// Arbitrary input and initialized destinations; preserve their surplus bytes.
pub fn standard_roundtrip(s: &[u8], enc: &mut Vec<u8>, dec: &mut Vec<u8>) -> (used: usize)
    requires capacities(s@.len() as int, old(enc)@.len() as int, old(dec)@.len() as int),
    ensures
        used == s@.len(),
        writes(old(enc)@, final(enc)@, encoded(s@)),
        writes(old(dec)@, final(dec)@, s@),
{
    let encoded_capacity = encoded_len(s.len(), true).unwrap();
    let decoded_capacity = decoded_len_estimate(encoded_capacity);
    assert!(enc.len() >= encoded_capacity);
    assert!(dec.len() >= decoded_capacity);
    let written = STANDARD.encode_slice(s, enc.as_mut_slice()).unwrap();
    // Pass exactly the API-reported prefix, not its capacity or trailing bytes.
    let used = STANDARD.decode_slice(&enc.as_slice()[..written], dec.as_mut_slice()).unwrap();
    used
}
}
