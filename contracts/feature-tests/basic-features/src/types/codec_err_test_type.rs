use dharitri_wasm::{
    derive::TypeAbi,
    dharitri_codec::{
        DecodeError, EncodeError, NestedDecode, NestedDecodeInput, NestedEncode,
        NestedEncodeOutput, TopDecode, TopDecodeInput, TopEncode, TopEncodeOutput,
    },
};

/// Helper type to explore encode/decode errors.
#[derive(TypeAbi)]
pub struct CodecErrorTestType;

impl TopEncode for CodecErrorTestType {
    #[inline]
    fn top_encode<O: TopEncodeOutput>(&self, _output: O) -> Result<(), EncodeError> {
        Err(EncodeError::from("deliberate top encode error"))
    }
}

impl NestedEncode for CodecErrorTestType {
    fn dep_encode<O: NestedEncodeOutput>(&self, _dest: &mut O) -> Result<(), EncodeError> {
        Err(EncodeError::from("deliberate nested encode error"))
    }
}

impl TopDecode for CodecErrorTestType {
    fn top_decode<I: TopDecodeInput>(_input: I) -> Result<Self, DecodeError> {
        Err(DecodeError::from("deliberate top decode error"))
    }
}

impl NestedDecode for CodecErrorTestType {
    fn dep_decode<I: NestedDecodeInput>(_input: &mut I) -> Result<Self, DecodeError> {
        Err(DecodeError::from("deliberate nested decode error"))
    }
}
