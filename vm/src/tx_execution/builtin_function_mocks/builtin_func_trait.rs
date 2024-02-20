use crate::{
    tx_execution::BlockchainVMRef,
    tx_mock::{BlockchainUpdate, TxCache, TxInput, TxResult, TxTokenTransfer},
    types::VMAddress,
};

pub trait BuiltinFunction {
    /// Function name corresponding the builtin function implementation.
    ///
    /// Currently not used.
    fn name(&self) -> &str;

    /// Extracts data relating DCT transfers handled by the builtin function, if applicable.
    fn extract_dct_transfers(&self, tx_input: &TxInput) -> BuiltinFunctionDctTransferInfo {
        BuiltinFunctionDctTransferInfo::empty(tx_input)
    }

    /// Executes builtin function for the givn `TxInput` and with access to the underlying contracts states via the `TxCache`.
    ///
    /// A few builtin functions (the ones transferring DCT) can also call the VM after they finish,
    /// so they are given the extra reference to the VM and a lambda closure to execute on the VM
    fn execute<F>(
        &self,
        tx_input: TxInput,
        tx_cache: TxCache,
        vm: &BlockchainVMRef,
        lambda: F,
    ) -> (TxResult, BlockchainUpdate)
    where
        F: FnOnce();
}

/// Contains a builtin function call DCT transfers (if any) and the real recipient of the transfer
/// (can be different from the "to" field.)
pub struct BuiltinFunctionDctTransferInfo {
    pub real_recipient: VMAddress,
    pub transfers: Vec<TxTokenTransfer>,
}

impl BuiltinFunctionDctTransferInfo {
    pub fn empty(tx_input: &TxInput) -> Self {
        BuiltinFunctionDctTransferInfo {
            real_recipient: tx_input.to.clone(),
            transfers: Vec::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.transfers.is_empty()
    }
}
