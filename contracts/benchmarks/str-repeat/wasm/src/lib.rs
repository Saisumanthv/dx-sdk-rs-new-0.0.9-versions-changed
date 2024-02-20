// Code generated by the dharitri-sc multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                            3
// Async Callback (empty):               1
// Total number of exported functions:   5

#![no_std]
#![feature(lang_items)]

dharitri_sc_wasm_adapter::allocator!(leaking);
dharitri_sc_wasm_adapter::panic_handler!();

dharitri_sc_wasm_adapter::endpoints! {
    str_repeat
    (
        init => init
        repeat => repeat
        getByteArrayLength => get_byte_array_length
        getByteArray => byte_array
    )
}

dharitri_sc_wasm_adapter::async_callback_empty! {}
