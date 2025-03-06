<p align="center"><a href="README.md">English</a> | 中文</p>

# Clean Maven Failed Product

一个简单实用的Rust命令行工具，用于清理Maven仓库中的失败下载和损坏的包文件。

## 项目简介

在使用Maven构建Java项目时，有时会因为网络问题或其他原因导致依赖包下载失败，这些失败的下载会以`.lastUpdated`结尾的文件形式保留在Maven本地仓库中。这些文件会导致后续的构建过程中反复尝试下载并失败，影响开发效率。

本工具会扫描Maven本地仓库，自动找出并删除所有包含`.lastUpdated`文件的依赖包文件夹，从而解决构建失败的问题。

## 功能特点

- 自动扫描Maven仓库目录
- 查找所有含有`.lastUpdated`的文件
- 删除包含失败下载的包文件夹
- 实时显示扫描进度
- 最终统计清理的包数量

## 使用方法

1. 运行程序
2. 输入Maven仓库路径（例如：`C:/Users/username/.m2/repository`）
3. 程序会自动扫描并清理失败的下载

## 构建方法

确保已安装Rust环境，然后执行：

```bash
cargo build --release
```

编译后的可执行文件将在`target/release`目录中生成。

## 依赖项

- [walkdir](https://crates.io/crates/walkdir) v2.5.0 - 用于递归遍历目录

## 注意事项

- 在使用前建议备份您的Maven仓库
- 该工具会永久删除文件，请谨慎使用

## 许可证

MIT License © 2025 [Clover You](https://github.com/Clover-You)
