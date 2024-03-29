use core::marker::PhantomData;

use crate::codec::TopEncodeMulti;

use crate::{
    api::CallTypeApi,
    types::{
        BigUint, MoaxOrDctTokenIdentifier, MoaxOrDctTokenPayment, MoaxOrMultiDctPayment,
        DctTokenPayment, ManagedAddress, ManagedBuffer, ManagedVec, TokenIdentifier,
    },
};

use super::{
    contract_call_exec::UNSPECIFIED_GAS_LIMIT, contract_call_with_moax::ContractCallWithMoax,
    contract_call_with_multi_dct::ContractCallWithMultiDct, ContractCall,
    ContractCallWithAnyPayment, ContractCallWithMoaxOrSingleDct, FunctionCall, ManagedArgBuffer,
};

/// Holds metadata for calling another contract, without payments.
///
/// Proxies generally create contract calls of this type
/// (unless there are payment arguments in the endpoint - but these are mostly obsolete now).
///
/// It is also the basis for all other contract call types, all of them contain this one.
#[must_use]
pub struct ContractCallNoPayment<SA, OriginalResult>
where
    SA: CallTypeApi + 'static,
{
    pub(super) _phantom: PhantomData<SA>,
    pub to: ManagedAddress<SA>,
    pub function_call: FunctionCall<SA>,
    pub explicit_gas_limit: u64,
    pub(super) _return_type: PhantomData<OriginalResult>,
}

impl<SA, OriginalResult> ContractCall<SA> for ContractCallNoPayment<SA, OriginalResult>
where
    SA: CallTypeApi + 'static,
    OriginalResult: TopEncodeMulti,
{
    type OriginalResult = OriginalResult;

    #[inline]
    fn into_normalized(self) -> ContractCallWithMoax<SA, Self::OriginalResult> {
        ContractCallWithMoax {
            basic: self,
            moax_payment: BigUint::zero(),
        }
    }

    #[inline]
    fn get_mut_basic(&mut self) -> &mut ContractCallNoPayment<SA, OriginalResult> {
        self
    }

    fn transfer_execute(self) {
        self.transfer_execute_moax(BigUint::zero());
    }
}

impl<SA, OriginalResult> ContractCallNoPayment<SA, OriginalResult>
where
    SA: CallTypeApi + 'static,
{
    pub fn new<N: Into<ManagedBuffer<SA>>>(to: ManagedAddress<SA>, endpoint_name: N) -> Self {
        ContractCallNoPayment {
            _phantom: PhantomData,
            to,
            function_call: FunctionCall {
                function_name: endpoint_name.into(),
                arg_buffer: ManagedArgBuffer::new(),
            },
            explicit_gas_limit: UNSPECIFIED_GAS_LIMIT,
            _return_type: PhantomData,
        }
    }

    /// Sets payment to be MOAX transfer.
    pub fn with_moax_transfer(
        self,
        moax_amount: BigUint<SA>,
    ) -> ContractCallWithMoax<SA, OriginalResult> {
        ContractCallWithMoax {
            basic: self,
            moax_payment: moax_amount,
        }
    }

    /// Adds a single DCT token transfer to a contract call.
    ///
    /// Can be called multiple times on the same call.
    pub fn with_dct_transfer<P: Into<DctTokenPayment<SA>>>(
        self,
        payment: P,
    ) -> ContractCallWithMultiDct<SA, OriginalResult> {
        let result = ContractCallWithMultiDct {
            basic: self,
            dct_payments: ManagedVec::new(),
        };
        result.with_dct_transfer(payment)
    }

    #[deprecated(
        since = "0.39.0",
        note = "Replace by `contract_call.with_dct_transfer((payment_token, payment_nonce, payment_amount))`. 
        The tuple argument will get automatically converted to DctTokenPayment."
    )]
    pub fn add_dct_token_transfer(
        self,
        payment_token: TokenIdentifier<SA>,
        payment_nonce: u64,
        payment_amount: BigUint<SA>,
    ) -> ContractCallWithMultiDct<SA, OriginalResult> {
        self.with_dct_transfer((payment_token, payment_nonce, payment_amount))
    }

    /// Sets payment to be a (potentially) multi-token transfer.
    #[inline]
    pub fn with_multi_token_transfer(
        self,
        payments: ManagedVec<SA, DctTokenPayment<SA>>,
    ) -> ContractCallWithMultiDct<SA, OriginalResult> {
        ContractCallWithMultiDct {
            basic: self,
            dct_payments: payments,
        }
    }

    /// Sets payment to be a (potentially) multi-token transfer.
    #[inline]
    pub fn with_any_payment(
        self,
        payment: MoaxOrMultiDctPayment<SA>,
    ) -> ContractCallWithAnyPayment<SA, OriginalResult> {
        ContractCallWithAnyPayment {
            basic: self,
            payment,
        }
    }

    /// Sets payment to be either MOAX or a single DCT transfer, as determined at runtime.
    pub fn with_moax_or_single_dct_transfer<P: Into<MoaxOrDctTokenPayment<SA>>>(
        self,
        payment: P,
    ) -> ContractCallWithMoaxOrSingleDct<SA, OriginalResult> {
        ContractCallWithMoaxOrSingleDct {
            basic: self,
            payment: payment.into(),
        }
    }

    #[deprecated(
        since = "0.39.0",
        note = "Replace by `contract_call.with_moax_or_single_dct_transfer((payment_token, payment_nonce, payment_amount))`. "
    )]
    pub fn with_moax_or_single_dct_token_transfer(
        self,
        payment_token: MoaxOrDctTokenIdentifier<SA>,
        payment_nonce: u64,
        payment_amount: BigUint<SA>,
    ) -> ContractCallWithMoaxOrSingleDct<SA, OriginalResult> {
        self.with_moax_or_single_dct_transfer((payment_token, payment_nonce, payment_amount))
    }

    pub fn into_function_call(self) -> FunctionCall<SA> {
        self.function_call
    }
}
