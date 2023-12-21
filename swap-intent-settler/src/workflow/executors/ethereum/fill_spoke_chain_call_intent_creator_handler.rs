use std::sync::Arc;

use anyhow::{anyhow, Result};
use async_trait::async_trait;
use bindings_khalani::swap_intent_filler::SwapIntentFiller;
use ethers::types::U256;

use intentbook_matchmaker::types::spoke_chain_call::{SpokeChainCall, SpokeChainCallStub};
use solver_common::config::addresses::AddressesConfig;
use solver_common::connectors::Connector;
use solver_common::error::ConfigError;
use solver_common::inventory::amount::Amount;
use solver_common::inventory::Inventory;

use crate::quote::quoted_swap_intent::QuotedSwapIntent;
use crate::workflow::executors::fill_spoke_chain_call_intent_creator_executor::{
    FillCreatorHandlerResult, FillSpokeChainCallIntentCreatorHandler,
};

pub struct FillSpokeChainCallIntentCreatorHandlerImpl {
    connector: Arc<Connector>,
    addresses_config: AddressesConfig,
    inventory: Arc<Inventory>,
}

impl FillSpokeChainCallIntentCreatorHandlerImpl {
    pub fn new(
        addresses_config: AddressesConfig,
        connector: Arc<Connector>,
        inventory: Arc<Inventory>,
    ) -> Self {
        Self {
            addresses_config,
            connector,
            inventory,
        }
    }
}

#[async_trait]
impl FillSpokeChainCallIntentCreatorHandler for FillSpokeChainCallIntentCreatorHandlerImpl {
    async fn create_swap_intent_filler(
        &self,
        quoted_intent: QuotedSwapIntent,
    ) -> Result<FillCreatorHandlerResult> {
        let spoke_chain_call = self.create_spoke_chain_call_intent(&quoted_intent)?;
        Ok(FillCreatorHandlerResult { spoke_chain_call })
    }
}

impl FillSpokeChainCallIntentCreatorHandlerImpl {
    fn create_spoke_chain_call_intent(
        &self,
        quoted_intent: &QuotedSwapIntent,
    ) -> Result<SpokeChainCall> {
        let settler_address = self.connector.get_address();
        let destination_chain_id = quoted_intent.swap_intent.destination_chain_id;
        let swap_intent_filler_address = self
            .addresses_config
            .swap_intent_fillers
            .get(&destination_chain_id)
            .ok_or_else(|| {
                ConfigError::ContractAddressNotFound(
                    String::from("Swap intent filler"),
                    destination_chain_id.into(),
                )
            })?;
        let rpc_client = self.connector.get_rpc_client(destination_chain_id)?;
        let swap_intent_filler_contract =
            SwapIntentFiller::new(*swap_intent_filler_address, rpc_client);
        let call_data = swap_intent_filler_contract
            .fill_swap_intent(
                quoted_intent.swap_intent.clone().into(),
                settler_address,
                quoted_intent.destination_amount.base_units,
            )
            .calldata()
            .ok_or(anyhow!("Unable to encode SwapIntentFiller"))?;

        let source_chain_mirror_token = self.inventory.find_mirror_token(
            quoted_intent.swap_intent.source_token,
            quoted_intent.swap_intent.source_chain_id,
        )?;

        // TODO: currently, the reward for the Spoke Chain Call is 1 USD denominated in the source mirror tokens.
        let reward_token = source_chain_mirror_token.address;
        let reward_amount = Amount::from_user_units_token(
            U256::from_dec_str("1").unwrap(),
            source_chain_mirror_token,
        );
        SpokeChainCall::create_signed(
            self.connector.clone(),
            SpokeChainCallStub {
                chain_id: destination_chain_id,
                contract_to_call: *swap_intent_filler_address,
                call_data,
                token: quoted_intent.swap_intent.destination_token,
                amount: quoted_intent.destination_amount.base_units,
                reward_token,
                reward_amount: reward_amount.base_units,
            },
        )
    }
}
