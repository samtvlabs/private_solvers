use std::sync::Arc;

use crate::types::swap_intent::SwapIntent;
use anyhow::Result;
use artemis_core::types::{Collector, CollectorStream};
use async_trait::async_trait;
use bindings_khalani::intents_mempool::{IntentCreatedFilter, IntentsMempool};
use ethers::contract::Event;
use ethers::middleware::Middleware;
use ethers::providers::PubsubClient;
use ethers::types::Address;
use futures::StreamExt;

/// A new intent event, containing the intent parameters.
#[derive(Debug, Clone)]
pub struct NewSwapIntent(pub SwapIntent);

/// A collector that listens for new intents, and generates a stream of
/// [events](NewSwapIntent) which contain the intent parameters.
pub struct IntentsCollector<M> {
    intent_created_filter: Event<Arc<M>, M, IntentCreatedFilter>,
}

impl<M: Middleware> IntentsCollector<M> {
    pub fn new(provider: Arc<M>, intents_mempool_address: Address) -> Self {
        let intents_mempool = IntentsMempool::new(intents_mempool_address, provider.clone());
        Self {
            intent_created_filter: intents_mempool.intent_created_filter(),
        }
    }
}

/// Implementation of the [Collector](Collector) trait for the [IntentsCollector](IntentsCollector).
/// This implementation uses the [PubsubClient](PubsubClient) to subscribe to new blocks.
#[async_trait]
impl<M> Collector<NewSwapIntent> for IntentsCollector<M>
where
    M: Middleware + 'static,
    M::Provider: PubsubClient,
    M::Error: 'static,
{
    async fn get_event_stream(&self) -> Result<CollectorStream<'_, NewSwapIntent>> {
        let intents_stream = self.intent_created_filter.subscribe().await?;
        let intents_stream = intents_stream.filter_map(|event| async {
            match event {
                Ok(event) => Some(NewSwapIntent(SwapIntent {
                    intent_id: event.intent_id.into(),
                    author: event.intent.author,
                    signature: event.intent.signature,
                    source_chain_id: event.intent.source_chain_id,
                    destination_chain_id: event.intent.destination_chain_id,
                    source_token: event.intent.source_token,
                    destination_token: event.intent.destination_token,
                    source_amount: event.intent.source_amount,
                    source_permit_2: event.intent.source_permit_2,
                })),
                Err(_) => None, // TODO: consider better error handling.
            }
        });
        Ok(Box::pin(intents_stream))
    }
}
