use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use strum_macros::Display;

use crate::models::{requests::RequestMethod, Model};

/// Represents the object types that an AccountObjects
/// Request can ask for.
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Display)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
pub enum AccountObjectType {
    Check,
    DepositPreauth,
    Escrow,
    Offer,
    PaymentChannel,
    SignerList,
    State,
    Ticket,
}

/// This request returns the raw ledger format for all objects
/// owned by an account. For a higher-level view of an account's
/// trust lines and balances, see AccountLines Request instead.
///
/// See Account Objects:
/// `<https://xrpl.org/account_objects.html>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct AccountObjects<'a> {
    /// A unique identifier for the account, most commonly the
    /// account's address.
    pub account: &'a str,
    /// The unique request id.
    pub id: Option<&'a str>,
    /// A 20-byte hex string for the ledger version to use.
    pub ledger_hash: Option<&'a str>,
    /// The ledger index of the ledger to use, or a shortcut
    /// string to choose a ledger automatically.
    pub ledger_index: Option<&'a str>,
    /// If included, filter results to include only this type
    /// of ledger object. The valid types are: check, deposit_preauth,
    /// escrow, offer, payment_channel, signer_list, ticket,
    /// and state (trust line).
    pub r#type: Option<AccountObjectType>,
    /// If true, the response only includes objects that would block
    /// this account from being deleted. The default is false.
    pub deletion_blockers_only: Option<bool>,
    /// The maximum number of objects to include in the results.
    /// Must be within the inclusive range 10 to 400 on non-admin
    /// connections. The default is 200.
    pub limit: Option<u16>,
    /// Value from a previous paginated response. Resume retrieving
    /// data where that response left off.
    pub marker: Option<u32>,
    /// The request method.
    #[serde(default = "RequestMethod::account_objects")]
    pub command: RequestMethod,
}

impl<'a> Default for AccountObjects<'a> {
    fn default() -> Self {
        AccountObjects {
            account: "",
            id: None,
            ledger_hash: None,
            ledger_index: None,
            r#type: None,
            deletion_blockers_only: None,
            limit: None,
            marker: None,
            command: RequestMethod::AccountObjects,
        }
    }
}

impl<'a> Model for AccountObjects<'a> {}

impl<'a> AccountObjects<'a> {
    fn new(
        account: &'a str,
        id: Option<&'a str>,
        ledger_hash: Option<&'a str>,
        ledger_index: Option<&'a str>,
        r#type: Option<AccountObjectType>,
        deletion_blockers_only: Option<bool>,
        limit: Option<u16>,
        marker: Option<u32>,
    ) -> Self {
        Self {
            account,
            id,
            ledger_hash,
            ledger_index,
            r#type,
            deletion_blockers_only,
            limit,
            marker,
            command: RequestMethod::AccountObjects,
        }
    }
}
