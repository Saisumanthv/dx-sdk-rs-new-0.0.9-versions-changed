use dharitri_wasm::{
    api::{EndpointFinishApi, ManagedTypeApi, SendApi},
    io::EndpointResult,
    types::{
        AsyncCall, BigUint, BoxedBytes, CodeMetadata, ManagedAddress, ManagedBuffer,
        OptionalResult, SendMoax, Vec,
    },
};

dharitri_wasm::derive_imports!();

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub enum Action<M: ManagedTypeApi> {
    Nothing,
    AddBoardMember(ManagedAddress<M>),
    AddProposer(ManagedAddress<M>),
    RemoveUser(ManagedAddress<M>),
    ChangeQuorum(usize),
    SendMoax {
        to: ManagedAddress<M>,
        amount: BigUint<M>,
        data: BoxedBytes,
    },
    SCDeploy {
        amount: BigUint<M>,
        code: ManagedBuffer<M>,
        code_metadata: CodeMetadata,
        arguments: Vec<BoxedBytes>,
    },
    SCCall {
        to: ManagedAddress<M>,
        moax_payment: BigUint<M>,
        endpoint_name: BoxedBytes,
        arguments: Vec<BoxedBytes>,
    },
}

impl<M: ManagedTypeApi> Action<M> {
    /// Only pending actions are kept in storage,
    /// both executed and discarded actions are removed (converted to `Nothing`).
    /// So this is equivalent to `action != Action::Nothing`.
    pub fn is_pending(&self) -> bool {
        !matches!(*self, Action::Nothing)
    }
}

/// Not used internally, just to retrieve results via endpoint.
#[derive(TopEncode, TypeAbi)]
pub struct ActionFullInfo<M: ManagedTypeApi> {
    pub action_id: usize,
    pub action_data: Action<M>,
    pub signers: Vec<ManagedAddress<M>>,
}

#[derive(TypeAbi)]
pub enum PerformActionResult<SA>
where
    SA: SendApi + ManagedTypeApi + 'static,
{
    Nothing,
    SendMoax(SendMoax<SA>),
    DeployResult(ManagedAddress<SA>),
    SendAsyncCall(AsyncCall<SA>),
}

impl<SA> EndpointResult for PerformActionResult<SA>
where
    SA: SendApi + Clone + 'static,
{
    type DecodeAs = OptionalResult<ManagedAddress<SA>>;

    fn finish<FA>(&self, api: FA)
    where
        FA: ManagedTypeApi + EndpointFinishApi + Clone + 'static,
    {
        match self {
            PerformActionResult::Nothing => (),
            PerformActionResult::SendMoax(send_moax) => send_moax.finish(api),
            PerformActionResult::DeployResult(address) => address.finish(api),
            PerformActionResult::SendAsyncCall(async_call) => async_call.finish(api),
        }
    }
}

#[cfg(test)]
mod test {
    use dharitri_wasm_debug::TxContext;

    use super::Action;

    #[test]
    fn test_is_pending() {
        assert!(!Action::<TxContext>::Nothing.is_pending());
        assert!(Action::<TxContext>::ChangeQuorum(5).is_pending());
    }
}
