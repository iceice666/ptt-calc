# ptt-calc: Arcaea B30 Calculator

ptt-calc is a Best 30 (B30) calculator for the rhythm game Arcaea. It helps you calculate your B30 rating based on your scores.

<!-- [中文文档看这里！](./README_zh.md) -->

[TOC]

## Before you start

Check you had `git` `rust` `just` installed

For Termux users, execute the command:
```bash
pkg install git rust
cargo install just
```

## Installation
### 1. Check Data Access Permissions

Before proceeding, ensure you have permission to access Arcaea's data, which is stored at `/data/data/moe.low.arc/files/st3`. Here's how to check permissions:

For Termux users, execute the  command:

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

Depends on your requirement, select a way to display:

- `cli`:
    It just print results in terminal/console.
    
    Run the command to compile 
    ```bash
    $HOME/.cargo/bin/just cli
    ```
- `web`:
    It will start a server at `localhost:8000/b30`, open this url in web browser to check your scores.

    Run the command to compile 
    ```bash
    $HOME/.cargo/bin/just web
    ```

And you will see the executable file in `output` folder.


### 3. Run the Calculator

In `output` folder, run `sudo ./cli` or `sudo ./web`.
For `cli`, has following options

- `list <num>`: It will print top `<num>` scores
- `b30`: It will print your b30.

For `web`, dont forget to check your scores at `localhost:8000/b30` and `localhost:8000/list`
