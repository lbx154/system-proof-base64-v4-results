//! Shared conditional decoder composition interfaces. Lower bodies remain obligations.
#![allow(missing_docs)]
use vstd::prelude::*;
use crate::engine::{DecodeMetadata, DecodePaddingMode};
use crate::DecodeSliceError;
verus! {
pub open spec fn table_ok(t: Seq<u8>) -> bool {
    t.len() == 256 && forall|v: int| 0 <= v < 64 ==>
        #[trigger] t[crate::round_trip_spec::symbol(v) as int] == v
}
pub open spec fn config_ok(p: crate::alphabet::Symbol, m: DecodePaddingMode, allow: bool) -> bool {
    p.byte() == 61 && m == DecodePaddingMode::RequireCanonical && !allow
}
pub open spec fn image(b: Seq<u8>, input: Seq<u8>) -> bool {
    crate::round_trip_spec::supported(b.len() as int)
    && input == crate::round_trip_spec::encoding(b)
}
pub open spec fn boundary(m: int) -> int { if m == 0 { 0 } else { 4 * ((m - 1) / 4) } }
pub open spec fn metadata(b: Seq<u8>, r: Result<DecodeMetadata, DecodeSliceError>) -> bool {
    r.is_ok() && r.unwrap().count() == b.len()
    && r.unwrap().first_padding() == if b.len() % 3 == 0 { None }
       else { Some(crate::round_trip_proof::unpadded_size(b.len() as int) as usize) }
}
// Exact range write and complete destination frame; also used by nonzero SIMD prefixes.
pub open spec fn range_write(before: Seq<u8>, after: Seq<u8>, b: Seq<u8>, lo: int, hi: int) -> bool {
    &&& before.len() == after.len()
    &&& 0 <= lo <= hi <= b.len()
    &&& hi <= after.len()
    &&& forall|j: int| lo <= j < hi ==> after[j] == b[j]
    &&& forall|j: int| 0 <= j < after.len() && !(lo <= j < hi) ==> after[j] == before[j]
}
pub proof fn compose(before: Seq<u8>, middle: Seq<u8>, after: Seq<u8>, b: Seq<u8>, k: int)
    requires range_write(before, middle, b, 0, k), range_write(middle, after, b, k, b.len() as int),
    ensures crate::round_trip_spec::write_frame(before, after, b),
{
    assert(after.take(b.len() as int) =~= b);
}
pub proof fn image_bounds(b: Seq<u8>, input: Seq<u8>)
    requires image(b, input),
    ensures input.len() % 4 == 0,
        0 <= boundary(input.len() as int) <= input.len(),
        boundary(input.len() as int) % 4 == 0,
        0 <= boundary(input.len() as int) / 4 * 3 <= b.len(),
        input.len() - boundary(input.len() as int) <= 4,
        b.len() <= crate::round_trip_spec::decoded_capacity(input.len() as int),
{}
// Public Result::map projection of the same metadata/frame guarantee.
pub proof fn public_projection(input: Seq<u8>, before: Seq<u8>, after: Seq<u8>,
    r: Result<DecodeMetadata, DecodeSliceError>)
    requires exists|b: Seq<u8>| image(b, input),
        forall|b: Seq<u8>| #[trigger] image(b, input) ==> metadata(b, r)
            && crate::round_trip_spec::write_frame(before, after, b),
    ensures r.is_ok(), forall|b: Seq<u8>| #[trigger] image(b, input)
        ==> crate::round_trip_spec::decode_post(b, before, after, r.unwrap().count() as int),
{
    let b = choose|b: Seq<u8>| image(b, input);
    assert(metadata(b, r));
    assert forall|b: Seq<u8>| #[trigger] image(b, input)
        implies crate::round_trip_spec::decode_post(b, before, after, r.unwrap().count() as int) by {
        assert(metadata(b, r));
    }
}

}
