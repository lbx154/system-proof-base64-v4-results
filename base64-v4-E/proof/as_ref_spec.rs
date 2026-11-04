//! Per-invocation standard-library conversion interfaces.
#![allow(missing_docs)]
use vstd::prelude::*;
verus! {
#[verifier::external_trait_specification]
#[verifier::external_trait_extension(AsRefSpec via AsRefSpecImpl)]
pub trait ExAsRef<T: core::marker::PointeeSized>: core::marker::PointeeSized {
    type ExternalTraitSpecificationFor: core::convert::AsRef<T>;
    spec fn as_ref_post(&self, ret: &T) -> bool;
    // Option B caller-code trust only: actual conversion and complete cleanup
    // return normally and preserve the explicitly observed bytes/buffers/capacities.
    spec fn conversion_environment(&self) -> bool;
    spec fn cleanup_environment(&self) -> bool;
    fn as_ref(&self) -> (ret: &T)
        requires self.conversion_environment(),
        ensures self.as_ref_post(ret);
}
impl<T> AsRefSpecImpl<[T]> for [T] {
    open spec fn as_ref_post(&self, ret: &[T]) -> bool { ret@ == self@ }
    open spec fn conversion_environment(&self) -> bool { true }
    open spec fn cleanup_environment(&self) -> bool { true }
}
impl<T: core::marker::PointeeSized, U: core::marker::PointeeSized> AsRefSpecImpl<U> for &T
    where T: core::convert::AsRef<U>
{
    open spec fn as_ref_post(&self, ret: &U) -> bool { AsRefSpec::as_ref_post(*self, ret) }
    open spec fn conversion_environment(&self) -> bool { AsRefSpec::conversion_environment(*self) }
    open spec fn cleanup_environment(&self) -> bool { true }
}
pub open spec fn returned_bytes<T: AsRef<[u8]>>(input: &T, bytes: Seq<u8>) -> bool {
    exists|returned: &[u8]| returned@ == bytes && AsRefSpec::as_ref_post(input, returned)
}
}

verus! {
/// Rust-valid byte slices have byte extent at most isize::MAX (environment boundary).
#[verifier::external_body]
pub proof fn byte_slice_valid(input: &[u8])
    ensures crate::round_trip_spec::represented(input@),
{}
}

verus! {
/// Explicit executable cleanup; never ghost-erased. The external boundary covers
/// caller-owned destructor AND recursive field cleanup, not any codec operation.
#[verifier::external_body]
pub fn complete_owned_cleanup<T: AsRef<[u8]>>(input: T, observed: &mut [u8])
    requires AsRefSpec::cleanup_environment(&input),
    ensures final(observed)@ == old(observed)@,
{
    core::mem::drop(input);
}
}
