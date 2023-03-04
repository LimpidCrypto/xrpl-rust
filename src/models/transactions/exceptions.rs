use thiserror_no_std::Error;

use super::TransactionFlag;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum XrplTransactionException<'a> {
    #[error("`{txn_type:?}`: The field `{field:?}`  can not be combined with flag `{flag:?}`. For more information see: {resource:?}")]
    IllegalFieldWithFlag {
        txn_type: &'a str,
        field: &'a str,
        flag: TransactionFlag,
        resource: &'a str,
    },
    #[error("`{txn_type:?}`: The field `{field1:?}`  can not be combined with field `{field2:?}`. For more information see: {resource:?}")]
    IllegalFieldWithField {
        txn_type: &'a str,
        field1: &'a str,
        field2: &'a str,
        resource: &'a str,
    },
    #[error("`{txn_type:?}`: The flag `{flag1:?}` can not be combined with flag `{flag2:?}`. For more information see: {resource:?}")]
    IllegalFlagCombo {
        txn_type: &'a str,
        flag1: TransactionFlag,
        flag2: TransactionFlag,
        resource: &'a str,
    },
    #[error("`{txn_type:?}`: For field `{field:?}` to be set, it is required to set flag `{flag:?}`. For more information see: {resource:?}")]
    FieldRequiresFlag {
        txn_type: &'a str,
        field: &'a str,
        flag: TransactionFlag,
        resource: &'a str,
    },
    #[error("`{txn_type:?}`: For flag `{flag:?}` to be set, it is required to define field `{field:?}`. For more information see: {resource:?}")]
    FlagRequiresField {
        txn_type: &'a str,
        flag: TransactionFlag,
        field: &'a str,
        resource: &'a str,
    },
    // TransferRateTooHigh {
    //     max: u32,
    //     found: u32,
    // },
    // TransferRateTooLow {
    //     min: u32,
    //     found: u32,
    // },
    // DomainIsNotLowercase,
    // DomainTooLong {
    //     max: usize,
    //     found: usize,
    // },
    // ClearFlagMustNotEqualSetFlag,
    // MustSetAsfAuthorizedNftokenMinterFlagToSetMinter,
    // NftokenMinterMustBeSetIfAsfAuthorizedNftokenMinterIsSet,
    // NftokenMinterMustNotBeSetIfAsfAuthorizedNftokenMinterIsUnset,
}

#[derive(Debug, Clone, PartialEq, Display, Error)]
pub enum XrplCheckCashException {
    MustSetAmountOrDeliverMin,
    MustNotSetAmountAndDeliverMin,
}

#[derive(Debug, Clone, PartialEq, Display, Error)]
pub enum XrplDepositPreauthException {
    MustSetAuthorizeOrUnauthorize,
    MustNotSetAuthorizeAndUnauthorize,
}

#[derive(Debug, Clone, PartialEq, Display, Error)]
pub enum XrplEscrowCreateException {
    CancelAfterMustNotBeBeforeFinishAfter,
}

#[derive(Debug, Clone, PartialEq, Display, Error)]
pub enum XrplEscrowFinishException {
    IfOneSetBothConditionAndFulfillmentMustBeSet,
}

#[derive(Debug, Clone, PartialEq, Display, Error)]
pub enum XrplNFTokenAcceptOfferException {
    MustSetEitherNftokenBuyOfferOrNftokenSellOffer,
    BrokerFeeMustBeGreaterZero,
}

#[derive(Debug, Clone, PartialEq, Display, Error)]
pub enum XrplNFTokenCancelOfferException {
    MustIncludeOneNFTokenOffer,
}

#[derive(Debug, Clone, PartialEq, Display, Error)]
pub enum XrplNFTokenCreateOfferException {
    AmountMustBeGreaterZero,
    DestinationMustNotEqualAccount,
    OwnerMustBeSetForBuyOffer,
    OwnerMustNotBeSetForSellOffer,
    OwnerMustNotEqualAccount,
}

#[derive(Debug, Clone, PartialEq, Display, Error)]
pub enum XrplNFTokenMintException {
    IssuerMustNotEqualAccount,
    TransferFeeTooHigh { max: u32, found: u32 },
    URITooLong { max: usize, found: usize },
}

#[derive(Debug, Clone, PartialEq, Display, Error)]
pub enum XrplPaymentException {
    XRPtoXRPPaymentsCannotContainPaths,
    DestinationMustNotEqualAccountForXRPtoXRPPayments,
    SendMaxMustBeSetForPartialPayments,
    DeliverMinMustNotBeSetForNonPartialPayments,
    SendMaxMustNotBeSetForXRPtoXRPNonPartialPayments,
    SendMaxMustBeSetForExchanges,
}

#[derive(Debug, Clone, PartialEq, Display, Error)]
pub enum XrplSignerListSetException {
    MustNotSetSignerEntriesIfSignerListIsBeingDeleted,
    SignerQuorumMustBeZeroIfSignerListIsBeingDeleted,
    TooFewSignerEntries { min: usize, found: usize },
    TooManySignerEntries { max: usize, found: usize },
    AccountMustNotBeInSignerEntry,
    MustBeLessOrEqualToSumOfSignerWeightInSignerEntries { max: u32, found: u32 },
    AnAccountCanNotBeInSignerEntriesTwice,
}

#[derive(Debug, Clone, PartialEq, Display, Error)]
pub enum XrplUNLModifyException {
    UNLModifyDisablingMustBeOneOrTwo,
}
