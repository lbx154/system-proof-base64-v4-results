//! Unpadded-size and padding composition facts for the actual encode wrapper.
use vstd::prelude::*;
use crate::round_trip_spec::*;
verus! {
/// Exact scalar encoder output length before padding.
pub open spec fn raw_size(n: nat) -> nat {
    4 * (n / 3) + if n % 3 == 0 { 0nat } else { n % 3 + 1 }
}
/// Arithmetic connecting the unpadded codec to the existing padded vocabulary.
pub proof fn raw_capacity(n: nat)
    ensures raw_size(n) <= encoded_size(n),
        raw_size(n) + (4 - raw_size(n) % 4) % 4 == encoded_size(n),
{
    assert(n % 3 < 3) by (nonlinear_arith);
    assert((4 * (n / 3)) % 4 == 0) by (nonlinear_arith);
    assert(raw_size(n) % 4 == if n % 3 == 0 { 0nat } else { n % 3 + 1 }) by (nonlinear_arith);
}
/// Canonical trailing bytes after the raw output are precisely ASCII '='.
pub proof fn padding_is_canonical(x: Seq<u8>)
    ensures forall|j: int| raw_size(x.len()) <= j < encoded_size(x.len()) ==>
        standard_encoding(x)[j] == 61u8,
{
    raw_capacity(x.len());
    assert forall|j: int| raw_size(x.len()) <= j < encoded_size(x.len()) implies
        standard_encoding(x)[j] == 61u8 by {
        let n = x.len();
        assert(n % 3 < 3) by (nonlinear_arith);
        assert(n % 3 != 0);
        assert(4 * (n / 3) + n % 3 + 1 <= j < 4 * (n / 3) + 4);
        assert(j / 4 == n / 3) by (nonlinear_arith)
            requires 4 * (n / 3) + n % 3 + 1 <= j < 4 * (n / 3) + 4, n % 3 < 3;

        assert(j == 4 * (j / 4) + j % 4) by (nonlinear_arith);
        assert(j % 4 >= n % 3 + 1) by (nonlinear_arith)
            requires j == 4 * (j / 4) + j % 4, j / 4 == n / 3,
                4 * (n / 3) + n % 3 + 1 <= j;

        assert(n == 3 * (n / 3) + n % 3) by (nonlinear_arith);
        assert(3 * (j / 4) + 1 >= n || j % 4 == 3) by (nonlinear_arith)
            requires j / 4 == n / 3, n == 3 * (n / 3) + n % 3,
                n % 3 < 3, j % 4 >= n % 3 + 1, j % 4 < 4;

        assert(3 * (j / 4) + 2 >= n) by (nonlinear_arith)
            requires j / 4 == n / 3, n == 3 * (n / 3) + n % 3, n % 3 < 3;

    }
}
}
