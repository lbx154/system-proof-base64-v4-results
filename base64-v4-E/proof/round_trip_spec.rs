//! Canonical caller-visible meaning for the STANDARD slice round trip.
//! Ghost-only definitions; no implementation readiness predicates or accepted-error premise.
use vstd::prelude::*;

verus! {
/// ceil(n/3), avoiding executable n+2 overflow.
pub open spec fn groups(n: nat) -> nat { n / 3 + if n % 3 == 0 { 0nat } else { 1nat } }
pub open spec fn encoded_size(n: nat) -> nat { 4 * groups(n) }
pub open spec fn decoded_estimate(e: nat) -> nat { 3 * (e / 4 + if e % 4 == 0 { 0nat } else { 1nat }) }
/// Maximum total byte size of each valid Rust slice, independently of codec acceptance.
pub open spec fn byte_slice_max() -> nat { isize::MAX as nat }
pub open spec fn supported(n: nat) -> bool { n <= 3 * (byte_slice_max() / 4) }
pub open spec fn represented(s: Seq<u8>) -> bool { s.len() <= byte_slice_max() }

/// The STANDARD alphabet, independent of the implementation's lookup tables.
pub open spec fn standard_symbol(v: nat) -> u8 {
    if v < 26 { (65 + v) as u8 }
    else if v < 52 { (97 + v - 26) as u8 }
    else if v < 62 { (48 + v - 52) as u8 }
    else if v == 62 { 43u8 } else { 47u8 }
}
/// Zero fill only the incomplete final input group; never assume input text/UTF-8.
pub open spec fn byte_or_zero(x: Seq<u8>, i: nat) -> nat {
    if i < x.len() { x[i as int] as nat } else { 0nat }
}
pub open spec fn encoded_byte(x: Seq<u8>, j: nat) -> u8 {
    let b = 3 * (j / 4);
    let a = byte_or_zero(x, b);
    let c = byte_or_zero(x, b + 1);
    let d = byte_or_zero(x, b + 2);
    if j % 4 == 0 { standard_symbol(a / 4) }
    else if j % 4 == 1 { standard_symbol((a % 4) * 16 + c / 16) }
    else if j % 4 == 2 {
        if b + 1 >= x.len() { 61u8 } else { standard_symbol((c % 16) * 4 + d / 64) }
    } else {
        if b + 2 >= x.len() { 61u8 } else { standard_symbol(d % 64) }
    }
}
pub open spec fn standard_encoding(x: Seq<u8>) -> Seq<u8> {
    Seq::new(encoded_size(x.len()), |j: int| encoded_byte(x, j as nat))
}
pub open spec fn suffix_unchanged(before: Seq<u8>, after: Seq<u8>, k: nat) -> bool {
    before.len() == after.len() && k <= after.len()
    && forall|j: int| k <= j < after.len() ==> after[j] == before[j]
}
pub open spec fn encode_requires(x: Seq<u8>, dst: Seq<u8>) -> bool {
    represented(x) && supported(x.len()) && represented(dst) && dst.len() >= encoded_size(x.len())
}
/// Actual target ensures must additionally say result == Ok(written), with no other result permitted.
pub open spec fn encode_post(x: Seq<u8>, x_after: Seq<u8>, before: Seq<u8>, after: Seq<u8>, written: nat) -> bool {
    x_after == x && written == encoded_size(x.len())
    && suffix_unchanged(before, after, written)
    && after.subrange(0, written as int) == standard_encoding(x)
}
/// x is a ghost witness established by the preceding encode result, not an executable input.
pub open spec fn decode_requires(x: Seq<u8>, encoded: Seq<u8>, dst: Seq<u8>) -> bool {
    represented(x) && supported(x.len()) && represented(encoded) && represented(dst)
    && encoded == standard_encoding(x) && dst.len() >= decoded_estimate(encoded.len())
}
/// No decoded-byte relation is opaque. Bytes, order, multiplicity, length and frame are explicit.
pub open spec fn decode_post(x: Seq<u8>, encoded: Seq<u8>, encoded_after: Seq<u8>, before: Seq<u8>, after: Seq<u8>, written: nat) -> bool {
    encoded_after == encoded && written == x.len()
    && suffix_unchanged(before, after, written)
    && after.subrange(0, written as int) == x
}
/// Attach to the native Result-returning target: Err is forbidden in this domain.
pub open spec fn encode_contract<E>(x: Seq<u8>, x_after: Seq<u8>, before: Seq<u8>, after: Seq<u8>, result: Result<usize, E>) -> bool {
    match result {
        Result::Ok(written) => encode_post(x, x_after, before, after, written as nat),
        Result::Err(_) => false,
    }
}
pub open spec fn decode_contract<E>(x: Seq<u8>, encoded: Seq<u8>, encoded_after: Seq<u8>, before: Seq<u8>, after: Seq<u8>, result: Result<usize, E>) -> bool {
    match result {
        Result::Ok(written) => decode_post(x, encoded, encoded_after, before, after, written as nat),
        Result::Err(_) => false,
    }
}
/// Public decoding can quantify the witness instead of adding it as a runtime argument.
pub open spec fn round_trip_decode_requires(encoded: Seq<u8>, dst: Seq<u8>) -> bool {
    exists|x: Seq<u8>| decode_requires(x, encoded, dst)
}
pub open spec fn round_trip_decode_contract<E>(encoded: Seq<u8>, encoded_after: Seq<u8>, before: Seq<u8>, after: Seq<u8>, result: Result<usize, E>) -> bool {
    forall|x: Seq<u8>| decode_requires(x, encoded, before) ==> decode_contract(x, encoded, encoded_after, before, after, result)
}

}
