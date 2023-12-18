use crate::quote::quoted_intent::QuotedIntent;
use crate::types::intent_id::IntentId;
use crate::types::limit_order_intent::LimitOrderIntent;
use crate::workflow::executors::post_limit_order_executor::PostLimitOrderHandlerResult;
use crate::workflow::executors::settle_intent_executor::SwapIntentSettlementData;
use crate::workflow::executors::swap_intent_filler_executor::SwapIntentFillerHandlerResult;

pub mod in_memory_state_manager;
pub mod state_manager;

#[derive(Debug, Clone)]
pub struct IntentState {
    pub intent_id: IntentId,
    pub limit_order_intent: LimitOrderIntent,
    pub quoted_intent: Option<QuotedIntent>,

    pub lock_intent_tokens_handler_result: Option<PostLimitOrderHandlerResult>,
    pub filler_handler_result: Option<SwapIntentFillerHandlerResult>,

    pub is_proved_that_tokens_locked_on_source_chain: bool,
    pub is_proved_that_filled_on_destination_chain: bool,
}

impl IntentState {
    pub fn new(limit_order_intent: LimitOrderIntent) -> Self {
        IntentState {
            intent_id: limit_order_intent.intent_id,
            limit_order_intent,
            quoted_intent: None,
            lock_intent_tokens_handler_result: None,
            filler_handler_result: None,
            is_proved_that_filled_on_destination_chain: false,
            is_proved_that_tokens_locked_on_source_chain: false,
        }
    }

    pub fn get_intent_id(&self) -> IntentId {
        self.intent_id
    }

    pub fn is_ready_to_settle(&self) -> bool {
        self.is_proved_that_tokens_locked_on_source_chain
            && self.is_proved_that_filled_on_destination_chain
    }

    pub fn get_settlement_data(&self) -> Option<SwapIntentSettlementData> {
        if !self.is_ready_to_settle() {
            return None;
        }
        let quoted_intent = match &self.quoted_intent {
            None => return None,
            Some(quoted_intent) => quoted_intent,
        };
        let filler_handler_result = match &self.filler_handler_result {
            None => return None,
            Some(filler_handler_result) => filler_handler_result,
        };
        Some(SwapIntentSettlementData {
            quoted_intent: quoted_intent.clone(),
            filler: filler_handler_result.filler,
            fill_timestamp: filler_handler_result.fill_timestamp,
            fill_amount: filler_handler_result.fill_amount,
        })
    }
}