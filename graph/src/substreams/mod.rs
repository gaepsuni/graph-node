mod codec;

use crate::blockchain;
use crate::prelude::{BlockHash, BlockPtr};
pub use codec::*;

impl blockchain::Block for BlockScopedData {
    fn ptr(&self) -> BlockPtr {
        let clock = self.clock.as_ref().unwrap();
        return BlockPtr {
            hash: BlockHash(Box::from(hex::decode(&clock.id).unwrap())),
            number: clock.number as i32,
        };
    }

    fn parent_ptr(&self) -> Option<BlockPtr> {
        Some(self.ptr())

        // todo: need to implement for parent_ptr
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

    fn data(&self) -> Result<serde_json::Value, serde_json::Error> {
        Ok(serde_json::Value::Null)
    }
}
