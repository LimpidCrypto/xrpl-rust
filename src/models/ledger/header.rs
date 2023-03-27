use crate::_serde::lgr_obj_flags;
use crate::models::Model;
use alloc::borrow::Cow;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use strum_macros::{AsRefStr, Display, EnumIter};

/// See Close Flags:
/// `<https://xrpl.org/ledger-header.html#close-flags>`
#[derive(
    Debug, Eq, PartialEq, Clone, Serialize_repr, Deserialize_repr, Display, AsRefStr, EnumIter,
)]
#[repr(u32)]
pub enum CloseFlag {
    /// If this flag is enabled, it means that validators had different close times for the
    /// ledger, but built otherwise the same ledger, so they declared consensus while
    /// "agreeing to disagree" on the close time.
    sLCFNoConsensusTime = 1,
}

/// Every ledger version has a unique header that describes the contents.
///
/// See Ledger Header:
/// `<https://xrpl.org/ledger-header.html>`
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Default)]
pub struct LedgerHeader<'a> {
    /// The SHA-512Half of this ledger's state tree information.
    account_hash: Cow<'a, str>,
    /// A bit-map of flags relating to the closing of this ledger.
    #[serde(with = "lgr_obj_flags")]
    close_flags: Vec<CloseFlag>,
    /// The approximate time this ledger version closed, as the number of seconds since the
    /// Ripple Epoch of 2000-01-01 00:00:00. This value is rounded based on the
    /// `close_time_resolution.
    close_time: u32,
    /// An integer in the range 2,120 indicating the maximum number of seconds by which
    /// the close_time could be rounded.
    close_time_resolution: u8,
    /// If true, this ledger version is no longer accepting new transactions.
    closed: bool,
    /// The ledger index of the ledger.
    ledger_index: Cow<'a, str>,
    /// The SHA-512Half of this ledger version. This serves as a unique identifier for
    /// this ledger and all its contents.
    ledger_hash: Cow<'a, str>,
    /// The ledger_hash value of the previous ledger version that is the direct
    /// predecessor of this one.
    parent_hash: Cow<'a, str>,
    /// The total number of drops of XRP owned by accounts in the ledger. This omits XRP
    /// that has been destroyed by transaction fees.
    total_coins: Cow<'a, str>,
    /// The SHA-512Half of the transactions included in this ledger.
    transaction_hash: Cow<'a, str>,
}

impl<'a> Model for LedgerHeader<'a> {}

impl<'a> LedgerHeader<'a> {
    pub fn new(
        account_hash: Cow<'a, str>,
        close_flags: Vec<CloseFlag>,
        close_time: u32,
        close_time_resolution: u8,
        closed: bool,
        ledger_index: Cow<'a, str>,
        ledger_hash: Cow<'a, str>,
        parent_hash: Cow<'a, str>,
        total_coins: Cow<'a, str>,
        transaction_hash: Cow<'a, str>,
    ) -> Self {
        Self {
            account_hash,
            close_flags,
            close_time,
            close_time_resolution,
            closed,
            ledger_index,
            ledger_hash,
            parent_hash,
            total_coins,
            transaction_hash,
        }
    }
}
