#### 逻辑层次

| 每层名称       | 对应文件或目录                    | 核心变量                  |
| -------------- | --------------------------------- | ------------------------- |
| 文件描述符层   | `syscall/fs.rs, fs/file_table.rs` | `FileTable`               |
| 索引结点缓存层 | `fs/cache.rs, fs/inode.rs`        | `INodeCache, DirCache`    |
| 索引结点层     | `fs/ext2`                         | `INode, Ext2`             |
| 块缓存层       | `fs/block`                        | `CachedPage`, `BLOCK_DEV` |
| 块设备驱动层   | `drivers/block`                   |                           |

#### 文件系统初始化过程

1. `fs::init()`

   被`aero_main()`调用，位置在`src/aero_kernel/src/fs/mod.rs`。其作用是载入文件系统。它仅仅是调用`cache::init()`。

2. `cache::init()`

   创建`INODE_CACHE`和`DIR_CACHE`，即`INodeCache`和`DirCache`。

3. `fs::block::launch()`

   被`kernel_main_thread()`里的`modules::init()`调用，位置在`src/aero_kernel/src/fs/block/mod.rs`。

   1. 通过嵌套的for循环查找GPT硬盘及其上的分区。如果是ext2分区则挂载它。
   1. `super::devfs::init()?;`，安装devfs。
   1. `super::procfs::init()?;`，安装procfs。


#### 使用文件系统

- aero_syscall库

  `sys_XXX()`最终执行的`syscall`指令，从而进到内核态执行系统调用。

- syscall模块

  1. 让系统调用号`SYS_XXX`对应到具体的函数`self::fs::xxx()`。

  2. `syscall::fs::xxx()`直接或间接调用到`crate::fs`模块实现具体的功能。


#### 文件描述符层

##### FileTable

代表了整个系统里打开的文件，是一个由文件柄组成的动态数组。

```rust
pub fn new();	// 创建文件表
pub fn get_handle(fd);	// 获取文件描述符fd对应的文件柄
pub fn log();	// 输出调试信息
pub fn close_on_exec();	// 如果设置了O_CLOEXEC标志，则关闭文件
pub fn duplicate(fd,hint,flags);	// 
pub fn deep_clone();	//
pub fn open_file();	// 
pub fn close_file(fd);	// 关闭文件描述符对应的文件
```



##### FileHandle

代表了文件柄。

```rust
pub fn new();	// 创建一个新的文件柄
pub fn read(buffer);	// 把文件的内容读入缓存区buffer
pub fn seek(off, whence);	// 把文件的偏移位置修改为off
pub fn write(buffer);	// 把buffer的内容写入文件
pub fn dirnode();	// 获取目录的索引结点号
pub fn inode();	// 获取文件的索引结点号
pub fn duplicate(dupfd, flags);	// 创建一个文件柄，其文件描述符为dupfd.
pub fn get_dents(buffer);	// 把"dir entry"的信息写入buffer
```

#### 索引结点缓存层

##### DirEntry

"directory entry"基本上就是文件名和索引结点之间的映射。

```rust
pub fn new(parent,inode,name);	// 创建一个新的目录条目。其父目录是parent,索引结点是inode,名字是name。
pub fn new_root(inode,name);	// 创建根目录的目录条目。其索引结点是inode,名字是name。
pub fn from_inode(inode,name);	// 从inode创建目录条目，此目录条目名字是name。
pub fn from_socket_inode(parent,name,inode);	// 从inode创建网络插座的目录条目。其父目录是parent,名字是name,网络插座的索引结点号是inode。
pub fn name();	// 获取目录条目的名字
pub fn set_name(name);	// 设置目录条目的名字是name。
pub fn set_parent(parent);	// 设置目录条目的父目录为parent。
pub fn inode();	// 获取当前目录条目里缓存的索引结点条目。
pub fn parent();	// 获取当前目录条目的父目录
pub fn drop_from_cache();	// 从缓存里删除目录条目
```



##### Cache

对缓存的抽象。是块缓存、索引结点缓存、目录缓存的基本结构。

```rust
pub struct Cache<K: CacheKey, V: Cacheable<K>> {
    index: BMutex<CacheIndex<K, V>>,
    self_ref: Weak<Cache<K, V>>,
}
struct CacheIndex<K: CacheKey, V: Cacheable<K>> {
    used: hashbrown::HashMap<K, Weak<CacheItem<K, V>>>,
    unused: lru::LruCache<K, Arc<CacheItem<K, V>>>,
}

impl<K: CacheKey, V: Cacheable<K>> Cache<K, V> {
    pub fn new() -> Arc<Self> {}	// 创建一个缓存
    pub fn clear(&self) {}			// 清空缓存里的索引
    /// 把value加入缓存
    pub fn make_item_cached(&self, value: V) -> CacheArc<CacheItem<K, V>> {}
    /// 把value移出缓存
    pub fn make_item_no_cache(&self, value: V) -> CacheArc<CacheItem<K, V>> {}
    /// 从缓存里通过key得到CacheItem
    pub fn get(&self, key: K) -> Option<CacheArc<CacheItem<K, V>>> {}
    /// 对item进行重新哈希
    pub fn rehash<F>(&self, item: CacheArc<CacheItem<K, V>>, update: F) {}
    pub fn log(&self) {}	// 输出调试信息
    /// remove: 从缓存里删除key对应的条目
    pub fn remove(&self, key: &K) {}
    /// 从缓存里删除CacheItem
    fn mark_item_unused(&self, item: CacheArc<CacheItem<K, V>>) {}
}
```

##### INodeCache

代表索引结点缓存

```rust
pub static INODE_CACHE: Once<Arc<INodeCache>> = Once::new();
// fs::init().unwrap()最终会调用到INODE_CACHE.call_once(|| INodeCache::new())，从而创建INodeCache
pub type INodeCache = Cache<INodeCacheKey, CachedINode>;
pub struct CachedINode(Arc<dyn INodeInterface>);
// INodeInterface定义在fs::inode模块
```

##### DirCache

代表目录缓存

```rust
pub static DIR_CACHE: Once<Arc<DirCache>> = Once::new();
// fs::init().unwrap()最终会调用到DIR_CACHE.call_once(|| DirCache::new())，从而创建DirCache
pub type DirCache = Cache<DirCacheKey, DirEntry>;
// DirEntry定义在fs::inode模块
```

##### icache

对已经初始化的INodeCache进行引用。

##### dcache

对已经初始化的DirCache进行引用。

#### 索引结点层

##### INodeInterface

代表了对索引结点的操作。定义在`fs::inode`。

##### INode

代表了索引结点。实现了`INodeInterface`。

```rust
pub fn new(ext2,id,proxy);	// 在ext2上创建索引结点，其中id是索引结点号、proxy是对索引结点的操作
pub fn read_mut<offset: usize);	// 
pub fn read(offset, buffer);	// 把索引结点offset处的内容读到内存buffer
pub fn write(offset, buffer);	// 把内存buffer的内容写入索引结点的offset处
pub fn append_block();	// 给索引结点追加块
pub fn get_block(block);	// 
pub fn make_disk_dirent(inode, file_type, name);	// 
pub fn make_inode(name,typ,proxy);	// 
pub fn make_dirent(parent,name,entry);	// 
```

##### Ext2

代表了Ext2文件系统的磁盘结构。

```rust
pub fn new(block);	// 
```



#### 块缓存层

##### BLOCK_DEVS

代表了所有的块设备。它里面的结构是B-tree。

- 对于具体的块设备，这是通过`drivers::pci::init()`里执行`driver.handle.start()`，进而在具体的`start()`实现里执行`install_block_device()`把该设备插入`BLOCK_DEVS`里的。
- 对于块设备上的分区，则是通过`fs::block::launch()`里执行`install_block_device()`把该分区插入到`BLOCK_DEVS`里的。

##### BlockDeviceInterface

代表了块设备的接口。

```rust
fn block_size();	// 返回当前块设备的块大小
fn read_dma(sector,start,size);	// 
fn write_dma(sector,start,size);	//
fn read_block(sector,dest);	//
fn write_block(sector,buf);	// 
```



##### BlockDevice

代表了一个抽象的块设备。具体的块设备实现在驱动层。

```rust
pub fn new(name,imp);	// 创建一个BlockDevice
pub fn name();	// 获取块设备的名字
```

##### PartitionBlockDevice

代表了块设备上的一个分区。

#### 驱动层

##### nvme驱动

- Controller : 代表了NVMe控制器。

```rust
fn new(header);
fn read_dma();
fn write_dma();
fn read_block();
fn block_size();
fn write_block();
```

- Handler : 代表了NVMe控制器的处理程序(handler)。

```rust
fn new();	// 创建一个Handler，其元素为一个空的动态数组
fn handles(_vendor_id, device_id);
	// 通过device_id判断是不是NVMe设备，是则返回true
fn start(header, _offset_table);
	// 1. 从header创建一个新的controller
	// 2. 注册块设备
	// 3. 把设备安装进/dev/文件系统，并插入到BLOCK_DEVS。
	// 4. 把controller加入到controllers
```



#### 其它模块

##### eventfd

计数相关的fd，用于传递事件。`write`加计数，`read`读计数并清零。

##### epoll

事件的池子。可以实现高效的IO多路复用。

##### pipe

管道。

##### procfs

`/proc`

##### ramfs

##### devfs

`/dev`

