mod chain;
pub mod codec;
mod data_source;
pub mod mapper;
mod trigger;
use crate::codec::EntitiesChanges;
pub use chain::*;
pub use data_source::*;
use graph::blockchain;
use graph::blockchain::{BlockHash, BlockPtr};
use graph::prelude::BlockNumber;
pub use trigger::*;

#[derive(Clone, Debug, Default)]
pub struct Block {
    pub block_num: BlockNumber,
    pub block_hash: BlockHash,
    pub parent_block_num: BlockNumber,
    pub parent_block_hash: BlockHash,
    pub entities_changes: EntitiesChanges,
}

impl blockchain::Block for Block {
    fn ptr(&self) -> BlockPtr {
        return BlockPtr {
            hash: self.block_hash.clone(),
            number: self.block_num as i32,
        };
    }

    fn parent_ptr(&self) -> Option<BlockPtr> {
        Some(BlockPtr {
            hash: BlockHash(Box::from(self.parent_block_hash.as_slice())),
            number: self.parent_block_num as i32,
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
