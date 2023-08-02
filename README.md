# ArceOS

一个用Rust写的实验性的操作系统。已支持unikernel，当前正在尝试支持宏内核。

以[aero](https://github.com/Andy-Python-Programmer/aero)操作系统为主要参考实现。

## TODO

- [x] 创建userland镜像

- [ ] ext2文件系统的支持

  (因为userland需要软链接，而当前使用的fat32文件系统不支持软链接)

- [ ] 多进程的支持

## 关于宏内核使用示例

```sh
make MODEL=mono disk_img	# 创建ext2镜像：disk.img。里面是用户态所需的文件。
	# 注：前提要求是这些文件先要放在arceos顶层的system-root目录。在aero操作系统里执行`./aero.py --sysroot`可以得到system-root目录。
```

