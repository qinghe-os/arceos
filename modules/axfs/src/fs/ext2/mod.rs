mod superblock;
mod block_group;

use alloc::boxed::Box;
use alloc::sync::Arc;

use superblock::Superblock;
use block_group::BlockGroupDescriptor;
use crate::dev::Disk;

pub struct Ext2FileSystem {
    superblock: Box<Superblock>,
    bgdt: Box<BlockGroupDescriptor>,
    dev: Arc<Disk>,
}

impl Ext2FileSystem {
    #[cfg(feature = "use-ramdisk")]
    pub fn new(mut disk: Disk) -> Self {
        unimplemented!();
    }

    #[cfg(not(feature = "use-ramdisk"))]
    pub fn new(disk: Disk) -> Self {
        unimplemented!();
    }

    pub fn init(&'static self) {
        unimplemented!();
    }
}