use core::fmt;
use std::{str::FromStr, sync::Arc};

use anyhow::Error;
use graph::blockchain::block_stream::SubstreamsError::{
    MultipleModuleOutputError, UnexpectedStoreDeltaOutput,
};
use graph::blockchain::block_stream::{
    BlockStreamEvent, BlockWithTriggers, SubstreamsError, SubstreamsMapper,
};
use graph::firehose::FirehoseEndpoint;
use graph::substreams::module_output::Data;
use graph::substreams::{BlockScopedData, ForkStep};
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
    substreams,
};
use prost::Message;

use crate::{
    data_source::*, EntitiesChanges, SubstreamBlock, TriggerData, TriggerFilter, TriggersAdapter,
};

#[derive(Clone, Debug)]
pub struct Block {}

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

    type Block = SubstreamBlock;
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

pub struct Mapper {}

#[async_trait]
impl SubstreamsMapper<Chain> for Mapper {
    async fn to_block_stream_event(
        &self,
        logger: &Logger,
        block_scoped_data: &BlockScopedData,
    ) -> Result<Option<BlockStreamEvent<Chain>>, SubstreamsError> {
        let step = ForkStep::from_i32(block_scoped_data.step).unwrap_or_else(|| {
            panic!(
                "unknown step i32 value {}, maybe you forgot update & re-regenerate the protobuf definitions?",
                block_scoped_data.step
            )
        });

        let clock = block_scoped_data.clock.as_ref().unwrap();

        if block_scoped_data.outputs.len() == 0 {
            return Ok(None);
        }

        if block_scoped_data.outputs.len() > 1 {
            return Err(MultipleModuleOutputError());
        }

        //todo: handle step
        let module_output = &block_scoped_data.outputs[0];
        let cursor = &block_scoped_data.cursor;
        match module_output.data.as_ref().unwrap() {
            Data::MapOutput(msg) => {
                let changes: EntitiesChanges = Message::decode(msg.value.as_slice()).unwrap();
                Ok(Some(BlockStreamEvent::ProcessBlock(
                    BlockWithTriggers::new(
                        SubstreamBlock {
                            block_num: clock.number,
                            block_hash: hex::decode(&clock.id).unwrap(),
                            parent_block_num: 0,       //todo
                            parent_block_hash: vec![], //toto
                            entities_changes: changes,
                        },
                        vec![],
                    ),
                    FirehoseCursor::from(cursor.clone()),
                )))
            }
            Data::StoreDeltas(_) => Err(UnexpectedStoreDeltaOutput()),
        }

        //
        // let any_block = response
        //     .block
        //     .as_ref()
        //     .expect("block payload information should always be present");
        //
        // // Right now, this is done in all cases but in reality, with how the BlockStreamEvent::Revert
        // // is defined right now, only block hash and block number is necessary. However, this information
        // // is not part of the actual bstream::BlockResponseV2 payload. As such, we need to decode the full
        // // block which is useless.
        // //
        // // Check about adding basic information about the block in the bstream::BlockResponseV2 or maybe
        // // define a slimmed down stuct that would decode only a few fields and ignore all the rest.
        // let block = codec::Block::decode(any_block.value.as_ref())?;
        //
        // use ForkStep::*;
        // match step {
        //     StepNew => Ok(BlockStreamEvent::ProcessBlock(
        //         adapter.triggers_in_block(logger, block, filter).await?,
        //         FirehoseCursor::from(response.cursor.clone()),
        //     )),
        //
        //     StepUndo => {
        //         let parent_ptr = block
        //             .header()
        //             .parent_ptr()
        //             .expect("Genesis block should never be reverted");
        //
        //         Ok(BlockStreamEvent::Revert(
        //             parent_ptr,
        //             FirehoseCursor::from(response.cursor.clone()),
        //         ))
        //     }
        //
        //     StepIrreversible => {
        //         panic!("irreversible step is not handled and should not be requested in the Firehose request")
        //     }
        //
        //     StepUnknown => {
        //         panic!("unknown step should not happen in the Firehose response")
        //     }
        // }
    }
}
