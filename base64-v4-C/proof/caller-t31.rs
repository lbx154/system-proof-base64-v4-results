//! Derived from Engine::encode_slice's prefix-truncating doctest and
//! engine/tests.rs's encode/decode slice composition and suffix observations.
use crate::{engine::general_purpose::STANDARD, Engine, encoded_len, decoded_len_estimate};
use crate::round_trip_spec::*;
use vstd::prelude::*;
verus! {
#[verus_spec(
    requires
        supported(input@.len() as int),
        encoded@.len() >= encoded_size(input@.len() as int),
        decoded@.len() >= decoded_capacity(encoded_size(input@.len() as int)),
    ensures
        final(encoded)@.take(encoded_size(input@.len() as int)) == encoding(input@),
        final(decoded)@.take(input@.len() as int) == input@,
        write_frame(old(encoded)@, final(encoded)@, encoding(input@)),
        write_frame(old(decoded)@, final(decoded)@, input@),
)]
pub fn caller(input: &[u8], encoded: &mut [u8], decoded: &mut [u8]) {
    proof! {
        crate::round_trip_proof::sizes(input.len());
        assert(crate::round_trip_spec::encode_domain(input));
    }
    let capacity = encoded_len(input.len(), true).unwrap();
    assert!(capacity <= encoded.len());
    let ghost input_before = input@;
    let ghost encoded_before = encoded@;
    let ghost decoded_before = decoded@;
    let written = { let tracked mut call_event = Ghost(Seq::<u8>::empty()); #[verus_spec(with Tracked(&mut call_event))] STANDARD.encode_slice(input, encoded) }.unwrap();
    proof! {
        let bytes = choose|bytes: &[u8]| crate::round_trip_spec::observed(input, bytes)
            && crate::round_trip_spec::encode_post(input@, encoded_before, encoded@, written as int);
        assert(crate::round_trip_spec::encode_post(input@, encoded_before, encoded@, written as int));
    }
    assert!(written == capacity);
    // Only the reported prefix is supplied to the decoder.
    let prefix = &encoded[..written];
    let decode_capacity = decoded_len_estimate(prefix.len());
    assert!(decode_capacity <= decoded.len());
    proof! {
        assert(prefix@ == crate::round_trip_spec::encoding(input@));
        assert(crate::round_trip_spec::canonical_input(prefix@));
        assert(crate::decoder_contract::image(input@, prefix@));
    }
    let recovered = { let tracked mut call_event = Ghost(Seq::<u8>::empty()); #[verus_spec(with Tracked(&mut call_event))] STANDARD.decode_slice(prefix, decoded) }.unwrap();
    proof! {
        let observed = choose|observed: &[u8]| crate::round_trip_spec::observed(prefix, observed)
            && (forall|b: Seq<u8>| #[trigger] crate::decoder_contract::image(b, observed@)
                ==> crate::round_trip_spec::decode_post(b, decoded_before, decoded@, recovered as int));
        assert(observed@ == prefix@);
        assert(crate::decoder_contract::image(input@, observed@));
        assert(crate::round_trip_spec::decode_post(input@, decoded_before, decoded@, recovered as int));
    }
    assert!(recovered == input.len());
    assert!(&decoded[..recovered] == input);
    proof! {
        assert(input@ == input_before);
        assert(round_trip_post(input_before, input@, encoded_before, encoded@,
            decoded_before, decoded@, true, true, written as int, recovered as int));
    }
}

}
