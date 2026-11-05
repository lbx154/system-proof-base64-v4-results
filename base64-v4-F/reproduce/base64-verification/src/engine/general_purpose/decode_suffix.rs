use vstd::prelude::*;
use crate::alphabet::Symbol;
use crate::{
    engine::{general_purpose::INVALID_VALUE, DecodeMetadata, DecodePaddingMode},
    DecodeError, DecodeSliceError,
};

verus! {
/// Number of non-padding symbols in a canonical terminal group.
pub(crate) open spec fn symbol_count(t: Seq<u8>) -> nat {
    if t.len() == 0 { 0 } else { t.len() + 1 }
}
pub(crate) open spec fn tail_value(t: Seq<u8>, k: int) -> u8 {
    let a = crate::round_trip_spec::byte_or_zero(t, 0);
    let b = crate::round_trip_spec::byte_or_zero(t, 1);
    let c = crate::round_trip_spec::byte_or_zero(t, 2);
    if k == 0 { (a / 4) as u8 }
    else if k == 1 { ((a % 4) * 16 + b / 16) as u8 }
    else if k == 2 { ((b % 16) * 4 + c / 64) as u8 }
    else { (c % 64) as u8 }
}
pub(crate) open spec fn tail_image(t: Seq<u8>, input: Seq<u8>, ii: nat) -> bool {
    t.len() <= 3 && ii <= input.len()
    && input.subrange(ii as int, input.len() as int) == crate::round_trip_spec::standard_encoding(t)
    && (input.len() == 0 <==> t.len() == 0)
}
pub(crate) proof fn canonical_tail(t: Seq<u8>, table: Seq<u8>)
    requires t.len() <= 3, table.len() == 256,
        forall|v: nat| v < 64 ==> table[crate::round_trip_spec::standard_symbol(v) as int] == v as u8,
    ensures
        crate::round_trip_spec::standard_encoding(t).len() == if t.len() == 0 { 0nat } else { 4nat },
        forall|k: int| 0 <= k < symbol_count(t) ==> #[trigger] tail_value(t,k) < 64
            && crate::round_trip_spec::standard_encoding(t)[k] == crate::round_trip_spec::standard_symbol(tail_value(t,k) as nat)
            && crate::round_trip_spec::standard_encoding(t)[k] != 61
            && table[crate::round_trip_spec::standard_encoding(t)[k] as int] == tail_value(t,k),
        forall|k: int| symbol_count(t) <= k < 4 ==> #[trigger] tail_value(t,k) == 0,
        forall|k: int| symbol_count(t) <= k < crate::round_trip_spec::standard_encoding(t).len() ==> crate::round_trip_spec::standard_encoding(t)[k] == 61,
{
    let a = crate::round_trip_spec::byte_or_zero(t,0);
    let b = crate::round_trip_spec::byte_or_zero(t,1);
    let c = crate::round_trip_spec::byte_or_zero(t,2);
    assert(a <= 255 && b <= 255 && c <= 255);
    assert forall|k: int| 0 <= k < symbol_count(t) implies #[trigger] tail_value(t,k) < 64
        && crate::round_trip_spec::standard_encoding(t)[k] == crate::round_trip_spec::standard_symbol(tail_value(t,k) as nat)
        && crate::round_trip_spec::standard_encoding(t)[k] != 61
        && table[crate::round_trip_spec::standard_encoding(t)[k] as int] == tail_value(t,k) by {
        assert(0 <= k < 4 && k / 4 == 0 && k % 4 == k);
        if k == 0 {} else if k == 1 {} else if k == 2 {} else {}
        let v = tail_value(t,k) as nat;
        assert(v < 64);
        assert(crate::round_trip_spec::standard_symbol(v) != 61) by {
            if v < 26 {} else if v < 52 {} else if v < 62 {} else if v == 62 {} else {}
        }
    }
    assert forall|k: int| symbol_count(t) <= k < 4 implies #[trigger] tail_value(t,k) == 0 by {
        if t.len() == 0 {} else if t.len() == 1 {} else if t.len() == 2 {} else {}
    }
    assert forall|k: int| symbol_count(t) <= k < crate::round_trip_spec::standard_encoding(t).len() implies crate::round_trip_spec::standard_encoding(t)[k] == 61 by {
        assert(0 <= k < 4 && k / 4 == 0 && k % 4 == k);
        if t.len() == 1 {} else if t.len() == 2 {} else {}
    }
}

pub(crate) open spec fn terminal_input_index(x: Seq<u8>) -> nat {
    if x.len() == 0 { 0 } else { (crate::round_trip_spec::encoded_size(x.len()) - 4) as nat }
}
pub(crate) open spec fn terminal_output_index(x: Seq<u8>) -> nat {
    3 * (terminal_input_index(x) / 4)
}
pub(crate) open spec fn terminal_tail(x: Seq<u8>) -> Seq<u8> {
    x.subrange(terminal_output_index(x) as int, x.len() as int)
}
/// Derive local membership from the preceding canonical encode observation.
pub(crate) proof fn canonical_suffix_image(x: Seq<u8>)
    ensures
        terminal_input_index(x) <= crate::round_trip_spec::standard_encoding(x).len(),
        crate::round_trip_spec::standard_encoding(x).len() - terminal_input_index(x) <= 4,
        terminal_input_index(x) % 4 == 0,
        terminal_output_index(x) <= x.len(),
        terminal_tail(x).len() <= 3,
        tail_image(terminal_tail(x), crate::round_trip_spec::standard_encoding(x), terminal_input_index(x)),
{
    let ii = terminal_input_index(x);
    let oi = terminal_output_index(x);
    let t = terminal_tail(x);
    let e = crate::round_trip_spec::standard_encoding(x);
    if x.len() == 0 {
        assert(ii == 0 && oi == 0 && t.len() == 0 && e.len() == 0);
    } else {
        if x.len() % 3 == 0 {} else if x.len() % 3 == 1 {} else {}
        assert(ii % 4 == 0 && oi <= x.len() && 1 <= x.len() - oi <= 3);
        assert(t.len() == x.len() - oi);
        assert(crate::round_trip_spec::standard_encoding(t).len() == 4);
        assert forall|k: nat| k < 3 implies #[trigger] crate::round_trip_spec::byte_or_zero(t,k) == crate::round_trip_spec::byte_or_zero(x,oi+k) by {
            if k < t.len() { assert(t[k as int] == x[(oi+k) as int]); } else {}
        }
        assert forall|j: int| 0 <= j < 4 implies e[ii as int+j] == crate::round_trip_spec::standard_encoding(t)[j] by {
            assert((ii+j) / 4 == ii / 4 && (ii+j) % 4 == j);
            assert(j / 4 == 0 && j % 4 == j);
            assert(3 * ((ii+j)/4) == oi);
            assert(crate::round_trip_spec::byte_or_zero(t,0) == crate::round_trip_spec::byte_or_zero(x,oi));
            assert(crate::round_trip_spec::byte_or_zero(t,1) == crate::round_trip_spec::byte_or_zero(x,oi+1));
            assert(crate::round_trip_spec::byte_or_zero(t,2) == crate::round_trip_spec::byte_or_zero(x,oi+2));
            if j == 0 {} else if j == 1 {} else if j == 2 {} else {}
        }
    }
    assert(e.subrange(ii as int,e.len() as int) =~= crate::round_trip_spec::standard_encoding(t));
}

pub(crate) open spec fn tail_word(t: Seq<u8>) -> u32 {
    ((crate::round_trip_spec::byte_or_zero(t,0) as u32) << 24)
    | ((crate::round_trip_spec::byte_or_zero(t,1) as u32) << 16)
    | ((crate::round_trip_spec::byte_or_zero(t,2) as u32) << 8)
}
pub(crate) proof fn pack_tail(t: Seq<u8>)
    requires t.len() <= 3,
    ensures
        (((tail_value(t,0) as u32) << 26) | ((tail_value(t,1) as u32) << 20)
            | ((tail_value(t,2) as u32) << 14) | ((tail_value(t,3) as u32) << 8)) == tail_word(t),
        (tail_word(t) & (!0u32 >> (t.len() * 8))) == 0,
{
    let a = crate::round_trip_spec::byte_or_zero(t,0) as u8;
    let b = crate::round_trip_spec::byte_or_zero(t,1) as u8;
    let c = crate::round_trip_spec::byte_or_zero(t,2) as u8;
    assert(a as nat == crate::round_trip_spec::byte_or_zero(t,0));
    assert(b as nat == crate::round_trip_spec::byte_or_zero(t,1));
    assert(c as nat == crate::round_trip_spec::byte_or_zero(t,2));
    let s0 = tail_value(t,0);
    let s1 = tail_value(t,1);
    let s2 = tail_value(t,2);
    let s3 = tail_value(t,3);
    assert(s0 == ((a as u32) / 4) as u8);
    assert(s1 == (((a as u32) % 4) * 16 + (b as u32) / 16) as u8);
    assert(s2 == (((b as u32) % 16) * 4 + (c as u32) / 64) as u8);
    assert(s3 == ((c as u32) % 64) as u8);
    assert((((s0 as u32) << 26) | ((s1 as u32) << 20) | ((s2 as u32) << 14) | ((s3 as u32) << 8))
        == (((a as u32) << 24) | ((b as u32) << 16) | ((c as u32) << 8))) by(bit_vector)
        requires s0 == ((a as u32) / 4) as u8,
            s1 == (((a as u32) % 4) * 16 + (b as u32) / 16) as u8,
            s2 == (((b as u32) % 16) * 4 + (c as u32) / 64) as u8,
            s3 == ((c as u32) % 64) as u8;
    let w = tail_word(t);
    assert(w == (((a as u32) << 24) | ((b as u32) << 16) | ((c as u32) << 8)));
    if t.len() == 0 {
        assert(a == 0 && b == 0 && c == 0);
        assert(w == 0) by(bit_vector) requires a == 0, b == 0, c == 0,
            w == (((a as u32) << 24) | ((b as u32) << 16) | ((c as u32) << 8));
        assert(w & (!0u32 >> 0) == 0) by(bit_vector) requires w == 0;
    } else if t.len() == 1 {
        assert(b == 0 && c == 0);
        assert(w & (!0u32 >> 8) == 0) by(bit_vector) requires b == 0, c == 0,
            w == (((a as u32) << 24) | ((b as u32) << 16) | ((c as u32) << 8));
    } else if t.len() == 2 {
        assert(c == 0);
        assert(w & (!0u32 >> 16) == 0) by(bit_vector) requires c == 0,
            w == (((a as u32) << 24) | ((b as u32) << 16) | ((c as u32) << 8));
    } else {
        assert(w & (!0u32 >> 24) == 0) by(bit_vector) requires
            w == (((a as u32) << 24) | ((b as u32) << 16) | ((c as u32) << 8));
    }
}
pub(crate) proof fn next_tail_byte(t: Seq<u8>, j: usize, w: u32)
    requires t.len() <= 3, j < t.len(), w == (tail_word(t) << (j * 8)),
    ensures (w >> 24) <= 255, (w >> 24) as u8 == t[j as int],
        (w << 8) == (tail_word(t) << ((j + 1) * 8)),
{
    let a = crate::round_trip_spec::byte_or_zero(t,0) as u8;
    let b = crate::round_trip_spec::byte_or_zero(t,1) as u8;
    let c = crate::round_trip_spec::byte_or_zero(t,2) as u8;
    assert((w >> 24) <= 255) by(bit_vector);
    let original = tail_word(t);
    assert(original == (((a as u32) << 24) | ((b as u32) << 16) | ((c as u32) << 8)));
    if j == 0 {
        assert((w >> 24) as u8 == a) by(bit_vector) requires j == 0, w == (original << (j * 8)),
            original == (((a as u32) << 24) | ((b as u32) << 16) | ((c as u32) << 8));
    } else if j == 1 {
        assert((w >> 24) as u8 == b) by(bit_vector) requires j == 1, w == (original << (j * 8)),
            original == (((a as u32) << 24) | ((b as u32) << 16) | ((c as u32) << 8));
    } else {
        assert(j == 2);
        assert((w >> 24) as u8 == c) by(bit_vector) requires j == 2, w == (original << (j * 8)),
            original == (((a as u32) << 24) | ((b as u32) << 16) | ((c as u32) << 8));
    }
    assert((w << 8) == (original << ((j + 1) * 8))) by(bit_vector)
        requires j < 3, w == (original << (j * 8));
}

/// Conditional canonical case; arbitrary dispatch needs no terminal witness.
pub(crate) open spec fn canonical_use(t: Seq<u8>, input: Seq<u8>, ii: nat,
    table: Seq<u8>, padding: u8, mode: DecodePaddingMode) -> bool {
    tail_image(t, input, ii) && padding == 61
    && mode == DecodePaddingMode::RequireCanonical
    && forall|v: nat| v < 64 ==> table[crate::round_trip_spec::standard_symbol(v) as int] == v as u8
}

/// Decode the last 0-4 bytes, checking for trailing set bits and padding per the provided
/// parameters.
///
/// Returns the decode metadata representing the total number of bytes decoded, including the ones
/// indicated as already written by `output_index`.
#[allow(clippy::too_many_arguments)]
#[verifier::verify]
#[verus_spec(result =>
    with Ghost(t): Ghost<Seq<u8>>
    requires input_index <= input.len(), input.len() - input_index <= 4,
        output_index <= old(output)@.len(),
        canonical_use(t, input@, input_index as nat, decode_table@, padding.byte(), padding_mode)
            ==> output_index as nat + t.len() <= old(output)@.len(),
    ensures final(output)@.len() == old(output)@.len(),
        (match result { Ok(m) => output_index <= m.decoded_len <= final(output)@.len()
            && m.decoded_len as nat <= output_index as nat + 3
            && (forall|j: int| 0 <= j < final(output)@.len()
                && (j < output_index || m.decoded_len <= j)
                ==> final(output)@[j] == old(output)@[j]), Err(_) => true }),
        canonical_use(t, input@, input_index as nat, decode_table@, padding.byte(), padding_mode) ==> match result {
        Ok(m) => m.decoded_len as nat == output_index as nat + t.len()
            && final(output)@.len() == old(output)@.len()
            && (forall|j: int| 0 <= j < t.len() ==> final(output)@[output_index as int + j] == t[j])
            && (forall|j: int| 0 <= j < final(output)@.len() && (j < output_index || output_index as nat + t.len() <= j) ==> final(output)@[j] == old(output)@[j])
            && m.padding_offset == if 0 < t.len() < 3 { Some((input_index as nat + symbol_count(t)) as usize) } else { None },
        Err(_) => false,
    },
)]
pub(crate) fn decode_suffix(
    input: &[u8],
    input_index: usize,
    output: &mut [u8],
    mut output_index: usize,
    decode_table: &[u8; 256],
    decode_allow_trailing_bits: bool,
    padding: Symbol,
    padding_mode: DecodePaddingMode,
) -> Result<DecodeMetadata, DecodeSliceError>
{
    let ghost initial_output_index = output_index;
    let ghost canonical = canonical_use(t, input@, input_index as nat, decode_table@, padding.byte(), padding_mode);
    proof { if canonical { canonical_tail(t, decode_table@); } }
    debug_assert!((input.len() - input_index) <= 4);

    // Decode any leftovers that might not be a complete input chunk of 4 bytes.
    // Use a u32 as a stack-resident 4 byte buffer.
    let mut morsels_in_leftover = 0;
    let mut padding_bytes_count = 0;
    // offset from input_index
    let mut first_padding_offset: usize = 0;
    let mut last_symbol = 0_u8;
    let mut last_symbol_value = 0_u8;
    let mut morsels = [0_u8; 4];

    let mut leftover_index = 0usize;
    while leftover_index < input.len() - input_index
        invariant
            canonical == canonical_use(t, input@, input_index as nat, decode_table@, padding.byte(), padding_mode),
            canonical ==> tail_image(t, input@, input_index as nat),
            canonical ==> (padding.byte() == 61 && padding_mode == DecodePaddingMode::RequireCanonical),
            canonical ==> forall|v: nat| v < 64 ==> decode_table@[crate::round_trip_spec::standard_symbol(v) as int] == v as u8,
            decode_table.len() == 256,
            output@ == old(output)@,
            output_index == initial_output_index,
            canonical ==> output_index as nat + t.len() <= output@.len(),
            canonical ==> morsels_in_leftover == if leftover_index < symbol_count(t) { leftover_index as nat } else { symbol_count(t) },
            canonical ==> padding_bytes_count == if leftover_index <= symbol_count(t) { 0nat } else { (leftover_index as nat - symbol_count(t)) as nat },
            canonical ==> padding_bytes_count > 0 ==> first_padding_offset == symbol_count(t),
            canonical ==> forall|j: int| 0 <= j < morsels_in_leftover ==> morsels@[j] == tail_value(t,j),
            input_index <= input.len(), input.len() - input_index <= 4,
            leftover_index <= input.len() - input_index,
            morsels_in_leftover + padding_bytes_count == leftover_index,
            padding_bytes_count > 0 ==> 2 <= first_padding_offset < leftover_index,
            padding_bytes_count > 0 ==> morsels_in_leftover == first_padding_offset,
            forall|j: int| morsels_in_leftover <= j < 4 ==> morsels@[j] == 0,
            output_index <= output.len(),
            morsels.len() == 4,
        decreases input.len() - input_index - leftover_index,
    {
        proof { if canonical { canonical_tail(t, decode_table@); } }
        let b = input[input_index + leftover_index];
        proof { if canonical {
            assert(b == crate::round_trip_spec::standard_encoding(t)[leftover_index as int]);
            if leftover_index < symbol_count(t) {
                assert(b != 61 && decode_table@[b as int] == tail_value(t,leftover_index as int));
            } else { assert(b == 61); }
        } }
        // '=' padding
        if b == padding.as_u8() {
            // There can be bad padding bytes in a few ways:
            // 1 - Padding with non-padding characters after it
            // 2 - Padding after zero or one characters in the current quad (should only
            //     be after 2 or 3 chars)
            // 3 - More than two characters of padding. If 3 or 4 padding chars
            //     are in the same quad, that implies it will be caught by #2.
            //     If it spreads from one quad to another, it will be an invalid byte
            //     in the first quad.
            // 4 - Non-canonical padding -- 1 byte when it should be 2, etc.
            //     Per config, non-canonical but still functional non- or partially-padded base64
            //     may be treated as an error condition.

            if leftover_index < 2 {
                // Check for error #2.
                // Either the previous byte was padding, in which case we would have already hit
                // this case, or it wasn't, in which case this is the first such error.
                debug_assert!(
                    leftover_index == 0 || (leftover_index == 1 && padding_bytes_count == 0)
                );
                let bad_padding_index = input_index + leftover_index;
                return Err(DecodeError::InvalidByte(bad_padding_index, b).into());
            }

            if padding_bytes_count == 0 {
                first_padding_offset = leftover_index;
            }

            padding_bytes_count += 1;
            leftover_index += 1;
            continue;
        }

        // Check for case #1.
        // To make '=' handling consistent with the main loop, don't allow
        // non-suffix '=' in trailing chunk either. Report error as first
        // erroneous padding.
        if padding_bytes_count > 0 {
            return Err(DecodeError::InvalidByte(
                input_index + first_padding_offset,
                padding.as_u8(),
            )
            .into());
        }

        last_symbol = b;

        // can use up to 8 * 6 = 48 bits of the u64, if last chunk has no padding.
        // Pack the leftovers from left to right.
        let morsel = decode_table[b as usize];
        last_symbol_value = morsel;
        if morsel == INVALID_VALUE {
            return Err(DecodeError::InvalidByte(input_index + leftover_index, b).into());
        }

        morsels[morsels_in_leftover] = morsel;
        morsels_in_leftover += 1;
        leftover_index += 1;
    }

    proof { if canonical { canonical_tail(t, decode_table@); } }
    // If there was 1 trailing byte, and it was valid, and we got to this point without hitting
    // an invalid byte, now we can report invalid length
    if !input.is_empty() && morsels_in_leftover < 2 {
        return Err(DecodeError::InvalidLength(input_index + morsels_in_leftover).into());
    }

    match padding_mode {
        DecodePaddingMode::Indifferent => { /* everything we care about was already checked */ }
        DecodePaddingMode::RequireCanonical => {
            // allow empty input
            if (padding_bytes_count + morsels_in_leftover) % 4 != 0 {
                return Err(DecodeError::InvalidPadding.into());
            }
        }
        DecodePaddingMode::RequireNone => {
            if padding_bytes_count > 0 {
                // check at the end to make sure we let the cases of padding that should be InvalidByte
                // get hit
                return Err(DecodeError::InvalidPadding.into());
            }
        }
    }

    // When encoding 1 trailing byte (e.g. 0xFF), 2 base64 bytes ("/w") are needed.
    // / is the symbol for 63 (0x3F, bottom 6 bits all set) and w is 48 (0x30, top 2 bits
    // of bottom 6 bits set).
    // When decoding two symbols back to one trailing byte, any final symbol higher than
    // w would still decode to the original byte because we only care about the top two
    // bits in the bottom 6, but would be a non-canonical encoding. So, we calculate a
    // mask based on how many bits are used for just the canonical encoding, and optionally
    // error if any other bits are set. In the example of one encoded byte -> 2 symbols,
    // 2 symbols can technically encode 12 bits, but the last 4 are non-canonical, and
    // useless since there are no more symbols to provide the necessary 4 additional bits
    // to finish the second original byte.

    let leftover_bytes_to_append = morsels_in_leftover * 6 / 8;
    // Put the up to 6 complete bytes as the high bytes.
    // Gain a couple percent speedup from nudging these ORs to use more ILP with a two-way split.
    let mut leftover_num = (u32::from(morsels[0]) << 26)
        | (u32::from(morsels[1]) << 20)
        | (u32::from(morsels[2]) << 14)
        | (u32::from(morsels[3]) << 8);

    // if there are bits set outside the bits we care about, last symbol encodes trailing bits that
    // will not be included in the output
    let mask = !0_u32 >> (leftover_bytes_to_append * 8);
    let ghost m0 = morsels@[0];
    let ghost m1 = morsels@[1];
    let ghost m2 = morsels@[2];
    let ghost m3 = morsels@[3];
    proof { if canonical {
        assert(morsels_in_leftover == symbol_count(t));
        assert(leftover_bytes_to_append == t.len());
        assert forall|k: int| 0 <= k < 4 implies morsels@[k] == tail_value(t,k) by {
            if k < morsels_in_leftover {} else {}
        }
        pack_tail(t);
        assert(leftover_num == tail_word(t));
        assert(leftover_num & mask == 0);
        assert(leftover_num == (leftover_num << 0)) by(bit_vector);
    }
        if morsels_in_leftover == 0 {
            assert(m0 == 0 && m1 == 0 && m2 == 0 && m3 == 0);
            assert(leftover_num == 0) by(bit_vector) requires m0 == 0, m1 == 0, m2 == 0, m3 == 0,
                leftover_num == ((m0 as u32) << 26) | ((m1 as u32) << 20) | ((m2 as u32) << 14) | ((m3 as u32) << 8);
            assert(leftover_num & mask == 0) by(bit_vector) requires leftover_num == 0;
        }
    }
    if !decode_allow_trailing_bits && (leftover_num & mask) != 0 {
        // last morsel is at `morsels_in_leftover` - 1
        return Err(DecodeError::InvalidLastSymbol {
            offset: input_index + morsels_in_leftover - 1,
            symbol: last_symbol,
            symbol_value: last_symbol_value,
        }
        .into());
    }

    // Strangely, this approach benchmarks better than writing bytes one at a time,
    // or copy_from_slice into output.
    for appended in 0..leftover_bytes_to_append
        invariant
            canonical == canonical_use(t, input@, input_index as nat, decode_table@, padding.byte(), padding_mode),
            output_index == initial_output_index + appended,
            output_index <= output@.len(),
            forall|j: int| 0 <= j < output@.len() && (j < initial_output_index || initial_output_index as nat + 3 <= j) ==> output@[j] == old(output)@[j],
            leftover_bytes_to_append <= 3,
            canonical ==> initial_output_index as nat + t.len() <= output@.len(),
            canonical ==> leftover_bytes_to_append == t.len() && t.len() <= 3,
            canonical ==> leftover_num == (tail_word(t) << (appended * 8)),
            output@.len() == old(output)@.len(), output@.len() <= usize::MAX,
            canonical ==> forall|j: int| 0 <= j < appended ==> output@[initial_output_index as int + j] == t[j],
            forall|j: int| 0 <= j < output@.len() && (j < initial_output_index || initial_output_index as nat + appended <= j) ==> output@[j] == old(output)@[j],
            canonical ==> (padding_bytes_count > 0 <==> 0 < t.len() < 3),
            canonical ==> (padding_bytes_count > 0 ==> first_padding_offset == symbol_count(t)),
            padding_bytes_count > 0 ==> first_padding_offset < input.len() - input_index,
            input_index <= input.len(), input.len() - input_index <= 4,
            canonical ==> symbol_count(t) <= input.len() - input_index,

    {
        proof { if canonical { next_tail_byte(t, appended, leftover_num); } }
        proof { assert((leftover_num >> 24) <= 255) by(bit_vector); }
        let hi_byte = (leftover_num >> 24) as u8;
        leftover_num <<= 8;
        proof {
            if canonical { assert(output_index < output@.len()); }
        }
        *output
            .get_mut(output_index)
            .ok_or(DecodeSliceError::OutputSliceTooSmall)? = hi_byte;
        proof { assert(output_index < output@.len()); }
        output_index += 1;
    }

    Ok(DecodeMetadata::new(
        output_index,
        if padding_bytes_count > 0 {
            Some(input_index + first_padding_offset)
        } else {
            None
        },
    ))
}

}
