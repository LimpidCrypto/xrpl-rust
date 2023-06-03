use alloc::borrow::Cow;
use alloc::vec::Vec;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::ser::SerializeStruct;
use serde_with::skip_serializing_none;

use crate::models::{requests::responses::RequestResponse, Model};
use crate::models::ledger::{AccountRoot, LedgerObject};
use crate::models::requests::responses::ResponseType;

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct AccountObjectsResponse<'a, T>
where
    T: Serialize + Deserialize<'a>
{
    #[serde(skip_serializing)]
    #[serde(default = "ResponseType::account_objects")]
    pub response_type: ResponseType,
    /// Unique Address of the account this request corresponds to.
    pub account: Cow<'a, str>,
    /// Array of objects owned by this account. Each object is in its raw ledger format.
    pub account_objects: Vec<T>,
    /// If included and set to true, the information in this response comes from a validated ledger
    /// version. Otherwise, the information is subject to change.
    pub validated: bool,
    /// The ledger index of the current in-progress ledger version, which was used to generate this
    /// response.
    pub ledger_current_index: Option<u32>,
    /// The identifying hash of the ledger that was used to generate this response.
    pub ledger_hash: Option<Cow<'a, str>>,
    /// The ledger index of the ledger version that was used to generate this response.
    pub ledger_index: Option<u32>,
    /// The limit that was used in this request, if any.
    pub limit: Option<u32>,
    /// Server-defined value indicating the response is paginated. Pass this to the next call to
    /// resume where this call left off. Omitted when there are no additional pages after this one.
    pub marker: Option<Cow<'a, str>>,
}

impl<'a, T> Default for AccountObjectsResponse<'a, T>
where
    T: Serialize + Deserialize<'a>
{
    fn default() -> Self {
        Self {
            response_type: ResponseType::AccountObjects,
            account: Default::default(),
            account_objects: Default::default(),
            validated: Default::default(),
            ledger_current_index: Default::default(),
            ledger_hash: Default::default(),
            ledger_index: Default::default(),
            limit: Default::default(),
            marker: Default::default(),
        }
    }
}

impl<'a, T> Model for AccountObjectsResponse<'a, T>
where
    T: Serialize + Deserialize<'a>
{}

impl<'a, T> RequestResponse for AccountObjectsResponse<'a, T>
where
    T: Serialize + Deserialize<'a>
{
    fn get_response_type(&self) -> ResponseType {
        self.response_type.clone()
    }
}

impl<'a, T> AccountObjectsResponse<'a, T>
where
    T: Serialize + Deserialize<'a> + LedgerObject
{
    pub fn new(
        account: Cow<'a, str>,
        account_objects: Vec<T>,
        validated: bool,
        ledger_current_index: Option<u32>,
        ledger_hash: Option<Cow<'a, str>>,
        ledger_index: Option<u32>,
        limit: Option<u32>,
        marker: Option<Cow<'a, str>>,
    ) -> Self {
        Self {
            response_type: ResponseType::AccountObjects,
            account,
            account_objects,
            validated,
            ledger_current_index,
            ledger_hash,
            ledger_index,
            limit,
            marker,
        }
    }
}

#[cfg(test)]
mod test_serde {
    use super::*;

    #[test]
    fn test_deserialize<'a>() {
        let json_string = r#"
            {
                "account": "r9cZA1mLK5R5Am25ArfXFmqgNwjZgnfk59",
                "account_objects": [
                    {
                        "Account": "rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn",
                        "AccountTxnID": "0D5FB50FA65C9FE1538FD7E398FFFE9D1908DFA4576D8D7A020040686F93C77D",
                        "Balance": "148446663",
                        "Domain": "6D64756F31332E636F6D",
                        "EmailHash": "98B4375E1D753E5B91627516F6D70977",
                        "Flags": 8388608,
                        "LedgerEntryType": "AccountRoot",
                        "MessageKey": "0000000000000000000000070000000300",
                        "OwnerCount": 3,
                        "PreviousTxnID": "0D5FB50FA65C9FE1538FD7E398FFFE9D1908DFA4576D8D7A020040686F93C77D",
                        "PreviousTxnLgrSeq": 14091160,
                        "Sequence": 336,
                        "TransferRate": 1004999999,
                        "index": "13F1A95D7AAB7108D5CE7EEAF504B2894B8C674E6D68499076441C4837282BF8"
                    }
                ],
                "ledger_hash": "053DF17D2289D1C4971C22F235BC1FCA7D4B3AE966F842E5819D0749E0B8ECD3",
                "ledger_index": 14378733,
                "limit": 10,
                "marker": "F60ADF645E78B69857D2E4AEC8B7742FEABC8431BD8611D099B428C3E816DF93,94A9F05FEF9A153229E2E997E64919FD75AAE2028C8153E8EBDB4440BD3ECBB5",
                "validated": true
            }
        "#;
        let account_objects: AccountObjectsResponse<'a, AccountRoot> = serde_json::from_str(&json_string).unwrap();
    }
}
