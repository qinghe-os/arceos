cfg_if::cfg_if! {
    if #[cfg(feature = "myfs")] {
        pub mod myfs;
    } else if #[cfg(feature = "ext2")] {
        pub mod ext2;
    } else if #[cfg(feature = "fatfs")] {
        pub mod fatfs;
    }
}

#[cfg(feature = "devfs")]
pub use axfs_devfs as devfs;

#[cfg(feature = "ramfs")]
pub use axfs_ramfs as ramfs;
