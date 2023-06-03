use alloc::borrow::Cow;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::models::{requests::responses::RequestResponse, Model};
use crate::models::requests::responses::ResponseType;

#[skip_serializing_none]
#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct TrustLine<'a> {
    /// The unique Address of the counterparty to this trust line.
    pub account: Cow<'a, str>,
    /// Representation of the numeric balance currently held against this line. A positive balance
    /// means that the perspective account holds value; a negative balance means that the
    /// perspective account owes value.
    pub balance: Cow<'a, str>,
    /// A Currency Code identifying what currency this trust line can hold.
    pub currency: Cow<'a, str>,
    /// The maximum amount of the given currency that this account is willing to owe the peer
    /// account.
    pub limit: Cow<'a, str>,
    /// The maximum amount of currency that the counterparty account is willing to owe the
    /// perspective account
    pub limit_peer: Cow<'a, str>,
    /// Rate at which the account values incoming balances on this trust line, as a ratio of this
    /// value per 1 billion units. (For example, a value of 500 million represents a 0.5:1 ratio.)
    /// As a special case, 0 is treated as a 1:1 ratio.
    pub quality_in: u32, // TODO check size
    /// Rate at which the account values outgoing balances on this trust line, as a ratio of this
    /// value per 1 billion units. (For example, a value of 500 million represents a 0.5:1 ratio.)
    /// As a special case, 0 is treated as a 1:1 ratio.
    pub quality_out: u32, // TODO check size
    /// If true, this account has authorized this trust line. The default is false.
    pub authorized: Option<bool>,
    /// If true, this account has frozen this trust line. The default is false.
    pub freeze: Option<bool>,
    /// If true, the peer account has frozen this trust line. The default is false.
    pub freeze_peer: Option<bool>,
    /// If true, this account has enabled the No Ripple flag for this trust line. If present and
    /// false, this account has disabled the No Ripple flag, but, because the account also has the
    /// Default Ripple flag disabled, that is not considered the default state. If omitted, the
    /// account has the No Ripple flag disabled for this trust line and Default Ripple enabled.
    pub no_ripple: Option<bool>,
    /// If true, the peer account has enabled the No Ripple flag for this trust line. If present and
    /// false, this account has disabled the No Ripple flag, but, because the account also has the
    /// Default Ripple flag disabled, that is not considered the default state. If omitted, the
    /// account has the No Ripple flag disabled for this trust line and Default Ripple enabled.
    pub no_ripple_peer: Option<bool>,
    /// If true, the peer account has authorized this trust line. The default is false.
    pub peer_authorized: Option<bool>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct AccountLinesResponse<'a> {
    #[serde(skip_serializing)]
    #[serde(default = "ResponseType::account_lines")]
    pub response_type: ResponseType,
    /// Unique Address of the account this request corresponds to. This is the "perspective account"
    /// for purpose of the trust lines.
    pub account: Cow<'a, str>,
    /// Array of trust line objects, as described below. If the number of trust lines is large, only
    /// returns up to the limit at a time.
    pub lines: Vec<TrustLine<'a>>,
    /// The ledger index of the current open ledger, which was used when retrieving this information.
    pub ledger_current_index: Option<u32>,
    /// The identifying hash the ledger version that was used when retrieving this data.
    pub ledger_hash: Option<Cow<'a, str>>,
    /// The ledger index of the ledger version that was used when retrieving this data.
    pub ledger_index: Option<u32>,
    /// Server-defined value indicating the response is paginated. Pass this to the next call to
    /// resume where this call left off. Omitted when there are no additional pages after this one.
    pub marker: Option<Cow<'a, str>>,
}

impl<'a> Default for AccountLinesResponse<'a> {
    fn default() -> Self {
        Self {
            response_type: ResponseType::AccountLines,
            account: Default::default(),
            lines: Default::default(),
            ledger_current_index: Default::default(),
            ledger_hash: Default::default(),
            ledger_index: Default::default(),
            marker: Default::default(),
        }
    }
}

impl<'a> Model for AccountLinesResponse<'a> {}

impl<'a> RequestResponse for AccountLinesResponse<'a> {
    fn get_response_type(&self) -> ResponseType {
        self.response_type.clone()
    }
}

impl<'a> AccountLinesResponse<'a> {
    pub fn new(
        account: Cow<'a, str>,
        lines: Vec<TrustLine<'a>>,
        ledger_current_index: Option<u32>,
        ledger_hash: Option<Cow<'a, str>>,
        ledger_index: Option<u32>,
        marker: Option<Cow<'a, str>>,
    ) -> Self {
        Self {
            response_type: ResponseType::AccountLines,
            account,
            lines,
            ledger_current_index,
            ledger_hash,
            ledger_index,
            marker,
        }
    }
}
