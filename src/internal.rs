pub mod block_list;
pub mod card_table;
pub mod collection_barrier;
pub mod finalize_trait;
pub mod gc_info;
pub mod pointer_policies;
pub mod space_bitmap;
pub mod stack_bounds;
pub mod trace_trait;

pub const BLOCK_SIZE: usize = 16 * 1024;
