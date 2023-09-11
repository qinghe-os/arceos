use core::fmt::{self, Debug};

/// The Superblock contains all information about the layout of the file system
/// and possibly contains other important information like what optional
/// features were used to create the file system.
///
/// The Superblock is always located at byte 1024 from the beginning of the
/// volume and is exactly 1024 bytes in length. For example, if the disk uses
/// 512 byte sectors, the Superblock will begin at LBA 2 and will occupy all of
/// sector 2 and 3.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Superblock {
    // taken from https://wiki.osdev.org/Ext2
    /// Total number of inodes in file system
    pub inodes_count: u32,
    /// Total number of blocks in file system
    pub blocks_count: u32,
    /// Number of blocks reserved for superuser (see offset 80)
    pub r_blocks_count: u32,
    /// Total number of unallocated blocks
    pub free_blocks_count: u32,
    /// Total number of unallocated inodes
    pub free_inodes_count: u32,
    /// Block number of the block containing the superblock
    pub first_data_block: u32,
    /// log2 (block size) - 10. (In other words, the number to shift 1,024
    /// to the left by to obtain the block size)
    pub log_block_size: u32,
    /// log2 (fragment size) - 10. (In other words, the number to shift
    /// 1,024 to the left by to obtain the fragment size)
    pub log_frag_size: i32,
    /// Number of blocks in each block group
    pub blocks_per_group: u32,
    /// Number of fragments in each block group
    pub frags_per_group: u32,
    /// Number of inodes in each block group
    pub inodes_per_group: u32,
    /// Last mount time (in POSIX time)
    pub mtime: u32,
    /// Last written time (in POSIX time)
    pub wtime: u32,
    /// Number of times the volume has been mounted since its last
    /// consistency check (fsck)
    pub mnt_count: u16,
    /// Number of mounts allowed before a consistency check (fsck) must be
    /// done
    pub max_mnt_count: i16,
    /// Ext2 signature (0xef53), used to help confirm the presence of Ext2
    /// on a volume
    pub magic: u16,
    /// File system state (see `FS_CLEAN` and `FS_ERR`)
    pub state: u16,
    /// What to do when an error is detected (see `ERR_IGNORE`, `ERR_RONLY` and
    /// `ERR_PANIC`)
    pub errors: u16,
    /// Minor portion of version (combine with Major portion below to
    /// construct full version field)
    pub rev_minor: u16,
    /// POSIX time of last consistency check (fsck)
    pub lastcheck: u32,
    /// Interval (in POSIX time) between forced consistency checks (fsck)
    pub checkinterval: u32,
    /// Operating system ID from which the filesystem on this volume was
    /// created
    pub creator_os: u32,
    /// Major portion of version (combine with Minor portion above to
    /// construct full version field)
    pub rev_major: u32,
    /// User ID that can use reserved blocks
    pub block_uid: u16,
    /// Group ID that can use reserved blocks
    pub block_gid: u16,

    /// First non-reserved inode in file system.
    pub first_inode: u32,
    /// SectorSize of each inode structure in bytes.
    pub inode_size: u16,
    /// Block group that this superblock is part of (if backup copy)
    pub block_group: u16,
    /// Optional features present (features that are not required to read
    /// or write, but usually result in a performance increase)
    pub features_opt: u32,
    /// Required features present (features that are required to be
    /// supported to read or write)
    pub features_req: u32,
    /// Features that if not supported, the volume must be mounted
    /// read-only)
    pub features_ronly: u32,
    /// File system ID (what is output by blkid)
    pub fs_id: [u8; 16],
    /// Volume name (C-style string: characters terminated by a 0 byte)
    pub volume_name: [u8; 16],
    /// Path volume was last mounted to (C-style string: characters
    /// terminated by a 0 byte)
    pub last_mnt_path: [u8; 64],
    /// Compression algorithms used (see Required features above)
    pub compression: u32,
    /// Number of blocks to preallocate for files
    pub prealloc_blocks_files: u8,
    /// Number of blocks to preallocate for directories
    pub prealloc_blocks_dirs: u8,
    #[doc(hidden)]
    _unused: [u8; 2],
    /// Journal ID (same style as the File system ID above)
    pub journal_id: [u8; 16],
    /// Journal inode
    pub journal_inode: u32,
    /// Journal device
    pub journal_dev: u32,
    /// Head of orphan inode list
    pub journal_orphan_head: u32,
    #[doc(hidden)]
    _reserved: [u8; 788],
}

impl Debug for Superblock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Superblock")
            .field("inodes_count", &self.inodes_count)
            .field("blocks_count", &self.blocks_count)
            .field("r_blocks_count", &self.r_blocks_count)
            .field("free_blocks_count", &self.free_blocks_count)
            .field("free_inodes_count", &self.free_inodes_count)
            .field("first_data_block", &self.first_data_block)
            .field("log_block_size", &self.log_block_size)
            .field("log_frag_size", &self.log_frag_size)
            .field("blocks_per_group", &self.blocks_per_group)
            .field("frags_per_group", &self.frags_per_group)
            .field("inodes_per_group", &self.inodes_per_group)
            .field("mtime", &self.mtime)
            .field("wtime", &self.wtime)
            .field("mnt_count", &self.mnt_count)
            .field("max_mnt_count", &self.max_mnt_count)
            .field("magic", &self.magic)
            .field("state", &self.state)
            .field("errors", &self.errors)
            .field("rev_minor", &self.rev_minor)
            .field("lastcheck", &self.lastcheck)
            .field("checkinterval", &self.checkinterval)
            .field("creator_os", &self.creator_os)
            .field("rev_major", &self.rev_major)
            .field("block_uid", &self.block_uid)
            .field("block_gid", &self.block_gid)
            .field("first_inode", &self.first_inode)
            .field("inode_size", &self.inode_size)
            .field("block_group", &self.block_group)
            .field("features_opt", &self.features_opt)
            .field("features_req", &self.features_req)
            .field("features_ronly", &self.features_ronly)
            .field("fs_id", &self.fs_id)
            .field("volume_name", &self.volume_name)
            .field("last_mnt_path", &self.last_mnt_path.as_ref())
            .field("compression", &self.compression)
            .field("prealloc_blocks_files", &self.prealloc_blocks_files)
            .field("prealloc_blocks_dirs", &self.prealloc_blocks_dirs)
            .field("journal_id", &self.journal_id)
            .field("journal_inode", &self.journal_inode)
            .field("journal_dev", &self.journal_dev)
            .field("journal_orphan_head", &self.journal_orphan_head)
            .finish()
    }
}

impl Superblock{
    pub const MAGIC: u16 = 0xef53;

    pub fn from_mut(buf: &mut[u8]) -> Self {
        Self {
            inodes_count: buf[0..3],
            blocks_count: buf[4..7],
            r_blocks_count: buf[8..11],
            free_blocks_count: buf[12..15],
            free_inodes_count: buf[16..19],
            first_data_block: buf[20..23],
            
        }
    }
}