//! Debug assertion failure is an unreachable panic boundary, never a trusted success path.
#![allow(unsafe_code)]
use vstd::prelude::*;
verus! {
#[verifier::external_type_specification]
pub struct ExAssertKind(core::panicking::AssertKind);
pub assume_specification<T, U> [core::panicking::assert_failed]
(kind: core::panicking::AssertKind, left: &T, right: &U, args: Option<core::fmt::Arguments<'_>>) -> !
where T: core::marker::MetaSized + core::fmt::Debug + ?Sized,
      U: core::marker::MetaSized + core::fmt::Debug + ?Sized,
    requires false,
;
}
