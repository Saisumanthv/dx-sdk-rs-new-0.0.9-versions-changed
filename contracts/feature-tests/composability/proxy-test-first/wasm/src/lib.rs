// Code generated by the dharitri-sc multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                            6
// Async Callback:                       1
// Total number of exported functions:   8

#![no_std]
#![feature(lang_items)]

dharitri_sc_wasm_adapter::allocator!(static64k);
dharitri_sc_wasm_adapter::panic_handler!();

dharitri_sc_wasm_adapter::endpoints! {
    proxy_test_first
    (
        init => init
        deploySecondContract => deploy_second_contract
        upgradeSecondContract => upgrade_second_contract
        forwardToOtherContract => forward_to_other_contract
        forwardToOtherContractWithCallback => forward_to_other_contract_with_callback
        messageOtherContract => message_other_contract
        messageOtherContractWithCallback => message_other_contract_with_callback
    )
}

dharitri_sc_wasm_adapter::async_callback! { proxy_test_first }
