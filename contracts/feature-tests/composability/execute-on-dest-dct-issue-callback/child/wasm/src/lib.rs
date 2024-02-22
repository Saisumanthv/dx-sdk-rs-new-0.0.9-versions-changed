// Code generated by the dharitri-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                            2
// Async Callback:                       1
// Total number of exported functions:   4

#![no_std]

// Configuration that works with rustc < 1.73.0.
// TODO: Recommended rustc version: 1.73.0 or newer.
#![feature(lang_items)]

dharitri_sc_wasm_adapter::allocator!();
dharitri_sc_wasm_adapter::panic_handler!();

dharitri_sc_wasm_adapter::endpoints! {
    child
    (
        init => init
        issueWrappedMoax => issue_wrapped_moax
        getWrappedMoaxTokenIdentifier => wrapped_moax_token_identifier
    )
}

dharitri_sc_wasm_adapter::async_callback! { child }
