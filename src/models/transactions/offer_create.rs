use alloc::vec::Vec;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use serde_with::skip_serializing_none;
use strum_macros::{AsRefStr, Display, EnumIter};

use crate::models::{model::Model, Amount, Flag, Memo, Signer, Transaction, TransactionType};

use super::flags_serde;

/// Transactions of the OfferCreate type support additional values
/// in the Flags field. This enum represents those options.
///
/// See OfferCreate flags:
/// `<https://xrpl.org/offercreate.html#offercreate-flags>`
#[derive(
    Debug, Eq, PartialEq, Clone, Serialize_repr, Deserialize_repr, Display, AsRefStr, EnumIter,
)]
#[repr(u32)]
pub enum OfferCreateFlag {
    /// If enabled, the Offer does not consume Offers that exactly match it,
    /// and instead becomes an Offer object in the ledger.
    /// It still consumes Offers that cross it.
    TfPassive = 0x00010000,
    /// Treat the Offer as an Immediate or Cancel order. The Offer never creates
    /// an Offer object in the ledger: it only trades as much as it can by
    /// consuming existing Offers at the time the transaction is processed. If no
    /// Offers match, it executes "successfully" without trading anything.
    /// In this case, the transaction still uses the result code tesSUCCESS.
    TfImmediateOrCancel = 0x00020000,
    /// Treat the offer as a Fill or Kill order . The Offer never creates an Offer
    /// object in the ledger, and is canceled if it cannot be fully filled at the
    /// time of execution. By default, this means that the owner must receive the
    /// full TakerPays amount; if the tfSell flag is enabled, the owner must be
    /// able to spend the entire TakerGets amount instead.
    TfFillOrKill = 0x00040000,
    /// Exchange the entire TakerGets amount, even if it means obtaining more than
    /// the TakerPays amount in exchange.
    TfSell = 0x00080000,
}

/// Places an Offer in the decentralized exchange.
///
/// See OfferCreate:
/// `<https://xrpl.org/offercreate.html>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct OfferCreate<'a> {
    // The base fields for all transaction models.
    //
    // See Transaction Types:
    // `<https://xrpl.org/transaction-types.html>`
    //
    // See Transaction Common Fields:
    // `<https://xrpl.org/transaction-common-fields.html>`
    /// The type of transaction.
    #[serde(default = "TransactionType::offer_create")]
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
    #[serde(with = "flags_serde")]
    pub flags: Option<Vec<OfferCreateFlag>>,
    /// Additional arbitrary information used to identify this transaction.
    pub memos: Option<Vec<Memo<'a>>>,
    /// Arbitrary integer used to identify the reason for this
    /// payment, or a sender on whose behalf this transaction is
    /// made. Conventionally, a refund should specify the initial
    /// payment's SourceTag as the refund payment's DestinationTag.
    pub signers: Option<Vec<Signer<'a>>>,
    /// The custom fields for the OfferCreate model.
    ///
    /// See OfferCreate fields:
    /// `<https://xrpl.org/offercreate.html#offercreate-fields>`
    pub taker_gets: Amount,
    pub taker_pays: Amount,
    pub expiration: Option<u32>,
    pub offer_sequence: Option<u32>,
}

impl<'a> Model for OfferCreate<'a> {}

impl<'a> Transaction for OfferCreate<'a> {
    fn has_flag(&self, flag: &Flag) -> bool {
        let mut flags = &Vec::new();

        if let Some(flag_set) = self.flags.as_ref() {
            flags = flag_set;
        }

        match flag {
            Flag::OfferCreate(offer_create_flag) => match offer_create_flag {
                OfferCreateFlag::TfFillOrKill => flags.contains(&OfferCreateFlag::TfFillOrKill),
                OfferCreateFlag::TfImmediateOrCancel => {
                    flags.contains(&OfferCreateFlag::TfImmediateOrCancel)
                }
                OfferCreateFlag::TfPassive => flags.contains(&OfferCreateFlag::TfPassive),
                OfferCreateFlag::TfSell => flags.contains(&OfferCreateFlag::TfSell),
            },
            _ => false,
        }
    }

    fn get_transaction_type(&self) -> TransactionType {
        self.transaction_type.clone()
    }
}

impl<'a> OfferCreate<'a> {
    fn new(
        account: &'a str,
        taker_gets: Amount,
        taker_pays: Amount,
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
        expiration: Option<u32>,
        offer_sequence: Option<u32>,
    ) -> Self {
        Self {
            transaction_type: TransactionType::OfferCreate,
            account,
            fee,
            sequence,
            last_ledger_sequence,
            account_txn_id,
            signing_pub_key,
            source_tag,
            ticket_sequence,
            txn_signature,
            flags,
            memos,
            signers,
            taker_gets,
            taker_pays,
            expiration,
            offer_sequence,
        }
    }
}

#[cfg(test)]
mod test {
    use alloc::borrow::Cow::Borrowed;
    use alloc::vec;

    use super::*;

    #[test]
    fn test_serde() {
        let txn: OfferCreate = OfferCreate {
            transaction_type: TransactionType::OfferCreate,
            account: "rpXhhWmCvDwkzNtRbm7mmD1vZqdfatQNEe",
            fee: Some("10"),
            sequence: Some(1),
            last_ledger_sequence: Some(72779837),
            account_txn_id: None,
            signing_pub_key: None,
            source_tag: None,
            ticket_sequence: None,
            txn_signature: None,
            flags: Some(vec![OfferCreateFlag::TfImmediateOrCancel]),
            memos: None,
            signers: None,
            taker_gets: Amount::Xrp(Borrowed("1000000")),
            taker_pays: Amount::IssuedCurrency {
                value: Borrowed("0.3"),
                currency: Borrowed("USD"),
                issuer: Borrowed("rhub8VRN55s94qWKDv6jmDy1pUykJzF3wq"),
            },
            expiration: None,
            offer_sequence: None,
        };
        let txn_as_string = serde_json::to_string(&txn).unwrap();
        let txn_json = txn_as_string.as_str();
        let expected_json = r#"{"TransactionType":"OfferCreate","Account":"rpXhhWmCvDwkzNtRbm7mmD1vZqdfatQNEe","Fee":"10","Sequence":1,"LastLedgerSequence":72779837,"Flags":131072,"TakerGets":"1000000","TakerPays":{"value":"0.3","currency":"USD","issuer":"rhub8VRN55s94qWKDv6jmDy1pUykJzF3wq"}}"#;
        let deserialized_txn: OfferCreate = serde_json::from_str(expected_json).unwrap();

        assert_eq!(txn_json, expected_json);
        assert_eq!(txn, deserialized_txn);
    }

    #[test]
    fn test_has_flag() {
        let txn: OfferCreate = OfferCreate {
            transaction_type: TransactionType::OfferCreate,
            account: "rpXhhWmCvDwkzNtRbm7mmD1vZqdfatQNEe",
            fee: Some("10"),
            sequence: Some(1),
            last_ledger_sequence: Some(72779837),
            account_txn_id: None,
            signing_pub_key: None,
            source_tag: None,
            ticket_sequence: None,
            txn_signature: None,
            flags: Some(vec![OfferCreateFlag::TfImmediateOrCancel]),
            memos: None,
            signers: None,
            taker_gets: Amount::Xrp(Borrowed("1000000")),
            taker_pays: Amount::IssuedCurrency {
                value: Borrowed("0.3"),
                currency: Borrowed("USD"),
                issuer: Borrowed("rhub8VRN55s94qWKDv6jmDy1pUykJzF3wq"),
            },
            expiration: None,
            offer_sequence: None,
        };
        assert!(txn.has_flag(&Flag::OfferCreate(OfferCreateFlag::TfImmediateOrCancel)));
        assert!(!txn.has_flag(&Flag::OfferCreate(OfferCreateFlag::TfPassive)));
    }

    #[test]
    fn test_get_transaction_type() {
        let txn: OfferCreate = OfferCreate {
            transaction_type: TransactionType::OfferCreate,
            account: "rpXhhWmCvDwkzNtRbm7mmD1vZqdfatQNEe",
            fee: Some("10"),
            sequence: Some(1),
            last_ledger_sequence: Some(72779837),
            account_txn_id: None,
            signing_pub_key: None,
            source_tag: None,
            ticket_sequence: None,
            txn_signature: None,
            flags: Some(vec![OfferCreateFlag::TfImmediateOrCancel]),
            memos: None,
            signers: None,
            taker_gets: Amount::Xrp(Borrowed("1000000")),
            taker_pays: Amount::IssuedCurrency {
                value: Borrowed("0.3"),
                currency: Borrowed("USD"),
                issuer: Borrowed("rhub8VRN55s94qWKDv6jmDy1pUykJzF3wq"),
            },
            expiration: None,
            offer_sequence: None,
        };
        let actual = txn.get_transaction_type();
        let expect = TransactionType::OfferCreate;
        assert_eq!(actual, expect)
    }
}
