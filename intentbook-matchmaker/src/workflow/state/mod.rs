use crate::types::intent::Intent;
use crate::types::intent_bid::IntentBid;

pub mod in_memory_state_manager;
pub mod state_manager;

#[derive(Debug, Clone)]
pub struct IntentState {
    pub intent: Intent,
    pub matched_bid: Option<IntentBid>,

    // TODO: this flag only applies to SpokeChainCall intent. Refactor structs and move it there.
    pub is_spoke_chain_called: bool,
}

impl IntentState {
    pub fn new(intent: Intent) -> Self {
        IntentState {
            intent,
            matched_bid: None,
            is_spoke_chain_called: false,
        }
    }

    pub fn is_ready_to_settle(&self) -> bool {
        match &self.intent {
            &Intent::SpokeChainCall(..) => self.is_spoke_chain_called,
            _ => false,
        }
    }
}
