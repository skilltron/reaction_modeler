//! Partitioned shard database for variants (MDNG-compatible).
//! O(1) lookup by (chromosome, position, ref, alt); merge/append; genomic order.
//! See doc/SHARD_DATABASE_GROWTH.md.

mod key;
mod record;
mod shard;
mod shard_db;

pub use key::VariantKey;
pub use record::ShardRecord;
pub use shard::Shard;
pub use shard_db::ShardDb;
