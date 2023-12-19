use bindings_khalani::spoke_chain_call_intent_book::SpokeChainCall as ContractSpokeChainCall;
use ethers::abi::AbiDecode;
use ethers::types::{Address, Bytes, U256};
use solver_common::config::chain::ChainId;
use solver_common::types::intent_id::{IntentId, WithIntentId};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SpokeChainCall {
    pub intent_id: IntentId,
    pub author: Address,
    pub chain_id: ChainId,
    pub contract_to_call: Address,
    pub call_data: Bytes,
    pub token: Address,
    pub amount: U256,
}

impl TryFrom<WithIntentId<bindings_khalani::base_intent_book::Intent>> for SpokeChainCall {
    type Error = anyhow::Error;

    fn try_from(
        value: WithIntentId<bindings_khalani::base_intent_book::Intent>,
    ) -> Result<Self, Self::Error> {
        let (intent_id, value) = value;
        let value: ContractSpokeChainCall = ContractSpokeChainCall::decode(value.intent)?;
        Ok(SpokeChainCall {
            intent_id,
            author: value.author,
            chain_id: value.chain_id.try_into()?,
            contract_to_call: value.contract_to_call,
            call_data: value.call_data,
            token: value.token,
            amount: value.amount,
        })
    }
}
