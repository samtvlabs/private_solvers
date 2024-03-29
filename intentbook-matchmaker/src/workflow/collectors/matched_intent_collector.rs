use anyhow::Result;
use artemis_core::types::{Collector, CollectorStream};
use async_trait::async_trait;
use futures::stream::StreamExt;

use crate::workflow::event::Event;
use solver_common::types::intent_bid::IntentBid;
pub use solver_common::workflow::event::Event as Common;

#[async_trait]
pub trait MatchedIntentsSource {
    async fn get_matched_intents_source(&self) -> Result<CollectorStream<'_, IntentBid>>;
}

pub struct MatchedIntentCollector<S: MatchedIntentsSource>(S);

impl<S: MatchedIntentsSource> MatchedIntentCollector<S> {
    pub fn new(source: S) -> Self {
        MatchedIntentCollector(source)
    }
}

#[async_trait]
impl<S: MatchedIntentsSource + Sync + Send> Collector<Event> for MatchedIntentCollector<S> {
    async fn get_event_stream(&self) -> Result<CollectorStream<'_, Event>> {
        let intents_stream = self.0.get_matched_intents_source().await?;
        let event_stream = intents_stream.map(|intent| {
            let cloned_intent = intent.clone();
            Event::NewMatchedIntent(Common::NewMatchedIntent(cloned_intent), intent)
        });
        Ok(Box::pin(event_stream))
    }
}
