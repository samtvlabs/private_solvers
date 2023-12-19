use anyhow::anyhow;
use bindings_khalani::limit_order_intent_book::Intent;
use ethers::abi::{AbiDecode, AbiEncode};
use ethers::types::{Address, Bytes, U256};
use solver_common::config::chain::ChainId;
use std::sync::Arc;

use solver_common::inventory::{amount::Amount, token::Token, Inventory};

use crate::types::intent_id::IntentId;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LimitOrderIntent {
    pub intent_id: IntentId,
    pub author: Address,
    pub signature: Bytes,
    pub volume: Amount,
    pub token: Token,
    pub out_token: Token,
    pub price: U256,
}

impl TryFrom<(Arc<Inventory>, Intent)> for LimitOrderIntent {
    type Error = anyhow::Error;

    fn try_from(value: (Arc<Inventory>, Intent)) -> Result<Self, Self::Error> {
        let (inventory, intent) = value;
        let limit_order =
            bindings_khalani::limit_order_intent_book::LimitOrder::decode(intent.intent)?;
        let token = inventory
            .find_token_by_address(limit_order.token, ChainId::Khalani)
            .ok_or(anyhow!("Unknown LimitOrder token {}", limit_order.token))?;
        let out_token = inventory
            .find_token_by_address(limit_order.out_token, ChainId::Khalani)
            .ok_or(anyhow!(
                "Unknown LimitOrder out token {}",
                limit_order.out_token
            ))?;
        Ok(Self {
            intent_id: Default::default(),
            author: limit_order.author,
            signature: intent.signature,
            price: limit_order.price,
            token: token.clone(),
            out_token: out_token.clone(),
            volume: Amount::from_token_base_units(limit_order.volume, token),
        })
    }
}

impl From<LimitOrderIntent> for bindings_khalani::limit_order_intent_book::LimitOrder {
    fn from(value: LimitOrderIntent) -> Self {
        Self {
            author: value.author,
            token: value.token.address,
            price: value.price,
            volume: value.volume.base_units,
            out_token: value.out_token.address,
        }
    }
}

impl From<LimitOrderIntent> for Intent {
    fn from(value: LimitOrderIntent) -> Self {
        let limit_order: bindings_khalani::limit_order_intent_book::LimitOrder =
            value.clone().into();
        Self {
            intent: Bytes::from(limit_order.encode()),
            signature: value.signature.clone(),
        }
    }
}
