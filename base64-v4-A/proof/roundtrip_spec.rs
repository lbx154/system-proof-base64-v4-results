//! Caller-visible candidate meaning for the STANDARD slice round trip.
//! The codec bridge is NOT yet established; see .system-proof/t1/handoff.md.
use vstd::prelude::*;
verus! {
/// Padded STANDARD length, in mathematical arithmetic.
pub open spec fn encoded_size(n: int) -> int { 4 * ((n + 2) / 3) }
/// Public conservative decode estimate, including nonmultiples of four.
pub open spec fn decode_capacity(m: int) -> int { 3 * ((m + 3) / 4) }
/// Arithmetic representability of the padded helper result, independent of buffers.
pub open spec fn helper_domain(n: int) -> bool {
    0 <= n <= usize::MAX && encoded_size(n) <= usize::MAX
}
/// Realizable byte-slice lengths (allocation availability is a separate condition).
pub open spec fn slice_length(n: int) -> bool { 0 <= n <= isize::MAX }
/// Domain for the exact public-helper-sized buffer recipe.
pub open spec fn roundtrip_domain(n: int) -> bool {
    slice_length(n) && helper_domain(n) && slice_length(encoded_size(n))
        && slice_length(decode_capacity(encoded_size(n)))
}
/// STANDARD's public alphabet, independent of implementation tables.
pub open spec fn alphabet(i: int) -> u8 {
    if i < 26 { (65 + i) as u8 }
    else if i < 52 { (97 + i - 26) as u8 }
    else if i < 62 { (48 + i - 52) as u8 }
    else if i == 62 { 43u8 } else { 47u8 }
}
/// Byte at an encoded position, with absent final source bytes zero extended.
pub open spec fn encoded_at(s: Seq<u8>, k: int) -> u8 {
    let b = (k / 4) * 3;
    let a = s[b] as int;
    let c = if b + 1 < s.len() { s[b + 1] as int } else { 0 };
    let d = if b + 2 < s.len() { s[b + 2] as int } else { 0 };
    if k % 4 == 0 { alphabet(a / 4) }
    else if k % 4 == 1 { alphabet((a % 4) * 16 + c / 16) }
    else if k % 4 == 2 {
        if b + 1 < s.len() { alphabet((c % 16) * 4 + d / 64) } else { 61u8 }
    } else {
        if b + 2 < s.len() { alphabet(d % 64) } else { 61u8 }
    }
}
/// Fully defined encoded bytes: no opaque relation between encoder and decoder.
pub open spec fn encoded(s: Seq<u8>) -> Seq<u8> {
    Seq::new(encoded_size(s.len() as int) as nat, |k: int| encoded_at(s, k))
}
/// Exact destination write plus frame, allowing arbitrary initial contents.
pub open spec fn writes(before: Seq<u8>, after: Seq<u8>, data: Seq<u8>) -> bool {
    data.len() <= before.len() && after.len() == before.len()
        && after.take(data.len() as int) == data
        && after.skip(data.len() as int) == before.skip(data.len() as int)
}
/// Preconditions at the byte-slice inner entry; buffers are already lawful slices.
pub open spec fn encode_pre(supplied: Seq<u8>, destination: Seq<u8>) -> bool {
    slice_length(supplied.len() as int) && slice_length(destination.len() as int)
        && helper_domain(supplied.len() as int)
        && encoded_size(supplied.len() as int) <= destination.len()
}
/// Decoder domain is encoder-produced strings, without implementation-success premises.
pub open spec fn decode_pre(original: Seq<u8>, supplied: Seq<u8>, destination: Seq<u8>) -> bool {
    roundtrip_domain(original.len() as int) && supplied == encoded(original)
        && slice_length(destination.len() as int)
        && decode_capacity(supplied.len() as int) <= destination.len()
}
/// `None` represents an API error for this observation; no error is permitted.
/// `supplied` denotes the bytes returned by the ONE actual as_ref invocation.
/// `before` is sampled at entry to the nested byte-slice inner body.
pub open spec fn encode_guarantee(
    supplied: Seq<u8>, before: Seq<u8>, after: Seq<u8>, reported: Option<usize>,
) -> bool {
    reported == Some(encoded_size(supplied.len() as int) as usize)
        && writes(before, after, encoded(supplied))
}
/// On strings produced by STANDARD, mandatory decode success and exact data.
/// `supplied` must equal encoded(original); caller passes exactly that prefix.
pub open spec fn decode_guarantee(
    original: Seq<u8>, supplied: Seq<u8>, before: Seq<u8>, after: Seq<u8>, reported: Option<usize>,
) -> bool {
    supplied == encoded(original) && reported == Some(original.len() as usize)
        && writes(before, after, original)
}
/// Independent buffer premises; no premise mentions implementation success.
pub open spec fn capacities(n: int, enc: int, dec: int) -> bool {
    roundtrip_domain(n) && slice_length(enc) && slice_length(dec)
        && encoded_size(n) <= enc && decode_capacity(encoded_size(n)) <= dec
}
}
