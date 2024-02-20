// Code generated by the dharitri-sc multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                           27
// Async Callback (empty):               1
// Total number of exported functions:  29

#![no_std]
#![feature(lang_items)]

dharitri_sc_wasm_adapter::allocator!();
dharitri_sc_wasm_adapter::panic_handler!();

dharitri_sc_wasm_adapter::endpoints! {
    abi_tester
    (
        init => init
        echo_abi_test_type => echo_abi_test_type
        echo_enum => echo_enum
        take_managed_type => take_managed_type
        multi_result_3 => multi_result_3
        multi_result_4 => multi_result_4
        var_args => var_args
        multi_result_vec => multi_result_vec
        optional_arg => optional_arg
        optional_result => optional_result
        address_vs_h256 => address_vs_h256
        managed_address_vs_byte_array => managed_address_vs_byte_array
        dct_local_role => dct_local_role
        dct_token_payment => dct_token_payment
        dct_token_data => dct_token_data
        sample_storage_mapper => sample_storage_mapper
        item_for_vec => item_for_vec
        item_for_array_vec => item_for_array_vec
        item_for_managed_vec => item_for_managed_vec
        item_for_array => item_for_array
        item_for_box => item_for_box
        item_for_boxed_slice => item_for_boxed_slice
        item_for_ref => item_for_ref
        item_for_slice => item_for_slice
        item_for_option => item_for_option
        payable_moax => payable_moax
        payable_some_token => payable_some_token
        payable_any_token => payable_any_token
    )
}

dharitri_sc_wasm_adapter::async_callback_empty! {}
