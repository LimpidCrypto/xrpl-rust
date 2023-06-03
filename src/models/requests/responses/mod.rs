pub mod account_channels;
pub mod account_currencies;
pub mod account_info;
pub mod account_lines;
pub mod account_nfts;
pub mod account_objects;
pub mod account_offers;
pub mod account_tx;
pub mod book_offers;
pub mod channel_authorize;
pub mod channel_verify;
pub mod deposit_authorized;
pub mod fee;
pub mod gateway_balances;
pub mod ledger;
pub mod ledger_closed;
pub mod ledger_current;
pub mod ledger_data;
pub mod ledger_entry;
pub mod manifest;
pub mod nft_buy_offers;
pub mod nft_sell_offers;
pub mod noripple_check;
pub mod path_find;
pub mod ripple_path_find;
pub mod server_info;
pub mod server_state;
pub mod submit;
pub mod submit_multisigned;
pub mod subscribe;
pub mod transaction_entry;
pub mod tx;
pub mod tx_history;
pub mod unsubscribe;

use serde::{Deserialize, Serialize};
use strum_macros::Display;

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Display)]
#[strum(serialize_all = "snake_case")]
pub enum ResponseType {
    // Account request responses
    AccountChannels,
    AccountCurrencies,
    AccountInfo,
    AccountLines,
    AccountNfts,
    AccountObjects,
    AccountOffers,
    AccountTx,
    GatewayBalances,
    #[serde(rename = "noripple_check")]
    NoRippleCheck,
    // Ledger request responses
    Ledger,
    LedgerClosed,
    LedgerCurrent,
    LedgerData,
    LedgerEntry,
    // Transaction request responses
    Submit,
    SubmitMultisigned,
    TransactionsEntry,
    Tx,
    TxHistory,
    // Path and Order Book request responses
    BookOffers,
    DepositAuthorized,
    NftBuyOffers,
    NftSellOffers,
    PathFind,
    RipplePathFind,
    // Payment Channel request responses
    ChannelAuthorize,
    ChannelVerify,
    // Subscription request responses
    Subscribe,
    Unsubscribe,
    // Server info request responses
    Fee,
    Manifest,
    ServerInfo,
    ServerState,
    // Utility request responses
    Json,
    Ping,
    Random,
}

impl ResponseType {
    fn account_channels() -> Self {
        ResponseType::AccountChannels
    }
    fn account_currencies() -> Self {
        ResponseType::AccountCurrencies
    }
    fn account_info() -> Self {
        ResponseType::AccountInfo
    }
    fn account_lines() -> Self {
        ResponseType::AccountLines
    }
    fn account_nfts() -> Self {
        ResponseType::AccountNfts
    }
    fn account_objects() -> Self {
        ResponseType::AccountObjects
    }
    fn account_offers() -> Self {
        ResponseType::AccountOffers
    }
    fn account_tx() -> Self {
        ResponseType::AccountTx
    }
    fn gateway_balances() -> Self {
        ResponseType::GatewayBalances
    }
    fn noripple_check() -> Self {
        ResponseType::NoRippleCheck
    }
    fn ledger() -> Self {
        ResponseType::Ledger
    }
    fn ledger_closed() -> Self {
        ResponseType::LedgerClosed
    }
    fn ledger_current() -> Self {
        ResponseType::LedgerCurrent
    }
    fn ledger_data() -> Self {
        ResponseType::LedgerData
    }
    fn ledger_entry() -> Self {
        ResponseType::LedgerEntry
    }
    fn submit() -> Self {
        ResponseType::Submit
    }
    fn submit_multisigned() -> Self {
        ResponseType::SubmitMultisigned
    }
    fn transaction_entry() -> Self {
        ResponseType::TransactionsEntry
    }
    fn tx() -> Self {
        ResponseType::Tx
    }
    fn tx_history() -> Self {
        ResponseType::TxHistory
    }
    fn book_offers() -> Self {
        ResponseType::BookOffers
    }
    fn deposit_authorized() -> Self {
        ResponseType::DepositAuthorized
    }
    fn nft_buy_offers() -> Self {
        ResponseType::NftBuyOffers
    }
    fn nft_sell_offers() -> Self {
        ResponseType::NftSellOffers
    }
    fn path_find() -> Self {
        ResponseType::PathFind
    }
    fn ripple_path_find() -> Self {
        ResponseType::RipplePathFind
    }
    fn channel_authorized() -> Self {
        ResponseType::ChannelAuthorize
    }
    fn channel_verify() -> Self {
        ResponseType::ChannelVerify
    }
    fn subscribe() -> Self {
        ResponseType::Subscribe
    }
    fn unsubscribe() -> Self {
        ResponseType::Unsubscribe
    }
    fn fee() -> Self {
        ResponseType::Fee
    }
    fn manifest() -> Self {
        ResponseType::Manifest
    }
    fn server_info() -> Self {
        ResponseType::ServerInfo
    }
    fn server_state() -> Self {
        ResponseType::ServerState
    }
    fn json() -> Self {
        ResponseType::Json
    }
    fn ping() -> Self {
        ResponseType::Ping
    }
    fn random() -> Self {
        ResponseType::Random
    }
}

pub trait RequestResponse {
    fn get_response_type(&self) -> ResponseType;
}
