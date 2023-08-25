# ptt-calc：Arcaea B30 计算器

ptt-calc 是用于音乐游戏 Arcaea 的 Best 30 (B30) 计算器。它可以基于您的分数帮助您计算 B30 评分。

## 目录

- [安装](#安装)
  - [1. 检查数据访问权限](#1-检查数据访问权限)
  - [2. 编译仓库](#2-编译仓库)
    - [方法 1：使用 Termux](#方法-1-使用-termux)
    - [方法 2：无需 Termux](#方法-2-无需-termux)
  - [3. 运行计算器](#3-运行计算器)
- [待办事项](#待办事项)

## 安装

### 1. 检查数据访问权限

在继续之前，请确保您具有访问 Arcaea 数据的权限，这些数据存储在 `/data/data/moe.low.arc/files/st3`。以下是如何检查权限：

对于 Termux 用户，执行以下命令：
```bash 
pkg install sudo file
sudo file /data/data/moe.low.arc/files/st3
```

您应该会看到类似于以下内容的输出：

```
/data/data/moe.low.arc/files/st3：SQLite 3.x database ...

```

这表示您具有必要的权限。如果没有权限，您可能需要对设备进行 root 处理，或者考虑手动记录分数（目前尚不支持）。

### 2. 编译仓库

选择以下方法之一来编译仓库：

#### 方法 1：使用 Termux

1. 首先安装 Termux。
2. 执行以下命令：

```bash

pkg upgrade
pkg install sudo git rust pkg-config binutils 
cd $HOME
git clone https://github.com/iceice666/ptt-calc.git
cd ptt-calc
export RUSTFLAGS+=" -C link-arg=$(clang -print-libgcc-file-name)"
cargo build --release
```
完成后，您将在 `target/release` 文件夹中找到 `ptt-calc` 可执行文件。

#### 方法 2：无需 Termux

确保您已安装了 [git](https://git-scm.com/book/en/v2/Getting-Started-Installing-Git) 和 [rust](https://www.rust-lang.org/tools/install)。然后，运行以下命令：

``` bash
git clone https://github.com/iceice666/ptt-calc.git
cd ptt-calc
cargo build --release
```

您将在 `target/release` 文件夹中找到 `ptt-calc` 可执行文件。

请记得将 ``/data/data/moe.low.arc/files/st3` 复制到 `target/release/`` 并将其重命名为 `score.db`。


#### 我该使用哪一个方法?

方法 1 适用于轻松获取最新分数，因为 Arcaea 的分数数据库和此程序都位于同一设备上。
方法 2 推荐用于在各种平台（Windows、MacOS、Linux）上获得更稳定的编译环境。

### 3. 运行计算器

编译后，在`target/release`资料夹中运行以下命令：

```bash
sudo ./ptt-calc
```

开始推分吧！ 😊

待办事项

- [ ] 手动记录分数
- [ ] 图像生成
- [ ] 分数历史记录
