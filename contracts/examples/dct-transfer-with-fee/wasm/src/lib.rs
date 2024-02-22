// Code generated by the dharitri-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                            6
// Async Callback (empty):               1
// Total number of exported functions:   8

#![no_std]

// Configuration that works with rustc < 1.73.0.
// TODO: Recommended rustc version: 1.73.0 or newer.
#![feature(lang_items)]

dharitri_sc_wasm_adapter::allocator!();
dharitri_sc_wasm_adapter::panic_handler!();

dharitri_sc_wasm_adapter::endpoints! {
    dct_transfer_with_fee
    (
        init => init
        setExactValueFee => set_exact_value_fee
        setPercentageFee => set_percentage_fee
        claimFees => claim_fees
        transfer => transfer
        getTokenFee => token_fee
        getPaidFees => paid_fees
    )
}

dharitri_sc_wasm_adapter::async_callback_empty! {}
