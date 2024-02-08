#![no_std]

dharitri_wasm::imports!();

/// Standard module for managing a single DCT.
#[dharitri_wasm::module]
pub trait DctModule {
    #[storage_mapper("token_id")]
    fn token_id(&self) -> SingleValueMapper<TokenIdentifier>;

    #[payable("MOAX")]
    #[endpoint(issueToken)]
    fn issue_token(
        &self,
        token_display_name: ManagedBuffer,
        token_ticker: ManagedBuffer,
        num_decimals: usize,
        #[payment] issue_cost: BigUint,
    ) -> SCResult<AsyncCall> {
        only_owner!(self, "only owner can issue token");
        require!(self.token_id().is_empty(), "Token already issued");

        let initial_supply = self.types().big_uint_from(1u32);

        Ok(self
            .send()
            .dct_system_sc_proxy()
            .issue_fungible(
                issue_cost,
                &token_display_name,
                &token_ticker,
                &initial_supply,
                FungibleTokenProperties {
                    can_burn: false,
                    can_mint: false,
                    num_decimals,
                    can_freeze: true,
                    can_wipe: true,
                    can_pause: true,
                    can_change_owner: false,
                    can_upgrade: false,
                    can_add_special_roles: true,
                },
            )
            .async_call()
            .with_callback(self.callbacks().issue_callback()))
    }

    /// optional address to set roles for. Defaults to SC's address.
    #[endpoint(setLocalRoles)]
    fn set_local_roles(
        &self,
        #[var_args] opt_dest_address: OptionalArg<ManagedAddress>,
    ) -> SCResult<AsyncCall> {
        only_owner!(self, "only owner can set roles");

        let dest_address = match opt_dest_address {
            OptionalArg::Some(addr) => addr,
            OptionalArg::None => self.blockchain().get_sc_address(),
        };
        let token_id = self.token_id().get();
        let roles = [DctLocalRole::Mint, DctLocalRole::Burn];

        Ok(self
            .send()
            .dct_system_sc_proxy()
            .set_special_roles(&dest_address, &token_id, (&roles[..]).into_iter().cloned())
            .async_call())
    }

    #[callback]
    fn issue_callback(&self, #[call_result] result: ManagedAsyncCallResult<TokenIdentifier>) {
        match result {
            ManagedAsyncCallResult::Ok(token_id) => {
                self.token_id().set(&token_id);
            },
            ManagedAsyncCallResult::Err(_) => {
                // return payment to initial caller
                let initial_caller = self.blockchain().get_owner_address();
                let moax_returned = self.call_value().moax_value();
                if moax_returned > 0u32 {
                    self.send()
                        .direct_moax(&initial_caller, &moax_returned, &[]);
                }
            },
        }
    }

    fn mint(&self, amount: &BigUint) -> SCResult<()> {
        let token_id = self.token_id().get();

        self.require_local_roles_set(&token_id)?;
        self.send().dct_local_mint(&token_id, 0, amount);

        Ok(())
    }

    fn burn(&self, amount: &BigUint) -> SCResult<()> {
        let token_id = self.token_id().get();

        self.require_local_roles_set(&token_id)?;
        self.send().dct_local_burn(&token_id, 0, amount);

        Ok(())
    }

    fn require_token_issued(&self) -> SCResult<()> {
        require!(!self.token_id().is_empty(), "Token must be issued first");
        Ok(())
    }

    fn require_local_roles_set(&self, token_id: &TokenIdentifier) -> SCResult<()> {
        let roles = self.blockchain().get_dct_local_roles(token_id);
        require!(
            roles.has_role(&DctLocalRole::Mint) && roles.has_role(&DctLocalRole::Burn),
            "Must set local roles first"
        );
        Ok(())
    }
}
