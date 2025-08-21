use bascoin_consensus_core::errors::tx::TxRuleError;
use bascoin_consensus_core::tx::{TransactionId, TransactionOutpoint};
use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum RuleError {
    /// A consensus transaction rule error
    ///
    /// Note that following variants are converted:
    ///
    /// - TxRuleError::ImmatureCoinbaseSpend => RuleError::RejectImmatureSpend
    /// - TxRuleError::MissingTxOutpoints => RuleError::RejectMissingOutpoint
    #[error(transparent)]
    RejectTxRule(TxRuleError),

    #[error("at least one outpoint of transaction is lacking a matching UTXO entry")]
    RejectMissingOutpoint,

    #[error("transaction {0} was already accepted by the consensus")]
    RejectAlreadyAccepted(TransactionId),

    #[error("transaction {0} is already in the mempool")]
    RejectDuplicate(TransactionId),

    #[error("output {0} already spent by transaction {1} in the mempool")]
    RejectDoubleSpendInMempool(TransactionOutpoint, TransactionId),

    #[error("replace by fee found no double spending transaction in the mempool")]
    RejectRbfNoDoubleSpend,

    #[error("replace by fee found more than one double spending transaction in the mempool")]
    RejectRbfTooManyDoubleSpendingTransactions,

    /// a transaction is rejected if the mempool is full
    #[error("transaction could not be added to the mempool because it's full with transactions with higher priority")]
    RejectMempoolIsFull,

    /// An error emitted by mining\src\mempool\check_transaction_standard.rs
    #[error("transaction {0} is not standard: {1}")]
    RejectNonStandard(TransactionId, String),

    #[error("one of the transaction inputs spends an immature UTXO: {0}")]
    RejectImmatureSpend(TxRuleError),

    #[error("transaction {0} doesn't exist in transaction pool")]
    RejectMissingTransaction(TransactionId),

    #[error("orphan transaction size of {0} bytes is larger than max allowed size of {1} bytes")]
    RejectBadOrphanMass(u64, u64),

    #[error("orphan transaction {0} is already in the orphan pool")]
    RejectDuplicateOrphan(TransactionId),

    #[error("orphan transaction {0} is double spending an input from already existing orphan {1}")]
    RejectDoubleSpendOrphan(TransactionId, TransactionId),

    #[error("transaction {0} is an orphan where orphan is disallowed")]
    RejectDisallowedOrphan(TransactionId),

    #[error("input No. {0} of {1} ({2}) doesn't exist in orphan_ids_by_previous_outpoint")]
    RejectMissingOrphanOutpoint(usize, TransactionId, TransactionOutpoint),

    #[error("transaction {0} doesn't exist in orphan pool")]
    RejectMissingOrphanTransaction(TransactionId),

    /// New behavior: a transaction is rejected if the orphan pool is full
    #[error("number of high-priority transactions in orphan pool ({0}) has reached the maximum allowed ({1})")]
    RejectOrphanPoolIsFull(usize, u64),

    #[error("transactions in mempool form a cycle")]
    RejectCycleInMempoolTransactions,

    // TODO: This error is added for the tx_relay flow but is never constructed neither in the golang nor in this version. Discuss if it can be removed.
    #[error("transaction {0} is invalid")]
    RejectInvalid(TransactionId),

    #[error("Rejected tx {0} from mempool due to incomputable storage mass")]
    RejectStorageMassIncomputable(TransactionId),
}

impl From<NonStandardError> for RuleError {
    fn from(item: NonStandardError) -> Self {
        RuleError::RejectNonStandard(*item.transaction_id(), item.to_string())
    }
}

impl From<TxRuleError> for RuleError {
    fn from(item: TxRuleError) -> Self {
        match item {
            TxRuleError::ImmatureCoinbaseSpend(_, _, _, _, _) => RuleError::RejectImmatureSpend(item),
            TxRuleError::MissingTxOutpoints => RuleError::RejectMissingOutpoint,
            _ => RuleError::RejectTxRule(item),
        }
    }
}

pub type RuleResult<T> = std::result::Result<T, RuleError>;

#[derive(Error, Debug, Clone)]
pub enum MempoolError {
    #[error("The transaction has already been accepted to the DAG: {0}")]
    RejectAlreadyAccepted(TransactionId),
    #[error("The transaction is a duplicate of a transaction already in the mempool: {0}")]
    RejectDuplicate(TransactionId),
    #[error("The transaction {1} attempts to spend the same outpoint {0} that is already spent by a transaction in the mempool")]
    RejectDoubleSpendInMempool(TransactionOutpoint, TransactionId),
    #[error("replace by fee found no double spending transaction in the mempool")]
    RejectRbfNoDoubleSpend,
    #[error("replace by fee found more than one double spending transaction in the mempool")]
    RejectRbfTooManyDoubleSpendingTransactions,
    /// a transaction is rejected if the mempool is full
    #[error("transaction could not be added to the mempool because it's full with transactions with higher priority")]
    RejectMempoolIsFull,
    /// An error emitted by mining\src\mempool\check_transaction_standard.rs
    #[error("Transaction {0} is not standard: {1}")]
    RejectNonStandard(TransactionId, String),
    #[error("one of the transaction inputs spends an immature UTXO: {0}")]
    RejectImmatureSpend(TxRuleError),
    #[error("Missing transaction {0}")]
    RejectMissingTransaction(TransactionId),
    #[error("orphan transaction size of {0} bytes is larger than max allowed size of {1} bytes")]
    RejectBadOrphanMass(u64, u64),
    #[error("Rejecting orphan {0} that is already in the orphan pool")]
    RejectDuplicateOrphan(TransactionId),
    #[error("Rejecting orphan {0} that attempts to spend the same outpoint as orphan {1}")]
    RejectDoubleSpendOrphan(TransactionId, TransactionId),
    #[error("Rejecting orphan {0} that is not allowed to enter the orphan pool")]
    RejectDisallowedOrphan(TransactionId),
    #[error("Missing orphan outpoint: index {0} transaction {1} outpoint {2}")]
    RejectMissingOrphanOutpoint(usize, TransactionId, TransactionOutpoint),
    #[error("Missing orphan transaction {0}")]
    RejectMissingOrphanTransaction(TransactionId),
    /// New behavior: a transaction is rejected if the orphan pool is full
    #[error("number of high-priority transactions in orphan pool ({0}) has reached the maximum allowed ({1})")]
    RejectOrphanPoolIsFull(usize, u64),
    #[error("transactions in mempool form a cycle")]
    RejectCycleInMempoolTransactions,
    // TODO: This error is added for the tx_relay flow but is never constructed neither in the golang nor in this version. Discuss if it can be removed.
    #[error("Transaction {0} is invalid")]
    RejectInvalid(TransactionId),
    #[error("Transaction {0} storage mass is incomputable")]
    RejectStorageMassIncomputable(TransactionId),
    #[error("Transaction {0} version is not supported: {1}, max supported: {2}, {3}")]
    RejectVersion(TransactionId, u16, u16, u16),
    #[error("Transaction {0} compute mass is too high: {1}, max allowed: {2}")]
    RejectComputeMass(TransactionId, u64, u64),
    #[error("Transaction {0} transient mass is too high: {1}, max allowed: {2}")]
    RejectTransientMass(TransactionId, u64, u64),
    #[error("Transaction {0} storage mass is too high: {1}, max allowed: {2}")]
    RejectStorageMass(TransactionId, u64, u64),
    #[error("Transaction {0} signature script size is too high: {1}, max allowed: {2}, {3}")]
    RejectSignatureScriptSize(TransactionId, usize, u64, u64),
    #[error("Transaction {0} script public key version is not supported in output index {1}")]
    RejectScriptPublicKeyVersion(TransactionId, usize),
    #[error("Transaction {0} output index {1} has an unsupported script class")]
    RejectOutputScriptClass(TransactionId, usize),
    #[error("Transaction {0} output index {1} has dust value {2}")]
    RejectDust(TransactionId, usize, u64),
    #[error("Transaction {0} input index {1} has an unsupported script class")]
    RejectInputScriptClass(TransactionId, usize),
    #[error("Transaction {0} fee {1} is too low, minimum is {2}")]
    RejectInsufficientFee(TransactionId, u64, u64),
    #[error("Transaction {0} has too many signature operations: {1}, max allowed: {2}, {3}")]
    RejectSignatureCount(TransactionId, usize, u64, u8),
    #[error(transparent)]
    RuleError(#[from] TxRuleError),
}

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum NonStandardError {
    #[error("transaction version {1} is not in the valid range of {2}-{3}")]
    RejectVersion(TransactionId, u16, u16, u16),

    #[error("transaction compute mass of {1} is larger than max allowed size of {2}")]
    RejectComputeMass(TransactionId, u64, u64),

    #[error("transaction transient (storage) mass of {1} is larger than max allowed size of {2}")]
    RejectTransientMass(TransactionId, u64, u64),

    #[error("transaction storage mass of {1} is larger than max allowed size of {2}")]
    RejectStorageMass(TransactionId, u64, u64),

    #[error("transaction input #{1}: signature script size of {2} bytes is larger than the maximum allowed size of {3} bytes")]
    RejectSignatureScriptSize(TransactionId, usize, u64, u64),

    #[error("transaction output #{1}: the version of the scriptPublicKey is higher than the known version")]
    RejectScriptPublicKeyVersion(TransactionId, usize),

    #[error("transaction output #{1}: non-standard script form")]
    RejectOutputScriptClass(TransactionId, usize),

    #[error("transaction output #{1}: payment of {2} is dust")]
    RejectDust(TransactionId, usize, u64),

    #[error("transaction input {1}: non-standard script form")]
    RejectInputScriptClass(TransactionId, usize),

    #[error("transaction has {1} fees which is under the required amount of {2}")]
    RejectInsufficientFee(TransactionId, u64, u64),

    #[error("transaction input #{1} has {2} signature operations which is more than the allowed max amount of {3}")]
    RejectSignatureCount(TransactionId, usize, u64, u8),
}

impl NonStandardError {
    pub fn transaction_id(&self) -> &TransactionId {
        match self {
            NonStandardError::RejectVersion(id, _, _, _) => id,
            NonStandardError::RejectComputeMass(id, _, _) => id,
            NonStandardError::RejectTransientMass(id, _, _) => id,
            NonStandardError::RejectStorageMass(id, _, _) => id,
            NonStandardError::RejectSignatureScriptSize(id, _, _, _) => id,
            NonStandardError::RejectScriptPublicKeyVersion(id, _) => id,
            NonStandardError::RejectOutputScriptClass(id, _) => id,
            NonStandardError::RejectDust(id, _, _) => id,
            NonStandardError::RejectInputScriptClass(id, _) => id,
            NonStandardError::RejectInsufficientFee(id, _, _) => id,
            NonStandardError::RejectSignatureCount(id, _, _, _) => id,
        }
    }
}

pub type NonStandardResult<T> = std::result::Result<T, NonStandardError>;
