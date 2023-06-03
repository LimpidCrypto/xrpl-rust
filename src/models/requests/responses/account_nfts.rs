use alloc::borrow::Cow;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::models::{requests::responses::RequestResponse, Model};
use crate::models::requests::responses::ResponseType;

#[skip_serializing_none]
#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct NFT<'a> {
    pub flags: u32,
    pub issuer: Cow<'a, str>,
    pub nftoken_id: Cow<'a, str>,
    pub nftoken_taxon: u32,
    pub uri: Cow<'a, str>,
    pub nft_serial: u32,
}

impl<'a> Model for NFT<'a> {}

impl<'a> NFT<'a> {
    pub fn new(
        flags: u32,
        issuer: Cow<'a, str>,
        nftoken_id: Cow<'a, str>,
        nftoken_taxon: u32,
        uri: Cow<'a, str>,
        nft_serial: u32,
    ) -> Self {
        Self {
            flags,
            issuer,
            nftoken_id,
            nftoken_taxon,
            uri,
            nft_serial,
        }
    }
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct AccountNftsResponse<'a> {
    #[serde(skip_serializing)]
    #[serde(default = "ResponseType::account_nfts")]
    pub response_type: ResponseType,
    pub account: Cow<'a, str>,
    pub account_nfts: Vec<NFT<'a>>,
    pub validated: bool,
    pub ledger_hash: Option<Cow<'a, str>>,
    pub ledger_index: Option<u32>,
    pub ledger_current_index: Option<u32>,
}

impl<'a> Default for AccountNftsResponse<'a> {
    fn default() -> Self {
        Self {
            response_type: ResponseType::AccountNfts,
            account: Default::default(),
            account_nfts: Default::default(),
            validated: Default::default(),
            ledger_hash: Default::default(),
            ledger_index: Default::default(),
            ledger_current_index: Default::default(),
        }
    }
}

impl<'a> Model for AccountNftsResponse<'a> {}

impl<'a> RequestResponse for AccountNftsResponse<'a> {
    fn get_response_type(&self) -> ResponseType {
        self.response_type.clone()
    }
}

impl<'a> AccountNftsResponse<'a> {
    pub fn new(
        account: Cow<'a, str>,
        account_nfts: Vec<NFT<'a>>,
        validated: bool,
        ledger_hash: Option<Cow<'a, str>>,
        ledger_index: Option<u32>,
        ledger_current_index: Option<u32>,
    ) -> Self {
        Self {
            response_type: ResponseType::AccountNfts,
            account,
            account_nfts,
            validated,
            ledger_hash,
            ledger_index,
            ledger_current_index,
        }
    }
}
