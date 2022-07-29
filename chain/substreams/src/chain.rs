use crate::{data_source::*, Block, TriggerData, TriggerFilter, TriggersAdapter};
use anyhow::Error;
use core::fmt;
use graph::{
    blockchain::{
        self,
        block_stream::{BlockStream, FirehoseCursor},
        BlockPtr, Blockchain, BlockchainKind, IngestorError, RuntimeAdapter,
    },
    components::store::DeploymentLocator,
    data::subgraph::UnifiedMappingApiVersion,
    impl_slog_value,
    prelude::{async_trait, BlockNumber, ChainStore},
    slog::Logger,
};
use std::{str::FromStr, sync::Arc};

#[derive(Debug)]
pub struct Chain {}

#[derive(Clone, Debug)]
pub enum GraphEntityFinality {
    // Final(Arc<>)
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Ord, Eq)]
pub struct NodeCapabilities {}

impl FromStr for NodeCapabilities {
    type Err = Error;

    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        Ok(NodeCapabilities {})
    }
}

impl fmt::Display for NodeCapabilities {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("substream")
    }
}

impl_slog_value!(NodeCapabilities, "{}");

impl graph::blockchain::NodeCapabilities<Chain> for NodeCapabilities {
    fn from_data_sources(_data_sources: &[DataSource]) -> Self {
        NodeCapabilities {}
    }
}

#[async_trait]
impl Blockchain for Chain {
    const KIND: BlockchainKind = BlockchainKind::Substream;

    type Block = Block;
    type DataSource = DataSource;
    type UnresolvedDataSource = UnresolvedDataSource;

    type DataSourceTemplate = NoopDataSourceTemplate;
    type UnresolvedDataSourceTemplate = NoopDataSourceTemplate;

    /// Trigger data as parsed from the triggers adapter.
    type TriggerData = TriggerData;

    /// Decoded trigger ready to be processed by the mapping.
    /// New implementations should have this be the same as `TriggerData`.
    type MappingTrigger = TriggerData;

    /// Trigger filter used as input to the triggers adapter.
    type TriggerFilter = TriggerFilter;

    type NodeCapabilities = NodeCapabilities;

    fn triggers_adapter(
        &self,
        _log: &DeploymentLocator,
        _capabilities: &Self::NodeCapabilities,
        _unified_api_version: UnifiedMappingApiVersion,
    ) -> Result<Arc<dyn blockchain::TriggersAdapter<Self>>, Error> {
        Ok(Arc::new(TriggersAdapter {}))
    }

    async fn new_firehose_block_stream(
        &self,
        _deployment: DeploymentLocator,
        _block_cursor: FirehoseCursor,
        _start_blocks: Vec<BlockNumber>,
        _subgraph_current_block: Option<BlockPtr>,
        _filter: Arc<Self::TriggerFilter>,
        _unified_api_version: UnifiedMappingApiVersion,
    ) -> Result<Box<dyn BlockStream<Self>>, Error> {
        unimplemented!("this should never be called for substreams")
    }

    async fn new_polling_block_stream(
        &self,
        _deployment: DeploymentLocator,
        _start_blocks: Vec<BlockNumber>,
        _subgraph_current_block: Option<BlockPtr>,
        _filter: Arc<Self::TriggerFilter>,
        _unified_api_version: UnifiedMappingApiVersion,
    ) -> Result<Box<dyn BlockStream<Self>>, Error> {
        unimplemented!("this should never be called for substreams")
    }

    fn chain_store(&self) -> Arc<dyn ChainStore> {
        unimplemented!()
    }

    async fn block_pointer_from_number(
        &self,
        _logger: &Logger,
        _number: BlockNumber,
    ) -> Result<BlockPtr, IngestorError> {
        unimplemented!()
    }
    fn runtime_adapter(&self) -> Arc<dyn RuntimeAdapter<Self>> {
        unimplemented!()
    }

    fn is_firehose_supported(&self) -> bool {
        unimplemented!()
    }
}
