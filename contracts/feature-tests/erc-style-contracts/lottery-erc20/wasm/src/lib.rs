// Code generated by the dharitri-sc multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                            7
// Async Callback:                       1
// Total number of exported functions:   9

#![no_std]
#![allow(internal_features)]
#![feature(lang_items)]

dharitri_sc_wasm_adapter::allocator!(static64k);
dharitri_sc_wasm_adapter::panic_handler!();

dharitri_sc_wasm_adapter::endpoints! {
    lottery_erc20
    (
        init => init
        start => start
        createLotteryPool => create_lottery_pool
        buy_ticket => buy_ticket
        determine_winner => determine_winner
        status => status
        lotteryInfo => get_lottery_info
        erc20ContractManagedAddress => get_erc20_contract_address
    )
}

dharitri_sc_wasm_adapter::async_callback! { lottery_erc20 }
