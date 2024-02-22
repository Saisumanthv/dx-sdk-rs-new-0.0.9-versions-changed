// Code generated by the dharitri-sc multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                           13
// Async Callback (empty):               1
// Total number of exported functions:  15

#![no_std]

// Configuration that works with rustc < 1.73.0.
// TODO: Recommended rustc version: 1.73.0 or newer.
#![feature(lang_items)]

dharitri_sc_wasm_adapter::allocator!();
dharitri_sc_wasm_adapter::panic_handler!();

dharitri_sc_wasm_adapter::endpoints! {
    rewards_distribution
    (
        init => init
        depositRoyalties => deposit_royalties
        raffle => raffle
        claimRewards => claim_rewards
        computeClaimableAmount => compute_claimable_amount
        getRaffleId => raffle_id
        getCompletedRaffleIdCount => completed_raffle_id_count
        getRoyalties => royalties
        getNftRewardPercent => nft_reward_percent
        getWasClaimed => was_claimed
        getSeedNftMinterAddress => seed_nft_minter_address
        getBrackets => brackets
        getLastRaffleEpoch => last_raffle_epoch
        getNftTokenId => nft_token_id
    )
}

dharitri_sc_wasm_adapter::async_callback_empty! {}
