use core::cell::UnsafeCell;
use axfs_vfs::VfsNodeRef;

use ext2::Ext2;

use crate::dev::Disk;

pub struct Ext2FileSystem {
    inner: ext2::Ext2<>,
    root_dir: UnsafeCell<Option<VfsNodeRef>>,
}

