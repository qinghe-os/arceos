mod superblock;
mod block_group;

use alloc::boxed::Box;
use alloc::sync::Arc;

use axfs_vfs::{VfsOps, VfsNodeRef};
use superblock::Superblock;
// use block_group::BlockGroupDescriptor;
use crate::dev::Disk;

pub struct Ext2FileSystem {
    pub superblock: Box<Superblock>,
    // bgdt: Box<BlockGroupDescriptor>,
    pub dev: Arc<Disk>,
}

impl Ext2FileSystem {
    #[cfg(feature = "use-ramdisk")]
    pub fn new(mut disk: Disk) -> Self {
        unimplemented!();
    }

    #[cfg(not(feature = "use-ramdisk"))]
    pub fn new(mut disk: Disk) -> Option<Ext2FileSystem> {
        let mut superblock = [0 as u8; 1024];
        disk.set_position(1024);
        disk.read_one(&mut superblock);
        let superblock = Superblock::new_from_buf(&mut superblock);
        let superblock = Box::new(superblock);
        if superblock.magic != Superblock::MAGIC {
            return None;
        }


        Some(Self {
            superblock,
            // bgdt: Box::new(BlockGroupDescriptor::new()),
            dev: Arc::new(disk),
        })
    }

    pub fn init(&'static self) {
        unimplemented!();
    }
}

impl VfsOps for Ext2FileSystem {
    fn root_dir(&self) -> VfsNodeRef {
        unimplemented!();
    }
}