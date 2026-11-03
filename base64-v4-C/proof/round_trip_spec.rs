//! Open, caller-visible meaning for the STANDARD slice round trip.
//! No executable codec and no trusted operation bodies live here.
#![allow(missing_docs)]
use vstd::prelude::*;
verus! {
pub open spec fn encoded_size(n: int) -> int { 4 * ((n + 2) / 3) }
// Size for either lawful Engine padding choice; no global STANDARD premise.
pub open spec fn encoding_size(n: int, padding: bool) -> int {
    if padding { encoded_size(n) } else { 4 * (n / 3) + if n % 3 == 0 { 0 } else { n % 3 + 1 } }
}
// Timeless admitted-reply relation, NOT an invocation identity or pure View.
// Public codec calls separately export their actual snapshot in an exclusive
// erased event slot; only that captured snapshot indexes their guarantees.
pub open spec fn observed<T: AsRef<[u8]>>(input: T, bytes: &[u8]) -> bool {
    call_ensures(<T as AsRef<[u8]>>::as_ref, (&input,), bytes)
}
// Optional sufficient admission envelope for legacy callers; not a public
// codec precondition and not required for actual-event lawfulness.
pub open spec fn encode_domain<T: AsRef<[u8]>>(input: T) -> bool {
    forall|bytes: &[u8]| #[trigger] observed(input, bytes) ==> supported(bytes@.len() as int)
}
pub open spec fn decoded_capacity(m: int) -> int { 3 * ((m + 3) / 4) }
pub open spec fn supported(n: int) -> bool {
    0 <= n <= 3 * ((isize::MAX as int) / 4)
}
pub open spec fn encoded_helper_post(n: int, r: Option<usize>) -> bool {
    if encoded_size(n) <= usize::MAX as int {
        r == Some(encoded_size(n) as usize)
    } else { r == None }
}
pub open spec fn decoded_helper_post(m: int, r: usize) -> bool {
    r as int == decoded_capacity(m)
}
pub open spec fn symbol(v: int) -> u8 {
    if v < 26 { (65 + v) as u8 }
    else if v < 52 { (97 + v - 26) as u8 }
    else if v < 62 { (48 + v - 52) as u8 }
    else if v == 62 { 43u8 } else { 47u8 }
}
pub open spec fn byte_or_zero(b: Seq<u8>, k: int) -> int {
    if 0 <= k < b.len() { b[k] as int } else { 0 }
}
pub open spec fn encoded_byte(b: Seq<u8>, i: int) -> u8 {
    let j = 3 * (i / 4);
    let a = byte_or_zero(b, j);
    let c = byte_or_zero(b, j + 1);
    let d = byte_or_zero(b, j + 2);
    if i % 4 == 0 { symbol(a / 4) }
    else if i % 4 == 1 { symbol((a % 4) * 16 + c / 16) }
    else if i % 4 == 2 {
        if j + 1 >= b.len() { 61u8 } else { symbol((c % 16) * 4 + d / 64) }
    } else {
        if j + 2 >= b.len() { 61u8 } else { symbol(d % 64) }
    }
}
pub open spec fn encoding(b: Seq<u8>) -> Seq<u8> {
    Seq::new(encoded_size(b.len() as int) as nat, |i: int| encoded_byte(b, i))
}
pub open spec fn write_frame(before: Seq<u8>, after: Seq<u8>, prefix: Seq<u8>) -> bool {
    &&& before.len() == after.len()
    &&& prefix.len() <= after.len()
    &&& after.take(prefix.len() as int) == prefix
    &&& forall|i: int| prefix.len() <= i < after.len() ==> after[i] == before[i]
}
pub open spec fn encode_post(b: Seq<u8>, before: Seq<u8>, after: Seq<u8>, written: int) -> bool {
    written == encoded_size(b.len() as int) && write_frame(before, after, encoding(b))
}
pub open spec fn canonical_input(input: Seq<u8>) -> bool {
    exists|b: Seq<u8>| supported(b.len() as int) && encoding(b) == input
}
/// Optional sufficient admission envelope for legacy callers. Actual-event
/// canonicality/capacity is sufficient even when unrelated admitted replies fail.
pub open spec fn decode_domain<T: AsRef<[u8]>>(input: T, capacity: int) -> bool {
    forall|bytes: &[u8]| #[trigger] observed(input, bytes)
        ==> canonical_input(bytes@) && capacity >= decoded_capacity(bytes.len() as int)
}
/// Lawfulness of the one snapshot actually consumed by an invocation.
/// These predicates say nothing about another conversion, admission, or context.
pub open spec fn encode_event_lawful(bytes: Seq<u8>, capacity: int) -> bool {
    supported(bytes.len() as int) && capacity >= encoded_size(bytes.len() as int)
}
pub open spec fn decode_event_lawful(bytes: Seq<u8>, capacity: int) -> bool {
    canonical_input(bytes) && capacity >= decoded_capacity(bytes.len() as int)
}
pub open spec fn decode_post(b: Seq<u8>, before: Seq<u8>, after: Seq<u8>, written: int) -> bool {
    written == b.len() && write_frame(before, after, b)
}
pub open spec fn round_trip_post(
    b: Seq<u8>, b_after: Seq<u8>, encoded_before: Seq<u8>, encoded_after: Seq<u8>,
    decoded_before: Seq<u8>, decoded_after: Seq<u8>, encode_ok: bool, decode_ok: bool,
    encoded_written: int, decoded_written: int,
) -> bool {
    &&& b_after == b
    &&& encode_ok && decode_ok
    &&& encode_post(b, encoded_before, encoded_after, encoded_written)
    &&& decode_post(b, decoded_before, decoded_after, decoded_written)
}
}
