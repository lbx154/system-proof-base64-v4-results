#[allow(unused_imports)]
use vstd::prelude::*;
use crate::alphabet::Symbol;
use crate::{
    engine::{general_purpose::INVALID_VALUE, DecodeEstimate, DecodeMetadata, DecodePaddingMode},
    DecodeError, DecodeSliceError,
};

#[doc(hidden)]
#[verifier::verify]
pub struct GeneralPurposeEstimate {
    /// input len % 4
    rem: usize,
    conservative_decoded_len: usize,
}

verus! {
impl GeneralPurposeEstimate {
    pub(crate) closed spec fn remainder(&self) -> usize { self.rem }
    pub(crate) proof fn compatible_facts(&self, input_len: nat)
        requires self.compatible_with(input_len),
        ensures self.remainder() as nat == input_len % 4,
            self.capacity() as nat == crate::round_trip_spec::decoded_estimate(input_len),
    {}
    /// Conservative capacity observation for the public sizing helper.
    pub closed spec fn capacity(&self) -> usize { self.conservative_decoded_len }
    pub(crate) fn new(encoded_len: usize) -> (result: Self)
        ensures
            result.compatible_with(encoded_len as nat),
            result.remainder() == encoded_len % 4,
            result.capacity() as nat == crate::round_trip_spec::decoded_estimate(encoded_len as nat),
            result.estimated_len() == crate::round_trip_spec::decoded_estimate(encoded_len as nat),
    {
        let rem = encoded_len % 4;
        Self {
            rem,
            conservative_decoded_len: (encoded_len / 4 + if rem > 0 { 1usize } else { 0usize }) * 3,
        }
    }
}

}

verus! {
#[verifier::verify]
impl DecodeEstimate for GeneralPurposeEstimate {
    closed spec fn estimated_len(&self) -> nat { self.conservative_decoded_len as nat }
    closed spec fn compatible_with(&self, input_len: nat) -> bool {
        self.rem as nat == input_len % 4
        && self.conservative_decoded_len as nat == crate::round_trip_spec::decoded_estimate(input_len)
    }
    #[verus_spec(result => ensures result == self.capacity())]
    fn decoded_len_estimate(&self) -> usize {
        self.conservative_decoded_len
    }
}

}

verus! {
/// Canonical success domain, separate from unrestricted native dispatch safety.
pub(crate) open spec fn canonical_helper_use(x: Seq<u8>, input: Seq<u8>, output: Seq<u8>,
    table: Seq<u8>, padding: u8, mode: DecodePaddingMode) -> bool {
    input == crate::round_trip_spec::standard_encoding(x)
    && output.len() >= crate::round_trip_spec::decoded_estimate(input.len())
    && padding == 61 && mode == DecodePaddingMode::RequireCanonical
    && forall|v: nat| v < 64 ==> table[crate::round_trip_spec::standard_symbol(v) as int] == v as u8
}
}

/// Helper to avoid duplicating `num_chunks` calculation, which is costly on short inputs.
/// Returns the decode metadata, or an error.
// We're on the fragile edge of compiler heuristics here. If this is not inlined, slow. If this is
// inlined(always), a different slow. plain ol' inline makes the benchmarks happiest at the moment,
// but this is fragile and the best setting changes with only minor code modifications.
#[allow(clippy::too_many_arguments)]
#[inline]
verus! {
#[verifier::verify]
#[verus_spec(result =>
    with Ghost(x): Ghost<Seq<u8>>,
    requires estimate.remainder() == input@.len() % 4,
        forall|i: &[u8], n: usize, o: &mut [u8]| #[trigger] call_requires(simd_prefix,(i,n,o)),
        forall|i: &[u8], n: usize, o: &mut [u8], r: (usize,usize)| #[trigger] call_ensures(simd_prefix,(i,n,o),r) ==> r == (0usize,0usize) && final(o)@ == o@,
    ensures canonical_helper_use(x,input@,old(output)@,decode_table@,padding.byte(),padding_mode) ==> match result {
        Ok(m) => m.decoded_len as nat == x.len()
            && crate::round_trip_spec::suffix_unchanged(old(output)@, final(output)@, x.len())
            && final(output)@.subrange(0,x.len() as int) == x
            && m.padding_offset == if 0 < super::decode_suffix::terminal_tail(x).len() < 3 {
                Some((super::decode_suffix::terminal_input_index(x) + super::decode_suffix::symbol_count(super::decode_suffix::terminal_tail(x))) as usize)
            } else { None },
        Err(_) => false,
    },
)]
pub(crate) fn decode_helper(
    input: &[u8],
    estimate: &GeneralPurposeEstimate,
    output: &mut [u8],
    decode_table: &[u8; 256],
    decode_allow_trailing_bits: bool,
    padding: Symbol,
    padding_mode: DecodePaddingMode,
    simd_prefix: impl FnOnce(&[u8], usize, &mut [u8]) -> (usize, usize),
) -> Result<DecodeMetadata, DecodeSliceError> {
    let ghost initial_output = output@;
    let ghost canonical = canonical_helper_use(x,input@,initial_output,decode_table@,padding.byte(),padding_mode);
    proof {
      if canonical {
        super::decode_suffix::canonical_suffix_image(x);
        if x.len() % 3 == 0 {} else if x.len() % 3 == 1 {} else {}
        assert(x.len() <= crate::round_trip_spec::decoded_estimate(input@.len()));
      }
    }
    let input_complete_nonterminal_quads_len = {
        proof_with! { Ghost(x) }
        complete_quads_len(input, estimate.rem, output.len(), decode_table, padding)?
    };

    let output_complete_quad_len = input_complete_nonterminal_quads_len / 4 * 3;

    // A SIMD backend, when applicable, decodes a leading prefix; the scalar loops decode the rest.
    // `simd_prefix` returns `(input_consumed, output_written)` and must consume only whole, valid
    // quads: `input_consumed % 4 == 0` and `<= input_complete_nonterminal_quads_len` (the terminal
    // quad is left to `decode_suffix`), `output_written == input_consumed / 4 * 3`, and only those
    // output bytes are written. It stops on the first invalid/ambiguous quad so the scalar decoder
    // reports the precise error offset. `(0, 0)` (pure scalar) is always valid.
    let (input_index, output_index) =
        simd_prefix(input, input_complete_nonterminal_quads_len, output);

    debug_assert!(input_index % 4 == 0, "prefix must consume whole quads");
    debug_assert!(
        input_index <= input_complete_nonterminal_quads_len,
        "prefix must not consume the terminal quad"
    );
    debug_assert!(
        output_index == input_index / 4 * 3,
        "prefix output must match consumed input"
    );

    proof {
      if canonical {
        super::decode_suffix::canonical_suffix_image(x);
        assert(input_complete_nonterminal_quads_len as nat == super::decode_suffix::terminal_input_index(x));
        assert(input_complete_nonterminal_quads_len % 4 == 0);
        reveal(decoder_image);
        assert(canonical_quads_use(x,input@,decode_table@,input_complete_nonterminal_quads_len as nat));
      }
    }
    proof_with! { Ghost(x) }
    decode_complete_quads(
        input,
        input_index,
        input_complete_nonterminal_quads_len,
        decode_table,
        output,
        output_index,
    )?;

    let ghost prefix_output = output@;
    let ghost t = if canonical { super::decode_suffix::terminal_tail(x) } else { Seq::<u8>::empty() };
    let result = { proof_with! { Ghost(t) }
        super::decode_suffix::decode_suffix(
        input,
        input_complete_nonterminal_quads_len,
        output,
        output_complete_quad_len,
        decode_table,
        decode_allow_trailing_bits,
        padding,
        padding_mode,
    ) };
    proof {
      if canonical {
        assert forall|j: int| 0 <= j < x.len() implies output@[j] == x[j] by {
            if j < output_complete_quad_len {
                assert(output@[j] == prefix_output[j]);
                assert(prefix_output[j] == x[j]);
            } else {
                let t = super::decode_suffix::terminal_tail(x);
                assert(t[j-output_complete_quad_len as int] == x[j]);
                assert(output@[j] == t[j-output_complete_quad_len as int]);
            }
        }
        assert(output@.subrange(0,x.len() as int) =~= x);
      }
    }
    result
}

}

/// Decode the complete non-terminal quads in `input[input_index_start..input_index_end]` (both
/// bounds multiples of 4), writing to `output` starting at `output_index_start`, which must equal
/// `input_index_start / 4 * 3`. Error offsets are reported in absolute input coordinates.
verus! {
#[verifier::opaque]
pub(crate) open spec fn decoder_image(x: Seq<u8>, e: Seq<u8>) -> bool {
    e == crate::round_trip_spec::standard_encoding(x)
}
pub(crate) proof fn image_window(x: Seq<u8>, e: Seq<u8>, table: Seq<u8>, ii: nat, quads: nat)
    requires decoder_image(x,e), ii % 4 == 0, 1 <= quads <= 2,
        ii + 4 * quads <= e.len(), 3 * (ii / 4 + quads) <= x.len(),
        table.len() == 256,
        forall|v: nat| v < 64 ==> table[crate::round_trip_spec::standard_symbol(v) as int] == v as u8,
    ensures
        forall|i: int| 0 <= i < 4*quads ==> #[trigger] table[e[ii as int+i] as int] < 64,
        forall|j: int| 0 <= j < 3*quads ==> #[trigger] chunk_byte(e.subrange(ii as int,(ii+4*quads) as int),table,j) == x[(3*(ii/4)) as int+j],
{
    reveal(decoder_image);
    canonical_chunk(x,table,ii/4,quads);
    assert(4*(ii/4) == ii) by(nonlinear_arith) requires ii % 4 == 0;
    assert(4*(ii/4+quads) == ii+4*quads) by(nonlinear_arith) requires ii % 4 == 0;
}
pub(crate) proof fn image_window_byte(x: Seq<u8>, e: Seq<u8>, table: Seq<u8>, ii: nat, quads: nat, j: int)
    requires decoder_image(x,e), ii % 4 == 0, 1 <= quads <= 2,
        ii + 4 * quads <= e.len(), 3 * (ii / 4 + quads) <= x.len(),
        table.len() == 256, 0 <= j < 3 * quads,
        forall|v: nat| v < 64 ==> table[crate::round_trip_spec::standard_symbol(v) as int] == v as u8,
    ensures chunk_byte(e.subrange(ii as int,(ii+4*quads) as int),table,j) == x[(3*(ii/4)) as int+j],
{
    image_window(x,e,table,ii,quads);
}

}

verus! {
pub(crate) open spec fn canonical_quads_use(x: Seq<u8>, input: Seq<u8>, table: Seq<u8>, end: nat) -> bool {
    decoder_image(x,input)
        && end <= crate::engine::general_purpose::decode_suffix::terminal_input_index(x)
        && forall|v: nat| v < 64 ==> table[crate::round_trip_spec::standard_symbol(v) as int] == v as u8
}
#[verifier::verify]
#[verus_spec(result =>
    with Ghost(x): Ghost<Seq<u8>>,
    requires input_index_start <= input_index_end <= input.len(),
        input_index_start % 4 == 0, input_index_end % 4 == 0,
        output_index_start == input_index_start / 4 * 3,
        old(output)@.len() >= input_index_end / 4 * 3,
    ensures final(output)@.len() == old(output)@.len(),
        forall|j: int| 0 <= j < old(output)@.len() && (j < output_index_start || j >= input_index_end / 4 * 3) ==> final(output)@[j] == old(output)@[j],
        canonical_quads_use(x,input@,decode_table@,input_index_end as nat) ==>
            (result == Ok::<(), DecodeSliceError>(()) &&
            forall|j: int| output_index_start <= j < input_index_end / 4 * 3 ==> final(output)@[j] == x[j]),
)]
fn decode_complete_quads(
    input: &[u8],
    input_index_start: usize,
    input_index_end: usize,
    decode_table: &[u8; 256],
    output: &mut [u8],
    output_index_start: usize,
) -> Result<(), DecodeSliceError> {
    hide(chunk_byte);
    let ghost canonical = canonical_quads_use(x,input@,decode_table@,input_index_end as nat);
    debug_assert!(
        input_index_start % 4 == 0,
        "quad start must be quad-aligned"
    );
    debug_assert!(input_index_end % 4 == 0, "quad end must be quad-aligned");
    debug_assert!(
        output_index_start == input_index_start / 4 * 3,
        "output start must match consumed input"
    );

    let UNROLLED_INPUT_CHUNK_SIZE: usize = 32;
    let UNROLLED_OUTPUT_CHUNK_SIZE: usize = UNROLLED_INPUT_CHUNK_SIZE / 4 * 3;

    let quads_len = input_index_end - input_index_start;
    let unrolled_loop_len = quads_len - quads_len % UNROLLED_INPUT_CHUNK_SIZE;
    let input_unrolled_loop_end = input_index_start + unrolled_loop_len;

    proof {
      if canonical {
        reveal(canonical_quads_use);
        reveal(decoder_image);
        crate::engine::general_purpose::decode_suffix::canonical_suffix_image(x);
        reveal(decoder_image);
        assert(decoder_image(x,input@));
    
      }
}
    let ghost initial_output = output@;
    let mut chunk_index: usize = 0;
    while chunk_index < unrolled_loop_len / UNROLLED_INPUT_CHUNK_SIZE
        invariant
            canonical == canonical_quads_use(x,input@,decode_table@,input_index_end as nat),
            canonical ==> decoder_image(x,input@),
            input_index_start <= input_index_end,
            input_index_end <= input@.len(),
            canonical ==> input_index_end / 4 * 3 <= x.len(),
            input_index_end / 4 * 3 <= output@.len(),
            input_index_start % 4 == 0, input_index_end % 4 == 0,
            canonical ==> input_index_end as nat <= crate::engine::general_purpose::decode_suffix::terminal_input_index(x),
            output_index_start == input_index_start / 4 * 3,
            output@.len() == initial_output.len(),
            old(output)@ == initial_output,

            canonical ==> (forall|v: nat| v < 64 ==> decode_table@[crate::round_trip_spec::standard_symbol(v) as int] == v as u8),
            UNROLLED_INPUT_CHUNK_SIZE == 32, UNROLLED_OUTPUT_CHUNK_SIZE == 24,
            quads_len == input_index_end - input_index_start,
            unrolled_loop_len == quads_len - quads_len % 32,
            input_unrolled_loop_end == input_index_start + unrolled_loop_len,
            chunk_index <= unrolled_loop_len / 32,
            canonical ==> (forall|j: int| output_index_start <= j < output_index_start + chunk_index * 24 ==> output@[j] == x[j]),
            forall|j: int| 0 <= j < output@.len() && (j < output_index_start || j >= output_index_start + chunk_index * 24) ==> output@[j] == initial_output[j],
        decreases unrolled_loop_len / 32 - chunk_index,
    {
        let chunk = &input[input_index_start + chunk_index * 32..input_index_start + (chunk_index + 1) * 32];
        let input_index = input_index_start + chunk_index * UNROLLED_INPUT_CHUNK_SIZE;
        let output_base = output_index_start + chunk_index * UNROLLED_OUTPUT_CHUNK_SIZE;
        proof {
      if canonical {

            image_window(x,input@,decode_table@,input_index as nat,2);
            image_window(x,input@,decode_table@,(input_index+8) as nat,2);
            image_window(x,input@,decode_table@,(input_index+16) as nat,2);
            image_window(x,input@,decode_table@,(input_index+24) as nat,2);
            assert(chunk@.subrange(0,8) =~= input@.subrange(input_index as int,(input_index+8) as int));
            assert(chunk@.subrange(8,16) =~= input@.subrange((input_index+8) as int,(input_index+16) as int));
            assert(chunk@.subrange(16,24) =~= input@.subrange((input_index+16) as int,(input_index+24) as int));
            assert(chunk@.subrange(24,32) =~= input@.subrange((input_index+24) as int,(input_index+32) as int));
            assert forall|i: int| 0 <= i < 32 implies decode_table@[chunk@[i] as int] < 64 by {
                if i < 8 { assert(decode_table@[input@[input_index as int+i] as int] < 64); }
                else if i < 16 { assert(decode_table@[input@[(input_index+8) as int+(i-8)] as int] < 64); }
                else if i < 24 { assert(decode_table@[input@[(input_index+16) as int+(i-16)] as int] < 64); }
                else { assert(decode_table@[input@[(input_index+24) as int+(i-24)] as int] < 64); }
            }
            assert(input_index / 4 * 3 == output_base);
            assert forall|j: int| 0 <= j < 6 implies chunk_byte(chunk@.subrange(0,8),decode_table@,j) == x[output_base as int+j] by {
                image_window_byte(x,input@,decode_table@,input_index as nat,2,j);
                assert(chunk_byte(input@.subrange(input_index as int,(input_index+8) as int),decode_table@,j) == x[(3*(input_index/4)) as int+j]);
            }
            assert forall|j: int| 0 <= j < 6 implies chunk_byte(chunk@.subrange(8,16),decode_table@,j) == x[output_base as int+6+j] by {
                image_window_byte(x,input@,decode_table@,(input_index+8) as nat,2,j);
                assert(chunk_byte(input@.subrange((input_index+8) as int,(input_index+16) as int),decode_table@,j) == x[(3*((input_index+8)/4)) as int+j]);
            }
            assert forall|j: int| 0 <= j < 6 implies chunk_byte(chunk@.subrange(16,24),decode_table@,j) == x[output_base as int+12+j] by {
                image_window_byte(x,input@,decode_table@,(input_index+16) as nat,2,j);
                assert(chunk_byte(input@.subrange((input_index+16) as int,(input_index+24) as int),decode_table@,j) == x[(3*((input_index+16)/4)) as int+j]);
            }
            assert forall|j: int| 0 <= j < 6 implies chunk_byte(chunk@.subrange(24,32),decode_table@,j) == x[output_base as int+18+j] by {
                image_window_byte(x,input@,decode_table@,(input_index+24) as nat,2,j);
                assert(chunk_byte(input@.subrange((input_index+24) as int,(input_index+32) as int),decode_table@,j) == x[(3*((input_index+24)/4)) as int+j]);
            }

        
      }
}
        let chunk_output = &mut output[output_base..output_base + UNROLLED_OUTPUT_CHUNK_SIZE];

        decode_chunk_8(
            &chunk[0..8],
            input_index,
            decode_table,
            &mut chunk_output[0..6],
        )?;
        decode_chunk_8(
            &chunk[8..16],
            input_index + 8,
            decode_table,
            &mut chunk_output[6..12],
        )?;
        decode_chunk_8(
            &chunk[16..24],
            input_index + 16,
            decode_table,
            &mut chunk_output[12..18],
        )?;
        decode_chunk_8(
            &chunk[24..32],
            input_index + 24,
            decode_table,
            &mut chunk_output[18..24],
        )?;
        proof {
      if canonical {

            assert forall|j: int| 0 <= j < 24 implies chunk_output@[j] == x[output_base as int+j] by {
                if j < 6 {} else if j < 12 {} else if j < 18 {} else {}
            }
        
      }
}
        chunk_index += 1;
    }

    // remaining quads, except for the last possibly partial one, as it may have padding
    let output_after_unroll_start = output_index_start + unrolled_loop_len / 4 * 3;
    let mut chunk_index: usize = 0;
    while chunk_index < (input_index_end - input_unrolled_loop_end) / 4
        invariant
            canonical == canonical_quads_use(x,input@,decode_table@,input_index_end as nat),
            canonical ==> decoder_image(x,input@),
            input_index_start <= input_index_end,
            input_index_end <= input@.len(),
            canonical ==> input_index_end / 4 * 3 <= x.len(),
            input_index_end / 4 * 3 <= output@.len(),
            input_index_start % 4 == 0, input_index_end % 4 == 0,
            canonical ==> input_index_end as nat <= crate::engine::general_purpose::decode_suffix::terminal_input_index(x),
            output_index_start == input_index_start / 4 * 3,
            output@.len() == initial_output.len(),
            old(output)@ == initial_output,

            canonical ==> (forall|v: nat| v < 64 ==> decode_table@[crate::round_trip_spec::standard_symbol(v) as int] == v as u8),
            quads_len == input_index_end - input_index_start,
            unrolled_loop_len == quads_len - quads_len % 32,
            input_unrolled_loop_end == input_index_start + unrolled_loop_len,
            output_after_unroll_start == output_index_start + unrolled_loop_len / 4 * 3,
            chunk_index <= (input_index_end - input_unrolled_loop_end) / 4,
            canonical ==> (forall|j: int| output_index_start <= j < output_after_unroll_start + chunk_index * 3 ==> output@[j] == x[j]),
            forall|j: int| 0 <= j < output@.len() && (j < output_index_start || j >= output_after_unroll_start + chunk_index * 3) ==> output@[j] == initial_output[j],
        decreases (input_index_end - input_unrolled_loop_end) / 4 - chunk_index,
    {
        let chunk = &input[input_unrolled_loop_end + chunk_index * 4..input_unrolled_loop_end + (chunk_index + 1) * 4];
        let output_base = output_after_unroll_start + chunk_index * 3;
        proof {
      if canonical {

            image_window(x,input@,decode_table@,(input_unrolled_loop_end+chunk_index*4) as nat,1);
            assert forall|i: int| 0 <= i < 4 implies decode_table@[chunk@[i] as int] < 64 by {
                assert(decode_table@[input@[(input_unrolled_loop_end+chunk_index*4) as int+i] as int] < 64);
            }
            assert((input_unrolled_loop_end+chunk_index*4) / 4 * 3 == output_base);
            assert forall|j: int| 0 <= j < 3 implies chunk_byte(chunk@,decode_table@,j) == x[output_base as int+j] by {
                let ii = (input_unrolled_loop_end+chunk_index*4) as nat;
                assert(chunk_byte(input@.subrange(ii as int,(ii+4) as int),decode_table@,j) == x[(3*(ii/4)) as int+j]);
                assert(chunk@ =~= input@.subrange(ii as int, (ii+4) as int));
            }

        
      }
}
        let chunk_output = &mut output[output_base..output_base + 3];

        decode_chunk_4(
            chunk,
            input_unrolled_loop_end + chunk_index * 4,
            decode_table,
            chunk_output,
        )?;
        proof {
      if canonical {

            assert forall|j: int| 0 <= j < 3 implies chunk_output@[j] == x[output_base as int+j] by {}
        
      }
}
        chunk_index += 1;
    }

    Ok(())
}

}

/// Returns the length of complete quads, except for the last one, even if it is complete.
///
/// Returns an error if the output len is not big enough for decoding those complete quads, or if
/// the input % 4 == 1, and that last byte is an invalid value other than a pad byte.
///
/// - `input` is the base64 input
/// - `input_len_rem` is input len % 4
/// - `output_len` is the length of the output slice
verus! {
pub(crate) open spec fn nonterminal_len(n: nat) -> nat {
    if n == 0 { 0 } else { (((n - 1) / 4) * 4) as nat }
}
pub(crate) open spec fn invalid_last(input: Seq<u8>, table: Seq<u8>, pad: u8) -> bool {
    input.len() % 4 == 1 && input[input.len() as int - 1] != pad
        && table[input[input.len() as int - 1] as int] == 255u8
}
#[verifier::verify]
#[verus_spec(result =>
    with Ghost(x): Ghost<Seq<u8>>,
    requires input_len_rem == input.len() % 4,
    ensures
        match result {
            Ok(k) => k as nat == nonterminal_len(input.len() as nat)
                && k % 4 == 0 && k <= input.len() && k / 4 * 3 <= output_len
                && !invalid_last(input@,decode_table@,padding.byte())
                && (input.len() == 0 || 1 <= input.len() - k <= 4),
            Err(e) => if invalid_last(input@,decode_table@,padding.byte()) {
                e == DecodeSliceError::DecodeError(DecodeError::InvalidByte((input.len() - 1) as usize, input@[input.len() as int - 1]))
            } else {
                e == DecodeSliceError::OutputSliceTooSmall && output_len < nonterminal_len(input.len() as nat) / 4 * 3
            },
        },
        (input@ == crate::round_trip_spec::standard_encoding(x)
            && output_len >= crate::round_trip_spec::decoded_estimate(input@.len())) ==>
            result == Ok(crate::engine::general_purpose::decode_suffix::terminal_input_index(x) as usize),
)]
pub(crate) fn complete_quads_len(
    input: &[u8],
    input_len_rem: usize,
    output_len: usize,
    decode_table: &[u8; 256],
    padding: Symbol,
) -> Result<usize, DecodeSliceError> {
    proof {
        if input@ == crate::round_trip_spec::standard_encoding(x) {
            crate::engine::general_purpose::decode_suffix::canonical_suffix_image(x);
        }
    }
    debug_assert!(input.len() % 4 == input_len_rem);

    // detect a trailing invalid byte, like a newline, as a user convenience
    if input_len_rem == 1 {
        let last_byte = input[input.len() - 1];
        // exclude pad bytes; might be part of padding that extends from earlier in the input
        if last_byte != padding.as_u8() && decode_table[usize::from(last_byte)] == INVALID_VALUE {
            return Err(DecodeError::InvalidByte(input.len() - 1, last_byte).into());
        }
    };

    // skip last quad, even if it's complete, as it may have padding
    let input_complete_nonterminal_quads_len = input
        .len()
        .saturating_sub(input_len_rem)
        // if rem was 0, subtract 4 to avoid padding
        .saturating_sub(if input_len_rem == 0 { 4usize } else { 0usize });
    debug_assert!(
        input.is_empty() || (1 <= input.len() - input_complete_nonterminal_quads_len && input.len() - input_complete_nonterminal_quads_len <= 4)
    );

    // check that everything except the last quad handled by decode_suffix will fit
    if output_len < input_complete_nonterminal_quads_len / 4 * 3 {
        return Err(DecodeSliceError::OutputSliceTooSmall);
    };
    Ok(input_complete_nonterminal_quads_len)
}
}

verus! {
use crate::round_trip_spec::*;
/// Checked arithmetic/bit-vector inversion of the canonical complete triple vocabulary.
pub(crate) proof fn invert_triple(a: u8, b: u8, c: u8)
    ensures
        ((((a as nat / 4) as u8) << 2) | ((((a as nat % 4) * 16 + b as nat / 16) as u8) >> 4)) == a,
        (((((a as nat % 4) * 16 + b as nat / 16) as u8) << 4) | ((((b as nat % 16) * 4 + c as nat / 64) as u8) >> 2)) == b,
        (((((b as nat % 16) * 4 + c as nat / 64) as u8) << 6) | ((c as nat % 64) as u8)) == c,
{
    assert((((a / 4) << 2) | ((((a % 4) * 16 + b / 16) as u8) >> 4)) == a) by(bit_vector);
    assert((((((a % 4) * 16 + b / 16) as u8) << 4) | ((((b % 16) * 4 + c / 64) as u8) >> 2)) == b) by(bit_vector);
    assert((((((b % 16) * 4 + c / 64) as u8) << 6) | (c % 64)) == c) by(bit_vector);
}

/// Derives table legitimacy and recovered bytes for any complete encoder-image quad.
pub(crate) proof fn canonical_quad(x: Seq<u8>, table: Seq<u8>, q: nat)
    requires 3 * q + 3 <= x.len(), table.len() == 256,
        forall|v: nat| v < 64 ==> table[standard_symbol(v) as int] == v as u8,
    ensures
        forall|i: int| 0 <= i < 4 ==> #[trigger] table[encoded_byte(x, 4 * q + i as nat) as int] < 64,
        forall|j: int| 0 <= j < 3 ==> #[trigger] chunk_byte(standard_encoding(x).subrange((4 * q) as int, (4 * q + 4) as int), table, j) == x[(3 * q) as int + j],
{
    let a = x[(3 * q) as int];
    let b = x[(3 * q + 1) as int];
    let c = x[(3 * q + 2) as int];
    invert_triple(a, b, c);
    assert((4 * q) / 4 == q && (4 * q) % 4 == 0) by(nonlinear_arith);
    assert((4 * q + 1) / 4 == q && (4 * q + 1) % 4 == 1) by(nonlinear_arith);
    assert((4 * q + 2) / 4 == q && (4 * q + 2) % 4 == 2) by(nonlinear_arith);
    assert((4 * q + 3) / 4 == q && (4 * q + 3) % 4 == 3) by(nonlinear_arith);
    assert forall|i: int| 0 <= i < 4 implies #[trigger] table[encoded_byte(x, 4 * q + i as nat) as int] < 64 by {
        if i == 0 {} else if i == 1 {} else if i == 2 {} else { assert(i == 3); }
    }
    assert forall|j: int| 0 <= j < 3 implies #[trigger] chunk_byte(standard_encoding(x).subrange((4 * q) as int, (4 * q + 4) as int), table, j) == x[(3 * q) as int + j] by {
        if j == 0 {} else if j == 1 {} else { assert(j == 2); }
    }
}

/// Consumer interface for either scalar chunk size; no implementation-readiness premise.
pub(crate) proof fn canonical_chunk(x: Seq<u8>, table: Seq<u8>, q: nat, quads: nat)
    requires 1 <= quads <= 2, 3 * (q + quads) <= x.len(), table.len() == 256,
        forall|v: nat| v < 64 ==> table[standard_symbol(v) as int] == v as u8,
    ensures
        forall|i: int| 0 <= i < 4 * quads ==> #[trigger] table[standard_encoding(x)[(4 * q) as int + i] as int] < 64,
        forall|j: int| 0 <= j < 3 * quads ==> #[trigger] chunk_byte(standard_encoding(x).subrange((4 * q) as int, (4 * (q + quads)) as int), table, j) == x[(3 * q) as int + j],
{
    canonical_quad(x, table, q);
    if quads == 2 { canonical_quad(x, table, q + 1); }
    assert forall|i: int| 0 <= i < 4 * quads implies #[trigger] table[standard_encoding(x)[(4 * q) as int + i] as int] < 64 by {
        if i < 4 {} else { assert(0 <= i - 4 < 4); }
    }
    assert forall|j: int| 0 <= j < 3 * quads implies #[trigger] chunk_byte(standard_encoding(x).subrange((4 * q) as int, (4 * (q + quads)) as int), table, j) == x[(3 * q) as int + j] by {
        if j < 3 {
            assert(chunk_byte(standard_encoding(x).subrange((4 * q) as int, (4 * (q + quads)) as int), table, j) == chunk_byte(standard_encoding(x).subrange((4 * q) as int, (4 * q + 4) as int), table, j));
        } else {
            assert(3 <= j < 6);
            assert(j / 3 == 1 && (j - 3) / 3 == 0 && j % 3 == (j - 3) % 3);
            assert(chunk_byte(standard_encoding(x).subrange((4 * q) as int, (4 * (q + quads)) as int), table, j) == chunk_byte(standard_encoding(x).subrange((4 * (q + 1)) as int, (4 * (q + 1) + 4) as int), table, j - 3));
        }
    }
}

/// Value decoded from a complete quad of legitimate six-bit table entries.
pub(crate) open spec fn chunk_byte(input: Seq<u8>, table: Seq<u8>, j: int) -> u8 {
    let q = 4 * (j / 3);
    if j % 3 == 0 { (table[input[q] as int] << 2) | (table[input[q + 1] as int] >> 4) }
    else if j % 3 == 1 { (table[input[q + 1] as int] << 4) | (table[input[q + 2] as int] >> 2) }
    else { (table[input[q + 2] as int] << 6) | table[input[q + 3] as int] }
}
}

/// Decode 8 bytes of input into 6 bytes of output.
///
/// `input` is the 8 bytes to decode.
/// `index_at_start_of_input` is the offset in the overall input (used for reporting errors
/// accurately)
/// `decode_table` is the lookup table for the particular base64 alphabet.
/// `output` will have its first 6 bytes overwritten
// yes, really inline (worth 30-50% speedup)
verus! {
#[verifier::verify]
#[verus_spec(result =>
    requires input.len() >= 8,
        (exists|i: int| 0 <= i < 8 && decode_table@[input@[i] as int] == 255u8)
            || old(output).len() >= 6,
        forall|i: int| 0 <= i < 8 && decode_table@[input@[i] as int] == 255u8
            ==> index_at_start_of_input as int + i <= usize::MAX,
    ensures
        result.is_ok() <==> (forall|i: int| 0 <= i < 8 ==> decode_table@[input@[i] as int] != 255u8),
        result.is_err() ==> final(output)@ == old(output)@,
        result.is_err() ==> exists|i: int| 0 <= i < 8
            && decode_table@[input@[i] as int] == 255u8
            && (forall|k: int| 0 <= k < i ==> decode_table@[input@[k] as int] != 255u8)
            && result == Err(DecodeError::InvalidByte((index_at_start_of_input as int + i) as usize, input@[i])),
        (forall|i: int| 0 <= i < 8 ==> decode_table@[input@[i] as int] < 64) ==>
            (result == Ok(()) && forall|j: int| 0 <= j < 6 ==> final(output)@[j] == chunk_byte(input@, decode_table@, j)),
        final(output)@.len() == old(output)@.len(),
        forall|i: int| 6 <= i < final(output)@.len() ==> final(output)@[i] == old(output)@[i],
)]
#[inline(always)]
fn decode_chunk_8(
    input: &[u8],
    index_at_start_of_input: usize,
    decode_table: &[u8; 256],
    output: &mut [u8],
) -> Result<(), DecodeError> {
    let morsel = decode_table[usize::from(input[0])];
    let ghost s0 = morsel;
    if morsel == INVALID_VALUE {
        return Err(DecodeError::InvalidByte(index_at_start_of_input, input[0]));
    }
    let mut accum = u64::from(morsel) << 58;

    let morsel = decode_table[usize::from(input[1])];
    let ghost s1 = morsel;
    if morsel == INVALID_VALUE {
        return Err(DecodeError::InvalidByte(
            index_at_start_of_input + 1,
            input[1],
        ));
    }
    accum |= u64::from(morsel) << 52;

    let morsel = decode_table[usize::from(input[2])];
    let ghost s2 = morsel;
    if morsel == INVALID_VALUE {
        return Err(DecodeError::InvalidByte(
            index_at_start_of_input + 2,
            input[2],
        ));
    }
    accum |= u64::from(morsel) << 46;

    let morsel = decode_table[usize::from(input[3])];
    let ghost s3 = morsel;
    if morsel == INVALID_VALUE {
        return Err(DecodeError::InvalidByte(
            index_at_start_of_input + 3,
            input[3],
        ));
    }
    accum |= u64::from(morsel) << 40;

    let morsel = decode_table[usize::from(input[4])];
    let ghost s4 = morsel;
    if morsel == INVALID_VALUE {
        return Err(DecodeError::InvalidByte(
            index_at_start_of_input + 4,
            input[4],
        ));
    }
    accum |= u64::from(morsel) << 34;

    let morsel = decode_table[usize::from(input[5])];
    let ghost s5 = morsel;
    if morsel == INVALID_VALUE {
        return Err(DecodeError::InvalidByte(
            index_at_start_of_input + 5,
            input[5],
        ));
    }
    accum |= u64::from(morsel) << 28;

    let morsel = decode_table[usize::from(input[6])];
    let ghost s6 = morsel;
    if morsel == INVALID_VALUE {
        return Err(DecodeError::InvalidByte(
            index_at_start_of_input + 6,
            input[6],
        ));
    }
    accum |= u64::from(morsel) << 22;

    let morsel = decode_table[usize::from(input[7])];
    let ghost s7 = morsel;
    if morsel == INVALID_VALUE {
        return Err(DecodeError::InvalidByte(
            index_at_start_of_input + 7,
            input[7],
        ));
    }
    accum |= u64::from(morsel) << 16;

    proof {
      if s0 < 64 && s1 < 64 && s2 < 64 && s3 < 64 && s4 < 64 && s5 < 64 && s6 < 64 && s7 < 64 {
        assert(((accum >> 56) as u8) == ((s0 << 2) | (s1 >> 4))) by(bit_vector)
            requires s0 < 64, s1 < 64, s2 < 64, s3 < 64, s4 < 64, s5 < 64, s6 < 64, s7 < 64, accum == (((s0 as u64) << 58) | ((s1 as u64) << 52) | ((s2 as u64) << 46) | ((s3 as u64) << 40) | ((s4 as u64) << 34) | ((s5 as u64) << 28) | ((s6 as u64) << 22) | ((s7 as u64) << 16));
        assert(((accum >> 48) as u8) == ((s1 << 4) | (s2 >> 2))) by(bit_vector)
            requires s0 < 64, s1 < 64, s2 < 64, s3 < 64, s4 < 64, s5 < 64, s6 < 64, s7 < 64, accum == (((s0 as u64) << 58) | ((s1 as u64) << 52) | ((s2 as u64) << 46) | ((s3 as u64) << 40) | ((s4 as u64) << 34) | ((s5 as u64) << 28) | ((s6 as u64) << 22) | ((s7 as u64) << 16));
        assert(((accum >> 40) as u8) == ((s2 << 6) | s3)) by(bit_vector)
            requires s0 < 64, s1 < 64, s2 < 64, s3 < 64, s4 < 64, s5 < 64, s6 < 64, s7 < 64, accum == (((s0 as u64) << 58) | ((s1 as u64) << 52) | ((s2 as u64) << 46) | ((s3 as u64) << 40) | ((s4 as u64) << 34) | ((s5 as u64) << 28) | ((s6 as u64) << 22) | ((s7 as u64) << 16));
        assert(((accum >> 32) as u8) == ((s4 << 2) | (s5 >> 4))) by(bit_vector)
            requires s0 < 64, s1 < 64, s2 < 64, s3 < 64, s4 < 64, s5 < 64, s6 < 64, s7 < 64, accum == (((s0 as u64) << 58) | ((s1 as u64) << 52) | ((s2 as u64) << 46) | ((s3 as u64) << 40) | ((s4 as u64) << 34) | ((s5 as u64) << 28) | ((s6 as u64) << 22) | ((s7 as u64) << 16));
        assert(((accum >> 24) as u8) == ((s5 << 4) | (s6 >> 2))) by(bit_vector)
            requires s0 < 64, s1 < 64, s2 < 64, s3 < 64, s4 < 64, s5 < 64, s6 < 64, s7 < 64, accum == (((s0 as u64) << 58) | ((s1 as u64) << 52) | ((s2 as u64) << 46) | ((s3 as u64) << 40) | ((s4 as u64) << 34) | ((s5 as u64) << 28) | ((s6 as u64) << 22) | ((s7 as u64) << 16));
        assert(((accum >> 16) as u8) == ((s6 << 6) | s7)) by(bit_vector)
            requires s0 < 64, s1 < 64, s2 < 64, s3 < 64, s4 < 64, s5 < 64, s6 < 64, s7 < 64, accum == (((s0 as u64) << 58) | ((s1 as u64) << 52) | ((s2 as u64) << 46) | ((s3 as u64) << 40) | ((s4 as u64) << 34) | ((s5 as u64) << 28) | ((s6 as u64) << 22) | ((s7 as u64) << 16));
    }
    }
    // Big-endian representation expansion of accum.to_be_bytes(); see t11 correspondence.
    let accum_bytes = [(accum >> 56) as u8, (accum >> 48) as u8, (accum >> 40) as u8, (accum >> 32) as u8, (accum >> 24) as u8, (accum >> 16) as u8, (accum >> 8) as u8, (accum >> 0) as u8];
    output[..6].copy_from_slice(&accum_bytes[..6]);

    Ok(())
}
}


/// Like [`decode_chunk_8`] but for 4 bytes of input and 3 bytes of output.
verus! {
#[verifier::verify]
#[verus_spec(result =>
    requires input.len() >= 4,
        (exists|i: int| 0 <= i < 4 && decode_table@[input@[i] as int] == 255u8)
            || old(output).len() >= 3,
        forall|i: int| 0 <= i < 4 && decode_table@[input@[i] as int] == 255u8
            ==> index_at_start_of_input as int + i <= usize::MAX,
    ensures
        result.is_ok() <==> (forall|i: int| 0 <= i < 4 ==> decode_table@[input@[i] as int] != 255u8),
        result.is_err() ==> final(output)@ == old(output)@,
        result.is_err() ==> exists|i: int| 0 <= i < 4
            && decode_table@[input@[i] as int] == 255u8
            && (forall|k: int| 0 <= k < i ==> decode_table@[input@[k] as int] != 255u8)
            && result == Err(DecodeError::InvalidByte((index_at_start_of_input as int + i) as usize, input@[i])),
        (forall|i: int| 0 <= i < 4 ==> decode_table@[input@[i] as int] < 64) ==>
            (result == Ok(()) && forall|j: int| 0 <= j < 3 ==> final(output)@[j] == chunk_byte(input@, decode_table@, j)),
        final(output)@.len() == old(output)@.len(),
        forall|i: int| 3 <= i < final(output)@.len() ==> final(output)@[i] == old(output)@[i],
)]
#[inline(always)]
fn decode_chunk_4(
    input: &[u8],
    index_at_start_of_input: usize,
    decode_table: &[u8; 256],
    output: &mut [u8],
) -> Result<(), DecodeError> {
    let morsel = decode_table[usize::from(input[0])];
    let ghost s0 = morsel;
    if morsel == INVALID_VALUE {
        return Err(DecodeError::InvalidByte(index_at_start_of_input, input[0]));
    }
    let mut accum = u32::from(morsel) << 26;

    let morsel = decode_table[usize::from(input[1])];
    let ghost s1 = morsel;
    if morsel == INVALID_VALUE {
        return Err(DecodeError::InvalidByte(
            index_at_start_of_input + 1,
            input[1],
        ));
    }
    accum |= u32::from(morsel) << 20;

    let morsel = decode_table[usize::from(input[2])];
    let ghost s2 = morsel;
    if morsel == INVALID_VALUE {
        return Err(DecodeError::InvalidByte(
            index_at_start_of_input + 2,
            input[2],
        ));
    }
    accum |= u32::from(morsel) << 14;

    let morsel = decode_table[usize::from(input[3])];
    let ghost s3 = morsel;
    if morsel == INVALID_VALUE {
        return Err(DecodeError::InvalidByte(
            index_at_start_of_input + 3,
            input[3],
        ));
    }
    accum |= u32::from(morsel) << 8;

    proof {
      if s0 < 64 && s1 < 64 && s2 < 64 && s3 < 64 {
        assert(((accum >> 24) as u8) == ((s0 << 2) | (s1 >> 4))) by(bit_vector)
            requires s0 < 64, s1 < 64, s2 < 64, s3 < 64, accum == (((s0 as u32) << 26) | ((s1 as u32) << 20) | ((s2 as u32) << 14) | ((s3 as u32) << 8));
        assert(((accum >> 16) as u8) == ((s1 << 4) | (s2 >> 2))) by(bit_vector)
            requires s0 < 64, s1 < 64, s2 < 64, s3 < 64, accum == (((s0 as u32) << 26) | ((s1 as u32) << 20) | ((s2 as u32) << 14) | ((s3 as u32) << 8));
        assert(((accum >> 8) as u8) == ((s2 << 6) | s3)) by(bit_vector)
            requires s0 < 64, s1 < 64, s2 < 64, s3 < 64, accum == (((s0 as u32) << 26) | ((s1 as u32) << 20) | ((s2 as u32) << 14) | ((s3 as u32) << 8));
    }
    }
    // Big-endian representation expansion of accum.to_be_bytes(); see t11 correspondence.
    let accum_bytes = [(accum >> 24) as u8, (accum >> 16) as u8, (accum >> 8) as u8, (accum >> 0) as u8];
    output[..3].copy_from_slice(&accum_bytes[..3]);

    Ok(())
}
}


#[cfg(test)]
mod tests {
    use super::*;

    use crate::engine::general_purpose::STANDARD;

    #[test]
    fn decode_chunk_8_writes_only_6_bytes() {
        let input = b"Zm9vYmFy"; // "foobar"
        let mut output = [0_u8, 1, 2, 3, 4, 5, 6, 7];

        decode_chunk_8(&input[..], 0, &STANDARD.decode_table, &mut output).unwrap();
        assert_eq!(&vec![b'f', b'o', b'o', b'b', b'a', b'r', 6, 7], &output);
    }

    #[test]
    fn decode_chunk_4_writes_only_3_bytes() {
        let input = b"Zm9v"; // "foobar"
        let mut output = [0_u8, 1, 2, 3];

        decode_chunk_4(&input[..], 0, &STANDARD.decode_table, &mut output).unwrap();
        assert_eq!(&vec![b'f', b'o', b'o', 3], &output);
    }

    #[test]
    fn estimate_short_lengths() {
        for (range, decoded_len_estimate) in [
            (0..=0, 0),
            (1..=4, 3),
            (5..=8, 6),
            (9..=12, 9),
            (13..=16, 12),
            (17..=20, 15),
        ] {
            for encoded_len in range {
                let estimate = GeneralPurposeEstimate::new(encoded_len);
                assert_eq!(decoded_len_estimate, estimate.decoded_len_estimate());
            }
        }
    }

    #[test]
    fn estimate_via_u128_inflation() {
        // cover both ends of usize
        (0..1000)
            .chain(usize::MAX - 1000..=usize::MAX)
            .for_each(|encoded_len| {
                // inflate to 128 bit type to be able to safely use the easy formulas
                let len_128 = encoded_len as u128;

                let estimate = GeneralPurposeEstimate::new(encoded_len);
                assert_eq!(
                    (len_128 + 3) / 4 * 3,
                    estimate.conservative_decoded_len as u128
                );
            })
    }
}
