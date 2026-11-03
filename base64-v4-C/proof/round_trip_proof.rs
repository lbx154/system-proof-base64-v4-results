//! Internal proof interfaces. Every opaque item here is a named unfinished obligation.
use vstd::prelude::*;
verus! {
// P-SIZES arithmetic from t1/check-open-spec.rs, now consumed by the actual helper.
pub proof fn sizes(n: usize)
    requires crate::round_trip_spec::supported(n as int),
    ensures
        0 <= crate::round_trip_spec::encoded_size(n as int) <= isize::MAX as int,
        n as int <= crate::round_trip_spec::decoded_capacity(
            crate::round_trip_spec::encoded_size(n as int)) <= isize::MAX as int,
        crate::round_trip_spec::encoded_size(n as int) % 4 == 0,
{}

// Exact unpadded Base64 count; the GeneralPurpose producer is checked in t18,
// conditional on the explicit unfinished read_u64 leaf and adaptation correspondence.
pub open spec fn unpadded_size(n: int) -> int {
    4 * (n / 3) + if n % 3 == 0 { 0 } else { n % 3 + 1 }
}
pub open spec fn padding_count(k: int) -> int { (4 - k % 4) % 4 }

pub proof fn padding_sizes(n: usize, k: usize)
    requires crate::round_trip_spec::supported(n as int),
        k as int == unpadded_size(n as int),
    ensures
        0 <= k <= crate::round_trip_spec::encoded_size(n as int),
        0 <= padding_count(k as int) <= 2,
        k + padding_count(k as int) == crate::round_trip_spec::encoded_size(n as int),
        k + padding_count(k as int) <= usize::MAX,
{
    sizes(n);
}

// The canonical trailing positions are '='; no premise about internal encoder bytes.
pub proof fn canonical_padding(b: Seq<u8>, k: usize)
    requires crate::round_trip_spec::supported(b.len() as int),
        k as int == unpadded_size(b.len() as int),
    ensures forall|j: int| k <= j < crate::round_trip_spec::encoded_size(b.len() as int)
        ==> crate::round_trip_spec::encoded_byte(b, j) == 61u8,
{
    assert forall|j: int| k <= j < crate::round_trip_spec::encoded_size(b.len() as int)
        implies crate::round_trip_spec::encoded_byte(b, j) == 61u8 by {
    }
}

// Always callable at the public path; exports only a conditional consequence.
// This does not claim that the executed internal encoder establishes the antecedent.
pub proof fn padding_sizes_if(n: usize, k: usize)
    ensures (n <= isize::MAX
        && k as int == unpadded_size(n as int)) ==>
        (0 <= k <= crate::round_trip_spec::encoded_size(n as int)
        && 0 <= padding_count(k as int) <= 2
        && k + padding_count(k as int) == crate::round_trip_spec::encoded_size(n as int)
        && k + padding_count(k as int) <= usize::MAX),
{
    if n <= isize::MAX && k as int == unpadded_size(n as int) {
    }
}

// T52-RUST-BYTE-SLICE-VALIDITY (new explicit library/unsafe-validity TCB).
// Rust Reference wide-reference validity forbids pointed-to size > isize::MAX;
// slice::from_raw_parts likewise requires this bound. For u8, size_of is one. This
// interface applies only to actual valid &[u8], including valid unsafe-created
// slices, never arbitrary Seq<u8>, usize lengths, or hypothetical allocations.
// vstd currently exports only the weaker usize bound. Independent review owed.
#[verifier::external_body]
pub proof fn byte_slice_size_bound(bytes: &[u8])
    ensures bytes@.len() <= isize::MAX as int,
{}

pub proof fn byte_slice_encoding_fits(n: usize, padding: bool)
    requires n <= isize::MAX,
    ensures crate::round_trip_spec::encoding_size(n as int, padding) <= usize::MAX as int,
{}

// A physical padded destination can hold only supported source lengths.
pub proof fn padded_capacity_supported(n: usize, capacity: usize)
    requires capacity <= isize::MAX,
        capacity as int >= crate::round_trip_spec::encoded_size(n as int),
    ensures crate::round_trip_spec::supported(n as int),
{}

// Encoding table meaning is now a type-owned predicate; STANDARD obtains it
// from the accepted concrete T21 table witness (not an uninterpreted capability).
pub open spec fn is_standard<E: crate::engine::Engine + ?Sized>(engine: &E) -> bool {
    engine.encoding_padding() && engine.canonical_encoding_table() && engine.padding_symbol().byte() == 61u8
}
// P-GENERIC-INTERFACE: translation only. No relation to a stable pure View,
// no totality/no-panic claim, no interior-effect or Drop/frame guarantee.
#[verifier::external_trait_specification]
pub trait ExAsRef<T: core::marker::PointeeSized>: core::marker::PointeeSized {
    type ExternalTraitSpecificationFor: core::convert::AsRef<T>;
    fn as_ref(&self) -> &T;
}
// P-STD-ASREF: exact core implementations, assumed library interfaces only.
// The slice implementation returns self; the reference implementation delegates once.
#[verifier::external_fn_specification]
pub fn slice_as_ref<T>(s: &[T]) -> (r: &[T])
    ensures r@ == s@
{ <[T] as AsRef<[T]>>::as_ref(s) }
#[verifier::external_fn_specification]
pub fn reference_as_ref<'a, 'b,
    T: core::marker::PointeeSized + AsRef<U>, U: core::marker::PointeeSized>
    (s: &'b &'a T) -> (r: &'b U)
    ensures call_ensures(<T as AsRef<U>>::as_ref, (*s,), r)
{ <&'a T as AsRef<U>>::as_ref(s) }

// P-STD-ASSERT-FAILED: core panic path; require it unreachable rather than
// granting any successful effect. The runtime function and formatting remain intact.
#[verifier::external_type_specification]
pub struct ExAssertKind(core::panicking::AssertKind);
#[verifier::external_fn_specification]
pub fn ex_assert_failed<T: core::fmt::Debug + ?Sized, U: core::fmt::Debug + ?Sized>(
        kind: core::panicking::AssertKind, left: &T, right: &U,
        args: Option<core::fmt::Arguments<'_>>,
    ) -> !
    requires false
{ core::panicking::assert_failed(kind, left, right, args) }
pub proof fn masked_u64(x: u64)
    ensures
        (x >> 58u64) & 63u64 < 64,
        (x >> 52u64) & 63u64 < 64,
        (x >> 46u64) & 63u64 < 64,
        (x >> 40u64) & 63u64 < 64,
        (x >> 34u64) & 63u64 < 64,
        (x >> 28u64) & 63u64 < 64,
        (x >> 22u64) & 63u64 < 64,
        (x >> 16u64) & 63u64 < 64,
{ assert((x >> 58u64) & 63u64 < 64 && (x >> 52u64) & 63u64 < 64 && (x >> 46u64) & 63u64 < 64 && (x >> 40u64) & 63u64 < 64 && (x >> 34u64) & 63u64 < 64 && (x >> 28u64) & 63u64 < 64 && (x >> 22u64) & 63u64 < 64 && (x >> 16u64) & 63u64 < 64) by(bit_vector); }
pub proof fn byte_indices(a: u8, b: u8, c: u8)
    ensures
        a >> 2u8 < 64,
        ((a << 4u8 | b >> 4u8) & 63u8) < 64,
        ((b << 2u8 | c >> 6u8) & 63u8) < 64,
        c & 63u8 < 64,
        (a << 4u8) & 63u8 < 64,
        (b << 2u8) & 63u8 < 64,
{ assert(a >> 2u8 < 64 && ((a << 4u8 | b >> 4u8) & 63u8) < 64 && ((b << 2u8 | c >> 6u8) & 63u8) < 64 && c & 63u8 < 64 && (a << 4u8) & 63u8 < 64 && (b << 2u8) & 63u8 < 64) by(bit_vector); }
}
