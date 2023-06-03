use alloc::borrow::Cow;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::models::{requests::responses::RequestResponse, Model};
use crate::models::ledger::AccountRoot;
use crate::models::ledger::signer_list::SignerList;
use crate::models::requests::responses::ResponseType;

#[skip_serializing_none]
#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Transaction<'a> {
    /// Whether this transaction changes this address's ways of authorizing transactions.
    pub auth_change: Option<bool>,
    /// The Transaction Cost of this transaction, in drops of XRP.
    pub fee: Option<Cow<'a, str>>,
    /// The transaction cost of this transaction, relative to the minimum cost for this type of
    /// transaction, in fee levels.
    pub fee_level: Option<Cow<'a, str>>,
    #[serde(rename = "LastLedgerSequence")]
    pub last_ledger_sequence: Option<u32>,
    /// The maximum amount of XRP, in drops, this transaction could send or destroy.
    pub max_spend_drops: Option<Cow<'a, str>>,
    /// The Sequence Number of this transaction.
    pub seq: Option<u32>,
}

impl<'a> Model for Transaction<'a> {}

impl<'a> Transaction<'a> {
    pub fn new(
        auth_change: Option<bool>,
        fee: Option<Cow<'a, str>>,
        fee_level: Option<Cow<'a, str>>,
        last_ledger_sequence: Option<u32>,
        max_spend_drops: Option<Cow<'a, str>>,
        seq: Option<u32>,
    ) -> Self {
        Self {
            auth_change,
            fee,
            fee_level,
            last_ledger_sequence,
            max_spend_drops,
            seq,
        }
    }
}

#[skip_serializing_none]
#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct QueueData<'a> {
    /// Number of queued transactions from this address.
    pub txn_count: u32,
    /// Whether a transaction in the queue changes this address's ways of authorizing transactions.
    /// If true, this address can queue no further transactions until that transaction has been
    /// executed or dropped from the queue.
    pub auth_change_queued: Option<bool>,
    /// The lowest Sequence Number among transactions queued by this address.
    pub lowest_sequence: Option<u32>,
    /// The highest Sequence Number among transactions queued by this address.
    pub highest_sequence: Option<u32>,
    /// Integer amount of drops of XRP that could be debited from this address if every transaction
    /// in the queue consumes the maximum amount of XRP possible.
    pub max_spend_drops_total: Option<Cow<'a, str>>,
    /// Information about each queued transaction from this address.
    pub transactions: Option<Vec<Transaction<'a>>>,
}

impl<'a> Model for QueueData<'a> {}

impl<'a> QueueData<'a> {
    pub fn new(
        txn_count: u32,
        auth_change_queued: Option<bool>,
        lowest_sequence: Option<u32>,
        highest_sequence: Option<u32>,
        max_spend_drops_total: Option<Cow<'a, str>>,
        transactions: Option<Vec<Transaction<'a>>>,
    ) -> Self {
        Self {
            txn_count,
            auth_change_queued,
            lowest_sequence,
            highest_sequence,
            max_spend_drops_total,
            transactions,
        }
    }
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct AccountInfoResponse<'a> {
    #[serde(skip_serializing)]
    #[serde(default = "ResponseType::account_info")]
    pub response_type: ResponseType,
    /// The AccountRoot ledger object with this account's information, as stored in the ledger.
    pub account_data: AccountRoot<'a>,
    /// The ledger index of the current in-progress ledger, which was used when retrieving this
    /// information.
    pub ledger_current_index: Option<u32>,
    /// The ledger index of the ledger version used when retrieving this information. The
    /// information does not contain any changes from ledger versions newer than this one.
    pub ledger_index: Option<u32>,
    /// Information about queued transactions sent by this account. This information describes the
    /// state of the local rippled server, which may be different from other servers in the
    /// peer-to-peer XRP Ledger network. Some fields may be omitted because the values are calculated
    /// "lazily" by the queuing mechanism.
    pub queue_data: Option<QueueData<'a>>,
    /// Array of SignerList ledger objects associated with this account for Multi-Signing. Since an
    /// account can own at most one SignerList, this array must have exactly one member if it is
    /// present.
    pub signer_lists: Option<Vec<SignerList<'a>>>,
    /// True if this data is from a validated ledger version; if omitted or set to false, this data
    /// is not final.
    pub validated: Option<bool>,
}

impl<'a> Default for AccountInfoResponse<'a> {
    fn default() -> Self {
        Self {
            response_type: ResponseType::AccountInfo,
            account_data: Default::default(),
            ledger_current_index: Default::default(),
            ledger_index: Default::default(),
            queue_data: Default::default(),
            signer_lists: Default::default(),
            validated: Default::default(),
        }
    }
}

impl<'a> Model for AccountInfoResponse<'a> {}

impl<'a> RequestResponse for AccountInfoResponse<'a> {
    fn get_response_type(&self) -> ResponseType {
        self.response_type.clone()
    }
}

impl<'a> AccountInfoResponse<'a> {
    pub fn new(
        account_data: AccountRoot<'a>,
        ledger_current_index: Option<u32>,
        ledger_index: Option<u32>,
        queue_data: Option<QueueData<'a>>,
        signer_lists: Option<Vec<SignerList<'a>>>,
        validated: Option<bool>,
    ) -> Self {
        Self {
            response_type: ResponseType::AccountInfo,
            account_data,
            ledger_current_index,
            ledger_index,
            queue_data,
            signer_lists,
            validated,
        }
    }
}
