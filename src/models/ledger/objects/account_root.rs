use crate::_serde::lgr_obj_flags;
use crate::models::ledger::{LedgerEntryType, LedgerObject};
use crate::models::{amount::XRPAmount, Model};
use alloc::borrow::Cow;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use serde_with::skip_serializing_none;
use strum_macros::{AsRefStr, Display, EnumIter};

/// There are several options which can be either enabled or disabled for an account.
/// These options can be changed with an `AccountSet` transaction.
///
/// See `AccountRoot` flags:
/// `<https://xrpl.org/accountroot.html#accountroot-flags>`
#[derive(
    Debug, Eq, PartialEq, Clone, Serialize_repr, Deserialize_repr, Display, AsRefStr, EnumIter,
)]
#[repr(u32)]
pub enum AccountRootFlag {
    /// This account is an Automated Market Maker instance.
    LsfAmm = 0x02000000,
    /// Enable rippling on this addresses's trust lines by default.
    /// Required for issuing addresses; discouraged for others.
    LsfDefaultRipple = 0x00800000,
    /// This account can only receive funds from transactions it sends, and from preauthorized
    /// accounts. (It has `DepositAuth` enabled.)
    LsfDepositAuth = 0x01000000,
    /// Disallows use of the master key to sign transactions for this account.
    LsfDisableMaster = 0x00100000,
    /// Client applications should not send XRP to this account. Not enforced by rippled.
    LsfDisallowXRP = 0x00080000,
    /// All assets issued by this address are frozen.
    LsfGlobalFreeze = 0x00400000,
    /// This address cannot freeze trust lines connected to it. Once enabled, cannot be disabled.
    LsfNoFreeze = 0x00200000,
    /// The account has used its free SetRegularKey transaction.
    LsfPasswordSpent = 0x00010000,
    /// This account must individually approve other users for those users to hold this account's
    /// tokens.
    LsfRequireAuth = 0x00040000,
    /// Requires incoming payments to specify a Destination Tag.
    LsfRequireDestTag = 0x00020000,
}

/// The `AccountRoot` object type describes a single account, its settings, and XRP balance.
///
/// `<https://xrpl.org/accountroot.html#accountroot>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct AccountRoot<'a> {
    /// The value `0x0061`, mapped to the string `AccountRoot`, indicates that this is an `AccountRoot`
    /// object.
    pub ledger_entry_type: LedgerEntryType,
    /// A bit-map of boolean flags enabled for this account.
    #[serde(with = "lgr_obj_flags")]
    pub flags: Vec<AccountRootFlag>,
    /// The object ID of a single object to retrieve from the ledger, as a
    /// 64-character (256-bit) hexadecimal string.
    #[serde(rename = "index")]
    pub index: Cow<'a, str>,
    /// The identifying (classic) address of this account.
    pub account: Cow<'a, str>,
    /// The number of objects this account owns in the ledger, which contributes to its owner
    /// reserve.
    pub owner_count: u32,
    /// The identifying hash of the transaction that most recently modified this object.
    #[serde(rename = "PreviousTxnID")]
    pub previous_txn_id: Cow<'a, str>,
    /// The index of the ledger that contains the transaction that most recently modified this object.
    pub previous_txn_lgr_seq: u32,
    /// The sequence number of the next valid transaction for this account.
    pub sequence: u32,
    /// The identifying hash of the transaction most recently sent by this account. This field must
    /// be enabled to use the `AccountTxnID` transaction field. To enable it, send an `AccountSet`
    /// transaction with the `asfAccountTxnID` flag enabled.
    #[serde(rename = "AccountTxnID")]
    pub account_txn_id: Option<Cow<'a, str>>,
    /// The account's current XRP balance in drops, represented as a string.
    pub balance: Option<XRPAmount<'a>>,
    /// How many total of this account's issued non-fungible tokens have been burned. This number
    /// is always equal or less than `MintedNFTokens`.
    #[serde(rename = "BurnedNFTokens")]
    pub burned_nftokens: Option<u32>,
    /// A domain associated with this account. In JSON, this is the hexadecimal for the ASCII
    /// representation of the domain. Cannot be more than 256 bytes in length.
    pub domain: Option<Cow<'a, str>>,
    /// The md5 hash of an email address. Clients can use this to look up an avatar through services
    /// such as Gravatar
    pub email_hash: Option<Cow<'a, str>>,
    /// A public key that may be used to send encrypted messages to this account. In JSON, uses
    /// hexadecimal. Must be exactly 33 bytes, with the first byte indicating the key type: 0x02 or
    /// 0x03 for secp256k1 keys, 0xED for Ed25519 keys.
    pub message_key: Option<Cow<'a, str>>,
    /// How many total non-fungible tokens have been minted by and on behalf of this account.
    #[serde(rename = "MintedNFTokens")]
    pub minted_nftokens: Option<u32>,
    /// Another account that can mint non-fungible tokens on behalf of this account.
    #[serde(rename = "NFTokenMinter")]
    pub nftoken_minter: Option<Cow<'a, str>>,
    /// The address of a key pair that can be used to sign transactions for this account instead of
    /// the master key. Use a `SetRegularKey` transaction to change this value.
    pub regular_key: Option<Cow<'a, str>>,
    /// How many `Tickets` this account owns in the ledger. This is updated automatically to ensure
    /// that the account stays within the hard limit of 250 Tickets at a time. This field is omitted
    /// if the account has zero `Tickets`.
    pub ticket_count: Option<u8>,
    /// How many significant digits to use for exchange rates of Offers involving currencies issued
    /// by this address. Valid values are 3 to 15, inclusive.
    pub tick_size: Option<u8>,
    /// A transfer fee to charge other users for sending currency issued by this account to each other.
    pub transfer_rate: Option<u32>,
    /// An arbitrary 256-bit value that users can set.
    pub wallet_locator: Option<Cow<'a, str>>,
    /// Unused. (The code supports this field but there is no way to set it.)
    pub wallet_size: Option<u32>,
}

impl<'a> Default for AccountRoot<'a> {
    fn default() -> Self {
        Self {
            flags: Default::default(),
            index: Default::default(),
            account: Default::default(),
            ledger_entry_type: LedgerEntryType::AccountRoot,
            owner_count: Default::default(),
            previous_txn_id: Default::default(),
            previous_txn_lgr_seq: Default::default(),
            sequence: Default::default(),
            account_txn_id: Default::default(),
            balance: Default::default(),
            burned_nftokens: Default::default(),
            domain: Default::default(),
            email_hash: Default::default(),
            message_key: Default::default(),
            minted_nftokens: Default::default(),
            nftoken_minter: Default::default(),
            regular_key: Default::default(),
            ticket_count: Default::default(),
            tick_size: Default::default(),
            transfer_rate: Default::default(),
            wallet_locator: Default::default(),
            wallet_size: Default::default(),
        }
    }
}

impl<'a> Model for AccountRoot<'a> {}

impl<'a> LedgerObject for AccountRoot<'a> {
    fn get_ledger_object_type(&self) -> LedgerEntryType {
        self.ledger_entry_type.clone()
    }
}

impl<'a> AccountRoot<'a> {
    pub fn new(
        flags: Vec<AccountRootFlag>,
        index: Cow<'a, str>,
        account: Cow<'a, str>,
        owner_count: u32,
        previous_txn_id: Cow<'a, str>,
        previous_txn_lgr_seq: u32,
        sequence: u32,
        account_txn_id: Option<Cow<'a, str>>,
        balance: Option<XRPAmount<'a>>,
        burned_nftokens: Option<u32>,
        domain: Option<Cow<'a, str>>,
        email_hash: Option<Cow<'a, str>>,
        message_key: Option<Cow<'a, str>>,
        minted_nftokens: Option<u32>,
        nftoken_minter: Option<Cow<'a, str>>,
        regular_key: Option<Cow<'a, str>>,
        ticket_count: Option<u8>,
        tick_size: Option<u8>,
        transfer_rate: Option<u32>,
        wallet_locator: Option<Cow<'a, str>>,
        wallet_size: Option<u32>,
    ) -> Self {
        Self {
            ledger_entry_type: LedgerEntryType::AccountRoot,
            flags,
            index,
            account,
            owner_count,
            previous_txn_id,
            previous_txn_lgr_seq,
            sequence,
            account_txn_id,
            balance,
            burned_nftokens,
            domain,
            email_hash,
            message_key,
            minted_nftokens,
            nftoken_minter,
            regular_key,
            ticket_count,
            tick_size,
            transfer_rate,
            wallet_locator,
            wallet_size,
        }
    }
}

#[cfg(test)]
mod test_serde {
    use super::*;
    use alloc::borrow::Cow;
    use alloc::vec;

    #[test]
    fn test_serialize() {
        let account_root = AccountRoot::new(
            vec![AccountRootFlag::LsfDefaultRipple],
            Cow::from("13F1A95D7AAB7108D5CE7EEAF504B2894B8C674E6D68499076441C4837282BF8"),
            Cow::from("rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn"),
            3,
            Cow::from("0D5FB50FA65C9FE1538FD7E398FFFE9D1908DFA4576D8D7A020040686F93C77D"),
            14091160,
            336,
            Some(Cow::from(
                "0D5FB50FA65C9FE1538FD7E398FFFE9D1908DFA4576D8D7A020040686F93C77D",
            )),
            Some("148446663".into()),
            None,
            Some(Cow::from("6D64756F31332E636F6D")),
            Some(Cow::from("98B4375E1D753E5B91627516F6D70977")),
            Some(Cow::from("0000000000000000000000070000000300")),
            None,
            None,
            None,
            None,
            None,
            Some(1004999999),
            None,
            None,
        );
        let account_root_json = serde_json::to_string(&account_root).unwrap();
        let actual = account_root_json.as_str();
        let expected = r#"{"LedgerEntryType":"AccountRoot","Flags":8388608,"index":"13F1A95D7AAB7108D5CE7EEAF504B2894B8C674E6D68499076441C4837282BF8","Account":"rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn","OwnerCount":3,"PreviousTxnID":"0D5FB50FA65C9FE1538FD7E398FFFE9D1908DFA4576D8D7A020040686F93C77D","PreviousTxnLgrSeq":14091160,"Sequence":336,"AccountTxnID":"0D5FB50FA65C9FE1538FD7E398FFFE9D1908DFA4576D8D7A020040686F93C77D","Balance":"148446663","Domain":"6D64756F31332E636F6D","EmailHash":"98B4375E1D753E5B91627516F6D70977","MessageKey":"0000000000000000000000070000000300","TransferRate":1004999999}"#;

        assert_eq!(expected, actual);
    }

    // TODO: test_deserialize
}
