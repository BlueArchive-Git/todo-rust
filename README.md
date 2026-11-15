# Todo - Rust
![Rust learn](https://skillicons.dev/icons?i=rust) **Rust Learn**
---
本仓库使用 **GPLv3** 许可证开源。
---
## 这是什么？
这是一个使用 **Rust 语言** 编写的事件待办最小示例程序。

## 怎么装？
确保你的环境安装了 rust：

```bash
rustc --version
cargo --version
```

克隆本仓库：

```bash
git clone https://github.com/BlueArchive-Git/todo-rust.git
cd todo-rust
```

编译 & 安装：

```bash
cargo run
cargo build
cargo install
```

随后按照提示将 $HOME/.cargo/bin 加入 PATH 环境变量即可。

## 怎么用？
它提供了四个子命令

| 命令 | 讲解                            |
|------|---------------------------------|
| list | 列出待办事务（包括状态）        |
| add  | 添加待办事务到文件 ~/.todo.json |
| done | 标记待办事务为已完成            |
| rm   | 删除待办事务                    |

示例
```simulate-shell
$ todo list
Tasks:

$ todo add coding homework
todo: added tasks: coding, homework

$ todo list
Tasks:
[ ] coding
[ ] homework

$ todo done coding
todo: task completed: coding

$ todo list
Tasks:
[x] coding
[ ] homework

$ todo rm homework
todo: remove task: homework? (y/N) > y
todo: task removed: homework

$ todo list
Tasks:
[x] coding

$
```
