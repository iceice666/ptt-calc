# ptt-calc: Arcaea B30 Calculator

ptt-calc is a Best 30 (B30) calculator for the rhythm game Arcaea. It helps you calculate your B30 rating based on your scores.

[中文文档看这里！](./README_zh.md)

## Table of Contents

- [Installation](#installation)
  - [1. Check Data Access Permissions](#1-check-data-access-permissions)
  - [2. Compile the Repository](#2-compile-the-repository)
    - [Method 1: Using Termux](#method-1-using-termux)
    - [Method 2: Without Termux](#method-2-without-termux)
  - [3. Run the Calculator](#3-run-the-calculator)
- [TODO](#todo)

## Installation
### 1. Check Data Access Permissions

Before proceeding, ensure you have permission to access Arcaea's data, which is stored at `/data/data/moe.low.arc/files/st3`. Here's how to check permissions:

For Termux users, execute the following command:

```bash

pkg install sudo file
sudo file /data/data/moe.low.arc/files/st3
```
You should see an output like:

```
/data/data/moe.low.arc/files/st3: SQLite 3.x database, ...
```

This indicates that you have the necessary permissions. If not, you might need to root your device or consider manual score recording (not supported yet).

### 2. Compile the Repository

Choose one of the following methods to compile the repository:

#### Method 1: Using Termux

Install [Termux](https://wiki.termux.com/wiki/Installing_from_F-Droid).
Run the following commands:

``` bash

pkg upgrade
pkg install sudo git rust pkg-config binutils 
cd $HOME
git clone https://github.com/iceice666/ptt-calc.git
cd ptt-calc
export RUSTFLAGS+=" -C link-arg=$(clang -print-libgcc-file-name)"
cargo build --release
```

After this, you'll find the ptt-calc executable in the target/release folder.

#### Method 2: Without Termux

Ensure you have [git](https://git-scm.com/book/en/v2/Getting-Started-Installing-Git) and [rust](https://www.rust-lang.org/tools/install) installed. Then, run these commands:

```bash

git clone https://github.com/iceice666/ptt-calc.git
cd ptt-calc
cargo build --release
```

You'll find the ptt-calc executable in the target/release folder.

Remember to copy `/data/data/moe.low.arc/files/st3` to `target/release/` and rename it to `score.db`.

#### Which one should I use?

Method 1 is ideal for obtaining your newest score easily since both Arcaea's score database and this program are on the same device.

Method 2 is recommended if you want a more stable compilation environment for various platforms (Windows, MacOS, Linux ... etc.) .

### 3. Run the Calculator

After compiling, run the following command at `target/release` folder:

``` bash
sudo ./ptt-calc
```
Enjoy calculating your B30 rating! 😊

## TODO

- [ ] A `WSAonLocal` installation guide
- [ ] Manual score recording
- [ ] Image generation
- [ ] Score history
