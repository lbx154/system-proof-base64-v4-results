//! Source-grounded diagnostic consumers of the public contracts.
use vstd::prelude::*;
use crate::{Engine, DecodeSliceError};
use crate::round_trip_spec::*;
/// Consume canonical decoder observations from an actual borrowed-slice call.
#[verus_spec(result =>
    requires round_trip_decode_requires(input@, old(output)@),
    ensures round_trip_decode_contract(input@, input@, old(output)@, final(output)@, result),
)]
pub fn borrowed_decode_use(input: &[u8], output: &mut [u8]) -> Result<usize, DecodeSliceError> {
    let engine = crate::engine::general_purpose::STANDARD;
    proof_decl! { let ghost mut observation = (Seq::<u8>::empty(), Seq::<u8>::empty(), Seq::<u8>::empty()); }
    let result = { proof_with! { => Ghost(observation) } engine.decode_slice(input, output) };
    proof! { assert(observation.0 == input@); }
    result
}

/// Complete source-grounded use with sufficient public-helper-sized destinations.
#[verus_spec(result =>
    requires
        supported(input@.len()),
        old(encoded_output)@.len() >= encoded_size(input@.len()),
        old(decoded_output)@.len() >= decoded_estimate(encoded_size(input@.len())),
    ensures
        encode_post(input@, input@, old(encoded_output)@, final(encoded_output)@, encoded_size(input@.len())),
        decode_contract(input@, standard_encoding(input@), standard_encoding(input@),
            old(decoded_output)@, final(decoded_output)@, result),
)]
pub fn helper_sized_round_trip(input: &[u8], encoded_output: &mut [u8], decoded_output: &mut [u8]) -> Result<usize, DecodeSliceError> {
    let engine = crate::engine::general_purpose::STANDARD;
    proof! {
        crate::as_ref_spec::byte_slice_valid(input);
        crate::as_ref_spec::byte_slice_valid(encoded_output);
        crate::as_ref_spec::byte_slice_valid(decoded_output);
        crate::round_trip_proof::capacity_domain(input@.len(), byte_slice_max());
    }
    let encode_capacity = crate::encoded_len(input.len(), true).expect("supported encode capacity");
    let decode_capacity = crate::decoded_len_estimate(encode_capacity);
    proof! {
        assert(encode_capacity as nat == encoded_size(input@.len()));
        assert(decode_capacity as nat == decoded_estimate(encoded_size(input@.len())));
        assert(encode_capacity <= encoded_output@.len());
        assert(decode_capacity <= decoded_output@.len());
        assert(encode_requires(input@, encoded_output@));
    }
    proof_decl! {
        let ghost encode_before = encoded_output@;
        let ghost decode_before = decoded_output@;
        let ghost mut encoded_observation = (Seq::<u8>::empty(), Seq::<u8>::empty(), Seq::<u8>::empty());
    }
    let encode_result = { proof_with! { => Ghost(encoded_observation) } engine.encode_slice(input, encoded_output) };
    proof! {
        assert(encoded_observation.0 == input@);
        assert(encode_contract(input@, input@, encode_before, encoded_output@, encode_result));
    }
    let written = match encode_result {
        Ok(written) => written,
        Err(_) => return Err(DecodeSliceError::OutputSliceTooSmall),
    };
    proof! {
        assert(encode_post(input@, input@, encode_before, encoded_output@, written as nat));
        crate::round_trip_proof::encoded_prefix_establishes_decode(input@, encode_before, encoded_output@, decoded_output@, written as nat);
    }
    let encoded_prefix = &encoded_output[..written];
    proof_decl! { let ghost mut decoded_observation = (Seq::<u8>::empty(), Seq::<u8>::empty(), Seq::<u8>::empty()); }
    let decode_result = { proof_with! { => Ghost(decoded_observation) } engine.decode_slice(encoded_prefix, decoded_output) };
    proof! {
        assert(decoded_observation.0 == encoded_prefix@);
        assert(round_trip_decode_contract(encoded_prefix@, encoded_prefix@, decode_before, decoded_output@, decode_result));
        crate::round_trip_proof::decoding_recovers_encode_input(input@, encoded_prefix@, decode_before, decoded_output@, decode_result);
    }
    decode_result
}

/// Owned generic consumption. Capacity premises are evaluated on THIS call's
/// returned observation, never on a second conversion or every possible return.
#[verus_spec(result =>
    with -> effective: Ghost<Seq<u8>>
    requires
        crate::as_ref_spec::AsRefSpec::conversion_environment(&input),
        crate::as_ref_spec::AsRefSpec::cleanup_environment(&input),
    ensures
        crate::as_ref_spec::returned_bytes(&input, effective@),
        encode_requires(effective@, old(encoded_output)@)
        && old(decoded_output)@.len() >= decoded_estimate(encoded_size(effective@.len())) ==>
            encode_post(effective@, effective@, old(encoded_output)@,
                final(encoded_output)@, encoded_size(effective@.len()))
            && decode_contract(effective@, standard_encoding(effective@), standard_encoding(effective@),
                old(decoded_output)@, final(decoded_output)@, result),
)]
pub fn owned_generic_round_trip<T: AsRef<[u8]>>(input: T, encoded_output: &mut [u8], decoded_output: &mut [u8]) -> Result<usize, DecodeSliceError> {
    proof! {
        crate::as_ref_spec::byte_slice_valid(encoded_output);
        crate::as_ref_spec::byte_slice_valid(decoded_output);
    }
    let engine = crate::engine::general_purpose::STANDARD;
    proof_decl! {
        let ghost encode_before = encoded_output@;
        let ghost decode_before = decoded_output@;
        let ghost mut observation = (Seq::<u8>::empty(), Seq::<u8>::empty(), Seq::<u8>::empty());
    }
    let encode_result = { proof_with! { => Ghost(observation) } engine.encode_slice(input, encoded_output) };
    let result = match encode_result {
        Err(_) => Err(DecodeSliceError::OutputSliceTooSmall),
        Ok(written) => {
            let decode_capacity = crate::decoded_len_estimate(written);
            proof! {
                if encode_requires(observation.0, encode_before)
                    && decode_before.len() >= decoded_estimate(encoded_size(observation.0.len())) {
                    assert(written as nat == encoded_size(observation.0.len()));
                    assert(decode_capacity as nat == decoded_estimate(encoded_size(observation.0.len())));
                    crate::round_trip_proof::encoded_prefix_establishes_decode(observation.0,
                        encode_before, encoded_output@, decoded_output@, written as nat);
                }
            }
            let encoded_prefix = &encoded_output[..written];
            proof_decl! { let ghost mut decoded_observation = (Seq::<u8>::empty(), Seq::<u8>::empty(), Seq::<u8>::empty()); }
            let decoded = { proof_with! { => Ghost(decoded_observation) } engine.decode_slice(encoded_prefix, decoded_output) };
            proof! {
                assert(decoded_observation.0 == encoded_prefix@);
                if encode_requires(observation.0, encode_before)
                    && decode_before.len() >= decoded_estimate(encoded_size(observation.0.len())) {
                    crate::round_trip_proof::decoding_recovers_encode_input(observation.0,
                        encoded_prefix@, decode_before, decoded_output@, decoded);
                }
            }
            decoded
        },
    };
    proof_decl! { let ghost effective_input = observation.0; }
    proof_with!(|= Ghost(effective_input));
    result
}
