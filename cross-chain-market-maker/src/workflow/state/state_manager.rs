use crate::workflow::state::IntentState;
use solver_common::types::intent_id::IntentId;
use solver_common::types::limit_order_intent::LimitOrderIntent;

pub trait StateManager {
    fn update_state(&mut self, intent_id: IntentId, new_state: IntentState);
    fn get_state(&self, intent_id: IntentId) -> Option<&IntentState>;
    fn get_all_intents(&self) -> Vec<IntentState>;

    fn create_intent_state(&mut self, limit_order_intent: LimitOrderIntent) -> IntentId;

    fn update_intent_state<F>(&mut self, intent_id: IntentId, updater: F) -> Option<IntentState>
    where
        F: FnOnce(&mut IntentState);
}
