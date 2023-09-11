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
    pub fn new(disk: Disk) -> Option<Ext2FileSystem> {
        let mut superblock = [0 as u8; 1024];
        disk.set_position(1024);
        disk.read_one(&mut superblock);
        let superblock = unsafe {superblock as Superblock};
        if superblock.magic != Superblock::MAGIC {
            return None;
        }


        Some(Self {
            superblock,
            bgdt: BlockGroupDescriptor::new(),
            dev: disk,
        })
    }

    pub fn init(&'static self) {
        unimplemented!();
    }
}