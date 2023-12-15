use std::sync::Arc;

use ethers::abi::{encode_packed, Token as AbiToken};
use ethers::types::H256;
use ethers::utils::keccak256;
use tracing::debug;

use crate::types::proof_id::ProofId;
use crate::workflow::event::Event;
use crate::workflow::state::IntentState;
use solver_common::connectors::Connector;

pub struct ProofsToEventsMapper {
    connector: Arc<Connector>,
}

impl ProofsToEventsMapper {
    pub fn new(connector: Arc<Connector>) -> Self {
        Self { connector }
    }

    pub fn map_new_proof_to_event(
        &self,
        all_intents: Vec<IntentState>,
        proof_id: ProofId,
    ) -> Option<Event> {
        for intent_state in all_intents {
            let event = self.try_map_source_chain_tokens_lock_proof_event(proof_id, &intent_state);
            if event.is_some() {
                return event;
            }

            let event = self.try_map_swap_intent_filled_proof_event(proof_id, &intent_state);
            if event.is_some() {
                return event;
            }
        }
        None
    }

    fn try_map_source_chain_tokens_lock_proof_event(
        &self,
        proof_id: ProofId,
        intent_state: &IntentState,
    ) -> Option<Event> {
        let intent_id = intent_state.intent_id;
        let swap_intent_token_lock_event = encode_packed(&[
            AbiToken::String(String::from("SwapIntentTokenLock")),
            AbiToken::FixedBytes(Vec::from(intent_id.as_bytes())),
        ])
        .ok()?;
        let expected_proof_id = keccak256(swap_intent_token_lock_event).into();
        debug!(
            ?intent_id,
            ?proof_id,
            ?expected_proof_id,
            "Trying to map SwapIntentTokenLock proof onto event"
        );
        if proof_id == expected_proof_id {
            Some(Event::ProvedTokensLockedOnSourceChain(intent_id))
        } else {
            None
        }
    }

    fn try_map_swap_intent_filled_proof_event(
        &self,
        proof_id: ProofId,
        intent_state: &IntentState,
    ) -> Option<Event> {
        let filler_handler_result = match intent_state.clone().filler_handler_result {
            Some(filler_handler_result) => filler_handler_result,
            None => return None,
        };
        let fill_timestamp = filler_handler_result.fill_timestamp;
        let quoted_intent = match intent_state.clone().quoted_intent {
            Some(quoted_intent) => quoted_intent,
            None => return None,
        };

        let intent_id = intent_state.intent_id;

        let filler_address = self.connector.get_address();
        let fill_amount = quoted_intent.destination_amount.base_units;
        let swap_intent_filled_event = encode_packed(&[
            AbiToken::String(String::from("SwapIntentFilled")),
            AbiToken::FixedBytes(Vec::from(intent_id.as_bytes())),
            AbiToken::Address(filler_address),
            AbiToken::FixedBytes(Vec::from(
                H256::from_low_u64_be(fill_timestamp.as_u64()).as_bytes(),
            )),
            AbiToken::FixedBytes(Vec::from(
                H256::from_low_u64_be(fill_amount.as_u64()).as_bytes(),
            )),
        ])
        .ok()?;
        let expected_proof_id = keccak256(swap_intent_filled_event).into();
        debug!(
            ?intent_id,
            ?proof_id,
            ?expected_proof_id,
            "Trying to map SwapIntentFilled proof onto event"
        );
        if proof_id == expected_proof_id {
            Some(Event::ProvedSwapIntentFilledOnDestinationChain(intent_id))
        } else {
            None
        }
    }
}