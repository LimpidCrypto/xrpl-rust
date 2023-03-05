pub mod account_delete;
pub mod account_set;
pub mod check_cancel;
pub mod check_cash;
pub mod check_create;
pub mod deposit_preauth;
pub mod escrow_cancel;
pub mod escrow_create;
pub mod escrow_finish;
pub mod exceptions;
pub mod nftoken_accept_offer;
pub mod nftoken_burn;
pub mod nftoken_cancel_offer;
pub mod nftoken_create_offer;
pub mod nftoken_mint;
pub mod offer_cancel;
pub mod offer_create;
pub mod payment;
pub mod payment_channel_claim;
pub mod payment_channel_create;
pub mod payment_channel_fund;
pub mod pseudo_transactions;
pub mod set_regular_key;
pub mod signer_list_set;
pub mod ticket_create;
pub mod trust_set;

pub use account_delete::*;
pub use account_set::*;
pub use check_cancel::*;
pub use check_cash::*;
pub use check_create::*;
pub use deposit_preauth::*;
pub use enable_amendment::*;
pub use escrow_cancel::*;
pub use escrow_create::*;
pub use escrow_finish::*;
pub use nftoken_accept_offer::*;
pub use nftoken_burn::*;
pub use nftoken_cancel_offer::*;
pub use nftoken_create_offer::*;
pub use nftoken_mint::*;
pub use offer_cancel::*;
pub use offer_create::*;
pub use payment::*;
pub use payment_channel_claim::*;
pub use payment_channel_create::*;
pub use payment_channel_fund::*;
pub use pseudo_transactions::*;
pub use set_fee::*;
pub use set_regular_key::*;
pub use signer_list_set::*;
pub use ticket_create::*;
pub use trust_set::*;
pub use unl_modify::*;

use thiserror_no_std::Error;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum TransactionFlag {
    AccountSet(AccountSetFlag),
    NFTokenCreateOffer(NFTokenCreateOfferFlag),
    NFTokenMint(NFTokenMintFlag),
    OfferCreate(OfferCreateFlag),
    Payment(PaymentFlag),
    PaymentChannelClaim(PaymentChannelClaimFlag),
    TrustSet(TrustSetFlag),
    EnableAmendment(EnableAmendmentFlag),
}
