use core::cell::UnsafeCell;
use axfs_vfs::VfsNodeRef;

use ext2::Ext2;

use crate::dev::Disk;

pub struct Ext2FileSystem {
    inner: ext2::Ext2<>,
    root_dir: UnsafeCell<Option<VfsNodeRef>>,
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