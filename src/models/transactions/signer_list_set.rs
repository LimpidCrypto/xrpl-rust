use alloc::borrow::Cow;
use alloc::vec::Vec;
use anyhow::Result;
use derive_new::new;
use serde::{ser::SerializeMap, Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    models::{
        exceptions::XrplModelException, model::Model, Memo, Signer, SignerListSetError,
        Transaction, TransactionType,
    },
    serialize_with_tag, Err,
};

serialize_with_tag! {
    // TODO: Impl Deserialize
    #[derive(Debug, Deserialize, PartialEq, Eq, Default, Clone, new)]
    #[skip_serializing_none]
    pub struct SignerEntry {
        account: Cow<'static, str>,
        signer_weight: u16,
    }
}

/// The SignerList object type represents a list of parties that,
/// as a group, are authorized to sign a transaction in place of an
/// individual account. You can create, replace, or remove a signer
/// list using a SignerListSet transaction.
///
/// See TicketCreate:
/// `<https://xrpl.org/signerlistset.html>`
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "PascalCase")]
#[skip_serializing_none]
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
    pub transaction_type: TransactionType,
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
    #[serde(rename = "AccountTxnID")]
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
    pub flags: Option<u32>,
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
    pub signer_entries: Option<Vec<SignerEntry>>,
}

impl<'a> Default for SignerListSet<'a> {
    fn default() -> Self {
        Self {
            transaction_type: TransactionType::SignerListSet,
            account: Default::default(),
            fee: Default::default(),
            sequence: Default::default(),
            last_ledger_sequence: Default::default(),
            account_txn_id: Default::default(),
            signing_pub_key: Default::default(),
            source_tag: Default::default(),
            ticket_sequence: Default::default(),
            txn_signature: Default::default(),
            flags: Default::default(),
            memos: Default::default(),
            signers: Default::default(),
            signer_quorum: Default::default(),
            signer_entries: Default::default(),
        }
    }
}

impl<'a> Model for SignerListSet<'a> {
    fn get_errors(&self) -> Result<()> {
        match self._get_signer_entries_error() {
            Err(error) => Err(error),
            Ok(_no_error) => match self._get_signer_quorum_error() {
                Err(error) => Err(error),
                Ok(_no_error) => Ok(()),
            },
        }
    }
}

impl<'a> Transaction for SignerListSet<'a> {
    fn get_transaction_type(&self) -> TransactionType {
        self.transaction_type.clone()
    }
}

impl<'a> SignerListSetError for SignerListSet<'a> {
    fn _get_signer_entries_error(&self) -> Result<()> {
        if let Some(signer_entries) = self.signer_entries {
            if self.signer_quorum == 0 {
                return Err!(XrplModelException::ValueZero {
                    model_type: stringify!(SignerListSet),
                    field: "signer_entries",
                    resource: ""
                });
            }
            if signer_entries.is_empty() {
                return Err!(XrplModelException::ValueEmpty {
                    model_type: stringify!(SignerListSet),
                    field: "signer_entries",
                    resource: ""
                });
            }
            if signer_entries.len() > 8 {
                return Err!(XrplModelException::ValueTooHigh {
                    model_type: stringify!(SignerListSet),
                    field: "signer_entries",
                    max: 8,
                    found: signer_entries.len() as u32,
                    resource: ""
                });
            }
        }

        Ok(())
    }

    fn _get_signer_quorum_error(&self) -> Result<()> {
        let mut accounts = Vec::new();
        let mut signer_weight_sum: u32 = 0;
        if let Some(signer_entries) = self.signer_entries {
            for signer_entry in signer_entries {
                accounts.push(&signer_entry.account);
                let weight: u32 = signer_entry.signer_weight.into();
                signer_weight_sum += weight;
            }
        }

        accounts.sort_unstable();
        let mut check_account = Vec::new();
        for account in accounts.clone() {
            if check_account.contains(&account) {
                return Err!(XrplModelException::ItemDuplicate {
                    model_type: stringify!(SignerListSet),
                    field: "signer_entries",
                    resource: ""
                });
            }

            check_account.push(account);
        }
        if let Some(signer_entries) = self.signer_entries {
            if accounts.contains(&&Cow::Borrowed(self.account)) {
                return Err!(XrplModelException::IllegalItem {
                    model_type: stringify!(SignerListSet),
                    field: "signer_entries",
                    illegal_value: "account",
                    resource: ""
                });
            }
            if self.signer_quorum > signer_weight_sum {
                return Err!(XrplModelException::ValueTooHigh {
                    model_type: stringify!(SignerListSet),
                    field: "signer_quorum",
                    max: signer_weight_sum,
                    found: self.signer_quorum,
                    resource: ""
                });
            }
        } else if self.signer_quorum != 0 {
            return Err!(XrplModelException::ValueTooHigh {
                model_type: stringify!(SignerListSet),
                field: "signer_quorum",
                max: 0,
                found: self.signer_quorum,
                resource: ""
            });
        }

        Ok(())
    }
}

impl<'a> SignerListSet<'a> {
    fn new(
        account: &'a str,
        signer_quorum: u32,
        fee: Option<&'a str>,
        sequence: Option<u32>,
        last_ledger_sequence: Option<u32>,
        account_txn_id: Option<&'a str>,
        signing_pub_key: Option<&'a str>,
        source_tag: Option<u32>,
        ticket_sequence: Option<u32>,
        txn_signature: Option<&'a str>,
        memos: Option<Vec<Memo<'a>>>,
        signers: Option<Vec<Signer<'a>>>,
        signer_entries: Option<Vec<SignerEntry>>,
    ) -> Self {
        Self {
            transaction_type: TransactionType::SignerListSet,
            account,
            fee,
            sequence,
            last_ledger_sequence,
            account_txn_id,
            signing_pub_key,
            source_tag,
            ticket_sequence,
            txn_signature,
            flags: None,
            memos,
            signers,
            signer_quorum,
            signer_entries,
        }
    }
}

#[cfg(test)]
mod test_signer_list_set_error {
    use alloc::borrow::Cow::Borrowed;
    use alloc::vec;

    use crate::models::{Model, TransactionType};

    use super::*;

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
                account: Borrowed("rsA2LpzuawewSBQXkiju3YQTMzW13pAAdW"),
                signer_weight: 2,
            }]),
        };
        let expected_error = XrplModelException::ValueZero {
            model_type: stringify!(SignerListSet),
            field: "signer_entries",
            resource: "",
        };

        assert_eq!(signer_list_set.validate(), Err(expected_error));

        signer_list_set.signer_quorum = 3;
        signer_list_set.signer_entries = None;
        let expected_error = XrplModelException::ValueTooHigh {
            model_type: stringify!(SignerListSet),
            field: "signer_quorum",
            max: 0,
            found: signer_list_set.signer_quorum,
            resource: "",
        };

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
        let expected_error = XrplModelException::ValueEmpty {
            model_type: stringify!(SignerListSet),
            field: "signer_entries",
            resource: "",
        };

        assert_eq!(signer_list_set.validate(), Err(expected_error));

        signer_list_set.signer_entries = Some(vec![
            SignerEntry {
                account: Borrowed("rsA2LpzuawewSBQXkiju3YQTMzW13pAAdW"),
                signer_weight: 1,
            },
            SignerEntry {
                account: Borrowed("rUpy3eEg8rqjqfUoLeBnZkscbKbFsKXC3v"),
                signer_weight: 1,
            },
            SignerEntry {
                account: Borrowed("rUpy3eEg8rqjqfUoLeBnZkscbKbFsKXC3v"),
                signer_weight: 2,
            },
            SignerEntry {
                account: Borrowed("rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn"),
                signer_weight: 2,
            },
            SignerEntry {
                account: Borrowed("rvYAfWj5gh67oV6fW32ZzP3Aw4Eubs59B"),
                signer_weight: 1,
            },
            SignerEntry {
                account: Borrowed("rXTZ5g8X7mrAYEe7iFeM9fiS4ccueyurG"),
                signer_weight: 1,
            },
            SignerEntry {
                account: Borrowed("rPbMHxs7vy5t6e19tYfqG7XJ6Fog8EPZLk"),
                signer_weight: 2,
            },
            SignerEntry {
                account: Borrowed("r3rhWeE31Jt5sWmi4QiGLMZnY3ENgqw96W"),
                signer_weight: 3,
            },
            SignerEntry {
                account: Borrowed("rchGBxcD1A1C2tdxF6papQYZ8kjRKMYcL"),
                signer_weight: 2,
            },
        ]);
        let expected_error = XrplModelException::ValueTooHigh {
            model_type: stringify!(SignerListSet),
            field: "signer_entries",
            max: 8,
            found: signer_list_set.signer_entries.unwrap().len() as u32,
            resource: "",
        };

        assert_eq!(signer_list_set.validate(), Err(expected_error));

        signer_list_set.signer_entries = Some(vec![
            SignerEntry {
                account: Borrowed("rU4EE1FskCPJw5QkLx1iGgdWiJa6HeqYyb"),
                signer_weight: 1,
            },
            SignerEntry {
                account: Borrowed("rUpy3eEg8rqjqfUoLeBnZkscbKbFsKXC3v"),
                signer_weight: 2,
            },
            SignerEntry {
                account: Borrowed("rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn"),
                signer_weight: 2,
            },
        ]);
        let expected_error = XrplModelException::IllegalItem {
            model_type: stringify!(SignerListSet),
            field: "signer_entries",
            illegal_value: "account",
            resource: "",
        };

        assert_eq!(signer_list_set.validate(), Err(expected_error));

        signer_list_set.signer_entries = Some(vec![SignerEntry {
            account: Borrowed("rsA2LpzuawewSBQXkiju3YQTMzW13pAAdW"),
            signer_weight: 3,
        }]);
        signer_list_set.signer_quorum = 10;
        let expected_error = XrplModelException::ValueTooHigh {
            model_type: stringify!(SignerListSet),
            field: "signer_quorum",
            max: 3,
            found: 10,
            resource: "",
        };
        assert_eq!(signer_list_set.validate(), Err(expected_error));

        signer_list_set.signer_entries = Some(vec![
            SignerEntry {
                account: Borrowed("rsA2LpzuawewSBQXkiju3YQTMzW13pAAdW"),
                signer_weight: 3,
            },
            SignerEntry {
                account: Borrowed("rsA2LpzuawewSBQXkiju3YQTMzW13pAAdW"),
                signer_weight: 2,
            },
        ]);
        signer_list_set.signer_quorum = 2;
        let expected_error = XrplModelException::ItemDuplicate {
            model_type: stringify!(SignerListSet),
            field: "signer_entries",
            resource: "",
        };

        assert_eq!(signer_list_set.validate(), Err(expected_error));
    }
}

#[cfg(test)]
mod test_serde {
    use alloc::borrow::Cow::Borrowed;
    use alloc::vec;

    use super::*;

    #[test]
    fn test_serialize() {
        let default_txn = SignerListSet::new(
            "rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn",
            3,
            Some("12"),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Some(vec![
                SignerEntry::new(Borrowed("rsA2LpzuawewSBQXkiju3YQTMzW13pAAdW"), 2),
                SignerEntry::new(Borrowed("rUpy3eEg8rqjqfUoLeBnZkscbKbFsKXC3v"), 1),
                SignerEntry::new(Borrowed("raKEEVSGnKSD9Zyvxu4z6Pqpm4ABH8FS6n"), 1),
            ]),
        );
        let default_json = r#"{"TransactionType":"SignerListSet","Account":"rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn","Fee":"12","Sequence":null,"LastLedgerSequence":null,"AccountTxnID":null,"SigningPubKey":null,"SourceTag":null,"TicketSequence":null,"TxnSignature":null,"Flags":null,"Memos":null,"Signers":null,"SignerQuorum":3,"SignerEntries":[{"SignerEntry":{"Account":"rsA2LpzuawewSBQXkiju3YQTMzW13pAAdW","SignerWeight":2}},{"SignerEntry":{"Account":"rUpy3eEg8rqjqfUoLeBnZkscbKbFsKXC3v","SignerWeight":1}},{"SignerEntry":{"Account":"raKEEVSGnKSD9Zyvxu4z6Pqpm4ABH8FS6n","SignerWeight":1}}]}"#;

        let txn_as_string = serde_json::to_string(&default_txn).unwrap();
        let txn_json = txn_as_string.as_str();

        assert_eq!(txn_json, default_json);
    }
}
//     #[test]
//     fn test_deserialize() {
//         let default_txn = SignerListSet::new(
//             "rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn",
//             3,
//             Some("12"),
//             None,
//             None,
//             None,
//             None,
//             None,
//             None,
//             None,
//             None,
//             None,
//             Some(vec![
//                 SignerEntry::new(Borrowed("rsA2LpzuawewSBQXkiju3YQTMzW13pAAdW"), 2),
//                 SignerEntry::new(Borrowed("rUpy3eEg8rqjqfUoLeBnZkscbKbFsKXC3v"), 1),
//                 SignerEntry::new(Borrowed("raKEEVSGnKSD9Zyvxu4z6Pqpm4ABH8FS6n"), 1),
//             ]),
//         );
//         let default_json = r#"{"TransactionType":"SignerListSet","Account":"rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn","Fee":"12","SignerQuorum":3,"SignerEntries":[{"SignerEntry":{"Account":"rsA2LpzuawewSBQXkiju3YQTMzW13pAAdW","SignerWeight":2}},{"SignerEntry":{"Account":"rUpy3eEg8rqjqfUoLeBnZkscbKbFsKXC3v","SignerWeight":1}},{"SignerEntry":{"Account":"raKEEVSGnKSD9Zyvxu4z6Pqpm4ABH8FS6n","SignerWeight":1}}]}"#;

//         let txn_as_obj: SignerListSet = serde_json::from_str(&default_json).unwrap();

//         assert_eq!(txn_as_obj, default_txn);
//     }
// }
