//! Provides the [Engine] abstraction and out of the box implementations.
#[allow(unused_imports)]
use vstd::prelude::*;
use crate::alphabet::Symbol;
use crate::as_ref_spec::AsRefSpec;
#[cfg(any(feature = "alloc", test))]
use crate::chunked_encoder;
use crate::{
    encode::{encode_with_padding, EncodeSliceError},
    encoded_len, DecodeError, DecodeSliceError,
};
#[cfg(any(feature = "alloc", test))]
use alloc::vec::Vec;

#[cfg(any(feature = "alloc", test))]
use alloc::{string::String, vec};

pub mod general_purpose;

#[cfg(all(
    feature = "simd-unsafe",
    any(
        target_arch = "x86_64",
        all(target_arch = "aarch64", target_feature = "neon")
    )
))]
pub mod simd;

#[cfg(test)]
mod naive;

#[cfg(test)]
mod tests;

pub use general_purpose::{GeneralPurpose, GeneralPurposeConfig, Scalar};

/// The runtime-detected SIMD engine. Requires the `simd-unsafe` feature.
#[cfg(all(
    feature = "simd-unsafe",
    feature = "std",
    any(
        target_arch = "x86_64",
        all(target_arch = "aarch64", target_feature = "neon")
    )
))]
pub use simd::Simd;

/// The AVX2 engine. Requires the `simd-unsafe` feature on an `x86_64` target.
#[cfg(all(feature = "simd-unsafe", target_arch = "x86_64"))]
pub use simd::Avx2;

/// The NEON engine. Requires the `simd-unsafe` feature on an `aarch64` target.
#[cfg(all(
    feature = "simd-unsafe",
    target_arch = "aarch64",
    target_feature = "neon"
))]
pub use simd::Neon;

verus! {
/// An `Engine` provides low-level encoding and decoding operations that all other higher-level parts of the API use. Users of the library will generally not need to implement this.
///
/// Different implementations offer different characteristics. The library currently ships with
/// [`GeneralPurpose`] that offers good speed and works on any CPU, with more choices
/// coming later, like a constant-time one when side channel resistance is called for, and vendor-specific vectorized ones for more speed.
///
/// See [`general_purpose::STANDARD_NO_PAD`] if you just want standard base64. Otherwise, when possible, it's
/// recommended to store the engine in a `const` so that references to it won't pose any lifetime
/// issues, and to avoid repeating the cost of engine setup.
///
/// Since almost nobody will need to implement `Engine`, docs for internal methods are hidden.
// When adding an implementation of Engine, include them in the engine test suite:
// - add an implementation of [engine::tests::EngineWrapper]
// - add the implementation to the `all_engines` macro
// All tests run on all engines listed in the macro.
#[verifier::verify]
pub trait Engine: Send + Sync {
    /// True exactly for the selected STANDARD representation (lower construction obligation).
    spec fn is_standard(&self) -> bool;
    /// The config type used by this engine
    type Config: Config;
    /// The decode estimate used by this engine
    type DecodeEstimate: DecodeEstimate;

    /// This is not meant to be called directly; it is only for `Engine` implementors.
    /// See the other `encode*` functions on this trait.
    ///
    /// Encode the `input` bytes into the `output` buffer based on the mapping in `encode_table`.
    ///
    /// `output` will be long enough to hold the encoded data.
    ///
    /// Returns the number of bytes written.
    ///
    /// No padding should be written; that is handled separately.
    ///
    /// Must not write any bytes into the output slice other than the encoded data.
    #[doc(hidden)]
    fn internal_encode(&self, input: &[u8], output: &mut [u8]) -> (result: usize)
        requires self.is_standard(), crate::round_trip_spec::encode_requires(input@, old(output)@),
        ensures
            result == crate::encode_bridge::raw_size(input@.len()),
            crate::round_trip_spec::suffix_unchanged(old(output)@, final(output)@, result as nat),
            final(output)@.subrange(0, result as int) ==
                crate::round_trip_spec::standard_encoding(input@).subrange(0, result as int),
    ;

    /// This is not meant to be called directly; it is only for `Engine` implementors.
    ///
    /// As an optimization to prevent the decoded length from being calculated twice, it is
    /// sometimes helpful to have a conservative estimate of the decoded size before doing the
    /// decoding, so this calculation is done separately and passed to [Engine::decode()] as needed.
    #[doc(hidden)]
    fn internal_decoded_len_estimate(&self, input_len: usize) -> (result: Self::DecodeEstimate)
        ensures result.compatible_with(input_len as nat),
            self.is_standard() ==> result.estimated_len() == crate::round_trip_spec::decoded_estimate(input_len as nat),
    ;

    /// This is not meant to be called directly; it is only for `Engine` implementors.
    /// See the other `decode*` functions on this trait.
    ///
    /// Decode `input` base64 bytes into the `output` buffer.
    ///
    /// `decode_estimate` is the result of [Engine::internal_decoded_len_estimate()], which is passed in to avoid
    /// calculating it again (expensive on short inputs).`
    ///
    /// Each complete 4-byte chunk of encoded data decodes to 3 bytes of decoded data, but this
    /// function must also handle the final possibly partial chunk.
    /// If the input length is not a multiple of 4, or uses padding bytes to reach a multiple of 4,
    /// the trailing 2 or 3 bytes must decode to 1 or 2 bytes, respectively, as per the
    /// [RFC](https://tools.ietf.org/html/rfc4648#section-3.5).
    ///
    /// Decoding must not write any bytes into the output slice other than the decoded data.
    ///
    /// Non-canonical trailing bits in the final symbols or non-canonical padding must be reported as
    /// errors unless the engine is configured otherwise.
    #[doc(hidden)]
    fn internal_decode(
        &self,
        input: &[u8],
        output: &mut [u8],
        decode_estimate: Self::DecodeEstimate,
    ) -> (result: Result<DecodeMetadata, DecodeSliceError>)
        requires decode_estimate.compatible_with(input@.len()),
        ensures
            self.is_standard() && decode_estimate.estimated_len() == crate::round_trip_spec::decoded_estimate(input@.len())
                && crate::round_trip_spec::round_trip_decode_requires(input@, old(output)@) ==>
                match result {
                    Ok(dm) => crate::round_trip_spec::round_trip_decode_contract(input@, input@, old(output)@, final(output)@, Ok::<usize, DecodeSliceError>(dm.written())),
                    Err(_) => false,
                },
    ;

    /// Returns the config for this engine.
    fn config(&self) -> (result: &Self::Config)
        ensures self.is_standard() ==> result.padding_enabled();

    /// Encode arbitrary octets as base64 using the provided `Engine`.
    /// Returns a `String`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use base64::{Engine as _, engine::{self, general_purpose}, alphabet};
    ///
    /// let b64 = general_purpose::STANDARD.encode(b"hello world~");
    /// println!("{}", b64);
    ///
    /// const CUSTOM_ENGINE: engine::GeneralPurpose =
    ///     engine::GeneralPurpose::new(&alphabet::URL_SAFE, general_purpose::NO_PAD);
    ///
    /// let b64_url = CUSTOM_ENGINE.encode(b"hello internet~");
    /// ```
    #[cfg(any(feature = "alloc", test))]
    #[inline]
    #[verifier::external_body]
    fn encode<T: AsRef<[u8]>>(&self, input: T) -> String {
        fn inner<E>(engine: &E, input_bytes: &[u8]) -> String
        where
            E: Engine + ?Sized,
        {
            let encoded_size = encoded_len(input_bytes.len(), engine.config().encode_padding())
                .expect("integer overflow when calculating buffer size");

            let mut buf = vec![0; encoded_size];

            encode_with_padding(input_bytes, &mut buf[..], engine, encoded_size);

            String::from_utf8(buf).expect("Invalid UTF8")
        }

        inner(self, input.as_ref())
    }

    /// Encode arbitrary octets as base64 into a supplied `String`.
    /// Writes into the supplied `String`, which may allocate if its internal buffer isn't big enough.
    ///
    /// # Example
    ///
    /// ```rust
    /// use base64::{Engine as _, engine::{self, general_purpose}, alphabet};
    /// const CUSTOM_ENGINE: engine::GeneralPurpose =
    ///     engine::GeneralPurpose::new(&alphabet::URL_SAFE, general_purpose::NO_PAD);
    ///
    /// fn main() {
    ///     let mut buf = String::new();
    ///     general_purpose::STANDARD.encode_string(b"hello world~", &mut buf);
    ///     println!("{}", buf);
    ///
    ///     buf.clear();
    ///     CUSTOM_ENGINE.encode_string(b"hello internet~", &mut buf);
    ///     println!("{}", buf);
    /// }
    /// ```
    #[cfg(any(feature = "alloc", test))]
    #[inline]
    #[verifier::external_body]
    fn encode_string<T: AsRef<[u8]>>(&self, input: T, output_buf: &mut String) {
        fn inner<E>(engine: &E, input_bytes: &[u8], output_buf: &mut String)
        where
            E: Engine + ?Sized,
        {
            let mut sink = chunked_encoder::StringSink::new(output_buf);

            chunked_encoder::ChunkedEncoder::new(engine)
                .encode(input_bytes, &mut sink)
                .expect("Writing to a String shouldn't fail");
        }

        inner(self, input.as_ref(), output_buf);
    }

    /// Encode arbitrary octets as base64 into a supplied slice.
    /// Writes into the supplied output buffer.
    ///
    /// This is useful if you wish to avoid allocation entirely (e.g. encoding into a stack-resident
    /// or statically-allocated buffer).
    ///
    /// # Example
    ///
    #[cfg_attr(feature = "alloc", doc = "```")]
    #[cfg_attr(not(feature = "alloc"), doc = "```ignore")]
    /// use base64::{Engine as _, engine::general_purpose};
    /// let s = b"hello internet!";
    /// let mut buf = Vec::new();
    /// // make sure we'll have a slice big enough for base64 + padding
    /// buf.resize(s.len() * 4 / 3 + 4, 0);
    ///
    /// let bytes_written = general_purpose::STANDARD.encode_slice(s, &mut buf).unwrap();
    ///
    /// // shorten our vec down to just what was written
    /// buf.truncate(bytes_written);
    ///
    /// assert_eq!(s, general_purpose::STANDARD.decode(&buf).unwrap().as_slice());
    /// ```
    #[inline]
    #[verus_spec(result =>
        with -> observation: Ghost<(Seq<u8>, Seq<u8>, Seq<u8>)>
        requires self.is_standard(),
            AsRefSpec::conversion_environment(&input),
            AsRefSpec::cleanup_environment(&input),
        ensures
            crate::as_ref_spec::returned_bytes(&input, observation@.0),
            observation@.1 == old(output_buf)@,
            observation@.2 == final(output_buf)@,
            match result { Ok(k) => k <= final(output_buf)@.len(), Err(_) => true },
            self.is_standard() && crate::round_trip_spec::encode_requires(observation@.0, observation@.1) ==>
                crate::round_trip_spec::encode_contract(observation@.0, observation@.0, observation@.1, final(output_buf)@, result),
    )]
    fn encode_slice<T: AsRef<[u8]>>(
        &self,
        input: T,
        output_buf: &mut [u8],
    ) -> Result<usize, EncodeSliceError> {
        let input_bytes = as_ref_bytes(&input);
        let ghost input_snapshot = input_bytes@;
        let ghost output_snapshot = output_buf@;
        proof {
            crate::as_ref_spec::byte_slice_valid(input_bytes);
            crate::as_ref_spec::byte_slice_valid(output_buf);
        }
        let encoded_size = encoded_len(input_bytes.len(), self.config().encode_padding())
            .expect("usize overflow when calculating buffer size");
        proof {
            if self.is_standard() && crate::round_trip_spec::encode_requires(input_snapshot, output_snapshot) {
                assert(encoded_size == crate::round_trip_spec::encoded_size(input_snapshot.len()));
                assert(output_buf@.len() >= encoded_size);
            }
        }
        let result = if output_buf.len() < encoded_size {
            Err(EncodeSliceError::OutputSliceTooSmall)
        } else {
            proof {
                crate::round_trip_proof::capacity_domain(input_snapshot.len(), crate::round_trip_spec::byte_slice_max());
                assert(crate::round_trip_spec::supported(input_snapshot.len()));
                crate::encode_bridge::raw_capacity(input_snapshot.len());
            }
            let b64_output = &mut output_buf[0..encoded_size];
            let ghost b64_before = b64_output@;
            proof {
                if self.is_standard() && crate::round_trip_spec::encode_requires(input_snapshot, output_snapshot) {
                    assert(crate::round_trip_spec::encode_requires(input_snapshot, b64_before));
                }
            }
            // Behavior-preserving expansion of encode_with_padding at its actual caller.
            debug_assert_eq!(encoded_size, b64_output.len());
            let b64_bytes_written = self.internal_encode(input_bytes, b64_output);
            let padding_bytes = if self.config().encode_padding() {
                crate::encode::add_padding(
                    b64_bytes_written,
                    self.padding(),
                    &mut b64_output[b64_bytes_written..],
                )
            } else {
                0
            };
            let encoded_bytes = b64_bytes_written.checked_add(padding_bytes)
                .expect("usize overflow when calculating b64 length");
            debug_assert_eq!(encoded_size, encoded_bytes);
            proof {
                if self.is_standard() && crate::round_trip_spec::encode_requires(input_snapshot, output_snapshot) {
                    assert(b64_output@ == crate::round_trip_spec::standard_encoding(input_snapshot));
                }
            }
            Ok(encoded_size)
        };
        let ghost codec_done = output_buf@;
        crate::as_ref_spec::complete_owned_cleanup(input, output_buf);
        proof_with!(|= Ghost((input_snapshot, output_snapshot, codec_done)));
        result
    }

    /// Decode the input into a new `Vec`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use base64::{Engine as _, alphabet, engine::{self, general_purpose}};
    ///
    /// let bytes = general_purpose::STANDARD
    ///     .decode("aGVsbG8gd29ybGR+Cg==").unwrap();
    /// println!("{:?}", bytes);
    ///
    /// // custom engine setup
    /// let bytes_url = engine::GeneralPurpose::new(
    ///              &alphabet::URL_SAFE,
    ///              general_purpose::NO_PAD)
    ///     .decode("aGVsbG8gaW50ZXJuZXR-Cg").unwrap();
    /// println!("{:?}", bytes_url);
    /// ```
    #[cfg(any(feature = "alloc", test))]
    #[inline]
    #[verifier::external_body]
    fn decode<T: AsRef<[u8]>>(&self, input: T) -> Result<Vec<u8>, DecodeError> {
        fn inner<E>(engine: &E, input_bytes: &[u8]) -> Result<Vec<u8>, DecodeError>
        where
            E: Engine + ?Sized,
        {
            let estimate = engine.internal_decoded_len_estimate(input_bytes.len());
            let mut buffer = vec![0; estimate.decoded_len_estimate()];

            let bytes_written = engine
                .internal_decode(input_bytes, &mut buffer, estimate)
                .map_err(|e| match e {
                    DecodeSliceError::DecodeError(e) => e,
                    DecodeSliceError::OutputSliceTooSmall => {
                        unreachable!("Vec is sized conservatively")
                    }
                })?
                .decoded_len;

            buffer.truncate(bytes_written);

            Ok(buffer)
        }

        inner(self, input.as_ref())
    }

    /// Decode the `input` into the supplied `buffer`.
    ///
    /// Writes into the supplied `Vec`, which may allocate if its internal buffer isn't big enough.
    /// Returns a `Result` containing an empty tuple, aka `()`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use base64::{Engine as _, alphabet, engine::{self, general_purpose}};
    /// const CUSTOM_ENGINE: engine::GeneralPurpose =
    ///     engine::GeneralPurpose::new(&alphabet::URL_SAFE, general_purpose::PAD);
    ///
    /// fn main() {
    ///     use base64::Engine;
    ///     let mut buffer = Vec::<u8>::new();
    ///     // with the default engine
    ///     general_purpose::STANDARD
    ///         .decode_vec("aGVsbG8gd29ybGR+Cg==", &mut buffer,).unwrap();
    ///     println!("{:?}", buffer);
    ///
    ///     buffer.clear();
    ///
    ///     // with a custom engine
    ///     CUSTOM_ENGINE.decode_vec(
    ///         "aGVsbG8gaW50ZXJuZXR-Cg==",
    ///         &mut buffer,
    ///     ).unwrap();
    ///     println!("{:?}", buffer);
    /// }
    /// ```
    #[cfg(any(feature = "alloc", test))]
    #[inline]
    #[verifier::external_body]
    fn decode_vec<T: AsRef<[u8]>>(
        &self,
        input: T,
        buffer: &mut Vec<u8>,
    ) -> Result<(), DecodeError> {
        fn inner<E>(engine: &E, input_bytes: &[u8], buffer: &mut Vec<u8>) -> Result<(), DecodeError>
        where
            E: Engine + ?Sized,
        {
            let starting_output_len = buffer.len();
            let estimate = engine.internal_decoded_len_estimate(input_bytes.len());

            let total_len_estimate = estimate
                .decoded_len_estimate()
                .checked_add(starting_output_len)
                .expect("Overflow when calculating output buffer length");

            buffer.resize(total_len_estimate, 0);

            let buffer_slice = &mut buffer.as_mut_slice()[starting_output_len..];

            let bytes_written = engine
                .internal_decode(input_bytes, buffer_slice, estimate)
                .map_err(|e| match e {
                    DecodeSliceError::DecodeError(e) => e,
                    DecodeSliceError::OutputSliceTooSmall => {
                        unreachable!("Vec is sized conservatively")
                    }
                })?
                .decoded_len;

            buffer.truncate(starting_output_len + bytes_written);

            Ok(())
        }

        inner(self, input.as_ref(), buffer)
    }

    /// Decode the input into the provided output slice.
    ///
    /// Returns the number of bytes written to the slice, or an error if `output` is smaller than
    /// the estimated decoded length.
    ///
    /// This will not write any bytes past exactly what is decoded (no stray garbage bytes at the end).
    ///
    /// See [`crate::decoded_len_estimate`] for calculating buffer sizes.
    ///
    /// See [`Engine::decode_slice_unchecked`] for a version that panics instead of returning an error
    /// if the output buffer is too small.
    #[inline]
    #[verus_spec(result =>
        with -> observation: Ghost<(Seq<u8>, Seq<u8>, Seq<u8>)>
        requires AsRefSpec::conversion_environment(&input),
            AsRefSpec::cleanup_environment(&input),
        ensures
            crate::as_ref_spec::returned_bytes(&input, observation@.0),
            observation@.1 == old(output)@,
            observation@.2 == final(output)@,
            self.is_standard() && crate::round_trip_spec::round_trip_decode_requires(observation@.0, observation@.1) ==>
                crate::round_trip_spec::round_trip_decode_contract(observation@.0, observation@.0, observation@.1, final(output)@, result),
    )]
    fn decode_slice<T: AsRef<[u8]>>(
        &self,
        input: T,
        output: &mut [u8],
    ) -> Result<usize, DecodeSliceError>
    {
        let input_bytes = as_ref_bytes(&input);
        let ghost encoded_snapshot = input_bytes@;
        let ghost output_snapshot = output@;
        let decoded = self
            .internal_decode(
                input_bytes,
                output,
                self.internal_decoded_len_estimate(input_bytes.len()),
            );
        let result = match decoded {
            Ok(dm) => Ok(dm.decoded_len),
            Err(error) => Err(error),
        };
        let ghost codec_done = output@;
        crate::as_ref_spec::complete_owned_cleanup(input, output);
        proof_with!(|= Ghost((encoded_snapshot, output_snapshot, codec_done)));
        result
    }

    /// Decode the input into the provided output slice.
    ///
    /// Returns the number of bytes written to the slice.
    ///
    /// This will not write any bytes past exactly what is decoded (no stray garbage bytes at the end).
    ///
    /// See [`crate::decoded_len_estimate`] for calculating buffer sizes.
    ///
    /// See [`Engine::decode_slice`] for a version that returns an error instead of panicking if the output
    /// buffer is too small.
    ///
    /// # Panics
    ///
    /// Panics if the provided output buffer is too small for the decoded data.
    #[inline]
    #[verifier::external_body]
    fn decode_slice_unchecked<T: AsRef<[u8]>>(
        &self,
        input: T,
        output: &mut [u8],
    ) -> Result<usize, DecodeError> {
        fn inner<E>(engine: &E, input_bytes: &[u8], output: &mut [u8]) -> Result<usize, DecodeError>
        where
            E: Engine + ?Sized,
        {
            engine
                .internal_decode(
                    input_bytes,
                    output,
                    engine.internal_decoded_len_estimate(input_bytes.len()),
                )
                .map(|dm| dm.decoded_len)
                .map_err(|e| match e {
                    DecodeSliceError::DecodeError(e) => e,
                    DecodeSliceError::OutputSliceTooSmall => {
                        panic!("Output slice is too small")
                    }
                })
        }

        inner(self, input.as_ref(), output)
    }

    /// Returns the symbol used for encode padding.
    ///
    /// Typically this is `'='`, but weird alphabets may use other values.
    fn padding(&self) -> (result: Symbol)
        ensures self.is_standard() ==> result.byte() == 61u8,
    ;
}

}

/// The minimal level of configuration that engines must support.
verus! {
/// Minimal configuration with a ghost padding observation.
#[verifier::verify]
pub trait Config {
    /// Ghost observation of the actual padding configuration.
    spec fn padding_enabled(&self) -> bool;
    /// Returns `true` if padding should be added after the encoded output.
    ///
    /// Padding is added outside the engine's `encode()` since the engine may be used
    /// to encode only a chunk of the overall output, so it can't always know when
    /// the output is "done" and would therefore need padding (if configured).
    // It could be provided as a separate parameter when encoding, but that feels like
    // leaking an implementation detail to the user, and it's hopefully more convenient
    // to have to only pass one thing (the engine) to any part of the API.
    fn encode_padding(&self) -> (result: bool)
        ensures result == self.padding_enabled();
}

}

/// The decode estimate used by an engine implementation. Users do not need to interact with this;
/// it is only for engine implementors.
///
/// Implementors may store relevant data here when constructing this to avoid having to calculate
/// them again during actual decoding.
verus! {
/// Numeric observation of an engine's conservative decode estimate.
#[verifier::verify]
pub trait DecodeEstimate {
    /// Ghost numeric observation of the actual estimate.
    spec fn estimated_len(&self) -> nat;
    /// The estimate has the remainder and capacity computed for this input length.
    spec fn compatible_with(&self, input_len: nat) -> bool;
    /// Returns a conservative (err on the side of too big) estimate of the decoded length to use
    /// for pre-allocating buffers, etc.
    ///
    /// The estimate must be no larger than the next largest complete triple of decoded bytes.
    /// That is, the final quad of symbols to decode may be assumed to be complete with no padding.
    fn decoded_len_estimate(&self) -> (result: usize)
        ensures result as nat == self.estimated_len(),
    ;
}

}

/// Controls how pad bytes are handled when decoding.
///
/// Each [Engine] must support at least the behavior indicated by
/// [`DecodePaddingMode::RequireCanonical`], and may support other modes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[verifier::verify]
pub enum DecodePaddingMode {
    /// Canonical padding is allowed, but any fewer padding bytes than that is also allowed.
    Indifferent,
    /// Padding must be canonical (0, 1, or 2 `=` as needed to produce a 4 byte suffix).
    RequireCanonical,
    /// Padding must be absent -- for when you want predictable padding, without any wasted bytes.
    RequireNone,
}

/// Metadata about the result of a decode operation
#[derive(PartialEq, Eq, Debug)]
#[verifier::verify]
pub struct DecodeMetadata {
    /// Number of decoded bytes output
    pub(crate) decoded_len: usize,
    /// Offset of the first padding byte in the input, if any
    pub(crate) padding_offset: Option<usize>,
}

verus! {
impl DecodeMetadata {
    pub(crate) fn new(decoded_bytes: usize, padding_index: Option<usize>) -> (result: Self)
        ensures result.decoded_len == decoded_bytes, result.padding_offset == padding_index,
    {
        Self {
            decoded_len: decoded_bytes,
            padding_offset: padding_index,
        }
    }
}
}

// Verification-only extraction of the single external user conversion call.
#[verifier::verify]
#[verus_spec(result =>
    requires AsRefSpec::conversion_environment(input),
    ensures AsRefSpec::as_ref_post(input, result))]
fn as_ref_bytes<T: AsRef<[u8]>>(input: &T) -> &[u8] {
    <T as AsRef<[u8]>>::as_ref(input)
}

verus! {
impl DecodeMetadata {
    /// Decoded-length observation used by the internal decode contract.
    pub closed spec fn written(&self) -> usize { self.decoded_len }
}
}
