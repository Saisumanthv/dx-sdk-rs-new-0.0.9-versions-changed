// Code generated by the dharitri-wasm multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                            8
// Async Callback (empty):               1
// Total number of exported functions:  10

#![no_std]

dharitri_wasm_node::wasm_endpoints! {
    nft_storage_prepay
    (
        setCostPerByte
        reserveFunds
        claim
        depositPaymentForStorage
        withdraw
        getCostForSize
        getDepositAmount
        getCostPerByte
    )
}

dharitri_wasm_node::wasm_empty_callback! {}
