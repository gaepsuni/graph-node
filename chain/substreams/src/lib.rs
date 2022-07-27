mod chain;
mod data_source;
mod trigger;
mod substreams_block_stream;
pub mod codec;

pub use chain::*;
pub use trigger::*;
pub use data_source::*;

use graph::blockchain;
use graph::blockchain::{BlockHash, BlockPtr};
use crate::codec::EntitiesChanges;

#[derive(Clone, Debug, Default)]
pub struct SubstreamBlock {
    pub block_num: u64,
    pub block_hash: Vec<u8>,
    pub parent_block_num: u64,
    pub parent_block_hash: Vec<u8>,
    pub entities_changes: EntitiesChanges,
}

impl blockchain::Block for SubstreamBlock {
    fn ptr(&self) -> BlockPtr {
        return BlockPtr {
            hash: BlockHash(Box::from(self.block_hash.as_slice())),
            number: self.block_num as i32
        }
    }

    fn parent_ptr(&self) -> Option<BlockPtr> {
        if self.parent_block_hash.is_empty() {
            return None;
        }

        Some(BlockPtr {
            hash: BlockHash(Box::from(self.parent_block_hash.as_slice())),
            number: self.parent_block_num as i32
        })
    }

    fn number(&self) -> i32 {
        self.ptr().number
    }

    fn hash(&self) -> BlockHash {
        self.ptr().hash
    }

    fn parent_hash(&self) -> Option<BlockHash> {
        self.parent_ptr().map(|ptr| ptr.hash)
    }

    fn data(&self) -> Result<jsonrpc_core::serde_json::Value, jsonrpc_core::serde_json::Error> {
        Ok(jsonrpc_core::serde_json::Value::Null)
    }
}
