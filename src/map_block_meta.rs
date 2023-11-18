use crate::pb::eth::block_meta::v1::BlockMeta;
use substreams::Hex;
use substreams_ethereum::pb::eth::v2::Block;

#[substreams::handlers::map]
fn map_block(blk: Block) -> Result<BlockMeta, substreams::errors::Error> {
    let header = blk.header.as_ref().unwrap();


    Ok(BlockMeta {
        number: blk.number,
        hash: Hex(&blk.hash).to_string(),
        parent_hash: Hex(&header.parent_hash).to_string(),
        timestamp: header.timestamp.as_ref().unwrap().to_string(),
    })
}