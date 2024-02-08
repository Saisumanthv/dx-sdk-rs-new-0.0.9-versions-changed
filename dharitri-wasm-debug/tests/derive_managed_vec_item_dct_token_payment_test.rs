use dharitri_wasm::{
    api::ManagedTypeApi,
    derive::ManagedVecItem,
    dharitri_codec,
    dharitri_codec::dharitri_codec_derive::{NestedDecode, NestedEncode, TopDecode, TopEncode},
    types::{BigUint, DctTokenPayment, ManagedByteArray, ManagedFrom, TokenIdentifier},
};
use dharitri_wasm_debug::DebugApi;

// to test, run the following command in dharitri-wasm-debug folder:
// cargo expand --test derive_managed_vec_item_dct_token_payment_test > expanded.rs

const ETH_ADDR_WIDTH: usize = 20;

#[derive(
    ManagedVecItem, NestedEncode, NestedDecode, TopEncode, TopDecode, PartialEq, Clone, Debug,
)]
pub struct ManagedStructWithToken<M: ManagedTypeApi> {
    pub token: dharitri_wasm::types::DctTokenPayment<M>,
    pub num: u32,
    pub eth_address_1: ManagedByteArray<M, ETH_ADDR_WIDTH>,
    pub eth_address_2: ManagedByteArray<M, 20>, // const generic also works
}

#[test]
fn struct_with_numbers_static() {
    assert_eq!(
        <ManagedStructWithToken<DebugApi> as dharitri_wasm::types::ManagedVecItem<DebugApi>>::PAYLOAD_SIZE,
        28
    );
    assert!(
        !<ManagedStructWithToken<DebugApi> as dharitri_wasm::types::ManagedVecItem<DebugApi>>::SKIPS_RESERIALIZATION
    );
}

#[test]
fn struct_to_bytes_writer() {
    let api = DebugApi::dummy();
    let s = ManagedStructWithToken::<DebugApi> {
        token: DctTokenPayment {
            token_identifier: TokenIdentifier::managed_from(api.clone(), &b"MYTOKEN-12345"[..]),
            token_nonce: 0u64,
            token_type: dharitri_wasm::types::DctTokenType::Fungible,
            amount: BigUint::managed_from(api.clone(), 42u64),
        },
        num: 0x12345,
        eth_address_1: ManagedByteArray::new_from_bytes(api.clone(), &[1u8; 20]),
        eth_address_2: ManagedByteArray::new_from_bytes(api.clone(), &[2u8; 20]),
    };
    let mut arr: [u8; 28] = [0u8; <ManagedStructWithToken<DebugApi> as dharitri_wasm::types::ManagedVecItem<
        DebugApi,
    >>::PAYLOAD_SIZE];

    <ManagedStructWithToken<DebugApi> as dharitri_wasm::types::ManagedVecItem<DebugApi>>::to_byte_writer(
        &s,
        |bytes| {
            arr[0..<ManagedStructWithToken::<DebugApi> as dharitri_wasm::types::ManagedVecItem<DebugApi>>::PAYLOAD_SIZE].copy_from_slice(bytes);

            assert_eq!(arr, [
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x01, 0x23, 0x45,
                0x00, 0x00, 0x00, 0x01,
                0x00, 0x00, 0x00, 0x02,
            ]);
        },
    );
}

#[test]
fn struct_from_bytes_reader() {
    let api = DebugApi::dummy();
    let s = ManagedStructWithToken::<DebugApi> {
        token: DctTokenPayment {
            token_identifier: TokenIdentifier::managed_from(api.clone(), &b"MYTOKEN-12345"[..]),
            token_nonce: 0u64,
            token_type: dharitri_wasm::types::DctTokenType::Fungible,
            amount: BigUint::managed_from(api.clone(), 42u64),
        },
        num: 0x12345,
        eth_address_1: ManagedByteArray::new_from_bytes(api.clone(), &[1u8; 20]),
        eth_address_2: ManagedByteArray::new_from_bytes(api.clone(), &[2u8; 20]),
    };
    let arr: [u8; 28] = [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x01, 0x23, 0x45, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x02,
    ];

    let struct_from_bytes = <ManagedStructWithToken<DebugApi> as dharitri_wasm::types::ManagedVecItem<
        DebugApi,
    >>::from_byte_reader(DebugApi::dummy(), |bytes| {
        bytes.copy_from_slice(
                    &arr
                        [0
                            ..<ManagedStructWithToken::<DebugApi> as dharitri_wasm::types::ManagedVecItem<
                                DebugApi,
                            >>::PAYLOAD_SIZE],
                );
    });
    assert_eq!(s.num, struct_from_bytes.num);
    assert_eq!(
        s.token.token_identifier,
        struct_from_bytes.token.token_identifier
    );
    assert_eq!(s.token.token_nonce, struct_from_bytes.token.token_nonce);
    assert_eq!(s.token.amount, struct_from_bytes.token.amount);
}
