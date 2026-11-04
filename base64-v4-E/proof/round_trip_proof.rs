//! Capacity lemmas, separate from caller-visible meaning.
use vstd::prelude::*;
use crate::round_trip_spec::*;
verus! {
pub proof fn capacity_domain(n: nat, s: nat)
    ensures
        (n <= 3 * (s / 4)) == (encoded_size(n) <= s),
        n <= encoded_size(n),
        decoded_estimate(encoded_size(n)) == 3 * groups(n),
        n <= decoded_estimate(encoded_size(n)) <= n + 2,
{
    assert(n == 3 * (n / 3) + n % 3) by (nonlinear_arith);
    assert(s == 4 * (s / 4) + s % 4) by (nonlinear_arith);
    assert(n % 3 < 3 && s % 4 < 4) by (nonlinear_arith);
    assert((4 * groups(n)) / 4 == groups(n)) by (nonlinear_arith);
    assert((4 * groups(n)) % 4 == 0) by (nonlinear_arith);
}
/// Contract-usability lemma only: no trusted executable consumer or target body.
pub proof fn encoded_prefix_establishes_decode(x: Seq<u8>, before: Seq<u8>, after: Seq<u8>, dst: Seq<u8>, k: nat)
    requires
        encode_requires(x, before), encode_post(x, x, before, after, k),
        represented(dst), dst.len() >= decoded_estimate(k),
    ensures
        decode_requires(x, after.subrange(0, k as int), dst),
        round_trip_decode_requires(after.subrange(0, k as int), dst),
{
    assert(after.subrange(0, k as int).len() == k);
    assert(k <= byte_slice_max());
    assert(decode_requires(x, after.subrange(0, k as int), dst));
}
/// Consume the public universal result using the actual earlier encode witness.
pub proof fn decoding_recovers_encode_input<E>(x: Seq<u8>, encoded: Seq<u8>, before: Seq<u8>, after: Seq<u8>, result: Result<usize, E>)
    requires
        decode_requires(x, encoded, before),
        round_trip_decode_contract(encoded, encoded, before, after, result),
    ensures
        decode_contract(x, encoded, encoded, before, after, result),
        result.is_ok(),
        result matches Result::Ok(k) && k as nat == x.len(),
        after.subrange(0, x.len() as int) == x,
        suffix_unchanged(before, after, x.len()),
{
    assert(decode_contract(x, encoded, encoded, before, after, result));
}


/// Concrete inverse alphabet used only to instantiate checked reconstruction lemmas.
pub open spec fn inverse_symbol(b: u8) -> u8 {
    if 65 <= b <= 90 { (b - 65) as u8 }
    else if 97 <= b <= 122 { (b - 71) as u8 }
    else if 48 <= b <= 57 { (b + 4) as u8 }
    else if b == 43 { 62 } else if b == 47 { 63 } else { 255 }
}
pub proof fn inverse_alphabet(v: nat)
    requires v < 64,
    ensures inverse_symbol(standard_symbol(v)) == v as u8,
{
    if v < 26 {} else if v < 52 {} else if v < 62 {} else if v == 62 {} else {}
}

pub proof fn terminal_injective(t: Seq<u8>, u: Seq<u8>, table: Seq<u8>)
    requires t.len() <= 3, u.len() <= 3, standard_encoding(t) == standard_encoding(u),
        table.len() == 256,
        forall|v: nat| v < 64 ==> table[standard_symbol(v) as int] == v as u8,
    ensures t == u,
{
    use crate::engine::general_purpose::decode_suffix::*;
    canonical_tail(t, table);
    canonical_tail(u, table);
    if t.len() == 0 || u.len() == 0 {
        assert(t.len() == 0 && u.len() == 0);
    } else {
        if t.len() < u.len() {
            let k = symbol_count(t) as int;
            assert(standard_encoding(t)[k] == 61);
            assert(standard_encoding(u)[k] != 61);
        } else if u.len() < t.len() {
            let k = symbol_count(u) as int;
            assert(standard_encoding(u)[k] == 61);
            assert(standard_encoding(t)[k] != 61);
        }
        assert(t.len() == u.len());
        assert forall|k: int| 0 <= k < 4 implies #[trigger] tail_value(t,k) == tail_value(u,k) by {
            if k < symbol_count(t) {
                assert(table[standard_encoding(t)[k] as int] == tail_value(t,k));
                assert(table[standard_encoding(u)[k] as int] == tail_value(u,k));
            }
        }
        pack_tail(t); pack_tail(u);
        assert(tail_word(t) == tail_word(u));
        assert forall|j: int| 0 <= j < t.len() implies t[j] == u[j] by {
            next_tail_byte(t, j as usize, tail_word(t) << (j * 8));
            next_tail_byte(u, j as usize, tail_word(u) << (j * 8));
        }
    }
    assert(t =~= u);
}

/// Canonical encoding determines every byte and the exact padded length.
/// Stronger than the supported domain: no capacity or decoder-acceptance premise.
pub proof fn standard_encoding_injective(x: Seq<u8>, y: Seq<u8>)
    requires standard_encoding(x) == standard_encoding(y),
    ensures x == y,
{
    use crate::engine::general_purpose::decode_suffix::*;
    use crate::engine::general_purpose::decode::canonical_quad;
    let table = Seq::new(256, |i: int| inverse_symbol(i as u8));
    assert forall|v: nat| v < 64 implies table[standard_symbol(v) as int] == v as u8 by {
        inverse_alphabet(v);
    }
    canonical_suffix_image(x); canonical_suffix_image(y);
    assert(terminal_input_index(x) == terminal_input_index(y));
    assert(terminal_output_index(x) == terminal_output_index(y));
    terminal_injective(terminal_tail(x), terminal_tail(y), table);
    assert(x.len() == y.len());
    assert forall|j: int| 0 <= j < x.len() implies x[j] == y[j] by {
        if j < terminal_output_index(x) {
            let q = j as nat / 3;
            assert(3 * q + 3 <= x.len());
            canonical_quad(x, table, q); canonical_quad(y, table, q);
            assert(0 <= j - 3 * q < 3);
            let k = j - 3 * q;
            let chunk = standard_encoding(x).subrange((4*q) as int, (4*q+4) as int);
            assert(crate::engine::general_purpose::decode::chunk_byte(chunk,table,k) == x[(3*q) as int+k]);
            assert(crate::engine::general_purpose::decode::chunk_byte(chunk,table,k) == y[(3*q) as int+k]);
            assert(x[j] == y[j]);
        } else {
            assert(terminal_tail(x)[j - terminal_output_index(x)] == x[j]);
            assert(terminal_tail(y)[j - terminal_output_index(y)] == y[j]);
        }
    }
    assert(x =~= y);
}

/// Lift the helper's chosen witness into the immutable universal public result.
pub proof fn chosen_witness_establishes_universal<E>(x: Seq<u8>, encoded: Seq<u8>, encoded_after: Seq<u8>, before: Seq<u8>, after: Seq<u8>, result: Result<usize,E>)
    requires decode_requires(x, encoded, before),
        decode_contract(x, encoded, encoded_after, before, after, result),
    ensures round_trip_decode_contract(encoded, encoded_after, before, after, result),
{
    assert forall|y: Seq<u8>| decode_requires(y, encoded, before) implies decode_contract(y, encoded, encoded_after, before, after, result) by {
        standard_encoding_injective(x,y);
    }
}

}
