//! Transaction models.

use crate::{
    constants::{
        DISABLE_TICK_SIZE, MAX_DOMAIN_LENGTH, MAX_TICK_SIZE, MAX_TRANSFER_FEE, MAX_TRANSFER_RATE,
        MAX_URI_LENGTH, MIN_TICK_SIZE, MIN_TRANSFER_RATE, SPECIAL_CASE_TRANFER_RATE,
    },
    models::*,
};
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use model::Model;

use super::exceptions::{
    CheckCashException, DepositPreauthException, EscrowCreateException,
    NFTokenAcceptOfferException, XRPLModelException, XRPLTransactionException,
};

/// Transaction

/// An AccountDelete transaction deletes an account and any objects it
/// owns in the XRP Ledger, if possible, sending the account's remaining
/// XRP to a specified destination account. See Deletion of Accounts for
/// the requirements to delete an account.
///
/// See AccountDelete:
/// `<https://xrpl.org/accountdelete.html>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "PascalCase", deserialize = "snake_case"))]
pub struct AccountDelete<'a> {
    /// The base fields for all transaction models.
    ///
    /// See Transaction Types:
    /// `<https://xrpl.org/transaction-types.html>`
    ///
    /// See Transaction Common Fields:
    /// `<https://xrpl.org/transaction-common-fields.html>`
    #[serde(skip_serializing)]
    #[serde(default = "TransactionType::account_delete")]
    transaction_type: TransactionType,
    account: &'a str,
    fee: Option<&'a str>,
    sequence: Option<u32>,
    last_ledger_sequence: Option<u32>,
    account_txn_id: Option<&'a str>,
    signing_pub_key: Option<&'a str>,
    source_tag: Option<u32>,
    ticket_sequence: Option<u32>,
    txn_signature: Option<&'a str>,
    flags: Option<Vec<u32>>,
    memos: Option<Vec<Memo<'a>>>,
    signers: Option<Vec<Signer<'a>>>,
    /// The custom fields for the AccountDelete model.
    ///
    /// See AccountDelete fields:
    /// `<https://xrpl.org/accountdelete.html#accountdelete-fields>`
    destination: &'a str,
    destination_tag: Option<u32>,
}

impl Model for AccountDelete<'static> {
    // fn to_json(&self) -> &str {
    //     let transaction_json = serde_json::to_string(&self).expect("Unable to convert `AccountDelete` json to string.");
    //     transaction_json
    // }

    fn to_json_value(&self) -> Value {
        let mut transaction_json =
            serde_json::to_value(&self).expect("Unable to serialize `AccountDelete` to json.");
        transaction_json["Flags"] = Value::from(self.iter_to_int());
        transaction_json
    }
}

impl Transaction for AccountDelete<'static> {
    fn get_transaction_type(&self) -> TransactionType {
        self.transaction_type.clone()
    }
}

/// An AccountSet transaction modifies the properties of an
/// account in the XRP Ledger.
///
/// See AccountSet:
/// `<https://xrpl.org/accountset.html>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "PascalCase", deserialize = "snake_case"))]
pub struct AccountSet<'a> {
    /// The base fields for all transaction models.
    ///
    /// See Transaction Types:
    /// `<https://xrpl.org/transaction-types.html>`
    ///
    /// See Transaction Common Fields:
    /// `<https://xrpl.org/transaction-common-fields.html>`
    #[serde(skip_serializing)]
    #[serde(default = "TransactionType::account_set")]
    transaction_type: TransactionType,
    account: &'a str,
    fee: Option<&'a str>,
    sequence: Option<u32>,
    last_ledger_sequence: Option<u32>,
    account_txn_id: Option<&'a str>,
    signing_pub_key: Option<&'a str>,
    source_tag: Option<u32>,
    ticket_sequence: Option<u32>,
    txn_signature: Option<&'a str>,
    flags: Option<Vec<AccountSetFlag>>,
    memos: Option<Vec<Memo<'a>>>,
    signers: Option<Vec<Signer<'a>>>,
    /// The custom fields for the AccountSet model.
    ///
    /// See AccountSet fields:
    /// `<https://xrpl.org/accountset.html#accountset-fields>`
    clear_flag: Option<u32>,
    domain: Option<&'a str>,
    email_hash: Option<&'a str>,
    message_key: Option<&'a str>,
    set_flag: Option<u32>,
    transfer_rate: Option<u32>,
    tick_size: Option<u32>,
    nftoken_minter: Option<&'a str>,
}

impl Model for AccountSet<'static> {
    // fn to_json(&self) -> &str {
    //     let transaction_json_str = serde_json::to_string(&self).expect("Unable to convert `AccountDelete` json to string.").as_str();
    //     transaction_json_str
    // }

    fn to_json_value(&self) -> Value {
        let mut transaction_json =
            serde_json::to_value(&self).expect("Unable to serialize `AccountSet` to json.");
        transaction_json["Flags"] = Value::from(self.iter_to_int());
        transaction_json
    }

    fn get_errors(&self) -> Result<(), XRPLModelException> {
        match self.get_tick_size_error() {
            Err(error) => Err(XRPLModelException::XRPLTransactionError(
                XRPLTransactionException::AccountSetError(error),
            )),
            Ok(_no_error) => match self.get_transfer_rate_error() {
                Err(error) => Err(XRPLModelException::XRPLTransactionError(
                    XRPLTransactionException::AccountSetError(error),
                )),
                Ok(_no_error) => match self.get_domain_error() {
                    Err(error) => Err(XRPLModelException::XRPLTransactionError(
                        XRPLTransactionException::AccountSetError(error),
                    )),
                    Ok(_no_error) => match self.get_clear_flag_error() {
                        Err(error) => Err(XRPLModelException::XRPLTransactionError(
                            XRPLTransactionException::AccountSetError(error),
                        )),
                        Ok(_no_error) => match self.get_nftoken_minter_error() {
                            Err(error) => Err(XRPLModelException::XRPLTransactionError(
                                XRPLTransactionException::AccountSetError(error),
                            )),
                            Ok(_no_error) => Ok(()),
                        },
                    },
                },
            },
        }
    }
}

impl Transaction for AccountSet<'static> {
    fn iter_to_int(&self) -> u32 {
        let mut flags_int: u32 = 0;
        if self.flags.is_some() {
            for flag in self.flags.as_ref().unwrap() {
                match flag {
                    AccountSetFlag::AsfAccountTxnID => flags_int += 0x00000005,
                    AccountSetFlag::AsfAuthorizedNFTokenMinter => flags_int += 0x0000000A,
                    AccountSetFlag::AsfDefaultRipple => flags_int += 0x00000008,
                    AccountSetFlag::AsfDepositAuth => flags_int += 0x00000009,
                    AccountSetFlag::AsfDisableMaster => flags_int += 0x00000004,
                    AccountSetFlag::AsfDisallowXRP => flags_int += 0x00000003,
                    AccountSetFlag::AsfGlobalFreeze => flags_int += 0x00000007,
                    AccountSetFlag::AsfNoFreeze => flags_int += 0x00000006,
                    AccountSetFlag::AsfRequireAuth => flags_int += 0x00000002,
                    AccountSetFlag::AsfRequireDest => flags_int += 0x00000001,
                }
            }
        }
        flags_int
    }

    fn has_flag(&self, flag: Flag) -> bool {
        let mut has_flag = false;
        if self.iter_to_int() > 0 {
            match flag {
                Flag::AccountSet(account_set_flag) => {
                    match account_set_flag {
                        AccountSetFlag::AsfAccountTxnID => {
                            if self
                                .flags
                                .as_ref()
                                .unwrap()
                                .contains(&AccountSetFlag::AsfAccountTxnID)
                            {
                                has_flag = true
                            };
                        }
                        AccountSetFlag::AsfAuthorizedNFTokenMinter => {
                            if self
                                .flags
                                .as_ref()
                                .unwrap()
                                .contains(&AccountSetFlag::AsfAuthorizedNFTokenMinter)
                            {
                                has_flag = true
                            };
                        }
                        AccountSetFlag::AsfDefaultRipple => {
                            if self
                                .flags
                                .as_ref()
                                .unwrap()
                                .contains(&AccountSetFlag::AsfDefaultRipple)
                            {
                                has_flag = true
                            };
                        }
                        AccountSetFlag::AsfDepositAuth => {
                            if self
                                .flags
                                .as_ref()
                                .unwrap()
                                .contains(&AccountSetFlag::AsfDepositAuth)
                            {
                                has_flag = true
                            };
                        }
                        AccountSetFlag::AsfDisableMaster => {
                            if self
                                .flags
                                .as_ref()
                                .unwrap()
                                .contains(&AccountSetFlag::AsfDisableMaster)
                            {
                                has_flag = true
                            };
                        }
                        AccountSetFlag::AsfDisallowXRP => {
                            if self
                                .flags
                                .as_ref()
                                .unwrap()
                                .contains(&AccountSetFlag::AsfDisallowXRP)
                            {
                                has_flag = true
                            };
                        }
                        AccountSetFlag::AsfGlobalFreeze => {
                            if self
                                .flags
                                .as_ref()
                                .unwrap()
                                .contains(&AccountSetFlag::AsfGlobalFreeze)
                            {
                                has_flag = true
                            };
                        }
                        AccountSetFlag::AsfNoFreeze => {
                            if self
                                .flags
                                .as_ref()
                                .unwrap()
                                .contains(&AccountSetFlag::AsfNoFreeze)
                            {
                                has_flag = true
                            };
                        }
                        AccountSetFlag::AsfRequireAuth => {
                            if self
                                .flags
                                .as_ref()
                                .unwrap()
                                .contains(&AccountSetFlag::AsfRequireAuth)
                            {
                                has_flag = true
                            };
                        }
                        AccountSetFlag::AsfRequireDest => {
                            if self
                                .flags
                                .as_ref()
                                .unwrap()
                                .contains(&AccountSetFlag::AsfRequireDest)
                            {
                                has_flag = true
                            };
                        }
                    };
                }
                _ => has_flag = false,
            }
        }
        has_flag
    }

    fn get_transaction_type(&self) -> TransactionType {
        self.transaction_type.clone()
    }
}

impl AccountSetError for AccountSet<'static> {
    fn get_tick_size_error(&self) -> Result<(), AccountSetException> {
        match self.tick_size {
            Some(tick_size) => match tick_size > MAX_TICK_SIZE {
                true => Err(AccountSetException::InvalidTickSizeTooHigh {
                    max: 15,
                    found: tick_size,
                }),
                false => match tick_size < MIN_TICK_SIZE && tick_size != DISABLE_TICK_SIZE {
                    true => Err(AccountSetException::InvalidTickSizeTooLow {
                        min: 3,
                        found: tick_size,
                    }),
                    false => Ok(()),
                },
            },
            None => Ok(()),
        }
    }

    fn get_transfer_rate_error(&self) -> Result<(), AccountSetException> {
        match self.transfer_rate {
            Some(transfer_rate) => match transfer_rate > MAX_TRANSFER_RATE {
                true => Err(AccountSetException::InvalidTransferRateTooHigh {
                    max: MAX_TRANSFER_RATE,
                    found: transfer_rate,
                }),
                false => match transfer_rate < MIN_TRANSFER_RATE
                    && transfer_rate != SPECIAL_CASE_TRANFER_RATE
                {
                    true => Err(AccountSetException::InvalidTransferRateTooLow {
                        min: MAX_TRANSFER_RATE,
                        found: transfer_rate,
                    }),
                    false => Ok(()),
                },
            },
            None => Ok(()),
        }
    }

    fn get_domain_error(&self) -> Result<(), AccountSetException> {
        match self.domain {
            Some(domain) => match domain.to_lowercase().as_str() != domain {
                true => Err(AccountSetException::InvalidDomainIsNotLowercase),
                false => match domain.len() > MAX_DOMAIN_LENGTH {
                    true => Err(AccountSetException::InvalidDomainTooLong {
                        max: MAX_DOMAIN_LENGTH,
                        found: domain.len(),
                    }),
                    false => Ok(()),
                },
            },
            None => Ok(()),
        }
    }

    fn get_clear_flag_error(&self) -> Result<(), AccountSetException> {
        match self.clear_flag {
            Some(_clear_flag) => match self.clear_flag == self.set_flag {
                true => Err(AccountSetException::InvalidClearFlagMustNotEqualSetFlag),
                false => Ok(()),
            },
            None => Ok(()),
        }
    }

    fn get_nftoken_minter_error(&self) -> Result<(), AccountSetException> {
        // TODO: `set_flag` and `clear_flag` should be typed as `AccountSetFlag`.
        // if self.nftoken_minter.is_some() && self.set_flag.unwrap() != AccountSetFlag::AsfAuthorizedNFTokenMinter {
        //     return Some("Will not set the minter unless AccountSetFlag.ASF_AUTHORIZED_NFTOKEN_MINTER is set.");
        // }
        // if self.nftoken_minter.is_none() && self.set_flag.unwrap() == AccountSetFlag::AsfAuthorizedNFTokenMinter {
        //     return Some("`nftoken_minter` must be present if `AccountSetFlag.AsfAuthorizedNFTokenMinter` is set.");
        // }
        // if self.nftoken_minter.is_some() && self.clear_flag.unwrap() == AccountSetFlag::AsfAuthorizedNFTokenMinter {
        //     return Some("`nftoken_minter` must not be present if AccountSetFlag.AsfAuthorizedNFTokenMinter is unset using `clear_flag`")
        // }
        Ok(())
    }
}

/// Cancels an unredeemed Check, removing it from the ledger without
/// sending any money. The source or the destination of the check can
/// cancel a Check at any time using this transaction type. If the Check
/// has expired, any address can cancel it.
///
/// See CheckCancel:
/// `<https://xrpl.org/checkcancel.html>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "PascalCase", deserialize = "snake_case"))]
pub struct CheckCancel<'a> {
    /// The base fields for all transaction models.
    ///
    /// See Transaction Types:
    /// `<https://xrpl.org/transaction-types.html>`
    ///
    /// See Transaction Common Fields:
    /// `<https://xrpl.org/transaction-common-fields.html>`
    #[serde(skip_serializing)]
    #[serde(default = "TransactionType::check_cancel")]
    transaction_type: TransactionType,
    account: &'a str,
    fee: Option<&'a str>,
    sequence: Option<u32>,
    last_ledger_sequence: Option<u32>,
    account_txn_id: Option<&'a str>,
    signing_pub_key: Option<&'a str>,
    source_tag: Option<u32>,
    ticket_sequence: Option<u32>,
    txn_signature: Option<&'a str>,
    flags: Option<Vec<u32>>,
    memos: Option<Vec<Memo<'a>>>,
    signers: Option<Vec<Signer<'a>>>,
    /// The custom fields for the CheckCancel model.
    ///
    /// See CheckCancel fields:
    /// `<https://xrpl.org/checkcancel.html#checkcancel-fields>`
    check_id: &'a str,
}

impl Model for CheckCancel<'static> {
    // fn to_json(&self) -> &str {
    //     let transaction_json = serde_json::to_string(&self).expect("Unable to convert `AccountDelete` json to string.");
    //     transaction_json
    // }

    fn to_json_value(&self) -> Value {
        let mut transaction_json =
            serde_json::to_value(&self).expect("Unable to serialize `CheckCancel` to json.");
        transaction_json["Flags"] = Value::from(self.iter_to_int());
        transaction_json
    }
}

impl Transaction for CheckCancel<'static> {
    fn get_transaction_type(&self) -> TransactionType {
        self.transaction_type.clone()
    }
}

/// Cancels an unredeemed Check, removing it from the ledger without
/// sending any money. The source or the destination of the check can
/// cancel a Check at any time using this transaction type. If the Check
/// has expired, any address can cancel it.
///
/// See CheckCash:
/// `<https://xrpl.org/checkcash.html>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "PascalCase", deserialize = "snake_case"))]
pub struct CheckCash<'a> {
    /// The base fields for all transaction models.
    ///
    /// See Transaction Types:
    /// `<https://xrpl.org/transaction-types.html>`
    ///
    /// See Transaction Common Fields:
    /// `<https://xrpl.org/transaction-common-fields.html>`
    #[serde(skip_serializing)]
    #[serde(default = "TransactionType::check_cash")]
    transaction_type: TransactionType,
    account: &'a str,
    fee: Option<&'a str>,
    sequence: Option<u32>,
    last_ledger_sequence: Option<u32>,
    account_txn_id: Option<&'a str>,
    signing_pub_key: Option<&'a str>,
    source_tag: Option<u32>,
    ticket_sequence: Option<u32>,
    txn_signature: Option<&'a str>,
    flags: Option<Vec<u32>>,
    memos: Option<Vec<Memo<'a>>>,
    signers: Option<Vec<Signer<'a>>>,
    /// The custom fields for the CheckCash model.
    ///
    /// See CheckCash fields:
    /// `<https://xrpl.org/checkcash.html#checkcash-fields>`
    check_id: &'a str,
    amount: Option<Currency>,
    deliver_min: Option<Currency>,
}

impl Model for CheckCash<'static> {
    // fn to_json(&self) -> &str {
    //     let transaction_json = serde_json::to_string(&self).expect("Unable to convert `AccountDelete` json to string.");
    //     transaction_json
    // }

    fn to_json_value(&self) -> Value {
        let mut transaction_json =
            serde_json::to_value(&self).expect("Unable to serialize `CheckCash` to json.");
        transaction_json["Flags"] = Value::from(self.iter_to_int());
        transaction_json
    }

    fn get_errors(&self) -> Result<(), XRPLModelException> {
        match self.get_amount_and_deliver_min_error() {
            Ok(_no_error) => Ok(()),
            Err(error) => Err(XRPLModelException::XRPLTransactionError(
                XRPLTransactionException::CheckCashError(error),
            )),
        }
    }
}

impl Transaction for CheckCash<'static> {
    fn get_transaction_type(&self) -> TransactionType {
        self.transaction_type.clone()
    }
}

impl CheckCashError for CheckCash<'static> {
    fn get_amount_and_deliver_min_error(&self) -> Result<(), CheckCashException> {
        match self.amount.is_none() && self.deliver_min.is_none() {
            true => Err(CheckCashException::InvalidMustSetAmountOrDeliverMin),
            false => match self.amount.is_some() && self.deliver_min.is_some() {
                true => Err(CheckCashException::InvalidMustNotSetAmountAndDeliverMin),
                false => Ok(()),
            },
        }
    }
}

/// Create a Check object in the ledger, which is a deferred
/// payment that can be cashed by its intended destination.
///
/// See CheckCreate:
/// `<https://xrpl.org/checkcreate.html>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "PascalCase", deserialize = "snake_case"))]
pub struct CheckCreate<'a> {
    /// The base fields for all transaction models.
    ///
    /// See Transaction Types:
    /// `<https://xrpl.org/transaction-types.html>`
    ///
    /// See Transaction Common Fields:
    /// `<https://xrpl.org/transaction-common-fields.html>`
    #[serde(skip_serializing)]
    #[serde(default = "TransactionType::check_create")]
    transaction_type: TransactionType,
    account: &'a str,
    fee: Option<&'a str>,
    sequence: Option<u32>,
    last_ledger_sequence: Option<u32>,
    account_txn_id: Option<&'a str>,
    signing_pub_key: Option<&'a str>,
    source_tag: Option<u32>,
    ticket_sequence: Option<u32>,
    txn_signature: Option<&'a str>,
    flags: Option<Vec<u32>>,
    memos: Option<Vec<Memo<'a>>>,
    signers: Option<Vec<Signer<'a>>>,
    /// The custom fields for the CheckCreate model.
    ///
    /// See CheckCreate fields:
    /// `<https://xrpl.org/checkcreate.html#checkcreate-fields>`
    destination: &'a str,
    send_max: Currency,
    destination_tag: Option<u32>,
    expiration: Option<u32>,
    invoice_id: Option<&'a str>,
}

impl Model for CheckCreate<'static> {
    // fn to_json(&self) -> &str {
    //     let transaction_json = serde_json::to_string(&self).expect("Unable to convert `AccountDelete` json to string.");
    //     transaction_json
    // }

    fn to_json_value(&self) -> Value {
        let mut transaction_json =
            serde_json::to_value(&self).expect("Unable to serialize `CheckCreate` to json.");
        transaction_json["Flags"] = Value::from(self.iter_to_int());
        transaction_json
    }
}

impl Transaction for CheckCreate<'static> {
    fn get_transaction_type(&self) -> TransactionType {
        self.transaction_type.clone()
    }
}

/// A DepositPreauth transaction gives another account pre-approval
/// to deliver payments to the sender of this transaction.
///
/// See DepositPreauth:
/// `<https://xrpl.org/depositpreauth.html>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "PascalCase", deserialize = "snake_case"))]
pub struct DepositPreauth<'a> {
    /// The base fields for all transaction models.
    ///
    /// See Transaction Types:
    /// `<https://xrpl.org/transaction-types.html>`
    ///
    /// See Transaction Common Fields:
    /// `<https://xrpl.org/transaction-common-fields.html>`
    #[serde(skip_serializing)]
    #[serde(default = "TransactionType::deposit_preauth")]
    transaction_type: TransactionType,
    account: &'a str,
    fee: Option<&'a str>,
    sequence: Option<u32>,
    last_ledger_sequence: Option<u32>,
    account_txn_id: Option<&'a str>,
    signing_pub_key: Option<&'a str>,
    source_tag: Option<u32>,
    ticket_sequence: Option<u32>,
    txn_signature: Option<&'a str>,
    flags: Option<Vec<u32>>,
    memos: Option<Vec<Memo<'a>>>,
    signers: Option<Vec<Signer<'a>>>,
    /// The custom fields for the DepositPreauth model.
    ///
    /// See DepositPreauth fields:
    /// `<https://xrpl.org/depositpreauth.html#depositpreauth-fields>`
    authorize: Option<&'a str>,
    unauthorize: Option<&'a str>,
}

impl Model for DepositPreauth<'static> {
    // fn to_json(&self) -> &str {
    //     let transaction_json = serde_json::to_string(&self).expect("Unable to convert `AccountDelete` json to string.");
    //     transaction_json
    // }

    fn to_json_value(&self) -> Value {
        let mut transaction_json =
            serde_json::to_value(&self).expect("Unable to serialize `DepositPreauth` to json.");
        transaction_json["Flags"] = Value::from(self.iter_to_int());
        transaction_json
    }

    fn get_errors(&self) -> Result<(), XRPLModelException> {
        match self.get_authorize_and_unauthorize_error() {
            Ok(_no_error) => Ok(()),
            Err(error) => Err(XRPLModelException::XRPLTransactionError(
                XRPLTransactionException::DepositPreauthError(error),
            )),
        }
    }
}

impl Transaction for DepositPreauth<'static> {
    fn get_transaction_type(&self) -> TransactionType {
        self.transaction_type.clone()
    }
}

impl DepositPreauthError for DepositPreauth<'static> {
    fn get_authorize_and_unauthorize_error(&self) -> Result<(), DepositPreauthException> {
        match self.authorize.is_none() && self.unauthorize.is_none() {
            true => Err(DepositPreauthException::InvalidMustSetAuthorizeOrUnauthorize),
            false => match self.authorize.is_some() && self.unauthorize.is_some() {
                true => Err(DepositPreauthException::InvalidMustNotSetAuthorizeAndUnauthorize),
                false => Ok(()),
            },
        }
    }
}

/// Cancels an Escrow and returns escrowed XRP to the sender.
///
/// See EscrowCancel:
/// `<https://xrpl.org/escrowcancel.html>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "PascalCase", deserialize = "snake_case"))]
pub struct EscrowCancel<'a> {
    /// The base fields for all transaction models.
    ///
    /// See Transaction Types:
    /// `<https://xrpl.org/transaction-types.html>`
    ///
    /// See Transaction Common Fields:
    /// `<https://xrpl.org/transaction-common-fields.html>`
    #[serde(skip_serializing)]
    #[serde(default = "TransactionType::escrow_cancel")]
    transaction_type: TransactionType,
    account: &'a str,
    fee: Option<&'a str>,
    sequence: Option<u32>,
    last_ledger_sequence: Option<u32>,
    account_txn_id: Option<&'a str>,
    signing_pub_key: Option<&'a str>,
    source_tag: Option<u32>,
    ticket_sequence: Option<u32>,
    txn_signature: Option<&'a str>,
    flags: Option<Vec<u32>>,
    memos: Option<Vec<Memo<'a>>>,
    signers: Option<Vec<Signer<'a>>>,
    /// The custom fields for the EscrowCancel model.
    ///
    /// See EscrowCancel fields:
    /// `<https://xrpl.org/escrowcancel.html#escrowcancel-flags>`
    owner: &'a str,
    offer_sequence: u32,
}

impl Model for EscrowCancel<'static> {
    // fn to_json(&self) -> &str {
    //     let transaction_json = serde_json::to_string(&self).expect("Unable to convert `AccountDelete` json to string.");
    //     transaction_json
    // }

    fn to_json_value(&self) -> Value {
        let mut transaction_json =
            serde_json::to_value(&self).expect("Unable to serialize `EscrowCancel` to json.");
        transaction_json["Flags"] = Value::from(self.iter_to_int());
        transaction_json
    }
}

impl Transaction for EscrowCancel<'static> {
    fn get_transaction_type(&self) -> TransactionType {
        self.transaction_type.clone()
    }
}

/// Creates an Escrow, which sequests XRP until the escrow process either finishes or is canceled.
///
/// See EscrowCreate:
/// `<https://xrpl.org/escrowcreate.html>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "PascalCase", deserialize = "snake_case"))]
pub struct EscrowCreate<'a> {
    /// The base fields for all transaction models.
    ///
    /// See Transaction Types:
    /// `<https://xrpl.org/transaction-types.html>`
    ///
    /// See Transaction Common Fields:
    /// `<https://xrpl.org/transaction-common-fields.html>`
    #[serde(skip_serializing)]
    #[serde(default = "TransactionType::escrow_create")]
    transaction_type: TransactionType,
    account: &'a str,
    fee: Option<&'a str>,
    sequence: Option<u32>,
    last_ledger_sequence: Option<u32>,
    account_txn_id: Option<&'a str>,
    signing_pub_key: Option<&'a str>,
    source_tag: Option<u32>,
    ticket_sequence: Option<u32>,
    txn_signature: Option<&'a str>,
    flags: Option<Vec<u32>>,
    memos: Option<Vec<Memo<'a>>>,
    signers: Option<Vec<Signer<'a>>>,
    /// The custom fields for the EscrowCreate model.
    ///
    /// See EscrowCreate fields:
    /// `<https://xrpl.org/escrowcreate.html#escrowcreate-flags>`
    amount: Currency,
    destination: &'a str,
    destination_tag: Option<&'a str>,
    cancel_after: Option<u32>,
    finish_after: Option<u32>,
    condition: Option<&'a str>,
}

impl Model for EscrowCreate<'static> {
    // fn to_json(&self) -> &str {
    //     let transaction_json = serde_json::to_string(&self).expect("Unable to convert `AccountDelete` json to string.");
    //     transaction_json
    // }

    fn to_json_value(&self) -> Value {
        let mut transaction_json =
            serde_json::to_value(&self).expect("Unable to serialize `EscrowCreate` to json.");
        transaction_json["Flags"] = Value::from(self.iter_to_int());
        transaction_json
    }

    fn get_errors(&self) -> Result<(), XRPLModelException> {
        match self.get_finish_after_error() {
            Ok(_no_error) => Ok(()),
            Err(error) => Err(XRPLModelException::XRPLTransactionError(
                XRPLTransactionException::EscrowCreateError(error),
            )),
        }
    }
}

impl Transaction for EscrowCreate<'static> {
    fn get_transaction_type(&self) -> TransactionType {
        self.transaction_type.clone()
    }
}

impl EscrowCreateError for EscrowCreate<'static> {
    fn get_finish_after_error(&self) -> Result<(), EscrowCreateException> {
        match self.finish_after {
            Some(finish_after) => match self.cancel_after {
                Some(cancel_after) => match finish_after >= cancel_after {
                    true => Err(EscrowCreateException::InvalidCancelAfterBeforeFinishAfter),
                    false => Ok(()),
                },
                None => Ok(()),
            },
            None => Ok(()),
        }
    }
}

/// Finishes an Escrow and delivers XRP from a held payment to the recipient.
///
/// See EscrowFinish:
/// `<https://xrpl.org/escrowfinish.html>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "PascalCase", deserialize = "snake_case"))]
pub struct EscrowFinish<'a> {
    /// The base fields for all transaction models.
    ///
    /// See Transaction Types:
    /// `<https://xrpl.org/transaction-types.html>`
    ///
    /// See Transaction Common Fields:
    /// `<https://xrpl.org/transaction-common-fields.html>`
    #[serde(skip_serializing)]
    #[serde(default = "TransactionType::escrow_finish")]
    transaction_type: TransactionType,
    account: &'a str,
    fee: Option<&'a str>,
    sequence: Option<u32>,
    last_ledger_sequence: Option<u32>,
    account_txn_id: Option<&'a str>,
    signing_pub_key: Option<&'a str>,
    source_tag: Option<u32>,
    ticket_sequence: Option<u32>,
    txn_signature: Option<&'a str>,
    flags: Option<Vec<u32>>,
    memos: Option<Vec<Memo<'a>>>,
    signers: Option<Vec<Signer<'a>>>,
    /// The custom fields for the EscrowFinish model.
    ///
    /// See EscrowFinish fields:
    /// `<https://xrpl.org/escrowfinish.html#escrowfinish-fields>`
    owner: &'a str,
    offer_sequence: u32,
    condition: Option<&'a str>,
    fulfillment: Option<&'a str>,
}

impl Model for EscrowFinish<'static> {
    // fn to_json(&self) -> &str {
    //     let transaction_json = serde_json::to_string(&self).expect("Unable to convert `AccountDelete` json to string.");
    //     transaction_json
    // }

    fn to_json_value(&self) -> Value {
        let mut transaction_json =
            serde_json::to_value(&self).expect("Unable to serialize `EscrowFinish` to json.");
        transaction_json["Flags"] = Value::from(self.iter_to_int());
        transaction_json
    }

    fn get_errors(&self) -> Result<(), XRPLModelException> {
        match self.get_condition_and_fulfillment_error() {
            Ok(_no_error) => Ok(()),
            Err(error) => Err(XRPLModelException::XRPLTransactionError(
                XRPLTransactionException::EscrowFinishError(error),
            )),
        }
    }
}

impl Transaction for EscrowFinish<'static> {
    fn get_transaction_type(&self) -> TransactionType {
        self.transaction_type.clone()
    }
}

impl EscrowFinishError for EscrowFinish<'static> {
    fn get_condition_and_fulfillment_error(&self) -> Result<(), EscrowFinishExeption> {
        match (self.condition.is_some() && self.fulfillment.is_none())
            || (self.condition.is_none() && self.condition.is_some())
        {
            true => Err(EscrowFinishExeption::InvalidBothConditionAndFulfillmentMustBeSet),
            false => Ok(()),
        }
    }
}

/// Accept offers to buy or sell an NFToken.
///
/// See NFTokenAcceptOffer:
/// `<https://xrpl.org/nftokenacceptoffer.html>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "PascalCase", deserialize = "snake_case"))]
pub struct NFTokenAcceptOffer<'a> {
    /// The base fields for all transaction models.
    ///
    /// See Transaction Types:
    /// `<https://xrpl.org/transaction-types.html>`
    ///
    /// See Transaction Common Fields:
    /// `<https://xrpl.org/transaction-common-fields.html>`
    #[serde(skip_serializing)]
    #[serde(default = "TransactionType::nftoken_accept_offer")]
    transaction_type: TransactionType,
    account: &'a str,
    fee: Option<&'a str>,
    sequence: Option<u32>,
    last_ledger_sequence: Option<u32>,
    account_txn_id: Option<&'a str>,
    signing_pub_key: Option<&'a str>,
    source_tag: Option<u32>,
    ticket_sequence: Option<u32>,
    txn_signature: Option<&'a str>,
    flags: Option<Vec<u32>>,
    memos: Option<Vec<Memo<'a>>>,
    signers: Option<Vec<Signer<'a>>>,
    /// The custom fields for the NFTokenAcceptOffer model.
    ///
    /// See NFTokenAcceptOffer fields:
    /// `<https://xrpl.org/nftokenacceptoffer.html#nftokenacceptoffer-fields>`
    nftoken_sell_offer: Option<&'a str>,
    nftoken_buy_offer: Option<&'a str>,
    nftoken_broker_fee: Option<Currency>,
}

impl Model for NFTokenAcceptOffer<'static> {
    // fn to_json(&self) -> &str {
    //     let transaction_json = serde_json::to_string(&self).expect("Unable to convert `AccountDelete` json to string.");
    //     transaction_json
    // }

    fn to_json_value(&self) -> Value {
        let mut transaction_json =
            serde_json::to_value(&self).expect("Unable to serialize `NFTokenAcceptOffer` to json.");
        transaction_json["Flags"] = Value::from(self.iter_to_int());
        transaction_json
    }

    fn get_errors(&self) -> Result<(), XRPLModelException> {
        match self.get_nftoken_sell_offer_error() {
            Err(error) => Err(XRPLModelException::XRPLTransactionError(
                XRPLTransactionException::NFTokenAcceptOfferError(error),
            )),
            Ok(_no_error) => match self.get_nftoken_buy_offer_error() {
                Err(error) => Err(XRPLModelException::XRPLTransactionError(
                    XRPLTransactionException::NFTokenAcceptOfferError(error),
                )),
                Ok(_no_error) => match self.get_nftoken_broker_fee_error() {
                    Err(error) => Err(XRPLModelException::XRPLTransactionError(
                        XRPLTransactionException::NFTokenAcceptOfferError(error),
                    )),
                    Ok(_no_error) => Ok(()),
                },
            },
        }
    }
}

impl Transaction for NFTokenAcceptOffer<'static> {
    fn get_transaction_type(&self) -> TransactionType {
        self.transaction_type.clone()
    }
}

impl NFTokenAcceptOfferError for NFTokenAcceptOffer<'static> {
    fn get_nftoken_sell_offer_error(&self) -> Result<(), NFTokenAcceptOfferException> {
        match self.nftoken_broker_fee.is_some() && self.nftoken_sell_offer.is_none() {
            true => Err(NFTokenAcceptOfferException::InvalidMustSetNftokenSellOfferIfBrokeredMode),
            false => match self.nftoken_sell_offer.is_none() && self.nftoken_buy_offer.is_none() {
                true => Err(NFTokenAcceptOfferException::InvalidMustSetEitherNftokenBuyOfferOrNftokenSellOffer),
                false => Ok(()),
            }
        }
    }

    fn get_nftoken_buy_offer_error(&self) -> Result<(), NFTokenAcceptOfferException> {
        match self.nftoken_broker_fee.is_some() && self.nftoken_buy_offer.is_none() {
            true => Err(NFTokenAcceptOfferException::InvalidMustSetNftokenBuyOfferIfBrokeredMode),
            false => match self.nftoken_sell_offer.is_none() && self.nftoken_buy_offer.is_none() {
                true => Err(NFTokenAcceptOfferException::InvalidMustSetEitherNftokenBuyOfferOrNftokenSellOffer),
                false => Ok(()),
            }
        }
    }

    fn get_nftoken_broker_fee_error(&self) -> Result<(), NFTokenAcceptOfferException> {
        match self.nftoken_broker_fee.as_ref() {
            Some(nftoken_broker_fee) => match nftoken_broker_fee.get_value_as_u32() == 0 {
                true => Err(NFTokenAcceptOfferException::InvalidBrokerFeeMustBeGreaterZero),
                false => Ok(()),
            },
            None => Ok(()),
        }
    }
}

/// Removes a NFToken object from the NFTokenPage in which it is being held,
/// effectively removing the token from the ledger (burning it).
///
/// See NFTokenBurn:
/// `<https://xrpl.org/nftokenburn.html>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "PascalCase", deserialize = "snake_case"))]
pub struct NFTokenBurn<'a> {
    /// The base fields for all transaction models.
    ///
    /// See Transaction Types:
    /// `<https://xrpl.org/transaction-types.html>`
    ///
    /// See Transaction Common Fields:
    /// `<https://xrpl.org/transaction-common-fields.html>`
    #[serde(skip_serializing)]
    #[serde(default = "TransactionType::nftoken_burn")]
    transaction_type: TransactionType,
    account: &'a str,
    fee: Option<&'a str>,
    sequence: Option<u32>,
    last_ledger_sequence: Option<u32>,
    account_txn_id: Option<&'a str>,
    signing_pub_key: Option<&'a str>,
    source_tag: Option<u32>,
    ticket_sequence: Option<u32>,
    txn_signature: Option<&'a str>,
    flags: Option<Vec<u32>>,
    memos: Option<Vec<Memo<'a>>>,
    signers: Option<Vec<Signer<'a>>>,
    /// The custom fields for the NFTokenBurn model.
    ///
    /// See NFTokenBurn fields:
    /// `<https://xrpl.org/nftokenburn.html#nftokenburn-fields>`
    nftoken_id: &'a str,
    owner: Option<&'a str>,
}

impl Model for NFTokenBurn<'static> {
    // fn to_json(&self) -> &str {
    //     let transaction_json = serde_json::to_string(&self).expect("Unable to convert `AccountDelete` json to string.");
    //     transaction_json
    // }

    fn to_json_value(&self) -> Value {
        let mut transaction_json =
            serde_json::to_value(&self).expect("Unable to serialize `NFTokenBurn` to json.");
        transaction_json["Flags"] = Value::from(self.iter_to_int());
        transaction_json
    }
}

impl Transaction for NFTokenBurn<'static> {
    fn get_transaction_type(&self) -> TransactionType {
        self.transaction_type.clone()
    }
}

/// Cancels existing token offers created using NFTokenCreateOffer.
///
/// See NFTokenCancelOffer:
/// `<https://xrpl.org/nftokencanceloffer.html>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "PascalCase", deserialize = "snake_case"))]
pub struct NFTokenCancelOffer<'a> {
    /// The base fields for all transaction models.
    ///
    /// See Transaction Types:
    /// `<https://xrpl.org/transaction-types.html>`
    ///
    /// See Transaction Common Fields:
    /// `<https://xrpl.org/transaction-common-fields.html>`
    #[serde(skip_serializing)]
    #[serde(default = "TransactionType::nftoken_cancel_offer")]
    transaction_type: TransactionType,
    account: &'a str,
    fee: Option<&'a str>,
    sequence: Option<u32>,
    last_ledger_sequence: Option<u32>,
    account_txn_id: Option<&'a str>,
    signing_pub_key: Option<&'a str>,
    source_tag: Option<u32>,
    ticket_sequence: Option<u32>,
    txn_signature: Option<&'a str>,
    flags: Option<Vec<u32>>,
    memos: Option<Vec<Memo<'a>>>,
    signers: Option<Vec<Signer<'a>>>,
    /// The custom fields for the NFTokenCancelOffer model.
    ///
    /// See NFTokenCancelOffer fields:
    /// `<https://xrpl.org/nftokencanceloffer.html#nftokencanceloffer-fields>`
    /// Lifetime issue
    #[serde(borrow)]
    nftoken_offers: Vec<&'a str>,
}

impl Model for NFTokenCancelOffer<'static> {
    // fn to_json(&self) -> &str {
    //     let transaction_json = serde_json::to_string(&self).expect("Unable to convert `AccountDelete` json to string.");
    //     transaction_json
    // }

    fn to_json_value(&self) -> Value {
        let mut transaction_json =
            serde_json::to_value(&self).expect("Unable to serialize `NFTokenCancelOffer` to json.");
        transaction_json["Flags"] = Value::from(self.iter_to_int());
        transaction_json
    }

    fn get_errors(&self) -> Result<(), XRPLModelException> {
        match self.get_nftoken_offers_error() {
            Err(error) => Err(XRPLModelException::XRPLTransactionError(
                XRPLTransactionException::NFTokenCancelOfferError(error),
            )),
            Ok(_no_error) => Ok(()),
        }
    }
}

impl Transaction for NFTokenCancelOffer<'static> {
    fn get_transaction_type(&self) -> TransactionType {
        self.transaction_type.clone()
    }
}

impl NFTokenCancelOfferError for NFTokenCancelOffer<'static> {
    fn get_nftoken_offers_error(&self) -> Result<(), NFTokenCancelOfferException> {
        match self.nftoken_offers.is_empty() {
            true => Err(NFTokenCancelOfferException::InvalidMustIncludeOneNFTokenOffer),
            false => Ok(()),
        }
    }
}

/// Creates either a new Sell offer for an NFToken owned by
/// the account executing the transaction, or a new Buy
/// offer for an NFToken owned by another account.
///
/// See NFTokenCreateOffer:
/// `<https://xrpl.org/nftokencreateoffer.html>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "PascalCase", deserialize = "snake_case"))]
pub struct NFTokenCreateOffer<'a> {
    /// The base fields for all transaction models.
    ///
    /// See Transaction Types:
    /// `<https://xrpl.org/transaction-types.html>`
    ///
    /// See Transaction Common Fields:
    /// `<https://xrpl.org/transaction-common-fields.html>`
    #[serde(skip_serializing)]
    #[serde(default = "TransactionType::nftoken_create_offer")]
    transaction_type: TransactionType,
    account: &'a str,
    fee: Option<&'a str>,
    sequence: Option<u32>,
    last_ledger_sequence: Option<u32>,
    account_txn_id: Option<&'a str>,
    signing_pub_key: Option<&'a str>,
    source_tag: Option<u32>,
    ticket_sequence: Option<u32>,
    txn_signature: Option<&'a str>,
    flags: Option<Vec<NFTokenCreateOfferFlag>>,
    memos: Option<Vec<Memo<'a>>>,
    signers: Option<Vec<Signer<'a>>>,
    /// The custom fields for the NFTokenCreateOffer model.
    ///
    /// See NFTokenCreateOffer fields:
    /// `<https://xrpl.org/nftokencreateoffer.html#nftokencreateoffer-fields>`
    nftoken_id: &'a str,
    amount: Currency,
    owner: Option<&'a str>,
    expiration: Option<u32>,
    destination: Option<&'a str>,
}

impl Model for NFTokenCreateOffer<'static> {
    // fn to_json(&self) -> &str {
    //     let transaction_json = serde_json::to_string(&self).expect("Unable to convert `AccountDelete` json to string.");
    //     transaction_json
    // }

    fn to_json_value(&self) -> Value {
        let mut transaction_json =
            serde_json::to_value(&self).expect("Unable to serialize `NFTokenCreateOffer` to json.");
        transaction_json["Flags"] = Value::from(self.iter_to_int());
        transaction_json
    }

    fn get_errors(&self) -> Result<(), XRPLModelException> {
        match self.get_amount_error() {
            Err(error) => Err(XRPLModelException::XRPLTransactionError(
                XRPLTransactionException::NFTokenCreateOfferError(error),
            )),
            Ok(_no_error) => match self.get_destination_error() {
                Err(error) => Err(XRPLModelException::XRPLTransactionError(
                    XRPLTransactionException::NFTokenCreateOfferError(error),
                )),
                Ok(_no_error) => match self.get_owner_error() {
                    Err(error) => Err(XRPLModelException::XRPLTransactionError(
                        XRPLTransactionException::NFTokenCreateOfferError(error),
                    )),
                    Ok(_no_error) => Ok(()),
                },
            },
        }
    }
}

impl Transaction for NFTokenCreateOffer<'static> {
    fn iter_to_int(&self) -> u32 {
        let mut flags_int: u32 = 0;
        if self.flags.is_some() {
            for flag in self.flags.as_ref().unwrap() {
                match flag {
                    &NFTokenCreateOfferFlag::TfSellOffer => flags_int += 0x00000001,
                }
            }
        }
        flags_int
    }

    fn has_flag(&self, flag: Flag) -> bool {
        let mut has_flag = false;
        if self.iter_to_int() > 0 {
            match flag {
                Flag::NFTokenCreateOffer(nftoken_create_offer_flag) => {
                    match nftoken_create_offer_flag {
                        NFTokenCreateOfferFlag::TfSellOffer => {
                            if self
                                .flags
                                .as_ref()
                                .unwrap()
                                .contains(&NFTokenCreateOfferFlag::TfSellOffer)
                            {
                                has_flag = true
                            };
                        }
                    }
                }
                _ => has_flag = false,
            };
        }
        has_flag
    }

    fn get_transaction_type(&self) -> TransactionType {
        self.transaction_type.clone()
    }
}

impl NFTokenCreateOfferError for NFTokenCreateOffer<'static> {
    fn get_amount_error(&self) -> Result<(), NFTokenCreateOfferException> {
        match !self.has_flag(Flag::NFTokenCreateOffer(
            NFTokenCreateOfferFlag::TfSellOffer,
        )) && self.amount.get_value_as_u32() == 0
        {
            true => Err(NFTokenCreateOfferException::InvalidAmountMustBeGreaterZero),
            false => Ok(()),
        }
    }

    fn get_destination_error(&self) -> Result<(), NFTokenCreateOfferException> {
        match self.destination {
            Some(destination) => match destination == self.account {
                true => Err(NFTokenCreateOfferException::InvalidDestinationMustNotEqualAccount),
                false => Ok(()),
            },
            None => Ok(()),
        }
    }

    fn get_owner_error(&self) -> Result<(), NFTokenCreateOfferException> {
        match self.owner {
            Some(owner) => match self.has_flag(Flag::NFTokenCreateOffer(
                NFTokenCreateOfferFlag::TfSellOffer,
            )) {
                true => Err(NFTokenCreateOfferException::InvalidOwnerMustNotBeSet),
                false => match owner == self.account {
                    true => Err(NFTokenCreateOfferException::InvalidOwnerMustNotEqualAccount),
                    false => Ok(()),
                },
            },
            None => match !self.has_flag(Flag::NFTokenCreateOffer(
                NFTokenCreateOfferFlag::TfSellOffer,
            )) {
                true => Err(NFTokenCreateOfferException::InvalidOwnerMustBeSet),
                false => Ok(()),
            },
        }
    }
}

/// The NFTokenMint transaction creates a non-fungible token and adds it to
/// the relevant NFTokenPage object of the NFTokenMinter as an NFToken object.
///
/// See NFTokenMint:
/// `<https://xrpl.org/nftokenmint.html>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "PascalCase", deserialize = "snake_case"))]
pub struct NFTokenMint<'a> {
    /// The base fields for all transaction models.
    ///
    /// See Transaction Types:
    /// `<https://xrpl.org/transaction-types.html>`
    ///
    /// See Transaction Common Fields:
    /// `<https://xrpl.org/transaction-common-fields.html>`
    #[serde(skip_serializing)]
    #[serde(default = "TransactionType::nftoken_mint")]
    transaction_type: TransactionType,
    account: &'a str,
    fee: Option<&'a str>,
    sequence: Option<u32>,
    last_ledger_sequence: Option<u32>,
    account_txn_id: Option<&'a str>,
    signing_pub_key: Option<&'a str>,
    source_tag: Option<u32>,
    ticket_sequence: Option<u32>,
    txn_signature: Option<&'a str>,
    flags: Option<Vec<NFTokenMintFlag>>,
    memos: Option<Vec<Memo<'a>>>,
    signers: Option<Vec<Signer<'a>>>,
    /// The custom fields for the NFTokenMint model.
    ///
    /// See NFTokenMint fields:
    /// `<https://xrpl.org/nftokenmint.html#nftokenmint-fields>`
    nftoken_taxon: u32,
    issuer: Option<&'a str>,
    transfer_fee: Option<u32>,
    uri: Option<&'a str>,
}

impl Model for NFTokenMint<'static> {
    // fn to_json(&self) -> &str {
    //     let transaction_json = serde_json::to_string(&self).expect("Unable to convert `AccountDelete` json to string.");
    //     transaction_json
    // }

    fn to_json_value(&self) -> Value {
        let mut transaction_json =
            serde_json::to_value(&self).expect("Unable to serialize `NFTokenMint` to json.");
        transaction_json["Flags"] = Value::from(self.iter_to_int());
        transaction_json
    }

    fn get_errors(&self) -> Result<(), XRPLModelException> {
        todo!()
    }
}

impl Transaction for NFTokenMint<'static> {
    fn iter_to_int(&self) -> u32 {
        let mut flags_int: u32 = 0;
        if self.flags.is_some() {
            for flag in self.flags.as_ref().unwrap() {
                match flag {
                    NFTokenMintFlag::TfBurnable => flags_int += 0x00000001,
                    NFTokenMintFlag::TfOnlyXRP => flags_int += 0x00000002,
                    NFTokenMintFlag::TfTrustline => flags_int += 0x00000004,
                    NFTokenMintFlag::TfTransferable => flags_int += 0x00000008,
                }
            }
        }
        flags_int
    }

    fn has_flag(&self, flag: Flag) -> bool {
        let mut has_flag = false;
        if self.iter_to_int() > 0 {
            match flag {
                Flag::NFTokenMint(nftoken_mint_flag) => match nftoken_mint_flag {
                    NFTokenMintFlag::TfBurnable => {
                        if self
                            .flags
                            .as_ref()
                            .unwrap()
                            .contains(&NFTokenMintFlag::TfBurnable)
                        {
                            has_flag = true
                        };
                    }
                    NFTokenMintFlag::TfOnlyXRP => {
                        if self
                            .flags
                            .as_ref()
                            .unwrap()
                            .contains(&NFTokenMintFlag::TfOnlyXRP)
                        {
                            has_flag = true
                        };
                    }
                    NFTokenMintFlag::TfTransferable => {
                        if self
                            .flags
                            .as_ref()
                            .unwrap()
                            .contains(&NFTokenMintFlag::TfTransferable)
                        {
                            has_flag = true
                        };
                    }
                    NFTokenMintFlag::TfTrustline => {
                        if self
                            .flags
                            .as_ref()
                            .unwrap()
                            .contains(&NFTokenMintFlag::TfTrustline)
                        {
                            has_flag = true
                        };
                    }
                },
                _ => has_flag = false,
            };
        }
        has_flag
    }

    fn get_transaction_type(&self) -> TransactionType {
        self.transaction_type.clone()
    }
}

impl NFTokenMintError for NFTokenMint<'static> {
    fn get_issuer_error(&self) -> Result<(), NFTokenMintException> {
        match self.issuer {
            Some(issuer) => match issuer == self.account {
                true => Err(NFTokenMintException::InvalidIssuerMustNotEqualAccount),
                false => Ok(()),
            },
            None => Ok(()),
        }
    }

    fn get_transfer_fee_error(&self) -> Result<(), NFTokenMintException> {
        match self.transfer_fee {
            Some(transfer_fee) => match transfer_fee > MAX_TRANSFER_FEE {
                true => Err(NFTokenMintException::InvalidTransferFeeTooHigh {
                    max: MAX_TRANSFER_FEE,
                    found: transfer_fee,
                }),
                false => Ok(()),
            },
            None => Ok(()),
        }
    }

    fn get_uri_error(&self) -> Result<(), NFTokenMintException> {
        match self.uri {
            Some(uri) => match uri.len() > MAX_URI_LENGTH {
                true => Err(NFTokenMintException::InvalidURITooLong {
                    max: MAX_URI_LENGTH,
                    found: uri.len(),
                }),
                false => Ok(()),
            },
            None => Ok(()),
        }
    }
}

/// Removes an Offer object from the XRP Ledger.
///
/// See OfferCancel:
/// `<https://xrpl.org/offercancel.html>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "PascalCase", deserialize = "snake_case"))]
pub struct OfferCancel<'a> {
    /// The base fields for all transaction models.
    ///
    /// See Transaction Types:
    /// `<https://xrpl.org/transaction-types.html>`
    ///
    /// See Transaction Common Fields:
    /// `<https://xrpl.org/transaction-common-fields.html>`
    #[serde(skip_serializing)]
    #[serde(default = "TransactionType::offer_cancel")]
    transaction_type: TransactionType,
    account: &'a str,
    fee: Option<&'a str>,
    sequence: Option<u32>,
    last_ledger_sequence: Option<u32>,
    account_txn_id: Option<&'a str>,
    signing_pub_key: Option<&'a str>,
    source_tag: Option<u32>,
    ticket_sequence: Option<u32>,
    txn_signature: Option<&'a str>,
    flags: Option<Vec<u32>>,
    memos: Option<Vec<Memo<'a>>>,
    signers: Option<Vec<Signer<'a>>>,
    /// The custom fields for the OfferCancel model.
    ///
    /// See OfferCancel fields:
    /// `<https://xrpl.org/offercancel.html#offercancel-fields>`
    offer_sequence: u32,
}

impl Model for OfferCancel<'static> {
    // fn to_json(&self) -> &str {
    //     let transaction_json = serde_json::to_string(&self).expect("Unable to convert `AccountDelete` json to string.");
    //     transaction_json
    // }

    fn to_json_value(&self) -> Value {
        let mut transaction_json =
            serde_json::to_value(&self).expect("Unable to serialize `OfferCancel` to json.");
        transaction_json["Flags"] = Value::from(self.iter_to_int());
        transaction_json
    }
}

impl Transaction for OfferCancel<'static> {
    fn get_transaction_type(&self) -> TransactionType {
        self.transaction_type.clone()
    }
}

/// Places an Offer in the decentralized exchange.
///
/// See OfferCreate:
/// `<https://xrpl.org/offercreate.html>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "PascalCase", deserialize = "snake_case"))]
pub struct OfferCreate<'a> {
    /// The base fields for all transaction models.
    ///
    /// See Transaction Types:
    /// `<https://xrpl.org/transaction-types.html>`
    ///
    /// See Transaction Common Fields:
    /// `<https://xrpl.org/transaction-common-fields.html>`
    #[serde(skip_serializing)]
    #[serde(default = "TransactionType::offer_create")]
    transaction_type: TransactionType,
    account: &'a str,
    fee: Option<&'a str>,
    sequence: Option<u32>,
    last_ledger_sequence: Option<u32>,
    account_txn_id: Option<&'a str>,
    signing_pub_key: Option<&'a str>,
    source_tag: Option<u32>,
    ticket_sequence: Option<u32>,
    txn_signature: Option<&'a str>,
    flags: Option<Vec<OfferCreateFlag>>,
    memos: Option<Vec<Memo<'a>>>,
    signers: Option<Vec<Signer<'a>>>,
    /// The custom fields for the OfferCreate model.
    ///
    /// See OfferCreate fields:
    /// `<https://xrpl.org/offercreate.html#offercreate-fields>`
    taker_gets: Currency,
    taker_pays: Currency,
    expiration: Option<u32>,
    offer_sequence: Option<u32>,
}

impl Model for OfferCreate<'static> {
    // fn to_json(&self) -> &str {
    //     let transaction_json = serde_json::to_string(&self).expect("Unable to convert `AccountDelete` json to string.");
    //     transaction_json
    // }

    fn to_json_value(&self) -> Value {
        let mut transaction_json =
            serde_json::to_value(&self).expect("Unable to serialize `OfferCreate` to json.");
        transaction_json["Flags"] = Value::from(self.iter_to_int());
        transaction_json
    }
}

impl Transaction for OfferCreate<'static> {
    fn iter_to_int(&self) -> u32 {
        let mut flags_int: u32 = 0;
        if self.flags.is_some() {
            for flag in self.flags.as_ref().unwrap() {
                match flag {
                    OfferCreateFlag::TfPassive => flags_int += 0x00010000,
                    OfferCreateFlag::TfImmediateOrCancel => flags_int += 0x00020000,
                    OfferCreateFlag::TfFillOrKill => flags_int += 0x00040000,
                    OfferCreateFlag::TfSell => flags_int += 0x00080000,
                }
            }
        }
        flags_int
    }

    fn has_flag(&self, flag: Flag) -> bool {
        let mut has_flag = false;
        if self.iter_to_int() > 0 {
            match flag {
                Flag::OfferCreate(offer_create_flag) => match offer_create_flag {
                    OfferCreateFlag::TfFillOrKill => {
                        if self
                            .flags
                            .as_ref()
                            .unwrap()
                            .contains(&OfferCreateFlag::TfFillOrKill)
                        {
                            has_flag = true
                        };
                    }
                    OfferCreateFlag::TfImmediateOrCancel => {
                        if self
                            .flags
                            .as_ref()
                            .unwrap()
                            .contains(&OfferCreateFlag::TfImmediateOrCancel)
                        {
                            has_flag = true
                        };
                    }
                    OfferCreateFlag::TfPassive => {
                        if self
                            .flags
                            .as_ref()
                            .unwrap()
                            .contains(&OfferCreateFlag::TfPassive)
                        {
                            has_flag = true
                        };
                    }
                    OfferCreateFlag::TfSell => {
                        if self
                            .flags
                            .as_ref()
                            .unwrap()
                            .contains(&OfferCreateFlag::TfSell)
                        {
                            has_flag = true
                        };
                    }
                },
                _ => has_flag = false,
            };
        }
        has_flag
    }

    fn get_transaction_type(&self) -> TransactionType {
        self.transaction_type.clone()
    }
}

/// Transfers value from one account to another.
///
/// See Payment:
/// `<https://xrpl.org/payment.html>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "PascalCase", deserialize = "snake_case"))]
pub struct Payment<'a> {
    /// The base fields for all transaction models.
    ///
    /// See Transaction Types:
    /// `<https://xrpl.org/transaction-types.html>`
    ///
    /// See Transaction Common Fields:
    /// `<https://xrpl.org/transaction-common-fields.html>`
    #[serde(skip_serializing)]
    #[serde(default = "TransactionType::payment")]
    transaction_type: TransactionType,
    account: &'a str,
    fee: Option<&'a str>,
    sequence: Option<u32>,
    last_ledger_sequence: Option<u32>,
    account_txn_id: Option<&'a str>,
    signing_pub_key: Option<&'a str>,
    source_tag: Option<u32>,
    ticket_sequence: Option<u32>,
    txn_signature: Option<&'a str>,
    flags: Option<Vec<PaymentFlag>>,
    memos: Option<Vec<Memo<'a>>>,
    signers: Option<Vec<Signer<'a>>>,
    /// The custom fields for the Payment model.
    ///
    /// See Payment fields:
    /// `<https://xrpl.org/payment.html#payment-fields>`
    amount: Currency,
    destination: &'a str,
    destination_tag: Option<u32>,
    invoice_id: Option<u32>,
    paths: Option<Vec<Vec<PathStep<'a>>>>,
    send_max: Option<Currency>,
    deliver_min: Option<Currency>,
}

impl Model for Payment<'static> {
    // fn to_json(&self) -> &str {
    //     let transaction_json = serde_json::to_string(&self).expect("Unable to convert `AccountDelete` json to string.");
    //     transaction_json
    // }

    fn to_json_value(&self) -> Value {
        let mut transaction_json =
            serde_json::to_value(&self).expect("Unable to serialize `Payment` to json.");
        transaction_json["Flags"] = Value::from(self.iter_to_int());
        transaction_json
    }

    fn get_errors(&self) -> Result<(), XRPLModelException> {
        match self.get_xrp_transaction_error() {
            Err(error) => Err(XRPLModelException::XRPLTransactionError(
                XRPLTransactionException::PaymentError(error),
            )),
            Ok(_no_error) => match self.get_partial_payment_error() {
                Err(error) => Err(XRPLModelException::XRPLTransactionError(
                    XRPLTransactionException::PaymentError(error),
                )),
                Ok(_no_error) => match self.get_exchange_error() {
                    Err(error) => Err(XRPLModelException::XRPLTransactionError(
                        XRPLTransactionException::PaymentError(error),
                    )),
                    Ok(_no_error) => Ok(()),
                },
            },
        }
    }
}

impl Transaction for Payment<'static> {
    fn iter_to_int(&self) -> u32 {
        let mut flags_int: u32 = 0;
        if self.flags.is_some() {
            for flag in self.flags.as_ref().unwrap() {
                match flag {
                    PaymentFlag::TfNoDirectRipple => flags_int += 0x00010000,
                    PaymentFlag::TfPartialPayment => flags_int += 0x00020000,
                    PaymentFlag::TfLimitQuality => flags_int += 0x00040000,
                }
            }
        }
        flags_int
    }

    fn has_flag(&self, flag: Flag) -> bool {
        let mut has_flag = false;
        if self.iter_to_int() > 0 {
            match flag {
                Flag::Payment(payment_flag) => match payment_flag {
                    PaymentFlag::TfLimitQuality => {
                        if self
                            .flags
                            .as_ref()
                            .unwrap()
                            .contains(&PaymentFlag::TfLimitQuality)
                        {
                            has_flag = true
                        };
                    }
                    PaymentFlag::TfNoDirectRipple => {
                        if self
                            .flags
                            .as_ref()
                            .unwrap()
                            .contains(&PaymentFlag::TfNoDirectRipple)
                        {
                            has_flag = true
                        };
                    }
                    PaymentFlag::TfPartialPayment => {
                        if self
                            .flags
                            .as_ref()
                            .unwrap()
                            .contains(&PaymentFlag::TfPartialPayment)
                        {
                            has_flag = true
                        };
                    }
                },
                _ => has_flag = false,
            };
        }
        has_flag
    }

    fn get_transaction_type(&self) -> TransactionType {
        self.transaction_type.clone()
    }
}

impl PaymentError for Payment<'static> {
    fn get_xrp_transaction_error(&self) -> Result<(), PaymentException> {
        match self.send_max.as_ref() {
            Some(_send_max) => Ok(()),
            None => match self.amount.is_xrp() {
                true => match self.paths.as_ref() {
                    Some(_paths) => Err(PaymentException::InvalidXRPtoXRPPaymentsCannotContainPaths),
                    None => match self.account == self.destination {
                        true => Err(PaymentException::InvalidDestinationMustNotEqualAccountForRPtoXRPPayments),
                        false => Ok(()),
                    }
                },
                false => Ok(()),
            }
        }
    }

    fn get_partial_payment_error(&self) -> Result<(), PaymentException> {
        match self.send_max.as_ref() {
            Some(send_max) => match !self.has_flag(Flag::Payment(PaymentFlag::TfPartialPayment)) {
                true => match send_max.is_xrp() && self.amount.is_xrp() {
                    true => Err(
                        PaymentException::InvalidSendMaxMustNotBeSetForXRPtoXRPNonPartialPayments,
                    ),
                    false => Ok(()),
                },
                false => Ok(()),
            },
            None => match self.has_flag(Flag::Payment(PaymentFlag::TfPartialPayment)) {
                true => Err(PaymentException::InvalidSendMaxMustBeSetForPartialPayments),
                false => match self.deliver_min.as_ref() {
                    Some(_deliver_min) => {
                        Err(PaymentException::InvalidDeliverMinMustNotBeSetForNonPartialPayments)
                    }
                    None => Ok(()),
                },
            },
        }
    }

    fn get_exchange_error(&self) -> Result<(), PaymentException> {
        match self.send_max.as_ref() {
            Some(_send_max) => Ok(()),
            None => match self.account == self.destination {
                true => Err(PaymentException::InvalidSendMaxMustBeSetForExchanges),
                false => Ok(()),
            },
        }
    }
}

/// Claim XRP from a payment channel, adjust
/// the payment channel's expiration, or both.
///
/// See PaymentChannelClaim:
/// `<https://xrpl.org/paymentchannelclaim.html>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "PascalCase", deserialize = "snake_case"))]
pub struct PaymentChannelClaim<'a> {
    /// The base fields for all transaction models.
    ///
    /// See Transaction Types:
    /// `<https://xrpl.org/transaction-types.html>`
    ///
    /// See Transaction Common Fields:
    /// `<https://xrpl.org/transaction-common-fields.html>`
    #[serde(skip_serializing)]
    #[serde(default = "TransactionType::payment_channel_claim")]
    transaction_type: TransactionType,
    account: &'a str,
    fee: Option<&'a str>,
    sequence: Option<u32>,
    last_ledger_sequence: Option<u32>,
    account_txn_id: Option<&'a str>,
    signing_pub_key: Option<&'a str>,
    source_tag: Option<u32>,
    ticket_sequence: Option<u32>,
    txn_signature: Option<&'a str>,
    flags: Option<Vec<PaymentChannelClaimFlag>>,
    memos: Option<Vec<Memo<'a>>>,
    signers: Option<Vec<Signer<'a>>>,
    /// The custom fields for the PaymentChannelClaim model.
    ///
    /// See PaymentChannelClaim fields:
    /// `<https://xrpl.org/paymentchannelclaim.html#paymentchannelclaim-fields>`
    channel: &'a str,
    balance: Option<&'a str>,
    amount: Option<&'a str>,
    signature: Option<&'a str>,
    public_key: Option<&'a str>,
}

impl Model for PaymentChannelClaim<'static> {
    // fn to_json(&self) -> &str {
    //     let transaction_json = serde_json::to_string(&self).expect("Unable to convert `AccountDelete` json to string.");
    //     transaction_json
    // }

    fn to_json_value(&self) -> Value {
        let mut transaction_json = serde_json::to_value(&self)
            .expect("Unable to serialize `PaymentChannelClaim` to json.");
        transaction_json["Flags"] = Value::from(self.iter_to_int());
        transaction_json
    }
}

impl Transaction for PaymentChannelClaim<'static> {
    fn iter_to_int(&self) -> u32 {
        let mut flags_int: u32 = 0;
        if self.flags.is_some() {
            for flag in self.flags.as_ref().unwrap() {
                match flag {
                    PaymentChannelClaimFlag::TfRenew => flags_int += 0x00010000,
                    PaymentChannelClaimFlag::TfClose => flags_int += 0x00020000,
                }
            }
        }
        flags_int
    }

    fn has_flag(&self, flag: Flag) -> bool {
        let mut has_flag = false;
        if self.iter_to_int() > 0 {
            match flag {
                Flag::PaymentChannelClaim(payment_channel_claim_flag) => {
                    match payment_channel_claim_flag {
                        PaymentChannelClaimFlag::TfClose => {
                            if self
                                .flags
                                .as_ref()
                                .unwrap()
                                .contains(&PaymentChannelClaimFlag::TfClose)
                            {
                                has_flag = true
                            };
                        }
                        PaymentChannelClaimFlag::TfRenew => {
                            if self
                                .flags
                                .as_ref()
                                .unwrap()
                                .contains(&PaymentChannelClaimFlag::TfRenew)
                            {
                                has_flag = true
                            };
                        }
                    }
                }
                _ => has_flag = false,
            };
        }
        has_flag
    }

    fn get_transaction_type(&self) -> TransactionType {
        self.transaction_type.clone()
    }
}

/// Create a unidirectional channel and fund it with XRP.
///
/// See PaymentChannelCreate fields:
/// `<https://xrpl.org/paymentchannelcreate.html>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "PascalCase", deserialize = "snake_case"))]
pub struct PaymentChannelCreate<'a> {
    /// The base fields for all transaction models.
    ///
    /// See Transaction Types:
    /// `<https://xrpl.org/transaction-types.html>`
    ///
    /// See Transaction Common Fields:
    /// `<https://xrpl.org/transaction-common-fields.html>`
    #[serde(skip_serializing)]
    #[serde(default = "TransactionType::payment_channel_create")]
    transaction_type: TransactionType,
    account: &'a str,
    fee: Option<&'a str>,
    sequence: Option<u32>,
    last_ledger_sequence: Option<u32>,
    account_txn_id: Option<&'a str>,
    signing_pub_key: Option<&'a str>,
    source_tag: Option<u32>,
    ticket_sequence: Option<u32>,
    txn_signature: Option<&'a str>,
    flags: Option<Vec<u32>>,
    memos: Option<Vec<Memo<'a>>>,
    signers: Option<Vec<Signer<'a>>>,
    /// The custom fields for the PaymentChannelCreate model.
    ///
    /// See PaymentChannelCreate fields:
    /// `<https://xrpl.org/paymentchannelcreate.html#paymentchannelcreate-fields>`
    amount: Currency,
    destination: &'a str,
    settle_delay: u32,
    public_key: &'a str,
    cancel_after: Option<u32>,
    destination_tag: Option<u32>,
}

impl Model for PaymentChannelCreate<'static> {
    // fn to_json(&self) -> &str {
    //     let transaction_json = serde_json::to_string(&self).expect("Unable to convert `AccountDelete` json to string.");
    //     transaction_json
    // }

    fn to_json_value(&self) -> Value {
        let mut transaction_json = serde_json::to_value(&self)
            .expect("Unable to serialize `PaymentChannelCreate` to json.");
        transaction_json["Flags"] = Value::from(self.iter_to_int());
        transaction_json
    }
}

impl Transaction for PaymentChannelCreate<'static> {
    fn get_transaction_type(&self) -> TransactionType {
        self.transaction_type.clone()
    }
}

/// Add additional XRP to an open payment channel,
/// and optionally update the expiration time of the channel.
///
/// See PaymentChannelFund:
/// `<https://xrpl.org/paymentchannelfund.html>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "PascalCase", deserialize = "snake_case"))]
pub struct PaymentChannelFund<'a> {
    /// The base fields for all transaction models.
    ///
    /// See Transaction Types:
    /// `<https://xrpl.org/transaction-types.html>`
    ///
    /// See Transaction Common Fields:
    /// `<https://xrpl.org/transaction-common-fields.html>`
    #[serde(skip_serializing)]
    #[serde(default = "TransactionType::payment_channel_fund")]
    transaction_type: TransactionType,
    account: &'a str,
    fee: Option<&'a str>,
    sequence: Option<u32>,
    last_ledger_sequence: Option<u32>,
    account_txn_id: Option<&'a str>,
    signing_pub_key: Option<&'a str>,
    source_tag: Option<u32>,
    ticket_sequence: Option<u32>,
    txn_signature: Option<&'a str>,
    flags: Option<Vec<u32>>,
    memos: Option<Vec<Memo<'a>>>,
    signers: Option<Vec<Signer<'a>>>,
    /// The custom fields for the PaymentChannelFund model.
    ///
    /// See PaymentChannelFund fields:
    /// `<https://xrpl.org/paymentchannelfund.html#paymentchannelfund-fields>`
    channel: &'a str,
    amount: &'a str,
    expiration: Option<u32>,
}

impl Model for PaymentChannelFund<'static> {
    // fn to_json(&self) -> &str {
    //     let transaction_json = serde_json::to_string(&self).expect("Unable to convert `AccountDelete` json to string.");
    //     transaction_json
    // }

    fn to_json_value(&self) -> Value {
        let mut transaction_json =
            serde_json::to_value(&self).expect("Unable to serialize `PaymentChannelFund` to json.");
        transaction_json["Flags"] = Value::from(self.iter_to_int());
        transaction_json
    }
}

impl Transaction for PaymentChannelFund<'static> {
    fn get_transaction_type(&self) -> TransactionType {
        self.transaction_type.clone()
    }
}

/// You can protect your account by assigning a regular key pair to
/// it and using it instead of the master key pair to sign transactions
/// whenever possible. If your regular key pair is compromised, but
/// your master key pair is not, you can use a SetRegularKey transaction
/// to regain control of your account.
///
/// See SetRegularKey:
/// `<https://xrpl.org/setregularkey.html>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "PascalCase", deserialize = "snake_case"))]
pub struct SetRegularKey<'a> {
    /// The base fields for all transaction models.
    ///
    /// See Transaction Types:
    /// `<https://xrpl.org/transaction-types.html>`
    ///
    /// See Transaction Common Fields:
    /// `<https://xrpl.org/transaction-common-fields.html>`
    #[serde(skip_serializing)]
    #[serde(default = "TransactionType::set_regular_key")]
    transaction_type: TransactionType,
    account: &'a str,
    fee: Option<&'a str>,
    sequence: Option<u32>,
    last_ledger_sequence: Option<u32>,
    account_txn_id: Option<&'a str>,
    signing_pub_key: Option<&'a str>,
    source_tag: Option<u32>,
    ticket_sequence: Option<u32>,
    txn_signature: Option<&'a str>,
    flags: Option<Vec<u32>>,
    memos: Option<Vec<Memo<'a>>>,
    signers: Option<Vec<Signer<'a>>>,
    /// The custom fields for the SetRegularKey model.
    ///
    /// See SetRegularKey fields:
    /// `<https://xrpl.org/setregularkey.html#setregularkey-fields>`
    regular_key: Option<&'a str>,
}

impl Model for SetRegularKey<'static> {
    // fn to_json(&self) -> &str {
    //     let transaction_json = serde_json::to_string(&self).expect("Unable to convert `AccountDelete` json to string.");
    //     transaction_json
    // }

    fn to_json_value(&self) -> Value {
        let mut transaction_json =
            serde_json::to_value(&self).expect("Unable to serialize `SetRegularKey` to json.");
        transaction_json["Flags"] = Value::from(self.iter_to_int());
        transaction_json
    }
}

impl Transaction for SetRegularKey<'static> {
    fn get_transaction_type(&self) -> TransactionType {
        self.transaction_type.clone()
    }
}

/// The SignerList object type represents a list of parties that,
/// as a group, are authorized to sign a transaction in place of an
/// individual account. You can create, replace, or remove a signer
/// list using a SignerListSet transaction.
///
/// See TicketCreate:
/// `<https://xrpl.org/signerlistset.html>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "PascalCase", deserialize = "snake_case"))]
pub struct SignerListSet<'a> {
    /// The base fields for all transaction models.
    ///
    /// See Transaction Types:
    /// `<https://xrpl.org/transaction-types.html>`
    ///
    /// See Transaction Common Fields:
    /// `<https://xrpl.org/transaction-common-fields.html>`
    #[serde(skip_serializing)]
    #[serde(default = "TransactionType::ticket_create")]
    transaction_type: TransactionType,
    account: &'a str,
    fee: Option<&'a str>,
    sequence: Option<u32>,
    last_ledger_sequence: Option<u32>,
    account_txn_id: Option<&'a str>,
    signing_pub_key: Option<&'a str>,
    source_tag: Option<u32>,
    ticket_sequence: Option<u32>,
    txn_signature: Option<&'a str>,
    flags: Option<Vec<u32>>,
    memos: Option<Vec<Memo<'a>>>,
    signers: Option<Vec<Signer<'a>>>,
    /// The custom fields for the TicketCreate model.
    ///
    /// See TicketCreate fields:
    /// `<https://xrpl.org/signerlistset.html#signerlistset-fields>`
    signer_quorum: u32,
    signer_entries: Option<Vec<SignerEntry<'a>>>,
}

impl Model for SignerListSet<'static> {
    // fn to_json(&self) -> &str {
    //     let transaction_json = serde_json::to_string(&self).expect("Unable to convert `AccountDelete` json to string.");
    //     transaction_json
    // }

    fn to_json_value(&self) -> Value {
        let mut transaction_json =
            serde_json::to_value(&self).expect("Unable to serialize `SignerListSet` to json.");
        transaction_json["Flags"] = Value::from(self.iter_to_int());
        transaction_json
    }

    fn get_errors(&self) -> Result<(), XRPLModelException> {
        match self.get_signer_entries_error() {
            Err(error) => Err(XRPLModelException::XRPLTransactionError(
                XRPLTransactionException::SignerListSetError(error),
            )),
            Ok(_no_error) => match self.get_signer_quorum_error() {
                Err(error) => Err(XRPLModelException::XRPLTransactionError(
                    XRPLTransactionException::SignerListSetError(error),
                )),
                Ok(_no_error) => Ok(()),
            },
        }
    }
}

impl Transaction for SignerListSet<'static> {
    fn get_transaction_type(&self) -> TransactionType {
        self.transaction_type.clone()
    }
}

impl SignerListSetError for SignerListSet<'static> {
    fn get_signer_entries_error(&self) -> Result<(), SignerListSetException> {
        match self.signer_entries.as_ref() {
            Some(signer_entries) => match self.signer_quorum == 0 {
                true => Err(SignerListSetException::InvalidMustNotSetSignerEntriesIfSignerListIsBeingDeleted),
                false => match self.signer_quorum == 0 {
                    true => Err(SignerListSetException::InvalidSignerQuorumMustBeGreaterZero),
                    false => match signer_entries.is_empty() {
                        true => Err(SignerListSetException::InvalidTooFewSignerEntries { min: 1, found: signer_entries.len() }),
                        false => match signer_entries.len() > 8 {
                            true => Err(SignerListSetException::InvalidTooManySignerEntries { max: 8, found: signer_entries.len() }),
                            false => Ok(())
                        },
                    },
                },
            },
            None => Ok(())
        }
    }

    fn get_signer_quorum_error(&self) -> Result<(), SignerListSetException> {
        let mut accounts = Vec::new();
        let mut signer_weight_sum: u32 = 0;
        if self.signer_entries.is_some() {
            for signer_entry in self.signer_entries.as_ref().unwrap() {
                accounts.push(signer_entry.account);
                let weight: u32 = signer_entry.signer_weight.into();
                signer_weight_sum += weight;
            }
        }
        accounts.sort_unstable();
        accounts.dedup();
        match self.signer_entries.as_ref() {
            Some(_signer_entries) => match accounts.contains(&self.account) {
                true => Err(SignerListSetException::InvalidAccountMustNotBeInSignerEntry),
                false => match self.signer_quorum > signer_weight_sum {
                    true => Err(SignerListSetException::InvalidMustBeLessOrEqualToSumOfSignerWeightInSignerEntries { max: signer_weight_sum, found: self.signer_quorum }),
                    false => Ok(())
                },
            },
            None => match self.signer_quorum != 0 {
                true => Err(SignerListSetException::InvalidSignerQuorumMustBeZeroIfSignerListIsBeingDeleted),
                false => Ok(()),
            }
        }
    }
}

/// Sets aside one or more sequence numbers as Tickets.
///
/// See TicketCreate:
/// `<https://xrpl.org/ticketcreate.html>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "PascalCase", deserialize = "snake_case"))]
pub struct TicketCreate<'a> {
    /// The base fields for all transaction models.
    ///
    /// See Transaction Types:
    /// `<https://xrpl.org/transaction-types.html>`
    ///
    /// See Transaction Common Fields:
    /// `<https://xrpl.org/transaction-common-fields.html>`
    #[serde(skip_serializing)]
    #[serde(default = "TransactionType::ticket_create")]
    transaction_type: TransactionType,
    account: &'a str,
    fee: Option<&'a str>,
    sequence: Option<u32>,
    last_ledger_sequence: Option<u32>,
    account_txn_id: Option<&'a str>,
    signing_pub_key: Option<&'a str>,
    source_tag: Option<u32>,
    ticket_sequence: Option<u32>,
    txn_signature: Option<&'a str>,
    flags: Option<Vec<u32>>,
    memos: Option<Vec<Memo<'a>>>,
    signers: Option<Vec<Signer<'a>>>,
    /// The custom fields for the TicketCreate model.
    ///
    /// See TicketCreate fields:
    /// `<https://xrpl.org/ticketcreate.html#ticketcreate-fields>`
    ticket_count: u32,
}

impl Model for TicketCreate<'static> {
    // fn to_json(&self) -> &str {
    //     let transaction_json = serde_json::to_string(&self).expect("Unable to convert `AccountDelete` json to string.");
    //     transaction_json
    // }

    fn to_json_value(&self) -> Value {
        let mut transaction_json =
            serde_json::to_value(&self).expect("Unable to serialize `TicketCreate` to json.");
        transaction_json["Flags"] = Value::from(self.iter_to_int());
        transaction_json
    }
}

impl Transaction for TicketCreate<'static> {
    fn get_transaction_type(&self) -> TransactionType {
        self.transaction_type.clone()
    }
}

/// Create or modify a trust line linking two accounts.
///
/// See TrustSet:
/// `<https://xrpl.org/trustset.html>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "PascalCase", deserialize = "snake_case"))]
pub struct TrustSet<'a> {
    /// The base fields for all transaction models.
    ///
    /// See Transaction Types:
    /// `<https://xrpl.org/transaction-types.html>`
    ///
    /// See Transaction Common Fields:
    /// `<https://xrpl.org/transaction-common-fields.html>`
    #[serde(skip_serializing)]
    #[serde(default = "TransactionType::trust_set")]
    transaction_type: TransactionType,
    account: &'a str,
    fee: Option<&'a str>,
    sequence: Option<u32>,
    last_ledger_sequence: Option<u32>,
    account_txn_id: Option<&'a str>,
    signing_pub_key: Option<&'a str>,
    source_tag: Option<u32>,
    ticket_sequence: Option<u32>,
    txn_signature: Option<&'a str>,
    flags: Option<Vec<TrustSetFlag>>,
    memos: Option<Vec<Memo<'a>>>,
    signers: Option<Vec<Signer<'a>>>,
    /// The custom fields for the TrustSet model.
    ///
    /// See TrustSet fields:
    /// `<https://xrpl.org/trustset.html#trustset-fields>`
    limit_amount: Currency,
    quality_in: Option<u32>,
    quality_out: Option<u32>,
}

impl Model for TrustSet<'static> {
    // fn to_json(&self) -> &str {
    //     let transaction_json = serde_json::to_string(&self).expect("Unable to convert `AccountDelete` json to string.");
    //     transaction_json
    // }

    fn to_json_value(&self) -> Value {
        let mut transaction_json =
            serde_json::to_value(&self).expect("Unable to serialize `TrustSet` to json.");
        transaction_json["Flags"] = Value::from(self.iter_to_int());
        transaction_json
    }
}

impl Transaction for TrustSet<'static> {
    fn iter_to_int(&self) -> u32 {
        let mut flags_int: u32 = 0;
        if self.flags.is_some() {
            for flag in self.flags.as_ref().unwrap() {
                match flag {
                    TrustSetFlag::TfSetAuth => flags_int += 0x00010000,
                    TrustSetFlag::TfSetNoRipple => flags_int += 0x00020000,
                    TrustSetFlag::TfClearNoRipple => flags_int += 0x00040000,
                    TrustSetFlag::TfSetFreeze => flags_int += 0x00100000,
                    TrustSetFlag::TfClearFreeze => flags_int += 0x00200000,
                }
            }
        }
        flags_int
    }

    fn has_flag(&self, flag: Flag) -> bool {
        let mut has_flag = false;
        if self.iter_to_int() > 0 {
            match flag {
                Flag::TrustSet(trust_set_flag) => match trust_set_flag {
                    TrustSetFlag::TfClearFreeze => {
                        if self
                            .flags
                            .as_ref()
                            .unwrap()
                            .contains(&TrustSetFlag::TfClearFreeze)
                        {
                            has_flag = true
                        };
                    }
                    TrustSetFlag::TfClearNoRipple => {
                        if self
                            .flags
                            .as_ref()
                            .unwrap()
                            .contains(&TrustSetFlag::TfClearNoRipple)
                        {
                            has_flag = true
                        };
                    }
                    TrustSetFlag::TfSetAuth => {
                        if self
                            .flags
                            .as_ref()
                            .unwrap()
                            .contains(&TrustSetFlag::TfSetAuth)
                        {
                            has_flag = true
                        };
                    }
                    TrustSetFlag::TfSetFreeze => {
                        if self
                            .flags
                            .as_ref()
                            .unwrap()
                            .contains(&TrustSetFlag::TfSetFreeze)
                        {
                            has_flag = true
                        };
                    }
                    TrustSetFlag::TfSetNoRipple => {
                        if self
                            .flags
                            .as_ref()
                            .unwrap()
                            .contains(&TrustSetFlag::TfSetNoRipple)
                        {
                            has_flag = true
                        };
                    }
                },
                _ => has_flag = false,
            };
        }
        has_flag
    }

    fn get_transaction_type(&self) -> TransactionType {
        self.transaction_type.clone()
    }
}

/// Pseudo-Transactions

/// See EnableAmendment:
/// `<https://xrpl.org/enableamendment.html>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "PascalCase", deserialize = "snake_case"))]
pub struct EnableAmendment<'a> {
    /// The base fields for all transaction models.
    ///
    /// See Transaction Types:
    /// `<https://xrpl.org/transaction-types.html>`
    ///
    /// See Transaction Common Fields:
    /// `<https://xrpl.org/transaction-common-fields.html>`
    #[serde(skip_serializing)]
    #[serde(default = "TransactionType::enable_amendment")]
    transaction_type: TransactionType,
    account: &'a str,
    fee: Option<&'a str>,
    sequence: Option<u32>,
    signing_pub_key: Option<&'a str>,
    source_tag: Option<u32>,
    txn_signature: Option<&'a str>,
    flags: Option<Vec<EnableAmendmentFlag>>,
    /// The custom fields for the EnableAmendment model.
    ///
    /// See EnableAmendment fields:
    /// `<https://xrpl.org/enableamendment.html#enableamendment-fields>`
    amendment: &'a str,
    ledger_sequence: u32,
}

impl Model for EnableAmendment<'static> {
    // fn to_json(&self) -> &str {
    //     let transaction_json = serde_json::to_string(&self).expect("Unable to convert `AccountDelete` json to string.");
    //     transaction_json
    // }

    fn to_json_value(&self) -> Value {
        let mut transaction_json =
            serde_json::to_value(&self).expect("Unable to serialize `EnableAmendment` to json.");
        transaction_json["Flags"] = Value::from(self.iter_to_int());
        transaction_json
    }
}

impl Transaction for EnableAmendment<'static> {
    fn iter_to_int(&self) -> u32 {
        let mut flags_int: u32 = 0;
        if self.flags.is_some() {
            for flag in self.flags.as_ref().unwrap() {
                match flag {
                    EnableAmendmentFlag::TfGotMajority => flags_int += 0x00010000,
                    EnableAmendmentFlag::TfLostMajority => flags_int += 0x00020000,
                }
            }
        }
        flags_int
    }

    fn has_flag(&self, flag: Flag) -> bool {
        let mut has_flag = false;
        if self.iter_to_int() > 0 {
            match flag {
                Flag::EnableAmendment(enable_amendment_flag) => match enable_amendment_flag {
                    EnableAmendmentFlag::TfGotMajority => {
                        if self
                            .flags
                            .as_ref()
                            .unwrap()
                            .contains(&EnableAmendmentFlag::TfGotMajority)
                        {
                            has_flag = true
                        };
                    }
                    EnableAmendmentFlag::TfLostMajority => {
                        if self
                            .flags
                            .as_ref()
                            .unwrap()
                            .contains(&EnableAmendmentFlag::TfLostMajority)
                        {
                            has_flag = true
                        };
                    }
                },
                _ => has_flag = false,
            };
        }
        has_flag
    }

    fn get_transaction_type(&self) -> TransactionType {
        self.transaction_type.clone()
    }
}

/// See SetFee:
/// `<https://xrpl.org/setfee.html>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "PascalCase", deserialize = "snake_case"))]
pub struct SetFee<'a> {
    /// The base fields for all transaction models.
    ///
    /// See Transaction Types:
    /// `<https://xrpl.org/transaction-types.html>`
    ///
    /// See Transaction Common Fields:
    /// `<https://xrpl.org/transaction-common-fields.html>`
    #[serde(skip_serializing)]
    #[serde(default = "TransactionType::set_fee")]
    transaction_type: TransactionType,
    account: &'a str,
    fee: Option<&'a str>,
    sequence: Option<u32>,
    signing_pub_key: Option<&'a str>,
    source_tag: Option<u32>,
    txn_signature: Option<&'a str>,
    flags: Option<Vec<u32>>,
    /// The custom fields for the SetFee model.
    ///
    /// See SetFee fields:
    /// `<https://xrpl.org/setfee.html#setfee-fields>`
    base_fee: u64,
    reference_fee_units: u32,
    reserve_base: u32,
    reserve_increment: u32,
    ledger_sequence: u32,
}

impl Model for SetFee<'static> {
    // fn to_json(&self) -> &str {
    //     let transaction_json = serde_json::to_string(&self).expect("Unable to convert `AccountDelete` json to string.");
    //     transaction_json
    // }

    fn to_json_value(&self) -> Value {
        let mut transaction_json =
            serde_json::to_value(&self).expect("Unable to serialize `SetFee` to json.");
        transaction_json["Flags"] = Value::from(self.iter_to_int());
        transaction_json
    }
}

impl Transaction for SetFee<'static> {
    fn get_transaction_type(&self) -> TransactionType {
        self.transaction_type.clone()
    }
}

/// See UNLModify:
/// `<https://xrpl.org/unlmodify.html>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "PascalCase", deserialize = "snake_case"))]
pub struct UNLModify<'a> {
    /// The base fields for all transaction models.
    ///
    /// See Transaction Types:
    /// `<https://xrpl.org/transaction-types.html>`
    ///
    /// See Transaction Common Fields:
    /// `<https://xrpl.org/transaction-common-fields.html>`
    #[serde(skip_serializing)]
    #[serde(default = "TransactionType::unl_modify")]
    transaction_type: TransactionType,
    #[serde(default = "default_account_zero")]
    account: &'a str,
    fee: Option<&'a str>,
    sequence: Option<u32>,
    signing_pub_key: Option<&'a str>,
    source_tag: Option<u32>,
    txn_signature: Option<&'a str>,
    flags: Option<Vec<u32>>,
    /// The custom fields for the UNLModify model.
    ///
    /// See UNLModify fields:
    /// `<https://xrpl.org/unlmodify.html#unlmodify-fields>`
    ledger_sequence: u16,
    unlmodify_disabling: u8,
    unlmodify_validator: &'a str,
}

impl Model for UNLModify<'static> {
    // fn to_json(&self) -> &str {
    //     let transaction_json = serde_json::to_string(&self).expect("Unable to convert `AccountDelete` json to string.");
    //     transaction_json
    // }

    fn to_json_value(&self) -> Value {
        let mut transaction_json =
            serde_json::to_value(&self).expect("Unable to serialize `UNLModify` to json.");
        transaction_json["Flags"] = Value::from(self.iter_to_int());
        transaction_json
    }

    fn get_errors(&self) -> Result<(), XRPLModelException> {
        match self.get_unl_modify_error() {
            Err(error) => Err(XRPLModelException::XRPLTransactionError(
                XRPLTransactionException::UNLModifyError(error),
            )),
            Ok(_no_error) => Ok(()),
        }
    }
}

impl Transaction for UNLModify<'static> {
    fn get_transaction_type(&self) -> TransactionType {
        self.transaction_type.clone()
    }
}

impl UNLModifyError for UNLModify<'static> {
    fn get_unl_modify_error(&self) -> Result<(), UNLModifyException> {
        let possible_unlmodify_disabling: [u8; 2] = [0, 1];
        match !possible_unlmodify_disabling.contains(&self.unlmodify_disabling) {
            true => Err(UNLModifyException::InvalidUNLModifyDisablingMustBeOneOrTwo),
            false => Ok(()),
        }
    }
}

#[cfg(test)]
mod test {
    use alloc::vec;

    use super::*;

    #[test]
    fn test_to_json() {
        let sequence: u32 = 1;
        let last_ledger_sequence: u32 = 72779837;
        let flags = vec![OfferCreateFlag::TfImmediateOrCancel];
        let xrp_amount = "1000000";
        let usd_amount = "0.3";
        let offer_create: OfferCreate = OfferCreate {
            transaction_type: TransactionType::OfferCreate,
            account: "rpXhhWmCvDwkzNtRbm7mmD1vZqdfatQNEe",
            fee: Some("10"),
            sequence: Some(sequence),
            last_ledger_sequence: Some(last_ledger_sequence),
            account_txn_id: None,
            signing_pub_key: None,
            source_tag: None,
            ticket_sequence: None,
            txn_signature: None,
            flags: Some(flags),
            memos: None,
            signers: None,
            taker_gets: Currency::Xrp {
                value: Some(Borrowed(xrp_amount)),
                currency: Borrowed("XRP"),
            },
            taker_pays: Currency::IssuedCurrency {
                value: Some(Borrowed(usd_amount)),
                currency: Borrowed("USD"),
                issuer: Borrowed("rhub8VRN55s94qWKDv6jmDy1pUykJzF3wq"),
            },
            expiration: None,
            offer_sequence: None,
        };
        let actual = offer_create.to_json_value();
        let json = r#"{"Account":"rpXhhWmCvDwkzNtRbm7mmD1vZqdfatQNEe","Fee":"10","Sequence":1,"LastLedgerSequence":72779837,"Flags":131072,"TakerGets":{"value":"1000000","currency":"XRP"},"TakerPays":{"value":"0.3","currency":"USD","issuer":"rhub8VRN55s94qWKDv6jmDy1pUykJzF3wq"}}"#;
        let expect: Value = serde_json::from_str(json).unwrap();
        assert_eq!(actual, expect)
    }

    #[test]
    fn test_has_flag() {
        let sequence: u32 = 1;
        let last_ledger_sequence: u32 = 72779837;
        let flags = vec![OfferCreateFlag::TfImmediateOrCancel];
        let xrp_amount = "1000000";
        let usd_amount = "0.3";
        let offer_create: OfferCreate = OfferCreate {
            transaction_type: TransactionType::OfferCreate,
            account: "rpXhhWmCvDwkzNtRbm7mmD1vZqdfatQNEe",
            fee: Some("10"),
            sequence: Some(sequence),
            last_ledger_sequence: Some(last_ledger_sequence),
            account_txn_id: None,
            signing_pub_key: None,
            source_tag: None,
            ticket_sequence: None,
            txn_signature: None,
            flags: Some(flags),
            memos: None,
            signers: None,
            taker_gets: Currency::Xrp {
                value: Some(Borrowed(xrp_amount)),
                currency: Borrowed("XRP"),
            },
            taker_pays: Currency::IssuedCurrency {
                value: Some(Borrowed(usd_amount)),
                currency: Borrowed("USD"),
                issuer: Borrowed("rhub8VRN55s94qWKDv6jmDy1pUykJzF3wq"),
            },
            expiration: None,
            offer_sequence: None,
        };
        assert!(offer_create.has_flag(Flag::OfferCreate(OfferCreateFlag::TfImmediateOrCancel)))
    }

    #[test]
    fn test_get_transaction_type() {
        let sequence: u32 = 1;
        let last_ledger_sequence: u32 = 72779837;
        let flags = vec![OfferCreateFlag::TfImmediateOrCancel];
        let xrp_amount = "1000000";
        let usd_amount = "0.3";
        let offer_create: OfferCreate = OfferCreate {
            transaction_type: TransactionType::OfferCreate,
            account: "rpXhhWmCvDwkzNtRbm7mmD1vZqdfatQNEe",
            fee: Some("10"),
            sequence: Some(sequence),
            last_ledger_sequence: Some(last_ledger_sequence),
            account_txn_id: None,
            signing_pub_key: None,
            source_tag: None,
            ticket_sequence: None,
            txn_signature: None,
            flags: Some(flags),
            memos: None,
            signers: None,
            taker_gets: Currency::Xrp {
                value: Some(Borrowed(xrp_amount)),
                currency: Borrowed("XRP"),
            },
            taker_pays: Currency::IssuedCurrency {
                value: Some(Borrowed(usd_amount)),
                currency: Borrowed("USD"),
                issuer: Borrowed("rhub8VRN55s94qWKDv6jmDy1pUykJzF3wq"),
            },
            expiration: None,
            offer_sequence: None,
        };
        let actual = offer_create.get_transaction_type();
        let expect = TransactionType::OfferCreate;
        assert_eq!(actual, expect)
    }
}

#[cfg(test)]
mod test_errors {
    // use crate::models::Model;

    use super::AccountSet;

    #[test]
    fn test_account_set_tick_size() {
        let _account_set = AccountSet {
            transaction_type: crate::models::TransactionType::AccountSet,
            account: "rU4EE1FskCPJw5QkLx1iGgdWiJa6HeqYyb",
            fee: None,
            sequence: None,
            last_ledger_sequence: None,
            account_txn_id: None,
            signing_pub_key: None,
            source_tag: None,
            ticket_sequence: None,
            txn_signature: None,
            flags: None,
            memos: None,
            signers: None,
            clear_flag: None,
            domain: None,
            email_hash: None,
            message_key: None,
            set_flag: None,
            transfer_rate: None,
            tick_size: Some(2),
            nftoken_minter: None,
        };
        // account_set.validate();
    }
}
