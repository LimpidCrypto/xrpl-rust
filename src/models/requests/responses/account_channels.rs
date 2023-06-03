use alloc::borrow::Cow;
use alloc::vec;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::models::requests::responses::ResponseType;
use crate::models::{requests::responses::RequestResponse, Model};

#[skip_serializing_none]
#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct AccountChannel<'a> {
    /// The owner of the channel, as an Address.
    pub account: Cow<'a, str>,
    /// The total amount of XRP, in drops allocated to this channel.
    pub amount: Cow<'a, str>,
    /// The total amount of XRP, in drops, paid out from this channel, as of the ledger version
    /// used. (You can calculate the amount of XRP left in the channel by subtracting balance from
    /// amount.)
    pub balance: Cow<'a, str>,
    /// A unique ID for this channel, as a 64-character hexadecimal string. This is also the ID of
    /// the channel object in the ledger's state data.
    pub channel_id: Cow<'a, str>,
    /// The destination account of the channel, as an Address. Only this account can receive the XRP
    /// in the channel while it is open.
    pub destination_account: Cow<'a, str>,
    /// The number of seconds the payment channel must stay open after the owner of the channel
    /// requests to close it.
    pub settle_delay: u32, // TODO: check if size
    /// Time, in seconds since the Ripple Epoch, of this channel's immutable expiration, if one was
    /// specified at channel creation. If this is before the close time of the most recent validated
    /// ledger, the channel is expired.
    pub cancel_after: Option<u32>,
    /// A 32-bit unsigned integer to use as a destination tag for payments through this channel,
    /// if one was specified at channel creation. This indicates the payment channel's beneficiary
    /// or other purpose at the destination account.
    pub destination_tag: Option<u32>,
    /// Time, in seconds since the Ripple Epoch, when this channel is set to expire. This expiration
    /// date is mutable. If this is before the close time of the most recent validated ledger, the
    /// channel is expired.
    pub expiration: Option<u32>,
    /// The public key for the payment channel in the XRP Ledger's base58 format. Signed claims
    /// against this channel must be redeemed with the matching key pair.
    pub public_key: Option<Cow<'a, str>>,
    /// The public key for the payment channel in hexadecimal format, if one was specified at channel
    /// creation. Signed claims against this channel must be redeemed with the matching key pair.
    pub public_key_hex: Option<Cow<'a, str>>,
    /// A 32-bit unsigned integer to use as a source tag for payments through this payment channel,
    /// if one was specified at channel creation. This indicates the payment channel's originator or
    /// other purpose at the source account. Conventionally, if you bounce payments from this
    /// channel, you should specify this value in the DestinationTag of the return payment.
    pub source_tag: Option<u32>,
}

impl<'a> Model for AccountChannel<'a> {}

impl<'a> AccountChannel<'a> {
    fn new(
        account: Cow<'a, str>,
        amount: Cow<'a, str>,
        balance: Cow<'a, str>,
        channel_id: Cow<'a, str>,
        destination_account: Cow<'a, str>,
        settle_delay: u32, // TODO: check if size
        cancel_after: Option<u32>,
        destination_tag: Option<u32>,
        expiration: Option<u32>,
        public_key: Option<Cow<'a, str>>,
        public_key_hex: Option<Cow<'a, str>>,
        source_tag: Option<u32>,
    ) -> Self {
        Self {
            account,
            amount,
            balance,
            channel_id,
            destination_account,
            settle_delay,
            cancel_after,
            destination_tag,
            expiration,
            public_key,
            public_key_hex,
            source_tag,
        }
    }
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct AccountChannelsResponse<'a> {
    #[serde(skip_serializing)]
    #[serde(default = "ResponseType::account_channels")]
    pub response_type: ResponseType,
    /// The address of the source/owner of the payment channels. This corresponds to the account
    /// field of the request.
    pub account: Cow<'a, str>,
    /// Payment channels owned by this account.
    pub channels: Vec<AccountChannel<'a>>,
    /// The Ledger Index of the ledger version used to generate this response.
    pub ledger_index: u32,
    /// The identifying Hash of the ledger version used to generate this response.
    pub ledger_hash: Option<Cow<'a, str>>,
    /// The limit to how many channel objects were actually returned by this request.
    pub limit: Option<u32>, // TODO check size
    /// Server-defined value for pagination. Pass this to the next call to resume getting results
    /// where this call left off. Omitted when there are no additional pages after this one.
    pub marker: Option<Cow<'a, str>>,
    /// If true, the information in this response comes from a validated ledger version.
    /// Otherwise, the information is subject to change.
    pub validated: Option<bool>,
}

impl<'a> Default for AccountChannelsResponse<'a> {
    fn default() -> Self {
        Self {
            response_type: ResponseType::AccountChannels,
            account: Default::default(),
            channels: Default::default(),
            ledger_index: Default::default(),
            ledger_hash: Default::default(),
            limit: Default::default(),
            marker: Default::default(),
            validated: Default::default(),
        }
    }
}

impl<'a> Model for AccountChannelsResponse<'a> {}

impl<'a> RequestResponse for AccountChannelsResponse<'a> {
    fn get_response_type(&self) -> ResponseType {
        self.response_type.clone()
    }
}

impl<'a> AccountChannelsResponse<'a> {
    pub fn new(
        account: Cow<'a, str>,
        channels: Vec<AccountChannel<'a>>,
        ledger_index: u32,
        ledger_hash: Option<Cow<'a, str>>,
        limit: Option<u32>,
        marker: Option<Cow<'a, str>>,
        validated: Option<bool>,
    ) -> Self {
        Self {
            response_type: ResponseType::AccountChannels,
            account,
            channels,
            ledger_index,
            ledger_hash,
            limit,
            marker,
            validated,
        }
    }
}
