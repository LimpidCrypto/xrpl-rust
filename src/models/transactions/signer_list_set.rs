use alloc::vec::Vec;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::models::{
    default_zero,
    exceptions::{SignerListSetException, XRPLModelException, XRPLTransactionException},
    model::Model,
    Memo, Signer, SignerEntry, SignerListSetError, Transaction, TransactionType,
};

/// The SignerList object type represents a list of parties that,
/// as a group, are authorized to sign a transaction in place of an
/// individual account. You can create, replace, or remove a signer
/// list using a SignerListSet transaction.
///
/// See TicketCreate:
/// `<https://xrpl.org/signerlistset.html>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct SignerListSet<'a> {
    // The base fields for all transaction models.
    //
    // See Transaction Types:
    // `<https://xrpl.org/transaction-types.html>`
    //
    // See Transaction Common Fields:
    // `<https://xrpl.org/transaction-common-fields.html>`
    /// The type of transaction.
    #[serde(default = "TransactionType::signer_list_set")]
    transaction_type: TransactionType,
    /// The unique address of the account that initiated the transaction.
    pub account: &'a str,
    /// Integer amount of XRP, in drops, to be destroyed as a cost
    /// for distributing this transaction to the network. Some
    /// transaction types have different minimum requirements.
    /// See Transaction Cost for details.
    pub fee: Option<&'a str>,
    /// The sequence number of the account sending the transaction.
    /// A transaction is only valid if the Sequence number is exactly
    /// 1 greater than the previous transaction from the same account.
    /// The special case 0 means the transaction is using a Ticket instead.
    pub sequence: Option<u32>,
    /// Highest ledger index this transaction can appear in.
    /// Specifying this field places a strict upper limit on how long
    /// the transaction can wait to be validated or rejected.
    /// See Reliable Transaction Submission for more details.
    pub last_ledger_sequence: Option<u32>,
    /// Hash value identifying another transaction. If provided, this
    /// transaction is only valid if the sending account's
    /// previously-sent transaction matches the provided hash.
    pub account_txn_id: Option<&'a str>,
    /// Hex representation of the public key that corresponds to the
    /// private key used to sign this transaction. If an empty string,
    /// indicates a multi-signature is present in the Signers field instead.
    pub signing_pub_key: Option<&'a str>,
    /// Arbitrary integer used to identify the reason for this
    /// payment, or a sender on whose behalf this transaction
    /// is made. Conventionally, a refund should specify the initial
    /// payment's SourceTag as the refund payment's DestinationTag.
    pub source_tag: Option<u32>,
    /// The sequence number of the ticket to use in place
    /// of a Sequence number. If this is provided, Sequence must
    /// be 0. Cannot be used with AccountTxnID.
    pub ticket_sequence: Option<u32>,
    /// The signature that verifies this transaction as originating
    /// from the account it says it is from.
    pub txn_signature: Option<&'a str>,
    /// Set of bit-flags for this transaction.
    #[serde(default = "default_zero")]
    flags: Option<u32>,
    /// Additional arbitrary information used to identify this transaction.
    pub memos: Option<Vec<Memo<'a>>>,
    /// Arbitrary integer used to identify the reason for this
    /// payment, or a sender on whose behalf this transaction is
    /// made. Conventionally, a refund should specify the initial
    /// payment's SourceTag as the refund payment's DestinationTag.
    pub signers: Option<Vec<Signer<'a>>>,
    /// The custom fields for the TicketCreate model.
    ///
    /// See TicketCreate fields:
    /// `<https://xrpl.org/signerlistset.html#signerlistset-fields>`
    pub signer_quorum: u32,
    pub signer_entries: Option<Vec<SignerEntry<'a>>>,
}

impl Model for SignerListSet<'static> {
    fn get_errors(&self) -> Result<(), XRPLModelException> {
        match self._get_signer_entries_error() {
            Err(error) => Err(XRPLModelException::XRPLTransactionError(
                XRPLTransactionException::SignerListSetError(error),
            )),
            Ok(_no_error) => match self._get_signer_quorum_error() {
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
    fn _get_signer_entries_error(&self) -> Result<(), SignerListSetException> {
        match self.signer_entries.as_ref() {
            Some(signer_entries) => match self.signer_quorum == 0 {
                true => Err(SignerListSetException::InvalidMustNotSetSignerEntriesIfSignerListIsBeingDeleted),
                false => match signer_entries.is_empty() {
                    true => Err(SignerListSetException::InvalidTooFewSignerEntries { min: 1, found: signer_entries.len() }),
                    false => match signer_entries.len() > 8 {
                        true => Err(SignerListSetException::InvalidTooManySignerEntries { max: 8, found: signer_entries.len() }),
                        false => Ok(())
                    },
                },
            },
            None => Ok(())
        }
    }

    fn _get_signer_quorum_error(&self) -> Result<(), SignerListSetException> {
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
        let mut check_account = Vec::new();
        for account in accounts.clone() {
            match &check_account.contains(&account) {
                true => {
                    return Err(
                        SignerListSetException::InvalidAnAccountCanNotBeInSignerEntriesTwice,
                    )
                }
                false => check_account.push(account),
            }
        }
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

#[cfg(test)]
mod test_signer_list_set_error {
    use alloc::vec;

    use crate::models::{
        exceptions::{SignerListSetException, XRPLModelException, XRPLTransactionException},
        Model, SignerEntry, TransactionType,
    };

    use super::SignerListSet;

    #[test]
    fn test_signer_list_deleted_error() {
        let mut signer_list_set = SignerListSet {
            transaction_type: TransactionType::SignerListSet,
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
            signer_quorum: 0,
            signer_entries: Some(vec![SignerEntry {
                account: "rsA2LpzuawewSBQXkiju3YQTMzW13pAAdW",
                signer_weight: 2,
            }]),
        };
        let expected_error =
            XRPLModelException::XRPLTransactionError(XRPLTransactionException::SignerListSetError(
                SignerListSetException::InvalidMustNotSetSignerEntriesIfSignerListIsBeingDeleted,
            ));
        assert_eq!(signer_list_set.validate(), Err(expected_error));

        signer_list_set.signer_quorum = 3;
        signer_list_set.signer_entries = None;
        let expected_error =
            XRPLModelException::XRPLTransactionError(XRPLTransactionException::SignerListSetError(
                SignerListSetException::InvalidSignerQuorumMustBeZeroIfSignerListIsBeingDeleted,
            ));
        assert_eq!(signer_list_set.validate(), Err(expected_error));
    }

    #[test]
    fn test_signer_entries_error() {
        let mut signer_list_set = SignerListSet {
            transaction_type: TransactionType::SignerListSet,
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
            signer_quorum: 3,
            signer_entries: Some(vec![]),
        };
        let expected_error =
            XRPLModelException::XRPLTransactionError(XRPLTransactionException::SignerListSetError(
                SignerListSetException::InvalidTooFewSignerEntries { min: 1, found: 0 },
            ));
        assert_eq!(signer_list_set.validate(), Err(expected_error));

        signer_list_set.signer_entries = Some(vec![
            SignerEntry {
                account: "rsA2LpzuawewSBQXkiju3YQTMzW13pAAdW",
                signer_weight: 1,
            },
            SignerEntry {
                account: "rUpy3eEg8rqjqfUoLeBnZkscbKbFsKXC3v",
                signer_weight: 1,
            },
            SignerEntry {
                account: "rUpy3eEg8rqjqfUoLeBnZkscbKbFsKXC3v",
                signer_weight: 2,
            },
            SignerEntry {
                account: "rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn",
                signer_weight: 2,
            },
            SignerEntry {
                account: "rvYAfWj5gh67oV6fW32ZzP3Aw4Eubs59B",
                signer_weight: 1,
            },
            SignerEntry {
                account: "rXTZ5g8X7mrAYEe7iFeM9fiS4ccueyurG",
                signer_weight: 1,
            },
            SignerEntry {
                account: "rPbMHxs7vy5t6e19tYfqG7XJ6Fog8EPZLk",
                signer_weight: 2,
            },
            SignerEntry {
                account: "r3rhWeE31Jt5sWmi4QiGLMZnY3ENgqw96W",
                signer_weight: 3,
            },
            SignerEntry {
                account: "rchGBxcD1A1C2tdxF6papQYZ8kjRKMYcL",
                signer_weight: 2,
            },
        ]);
        let expected_error =
            XRPLModelException::XRPLTransactionError(XRPLTransactionException::SignerListSetError(
                SignerListSetException::InvalidTooManySignerEntries { max: 8, found: 9 },
            ));
        assert_eq!(signer_list_set.validate(), Err(expected_error));

        signer_list_set.signer_entries = Some(vec![
            SignerEntry {
                account: "rU4EE1FskCPJw5QkLx1iGgdWiJa6HeqYyb",
                signer_weight: 1,
            },
            SignerEntry {
                account: "rUpy3eEg8rqjqfUoLeBnZkscbKbFsKXC3v",
                signer_weight: 2,
            },
            SignerEntry {
                account: "rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn",
                signer_weight: 2,
            },
        ]);
        let expected_error =
            XRPLModelException::XRPLTransactionError(XRPLTransactionException::SignerListSetError(
                SignerListSetException::InvalidAccountMustNotBeInSignerEntry,
            ));
        assert_eq!(signer_list_set.validate(), Err(expected_error));

        signer_list_set.signer_entries = Some(vec![SignerEntry {
            account: "rsA2LpzuawewSBQXkiju3YQTMzW13pAAdW",
            signer_weight: 3,
        }]);
        signer_list_set.signer_quorum = 10;
        let expected_error =
            XRPLModelException::XRPLTransactionError(XRPLTransactionException::SignerListSetError(
                SignerListSetException::InvalidMustBeLessOrEqualToSumOfSignerWeightInSignerEntries { max: 3, found: 10 },
            ));
        assert_eq!(signer_list_set.validate(), Err(expected_error));

        signer_list_set.signer_entries = Some(vec![
            SignerEntry {
                account: "rsA2LpzuawewSBQXkiju3YQTMzW13pAAdW",
                signer_weight: 3,
            },
            SignerEntry {
                account: "rsA2LpzuawewSBQXkiju3YQTMzW13pAAdW",
                signer_weight: 2,
            },
        ]);
        signer_list_set.signer_quorum = 2;
        let expected_error =
            XRPLModelException::XRPLTransactionError(XRPLTransactionException::SignerListSetError(
                SignerListSetException::InvalidAnAccountCanNotBeInSignerEntriesTwice,
            ));
        assert_eq!(signer_list_set.validate(), Err(expected_error));
    }
}
