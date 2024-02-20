// Code generated by the dharitri-sc multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                           26
// Async Callback:                       1
// Total number of exported functions:  28

#![no_std]
#![feature(lang_items)]

dharitri_sc_wasm_adapter::allocator!(static64k);
dharitri_sc_wasm_adapter::panic_handler!();

dharitri_sc_wasm_adapter::endpoints! {
    rust_testing_framework_tester
    (
        init => init
        sum => sum
        sum_sc_result => sum_sc_result
        get_caller_legacy => get_caller_legacy
        get_moax_balance => get_moax_balance
        get_dct_balance => get_dct_balance
        receive_moax => receive_moax
        recieve_moax_half => recieve_moax_half
        receive_dct => receive_dct
        reject_payment => reject_payment
        receive_dct_half => receive_dct_half
        receive_multi_dct => receive_multi_dct
        send_nft => send_nft
        mint_dct => mint_dct
        burn_dct => burn_dct
        create_nft => create_nft
        get_block_epoch => get_block_epoch
        get_block_nonce => get_block_nonce
        get_block_timestamp => get_block_timestamp
        get_random_buffer_once => get_random_buffer_once
        get_random_buffer_twice => get_random_buffer_twice
        call_other_contract_execute_on_dest => call_other_contract_execute_on_dest
        call_other_contract_add_async_call => call_other_contract_add_async_call
        getTotalValue => get_total_value
        execute_on_dest_add_value => execute_on_dest_add_value
        addValue => add
        panic => panic
    )
}

dharitri_sc_wasm_adapter::async_callback! { rust_testing_framework_tester }
