//! Provides the [`GeneralPurpose`] engine and associated config types.
//!
//! See preconfigured engines like [`STANDARD_NO_PAD`] or [`STANDARD_NO_PAD_INDIFFERENT`].
#[allow(unused_imports)]
use vstd::prelude::*;
use crate::alphabet::Symbol;
use crate::{
    alphabet,
    alphabet::Alphabet,
    engine::{Config, DecodeMetadata, DecodePaddingMode},
    DecodeSliceError,
};
use core::convert::TryInto;

pub(crate) mod decode;
pub(crate) mod decode_suffix;

pub use decode::GeneralPurposeEstimate;

vstd::prelude::verus! { pub(crate) exec const INVALID_VALUE: u8 
 ensures INVALID_VALUE == 255u8
 { 255 }
}

/// A general-purpose base64 engine.
///
/// - It uses no vector CPU instructions, so it will work on any system. For a version that uses
///   SIMD where available, see the SIMD engines behind the `simd-unsafe` feature.
/// - It is reasonably fast (~2-3GiB/s).
/// - It is not constant-time, though, so it is vulnerable to timing side-channel attacks. For loading cryptographic keys, etc, it is suggested to use the forthcoming constant-time implementation.

#[derive(Debug, Clone)]
#[verifier::verify]
pub struct GeneralPurpose {
    encode_table: [u8; 64],
    decode_table: [u8; 256],
    pub(crate) padding: Symbol,
    config: GeneralPurposeConfig,
}

/// A purely scalar base64 engine that never uses hardware-specific vector instructions.
///
/// This is an alias for [`GeneralPurpose`], giving an explicit name for callers who want to
/// guarantee a scalar-only implementation.
pub type Scalar = GeneralPurpose;

impl GeneralPurpose {
    /// The 6-bit-index-to-ASCII encode table.
    #[cfg(all(
        feature = "simd-unsafe",
        any(
            target_arch = "x86_64",
            all(target_arch = "aarch64", target_feature = "neon")
        )
    ))]
    pub(crate) fn encode_table(&self) -> &[u8; 64] {
        &self.encode_table
    }

    /// The ASCII-to-6-bit-value decode table.
    #[cfg(all(
        feature = "simd-unsafe",
        any(
            target_arch = "x86_64",
            all(target_arch = "aarch64", target_feature = "neon")
        )
    ))]
    pub(crate) fn decode_table(&self) -> &[u8; 256] {
        &self.decode_table
    }
}

verus! {
#[verifier::verify]
impl super::Engine for GeneralPurpose {
    closed spec fn is_standard(&self) -> bool {
        self.config.encode_padding && !self.config.decode_allow_trailing_bits
        && self.config.decode_padding_mode == DecodePaddingMode::RequireCanonical
        && self.padding.byte() == 61u8
        && self.encode_table@ == Seq::new(64, |i: int| crate::round_trip_spec::standard_symbol(i as nat))
        && forall|i: int| 0 <= i < 64 ==> self.decode_table@[self.encode_table@[i] as int] == i as u8
    }
    type Config = GeneralPurposeConfig;
    type DecodeEstimate = GeneralPurposeEstimate;

    fn internal_encode(&self, input: &[u8], output: &mut [u8]) -> usize {
        // Verification-only expansion of encode_helper for this exact scalar callback.
        let simd_prefix = |_: &[u8], out: &mut [u8]| -> (result: (usize, usize))
            ensures result == (0usize, 0usize), final(out)@ == old(out)@,
        { (0usize, 0usize) };
        let (input_index, output_index) = simd_prefix(input, output);
        debug_assert!(input_index % 3 == 0, "prefix must consume whole 3-byte groups");
        debug_assert!(output_index == input_index / 3 * 4, "prefix output must match consumed input");
        debug_assert!(input_index <= input.len());
        debug_assert!(output_index <= output.len());
        proof { crate::encode_bridge::raw_capacity(input@.len()); }
        encode_scalar_tail(&self.encode_table, input, output, input_index, output_index)
    }

    fn internal_decoded_len_estimate(&self, input_len: usize) -> Self::DecodeEstimate {
        GeneralPurposeEstimate::new(input_len)
    }

    #[verus_spec]
    fn internal_decode(
        &self,
        input: &[u8],
        output: &mut [u8],
        estimate: Self::DecodeEstimate,
    ) -> Result<DecodeMetadata, DecodeSliceError> {
        let ghost before = output@;
        proof { estimate.compatible_facts(input@.len()); }
        let ghost x = if crate::round_trip_spec::round_trip_decode_requires(input@, before) {
            choose|x: Seq<u8>| crate::round_trip_spec::decode_requires(x,input@,before)
        } else { Seq::<u8>::empty() };
        let scalar_prefix = |_: &[u8], _: usize, out: &mut [u8]| -> (r: (usize,usize))
            ensures r == (0usize,0usize), final(out)@ == old(out)@,
        { (0usize,0usize) };
        proof {
            if self.is_standard() && crate::round_trip_spec::round_trip_decode_requires(input@, before) {
                assert(crate::round_trip_spec::decode_requires(x,input@,before));
                assert forall|v: nat| v < 64 implies self.decode_table@[crate::round_trip_spec::standard_symbol(v) as int] == v as u8 by {
                    assert(self.encode_table@[v as int] == crate::round_trip_spec::standard_symbol(v));
                }
                assert(decode::canonical_helper_use(x,input@,before,self.decode_table@,self.padding.byte(),self.config.decode_padding_mode));
            }
        }
        proof {
            assert(estimate.remainder() == input@.len() % 4);
            assert forall|i: &[u8], n: usize, o: &mut [u8]| #[trigger] call_requires(scalar_prefix,(i,n,o)) by {};
            assert forall|i: &[u8], n: usize, o: &mut [u8], r: (usize,usize)| #[trigger] call_ensures(scalar_prefix,(i,n,o),r)
                implies r == (0usize,0usize) && final(o)@ == o@ by {};
        }
        let result = { proof_with! { Ghost(x) }
        decode::decode_helper(
            input,
            &estimate,
            output,
            &self.decode_table,
            self.config.decode_allow_trailing_bits,
            self.padding,
            self.config.decode_padding_mode,
            scalar_prefix,
        ) };
        proof {
            if self.is_standard() && crate::round_trip_spec::round_trip_decode_requires(input@, before) {
                match &result {
                    Ok(dm) => {
                        crate::round_trip_proof::chosen_witness_establishes_universal(
                            x,input@,input@,before,output@,Ok::<usize,DecodeSliceError>(dm.written()));
                    },
                    Err(_) => {},
                }
            }
        }
        result
    }

    fn config(&self) -> (result: &Self::Config)
        ensures self.is_standard() ==> result.padding_enabled(),
    {
        &self.config
    }

    fn padding(&self) -> Symbol {
        self.padding
    }
}

}

/// Scalar base64 encode of `input` into `output`, returning the number of bytes written.
///
/// `simd_prefix` gets first crack at the input, returning `(input_consumed, output_written)`. It
/// must consume whole 3-byte groups: `input_consumed % 3 == 0`, `output_written == input_consumed /
/// 3 * 4`, both in bounds, and only those `output_written` bytes are written. `(0, 0)` (pure scalar)
/// is always valid.
verus! {
#[inline]
#[verifier::external_body]
pub(crate) fn encode_helper(
    encode_table: &[u8; 64],
    input: &[u8],
    output: &mut [u8],
    simd_prefix: impl FnOnce(&[u8], &mut [u8]) -> (usize, usize),
) -> usize {
    let (input_index, output_index) = simd_prefix(input, output);

    debug_assert!(
        input_index % 3 == 0,
        "prefix must consume whole 3-byte groups"
    );
    debug_assert!(
        output_index == input_index / 3 * 4,
        "prefix output must match consumed input"
    );
    debug_assert!(input_index <= input.len());
    debug_assert!(output_index <= output.len());

    encode_scalar_tail(encode_table, input, output, input_index, output_index)
}

/// Scalar encode of `input[input_index..]` into `output[output_index..]`, resuming from a 3-byte
/// group boundary. Returns the total number of output bytes written.
// Solver facts are reintroduced inside each loop: pre-loop facts are not loop invariants.
proof fn encode_index_bounds()
    ensures
        forall|v: u64| #[trigger] (v & 0x3fu64) < 64u64,
        forall|v: u8| #[trigger] (v & 0x3fu8) < 64u8,
        forall|v: u8| #[trigger] (v >> 2u8) < 64u8,
{
    assert forall|v: u64| #[trigger] (v & 0x3fu64) < 64u64 by {
        assert((v & 0x3fu64) < 64u64) by (bit_vector);
    }
    assert forall|v: u8| #[trigger] (v & 0x3fu8) < 64u8 by {
        assert((v & 0x3fu8) < 64u8) by (bit_vector);
    }
    assert forall|v: u8| #[trigger] (v >> 2u8) < 64u8 by {
        assert((v >> 2u8) < 64u8) by (bit_vector);
    }
}

proof fn encode_three_arithmetic(a: u8, b: u8, c: u8)
    ensures
        (a >> 2u8) as nat == (a as nat) / 4,
        ((a << 4u8 | b >> 4u8) & 63u8) as nat == (a as nat % 4) * 16 + (b as nat) / 16,
        ((b << 2u8 | c >> 6u8) & 63u8) as nat == (b as nat % 16) * 4 + (c as nat) / 64,
        (c & 63u8) as nat == (c as nat) % 64,
{
    assert((a >> 2u8) == a / 4u8) by (bit_vector);
    assert(((a << 4u8 | b >> 4u8) & 63u8) == ((a % 4u8) * 16u8 + b / 16u8) as u8) by (bit_vector);
    assert(((b << 2u8 | c >> 6u8) & 63u8) == ((b % 16u8) * 4u8 + c / 64u8) as u8) by (bit_vector);
    assert((c & 63u8) == c % 64u8) by (bit_vector);
}

proof fn encode_three_at(x: Seq<u8>, i: nat, o: nat)
    requires i % 3 == 0, o == i / 3 * 4, i + 3 <= x.len(),
    ensures
        crate::round_trip_spec::encoded_byte(x, o) == crate::round_trip_spec::standard_symbol((x[i as int] >> 2u8) as nat),
        crate::round_trip_spec::encoded_byte(x, o+1) == crate::round_trip_spec::standard_symbol(((x[i as int] << 4u8 | x[(i+1) as int] >> 4u8) & 63u8) as nat),
        crate::round_trip_spec::encoded_byte(x, o+2) == crate::round_trip_spec::standard_symbol(((x[(i+1) as int] << 2u8 | x[(i+2) as int] >> 6u8) & 63u8) as nat),
        crate::round_trip_spec::encoded_byte(x, o+3) == crate::round_trip_spec::standard_symbol((x[(i+2) as int] & 63u8) as nat),
{
    encode_three_arithmetic(x[i as int], x[(i+1) as int], x[(i+2) as int]);
    assert(i == 3 * (i / 3)) by (nonlinear_arith) requires i % 3 == 0, o == i / 3 * 4;
    assert(o % 4 == 0 && (o+1) % 4 == 1 && (o+2) % 4 == 2 && (o+3) % 4 == 3) by (nonlinear_arith) requires i % 3 == 0, o == i / 3 * 4;
    assert(3*(o/4) == i && 3*((o+1)/4) == i && 3*((o+2)/4) == i && 3*((o+3)/4) == i) by (nonlinear_arith) requires i % 3 == 0, o == i / 3 * 4;
}

proof fn encode_word_bits(w: u64, a: u8, b: u8, c: u8, d: u8, e: u8, f: u8, g: u8, h: u8)
    requires w == ((a as u64) << 56u64) | ((b as u64) << 48u64) | ((c as u64) << 40u64) | ((d as u64) << 32u64) | ((e as u64) << 24u64) | ((f as u64) << 16u64) | ((g as u64) << 8u64) | (h as u64),
    ensures
        ((w >> 58u64) & 63u64) < 64u64,
        ((w >> 52u64) & 63u64) < 64u64,
        ((w >> 46u64) & 63u64) < 64u64,
        ((w >> 40u64) & 63u64) < 64u64,
        ((w >> 34u64) & 63u64) < 64u64,
        ((w >> 28u64) & 63u64) < 64u64,
        ((w >> 22u64) & 63u64) < 64u64,
        ((w >> 16u64) & 63u64) < 64u64,
        ((w >> 58u64) & 63u64) == (a >> 2u8) as u64,
        ((w >> 52u64) & 63u64) == ((a << 4u8 | b >> 4u8) & 63u8) as u64,
        ((w >> 46u64) & 63u64) == ((b << 2u8 | c >> 6u8) & 63u8) as u64,
        ((w >> 40u64) & 63u64) == (c & 63u8) as u64,
        ((w >> 34u64) & 63u64) == (d >> 2u8) as u64,
        ((w >> 28u64) & 63u64) == ((d << 4u8 | e >> 4u8) & 63u8) as u64,
        ((w >> 22u64) & 63u64) == ((e << 2u8 | f >> 6u8) & 63u8) as u64,
        ((w >> 16u64) & 63u64) == (f & 63u8) as u64,
{
    assert(((w >> 58u64) & 63u64) < 64u64) by (bit_vector);
    assert(((w >> 52u64) & 63u64) < 64u64) by (bit_vector);
    assert(((w >> 46u64) & 63u64) < 64u64) by (bit_vector);
    assert(((w >> 40u64) & 63u64) < 64u64) by (bit_vector);
    assert(((w >> 34u64) & 63u64) < 64u64) by (bit_vector);
    assert(((w >> 28u64) & 63u64) < 64u64) by (bit_vector);
    assert(((w >> 22u64) & 63u64) < 64u64) by (bit_vector);
    assert(((w >> 16u64) & 63u64) < 64u64) by (bit_vector);
    assert(((w >> 58u64) & 63u64) == (a >> 2u8) as u64) by (bit_vector) requires w == ((a as u64) << 56u64) | ((b as u64) << 48u64) | ((c as u64) << 40u64) | ((d as u64) << 32u64) | ((e as u64) << 24u64) | ((f as u64) << 16u64) | ((g as u64) << 8u64) | (h as u64);
    assert(((w >> 52u64) & 63u64) == ((a << 4u8 | b >> 4u8) & 63u8) as u64) by (bit_vector) requires w == ((a as u64) << 56u64) | ((b as u64) << 48u64) | ((c as u64) << 40u64) | ((d as u64) << 32u64) | ((e as u64) << 24u64) | ((f as u64) << 16u64) | ((g as u64) << 8u64) | (h as u64);
    assert(((w >> 46u64) & 63u64) == ((b << 2u8 | c >> 6u8) & 63u8) as u64) by (bit_vector) requires w == ((a as u64) << 56u64) | ((b as u64) << 48u64) | ((c as u64) << 40u64) | ((d as u64) << 32u64) | ((e as u64) << 24u64) | ((f as u64) << 16u64) | ((g as u64) << 8u64) | (h as u64);
    assert(((w >> 40u64) & 63u64) == (c & 63u8) as u64) by (bit_vector) requires w == ((a as u64) << 56u64) | ((b as u64) << 48u64) | ((c as u64) << 40u64) | ((d as u64) << 32u64) | ((e as u64) << 24u64) | ((f as u64) << 16u64) | ((g as u64) << 8u64) | (h as u64);
    assert(((w >> 34u64) & 63u64) == (d >> 2u8) as u64) by (bit_vector) requires w == ((a as u64) << 56u64) | ((b as u64) << 48u64) | ((c as u64) << 40u64) | ((d as u64) << 32u64) | ((e as u64) << 24u64) | ((f as u64) << 16u64) | ((g as u64) << 8u64) | (h as u64);
    assert(((w >> 28u64) & 63u64) == ((d << 4u8 | e >> 4u8) & 63u8) as u64) by (bit_vector) requires w == ((a as u64) << 56u64) | ((b as u64) << 48u64) | ((c as u64) << 40u64) | ((d as u64) << 32u64) | ((e as u64) << 24u64) | ((f as u64) << 16u64) | ((g as u64) << 8u64) | (h as u64);
    assert(((w >> 22u64) & 63u64) == ((e << 2u8 | f >> 6u8) & 63u8) as u64) by (bit_vector) requires w == ((a as u64) << 56u64) | ((b as u64) << 48u64) | ((c as u64) << 40u64) | ((d as u64) << 32u64) | ((e as u64) << 24u64) | ((f as u64) << 16u64) | ((g as u64) << 8u64) | (h as u64);
    assert(((w >> 16u64) & 63u64) == (f & 63u8) as u64) by (bit_vector) requires w == ((a as u64) << 56u64) | ((b as u64) << 48u64) | ((c as u64) << 40u64) | ((d as u64) << 32u64) | ((e as u64) << 24u64) | ((f as u64) << 16u64) | ((g as u64) << 8u64) | (h as u64);
}

proof fn encode_partial_at(x: Seq<u8>)
    requires x.len() % 3 != 0,
    ensures
        crate::round_trip_spec::encoded_byte(x, 4 * (x.len()/3)) ==
            crate::round_trip_spec::standard_symbol((x[(x.len() - x.len()%3) as int] >> 2u8) as nat),
        x.len() % 3 == 1 ==> crate::round_trip_spec::encoded_byte(x, 4 * (x.len()/3)+1) ==
            crate::round_trip_spec::standard_symbol(((x[(x.len() - x.len()%3) as int] << 4u8) & 63u8) as nat),
        x.len() % 3 == 2 ==> crate::round_trip_spec::encoded_byte(x, 4 * (x.len()/3)+1) ==
            crate::round_trip_spec::standard_symbol(((x[(x.len() - x.len()%3) as int] << 4u8 | x[(x.len() - 1) as int] >> 4u8) & 63u8) as nat),
        x.len() % 3 == 2 ==> crate::round_trip_spec::encoded_byte(x, 4 * (x.len()/3)+2) ==
            crate::round_trip_spec::standard_symbol(((x[(x.len() - 1) as int] << 2u8) & 63u8) as nat),
{
    let n = x.len();
    let i = (n - n%3) as nat;
    let o = 4*(n/3);
    assert(n%3 == 1 || n%3 == 2) by (nonlinear_arith) requires n%3 != 0;
    assert(i == 3*(n/3)) by (nonlinear_arith) requires i == n - n%3;
    assert(o%4 == 0 && (o+1)%4 == 1 && (o+2)%4 == 2) by (nonlinear_arith) requires o == 4*(n/3);
    assert(3*(o/4) == i && 3*((o+1)/4) == i && 3*((o+2)/4) == i) by (nonlinear_arith) requires o == 4*(n/3), i == 3*(n/3);
    if n%3 == 1 {
        let a = x[i as int];
        encode_three_arithmetic(a, 0u8, 0u8);
        assert(((a << 4u8 | 0u8 >> 4u8) & 63u8) == ((a << 4u8) & 63u8)) by (bit_vector);
    } else {
        let b = x[(n-1) as int];
        encode_three_arithmetic(x[i as int], b, 0u8);
        assert(((b << 2u8 | 0u8 >> 6u8) & 63u8) == ((b << 2u8) & 63u8)) by (bit_vector);
    }
}

// Verification-only extraction of each original read_u64 + eight-write group.
fn encode_eight(encode_table: &[u8; 64], input: &[u8], output: &mut [u8])
    requires input.len() >= 8, output.len() >= 8,
        encode_table@ == Seq::new(64, |i: int| crate::round_trip_spec::standard_symbol(i as nat)),
    ensures
        forall|j: int| 0 <= j < 8 ==> final(output)@[j] == crate::round_trip_spec::encoded_byte(input@, j as nat),
        crate::round_trip_spec::suffix_unchanged(old(output)@, final(output)@, 8),
{
    const LOW_SIX_BITS: u64 = 0x3F;
    let input_u64 = read_u64(&input[0..]);
    proof { encode_word_bits(input_u64, input[0], input[1], input[2], input[3], input[4], input[5], input[6], input[7]);
        encode_three_at(input@, 0nat, 0nat);
        encode_three_at(input@, 3nat, 4nat);
    }
    
    output[0] = encode_table[((input_u64 >> 58) & LOW_SIX_BITS) as usize];
    proof { assert(output@[0] == crate::round_trip_spec::encoded_byte(input@, 0nat)); }
    output[1] = encode_table[((input_u64 >> 52) & LOW_SIX_BITS) as usize];
    proof { assert(output@[1] == crate::round_trip_spec::encoded_byte(input@, 1nat)); }
    output[2] = encode_table[((input_u64 >> 46) & LOW_SIX_BITS) as usize];
    proof { assert(output@[2] == crate::round_trip_spec::encoded_byte(input@, 2nat)); }
    output[3] = encode_table[((input_u64 >> 40) & LOW_SIX_BITS) as usize];
    proof { assert(output@[3] == crate::round_trip_spec::encoded_byte(input@, 3nat)); }
    output[4] = encode_table[((input_u64 >> 34) & LOW_SIX_BITS) as usize];
    proof { assert(output@[4] == crate::round_trip_spec::encoded_byte(input@, 4nat)); }
    output[5] = encode_table[((input_u64 >> 28) & LOW_SIX_BITS) as usize];
    proof { assert(output@[5] == crate::round_trip_spec::encoded_byte(input@, 5nat)); }
    output[6] = encode_table[((input_u64 >> 22) & LOW_SIX_BITS) as usize];
    proof { assert(output@[6] == crate::round_trip_spec::encoded_byte(input@, 6nat)); }
    output[7] = encode_table[((input_u64 >> 16) & LOW_SIX_BITS) as usize];
    proof { assert(output@[7] == crate::round_trip_spec::encoded_byte(input@, 7nat)); }
    
    
    proof {
        assert forall|j: int| 0 <= j < 8 implies
        output@[j] == crate::round_trip_spec::encoded_byte(input@, j as nat) by { }
    }
}

proof fn encode_window(x: Seq<u8>, i: nat, end: nat, j: nat)
    requires i % 3 == 0, i <= end <= x.len(), j < 4*((end-i)/3),
    ensures crate::round_trip_spec::encoded_byte(x.subrange(i as int, end as int), j) ==
        crate::round_trip_spec::encoded_byte(x, 4*(i/3)+j),
{
    let o = 4*(i/3);
    assert(i == 3*(i/3)) by (nonlinear_arith) requires i%3 == 0;
    assert((o+j)%4 == j%4) by (nonlinear_arith) requires o == 4*(i/3);
    assert(3*((o+j)/4) == i+3*(j/4)) by (nonlinear_arith) requires o == 4*(i/3), i == 3*(i/3);
    assert(3*(j/4)+2 < end-i) by (nonlinear_arith) requires j < 4*((end-i)/3), i <= end;
}

// Verification-only extraction of the original four unrolled groups; calls retain their order.
fn encode_thirtytwo(encode_table: &[u8; 64], input: &[u8], output: &mut [u8])
    requires input.len() == 26, output.len() == 32,
        encode_table@ == Seq::new(64, |i: int| crate::round_trip_spec::standard_symbol(i as nat)),
    ensures
        forall|j: int| 0 <= j < 32 ==> final(output)@[j] == crate::round_trip_spec::encoded_byte(input@, j as nat),
        final(output)@.len() == old(output)@.len(),
{
    {
        let output_block = &mut output[0..8];
        encode_eight(encode_table, &input[0..], output_block);
        proof {
            assert forall|j: int| 0 <= j < 8 implies
            output_block@[j] == crate::round_trip_spec::encoded_byte(input@, (0 + j) as nat) by {
                encode_window(input@, 0nat, 26nat, j as nat);
            }
        }
    }
    {
        let output_block = &mut output[8..16];
        encode_eight(encode_table, &input[6..], output_block);
        proof {
            assert forall|j: int| 0 <= j < 8 implies
            output_block@[j] == crate::round_trip_spec::encoded_byte(input@, (8 + j) as nat) by {
                encode_window(input@, 6nat, 26nat, j as nat);
            }
        }
    }
    {
        let output_block = &mut output[16..24];
        encode_eight(encode_table, &input[12..], output_block);
        proof {
            assert forall|j: int| 0 <= j < 8 implies
            output_block@[j] == crate::round_trip_spec::encoded_byte(input@, (16 + j) as nat) by {
                encode_window(input@, 12nat, 26nat, j as nat);
            }
        }
    }
    {
        let output_block = &mut output[24..32];
        encode_eight(encode_table, &input[18..], output_block);
        proof {
            assert forall|j: int| 0 <= j < 8 implies
            output_block@[j] == crate::round_trip_spec::encoded_byte(input@, (24 + j) as nat) by {
                encode_window(input@, 18nat, 26nat, j as nat);
            }
        }
    }
    proof {
        assert forall|j: int| 0 <= j < 32 implies
        output@[j] == crate::round_trip_spec::encoded_byte(input@, (j) as nat) by {
            if j < 8 { } else if j < 16 { } else if j < 24 { } else { }
        }
    }
}


fn encode_scalar_tail(
    encode_table: &[u8; 64],
    input: &[u8],
    output: &mut [u8],
    mut input_index: usize,
    mut output_index: usize,
) -> (result: usize)
    requires
        encode_table@ == Seq::new(64, |i: int| crate::round_trip_spec::standard_symbol(i as nat)),
        forall|j: int| 0 <= j < output_index ==> old(output)@[j] == crate::round_trip_spec::encoded_byte(input@, j as nat),
        crate::round_trip_spec::supported(input@.len()),
        output.len() >= crate::encode_bridge::raw_size(input@.len()),
        input_index <= input.len(), input_index % 3 == 0,
        output_index == input_index / 3 * 4,
    ensures
        forall|j: int| 0 <= j < result ==> final(output)@[j] == crate::round_trip_spec::encoded_byte(input@, j as nat),
        result == crate::encode_bridge::raw_size(input@.len()),
        crate::round_trip_spec::suffix_unchanged(old(output)@, final(output)@, result as nat),
{ 
    let ghost before = output@;
    const BLOCKS_PER_FAST_LOOP: usize = 4;
    const LOW_SIX_BITS: u64 = 0x3F;

    // we read 8 bytes at a time (u64) but only actually consume 6 of those bytes. Thus, we need
    // 2 trailing bytes to be available to read..
    let last_fast_index = input.len().saturating_sub(BLOCKS_PER_FAST_LOOP * 6 + 2);

    if last_fast_index > 0 {
        while input_index <= last_fast_index
            invariant
                crate::round_trip_spec::supported(input@.len()),
                output.len() >= crate::encode_bridge::raw_size(input@.len()),
                input_index <= input.len(), input_index % 3 == 0,
                output_index == input_index / 3 * 4,
                last_fast_index == input.len() - 26,
                encode_table@ == Seq::new(64, |i: int| crate::round_trip_spec::standard_symbol(i as nat)),
                forall|j: int| 0 <= j < output_index ==> output@[j] == crate::round_trip_spec::encoded_byte(input@, j as nat),
                output@.len() == before.len(),
                forall|j: int| output_index <= j < output@.len() ==> output@[j] == before[j],
            decreases input.len() - input_index,
        {
            // Major performance wins from letting the optimizer do the bounds check once, mostly
            // on the output side
            let input_chunk = &input[input_index..(input_index + (BLOCKS_PER_FAST_LOOP * 6 + 2))];
            let output_chunk = &mut output[output_index..(output_index + BLOCKS_PER_FAST_LOOP * 8)];

            // Hand-unrolling for 32 vs 16 or 8 bytes produces yields performance about equivalent
            // to unsafe pointer code on a Xeon E5-1650v3. 64 byte unrolling was slightly better for
            // large inputs but significantly worse for 50-byte input, unsurprisingly. I suspect
            // that it's a not uncommon use case to encode smallish chunks of data (e.g. a 64-byte
            // SHA-512 digest), so it would be nice if that fit in the unrolled loop at least once.
            // Plus, single-digit percentage performance differences might well be quite different
            // on different hardware.

            encode_thirtytwo(encode_table, input_chunk, output_chunk);
            proof {
                assert forall|j: int| 0 <= j < 32 implies
                    output_chunk@[j] == crate::round_trip_spec::encoded_byte(input@, (output_index + j) as nat) by {
                    encode_window(input@, input_index as nat, (input_index + 26) as nat, j as nat);
                }
            }
            output_index += BLOCKS_PER_FAST_LOOP * 8;
            input_index += BLOCKS_PER_FAST_LOOP * 6;
        }
    }

    // Encode what's left after the fast loop.

    const LOW_SIX_BITS_U8: u8 = 0x3F;

    let rem = input.len() % 3;
    let start_of_rem = input.len() - rem;

    // start at the first index not handled by fast loop, which may be 0.

    while input_index < start_of_rem
        invariant
            crate::round_trip_spec::supported(input@.len()),
            output.len() >= crate::encode_bridge::raw_size(input@.len()),
            input_index <= start_of_rem, input_index % 3 == 0,
            output_index == input_index / 3 * 4,
            rem == input.len() % 3,
            start_of_rem == input.len() - rem, start_of_rem % 3 == 0,
            encode_table@ == Seq::new(64, |i: int| crate::round_trip_spec::standard_symbol(i as nat)),
            forall|j: int| 0 <= j < output_index ==> output@[j] == crate::round_trip_spec::encoded_byte(input@, j as nat),
            output@.len() == before.len(),
            forall|j: int| output_index <= j < output@.len() ==> output@[j] == before[j],
        decreases start_of_rem - input_index,
    {
        proof { encode_index_bounds(); }
        let input_chunk = &input[input_index..(input_index + 3)];
        let output_chunk = &mut output[output_index..(output_index + 4)];

        proof { encode_three_at(input@, input_index as nat, output_index as nat); }
        output_chunk[0] = encode_table[(input_chunk[0] >> 2) as usize];
        output_chunk[1] =
            encode_table[((input_chunk[0] << 4 | input_chunk[1] >> 4) & LOW_SIX_BITS_U8) as usize];
        output_chunk[2] =
            encode_table[((input_chunk[1] << 2 | input_chunk[2] >> 6) & LOW_SIX_BITS_U8) as usize];
        output_chunk[3] = encode_table[(input_chunk[2] & LOW_SIX_BITS_U8) as usize];

        proof {
            assert forall|j: int| 0 <= j < 4 implies
                output_chunk@[j] == crate::round_trip_spec::encoded_byte(input@, (output_index + j) as nat) by { }
        }
        input_index += 3;
        output_index += 4;
    }

    proof { encode_index_bounds(); }
    proof { if rem != 0 { encode_partial_at(input@); } }
    if rem == 2 {
        output[output_index] = encode_table[(input[start_of_rem] >> 2) as usize];
        output[output_index + 1] = encode_table[((input[start_of_rem] << 4
            | input[start_of_rem + 1] >> 4)
            & LOW_SIX_BITS_U8) as usize];
        output[output_index + 2] =
            encode_table[((input[start_of_rem + 1] << 2) & LOW_SIX_BITS_U8) as usize];
        output_index += 3;
    } else if rem == 1 {
        output[output_index] = encode_table[(input[start_of_rem] >> 2) as usize];
        output[output_index + 1] =
            encode_table[((input[start_of_rem] << 4) & LOW_SIX_BITS_U8) as usize];
        output_index += 2;
    }

    output_index
}

}

/// Returns a table mapping a 6-bit index to the ASCII byte encoding of the index
vstd::prelude::verus! {
pub(crate) const fn encode_table(alphabet: &Alphabet) -> (result: [u8; 64])
 ensures result@ == alphabet.symbols@,
    {
    // the encode table is just the alphabet:
    // 6-bit index lookup -> printable byte
    let mut encode_table = [0_u8; 64];
    {
        let mut index = 0;
        while index < 64
            invariant index <= 64,
                forall|j: int| 0 <= j < index ==> encode_table@[j] == alphabet.symbols@[j],
            decreases 64 - index,
        {
            encode_table[index] = alphabet.symbols[index];
            index += 1;
        }
    }

    encode_table
}
}

/// Returns a table mapping base64 bytes as the lookup index to either:
/// - [`INVALID_VALUE`] for bytes that aren't members of the alphabet
/// - a byte whose lower 6 bits are the value that was encoded into the index byte
vstd::prelude::verus! {
pub(crate) const fn decode_table(alphabet: &Alphabet) -> (result: [u8; 256])

    ensures distinct(alphabet.bytes()) ==> forall|i: int| 0 <= i < 64 ==> result@[alphabet.symbols@[i] as int] == i as u8,

    {
    proof { alphabet.representation(); }
    let mut decode_table = [INVALID_VALUE; 256];

    // Since the table is full of `INVALID_VALUE` already, we only need to overwrite
    // the parts that are valid.
    let mut index = 0_usize;
    while index < 64
        invariant index <= 64, alphabet.bytes() == alphabet.symbols@,
            distinct(alphabet.bytes()) ==> forall|j: int| 0 <= j < index ==> decode_table@[alphabet.symbols@[j] as int] == j as u8,
        decreases 64 - index,
    {
        // The index in the alphabet is the 6-bit value we care about.
        // Since the index is in 0-63, it is safe to cast to u8.
        let ghost previous = decode_table@;
        decode_table[alphabet.symbols[index] as usize] = index as u8;
        proof {
            assert forall|j: int| 0 <= j < index + 1 && distinct(alphabet.bytes()) implies
                decode_table@[alphabet.symbols@[j] as int] == j as u8 by {
                if j != index {
                    assert(alphabet.symbols@[j] != alphabet.symbols@[index as int]);
                    assert(previous[alphabet.symbols@[j] as int] == j as u8);
                }
            }
        }
        index += 1;
    }

    decode_table
}
}

verus! {
#[inline]
pub(crate) open spec fn be_word(s: Seq<u8>) -> u64 {
    ((s[0] as u64) << 56) | ((s[1] as u64) << 48)
    | ((s[2] as u64) << 40) | ((s[3] as u64) << 32)
    | ((s[4] as u64) << 24) | ((s[5] as u64) << 16)
    | ((s[6] as u64) << 8) | (s[7] as u64)
}
fn read_u64(s: &[u8]) -> (result: u64)
    requires s.len() >= 8,
    ensures result == be_word(s@),
{
    // Verification-only expansion of from_be_bytes on the first eight bytes.
    ((s[0] as u64) << 56) | ((s[1] as u64) << 48)
    | ((s[2] as u64) << 40) | ((s[3] as u64) << 32)
    | ((s[4] as u64) << 24) | ((s[5] as u64) << 16)
    | ((s[6] as u64) << 8) | (s[7] as u64)
}
}

/// Contains configuration parameters for base64 encoding and decoding.
///
/// ```
/// # use base64::engine::GeneralPurposeConfig;
/// let config = GeneralPurposeConfig::new()
///     .with_encode_padding(false);
///     // further customize using `.with_*` methods as needed
/// ```
///
/// The constants [PAD] and [`NO_PAD`] cover most use cases.
///
/// To specify the characters used, see [Alphabet].
#[derive(Clone, Copy, Debug)]
#[verifier::verify]
pub struct GeneralPurposeConfig {
    encode_padding: bool,
    decode_allow_trailing_bits: bool,
    decode_padding_mode: DecodePaddingMode,
}

impl GeneralPurposeConfig {
    /// Create a new config with `padding` = `true`, `decode_allow_trailing_bits` = `false`, and
    /// `decode_padding_mode = DecodePaddingMode::RequireCanonicalPadding`.
    ///
    /// This probably matches most people's expectations, but consider disabling padding to save
    /// a few bytes unless you specifically need it for compatibility with some legacy system.
    #[must_use]


    /// Create a new config based on `self` with an updated `padding` setting.
    ///
    /// If `padding` is `true`, encoding will append either 1 or 2 `=` padding characters as needed
    /// to produce an output whose length is a multiple of 4.
    ///
    /// Padding is not needed for correct decoding and only serves to waste bytes, but it's in the
    /// [spec](https://datatracker.ietf.org/doc/html/rfc4648#section-3.2).
    ///
    /// For new applications, consider not using padding if the decoders you're using don't require
    /// padding to be present.
    #[must_use]
    pub const fn with_encode_padding(self, padding: bool) -> Self {
        Self {
            encode_padding: padding,
            ..self
        }
    }

    /// Create a new config based on `self` with an updated `decode_allow_trailing_bits` setting.
    ///
    /// Most users will not need to configure this. It's useful if you need to decode base64
    /// produced by a buggy encoder that has bits set in the unused space on the last base64
    /// character as per [forgiving-base64 decode](https://infra.spec.whatwg.org/#forgiving-base64-decode).
    /// If invalid trailing bits are present and this is `true`, those bits will
    /// be silently ignored, else `DecodeError::InvalidLastSymbol` will be emitted.
    #[must_use]
    pub const fn with_decode_allow_trailing_bits(self, allow: bool) -> Self {
        Self {
            decode_allow_trailing_bits: allow,
            ..self
        }
    }

    /// Create a new config based on `self` with an updated `decode_padding_mode` setting.
    ///
    /// Padding is not useful in terms of representing encoded data -- it makes no difference to
    /// the decoder if padding is present or not, so if you have some un-padded input to decode, it
    /// is perfectly fine to use `DecodePaddingMode::Indifferent` to prevent errors from being
    /// emitted.
    ///
    /// However, since in practice
    /// [people who learned nothing from BER vs DER seem to expect base64 to have one canonical encoding](https://eprint.iacr.org/2022/361),
    /// the default setting is the stricter `DecodePaddingMode::RequireCanonicalPadding`.
    ///
    /// Or, if "canonical" in your circumstance means _no_ padding rather than padding to the
    /// next multiple of four, there's `DecodePaddingMode::RequireNoPadding`.
    #[must_use]
    pub const fn with_decode_padding_mode(self, mode: DecodePaddingMode) -> Self {
        Self {
            decode_padding_mode: mode,
            ..self
        }
    }
}

impl Default for GeneralPurposeConfig {
    /// Delegates to [`GeneralPurposeConfig::new`].
    fn default() -> Self {
        Self::new()
    }
}

verus! {
#[verifier::verify]
impl Config for GeneralPurposeConfig {
    closed spec fn padding_enabled(&self) -> bool { self.encode_padding }
    fn encode_padding(&self) -> (result: bool)
        ensures result == self.padding_enabled(),
    {
        self.encode_padding
    }
}
}

#[cfg(all(
    feature = "simd-unsafe",
    any(
        target_arch = "x86_64",
        all(target_arch = "aarch64", target_feature = "neon")
    )
))]
impl GeneralPurposeConfig {
    /// Whether trailing bits are allowed when decoding.
    pub(crate) fn decode_allow_trailing_bits(&self) -> bool {
        self.decode_allow_trailing_bits
    }

    /// The decode padding mode.
    pub(crate) fn decode_padding_mode(&self) -> DecodePaddingMode {
        self.decode_padding_mode
    }
}

/// A [`GeneralPurpose`] engine using the [`alphabet::STANDARD`] base64 alphabet and [`PAD`] config.
///
/// Does not allow trailing bits when decoding.
vstd::prelude::verus! {
/// The canonical STANDARD initializer, with checked representation guarantees.
pub exec const STANDARD: GeneralPurpose 
 ensures crate::engine::Engine::is_standard(&STANDARD)
 {
    GeneralPurpose::new(&alphabet::STANDARD, PAD)
 }
}

/// A [`GeneralPurpose`] engine using the [`alphabet::STANDARD`] base64 alphabet and
/// [`PAD_INDIFFERENT`] config.
///
/// Does not allow trailing bits when decoding.
pub const STANDARD_PAD_INDIFFERENT: GeneralPurpose =
    GeneralPurpose::new(&alphabet::STANDARD, PAD_INDIFFERENT);

/// A [`GeneralPurpose`] engine using the [`alphabet::STANDARD`] base64 alphabet and [`NO_PAD`] config.
///
/// Does not allow trailing bits when decoding.
pub const STANDARD_NO_PAD: GeneralPurpose = GeneralPurpose::new(&alphabet::STANDARD, NO_PAD);

/// A [`GeneralPurpose`] engine using the [`alphabet::STANDARD`] base64 alphabet and
/// [`NO_PAD_INDIFFERENT`] config.
///
/// Does not allow trailing bits when decoding.
pub const STANDARD_NO_PAD_INDIFFERENT: GeneralPurpose =
    GeneralPurpose::new(&alphabet::STANDARD, NO_PAD_INDIFFERENT);

/// A [`GeneralPurpose`] engine using the [`alphabet::URL_SAFE`] base64 alphabet and [`PAD`] config.
///
/// Does not allow trailing bits when decoding.
pub const URL_SAFE: GeneralPurpose = GeneralPurpose::new(&alphabet::URL_SAFE, PAD);

/// A [`GeneralPurpose`] engine using the [`alphabet::URL_SAFE`] base64 alphabet and
/// [`PAD_INDIFFERENT`] config.
///
/// Does not allow trailing bits when decoding.
pub const URL_SAFE_PAD_INDIFFERENT: GeneralPurpose =
    GeneralPurpose::new(&alphabet::URL_SAFE, PAD_INDIFFERENT);

/// A [`GeneralPurpose`] engine using the [`alphabet::URL_SAFE`] base64 alphabet and [`NO_PAD`] config.
///
/// Does not allow trailing bits when decoding.
pub const URL_SAFE_NO_PAD: GeneralPurpose = GeneralPurpose::new(&alphabet::URL_SAFE, NO_PAD);

/// A [`GeneralPurpose`] engine using the [`alphabet::URL_SAFE`] base64 alphabet and
/// [`NO_PAD_INDIFFERENT`] config.
///
/// Does not allow trailing bits when decoding.
pub const URL_SAFE_NO_PAD_INDIFFERENT: GeneralPurpose =
    GeneralPurpose::new(&alphabet::URL_SAFE, NO_PAD_INDIFFERENT);

/// Include padding bytes when encoding, and require that they be present when decoding.
///
/// Does not allow trailing bits when decoding.
///
/// This is the standard per the base64 RFC, but consider using [`NO_PAD`] or [`NO_PAD_INDIFFERENT`]
/// instead as padding serves little purpose in practice.
vstd::prelude::verus! {
/// Canonical padding configuration.
pub exec const PAD: GeneralPurposeConfig 
 ensures PAD.canonical()
 { GeneralPurposeConfig::new() }
}

/// Include padding bytes when encoding, but allow input with or without padding when decoding.
///
/// Does not allow trailing bits when decoding.
pub const PAD_INDIFFERENT: GeneralPurposeConfig = GeneralPurposeConfig::new()
    .with_encode_padding(true)
    .with_decode_padding_mode(DecodePaddingMode::Indifferent);

/// Don't add padding when encoding, and require that there is no padding when decoding.
///
/// Does not allow trailing bits when decoding.
pub const NO_PAD: GeneralPurposeConfig = GeneralPurposeConfig::new()
    .with_encode_padding(false)
    .with_decode_padding_mode(DecodePaddingMode::RequireNone);

/// Don't add padding when encoding, and allow input with or without padding when decoding.
///
/// Does not allow trailing bits when decoding.
pub const NO_PAD_INDIFFERENT: GeneralPurposeConfig = GeneralPurposeConfig::new()
    .with_encode_padding(false)
    .with_decode_padding_mode(DecodePaddingMode::Indifferent);

vstd::prelude::verus! {
impl GeneralPurpose {
    /// Concrete representation produced by the actual initializer.
    pub closed spec fn initialized_from(&self, alphabet: &Alphabet, config: GeneralPurposeConfig) -> bool {
        self.encode_table@ == alphabet.bytes() && self.padding == alphabet.pad() && self.config == config
        && (distinct(alphabet.bytes()) ==> forall|i: int| 0 <= i < 64 ==> self.decode_table@[alphabet.bytes()[i] as int] == i as u8)
    }

    /// Create a `GeneralPurpose` engine from an [Alphabet].
    ///
    /// While not very expensive to initialize, ideally these should be cached
    /// if the engine will be used repeatedly.
    #[must_use]
    pub const fn new(alphabet: &Alphabet, config: GeneralPurposeConfig) -> (result: Self)

        ensures result.initialized_from(alphabet, config),
    
    {
        proof { alphabet.representation(); }
        Self {
            encode_table: encode_table(alphabet),
            decode_table: decode_table(alphabet),
            padding: alphabet.padding,
            config,
        }
    }
    }
}

vstd::prelude::verus! {
impl GeneralPurposeConfig {
    /// Canonical PAD configuration predicate.
    pub closed spec fn canonical(&self) -> bool {
        self.encode_padding && !self.decode_allow_trailing_bits && self.decode_padding_mode == DecodePaddingMode::RequireCanonical
    }
    /// Create the canonical padding configuration.
    #[must_use]
    pub const fn new() -> (result: Self)

        ensures result.canonical(),
    
    {
        Self {
            // RFC states that padding must be applied by default
            encode_padding: true,
            decode_allow_trailing_bits: false,
            decode_padding_mode: DecodePaddingMode::RequireCanonical,
        }
    }
    }
}

verus! {
/// Pairwise distinct alphabet symbols.
    pub open spec fn distinct(bytes: Seq<u8>) -> bool {
    forall|i: int, j: int| 0 <= i < bytes.len() && 0 <= j < bytes.len() && i != j ==> bytes[i] != bytes[j]
}
}

verus! {
/// Checked caller use of the publicly selected constant; no readiness precondition.
#[allow(dead_code)]
fn standard_initialization_use() {
    let selected = STANDARD;
    assert(crate::engine::Engine::is_standard(&selected));
}
}
