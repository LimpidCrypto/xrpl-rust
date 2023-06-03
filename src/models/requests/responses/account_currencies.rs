use alloc::borrow::Cow;
use alloc::vec;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::models::{requests::responses::RequestResponse, Model};
use crate::models::requests::responses::ResponseType;

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct AccountCurrenciesResponse<'a> {
    #[serde(skip_serializing)]
    #[serde(default = "ResponseType::account_currencies")]
    pub response_type: ResponseType,
    /// The ledger index of the ledger version used to retrieve this data.
    pub ledger_index: u32,
    /// Array of Currency Codes for currencies that this account can receive.
    pub receive_currencies: Vec<Cow<'a, str>>,
    /// Array of Currency Codes for currencies that this account can send.
    pub send_currencies: Vec<Cow<'a, str>>,
    /// If true, this data comes from a validated ledger.
    pub validated: bool,
    /// The identifying hash of the ledger version used to retrieve this data, as hex.
    pub ledger_hash: Option<Cow<'a, str>>,
}

impl<'a> Default for AccountCurrenciesResponse<'a> {
    fn default() -> Self {
        Self {
            response_type: ResponseType::AccountCurrencies,
            ledger_hash: Default::default(),
            ledger_index: Default::default(),
            receive_currencies: Default::default(),
            send_currencies: Default::default(),
            validated: Default::default(),
        }
    }
}

impl<'a> Model for AccountCurrenciesResponse<'a> {}

impl<'a> RequestResponse for AccountCurrenciesResponse<'a> {
    fn get_response_type(&self) -> ResponseType {
        self.response_type.clone()
    }
}

impl<'a> AccountCurrenciesResponse<'a> {
    pub fn new(
        ledger_index: u32,
        receive_currencies: Vec<Cow<'a, str>>,
        send_currencies: Vec<Cow<'a, str>>,
        validated: bool,
        ledger_hash: Option<Cow<'a, str>>,
    ) -> Self {
        Self {
            response_type: ResponseType::AccountCurrencies,
            ledger_index,
            receive_currencies,
            send_currencies,
            validated,
            ledger_hash,
        }
    }
}
