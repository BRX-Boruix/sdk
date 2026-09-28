# sdk

**简体中文** | [English](#english)

> **本项目处于规划阶段，尚不可用。** 仓库目前只有这份说明文档，没有任何可用的工具或产物。

BORUIX 的**第三方开发工具集**——目标是让开发者在一台普通电脑上、**不克隆系统源码**，就能交叉
编译出能在 BORUIX 上运行的程序。

---

## 它要解决什么问题

到目前为止，为这个系统写一个程序需要先拿到完整的系统源码：编译器配置、头文件、链接脚本、目标
定义全都散在源码树里，构建流程也假定你在系统源码目录下工作。

这对**系统自己的开发者**没问题，对**只想写个程序的人**就很不友好——他得先下载整个操作系统，
搞清楚内部目录结构，才能编译一个"你好，世界"。

这个项目的目标就是把这条路径独立出来：**给第三方一套自包含的工具**，装上就能编译，不需要关心
系统内部长什么样。

这也会让"支持第三方程序"从一件需要内部知识的事，变成一件照着文档做就能完成的事。

## 和系统工具的区别

项目里有两个容易混淆的目录，区别在于**服务对象**：

| 目录 | 服务对象 | 装了什么 |
| --- | --- | --- |
| [`tools/`](https://github.com/BRX-Boruix/tools) | 系统自己的开发者 | 编译内核、打包镜像、虚拟机验收 |
| **`sdk/`（本仓库）** | **第三方开发者** | 交叉编译一个程序所需的一切 |

一句话的划分规则：**这里只放"给第三方的东西"**。

这两者曾经是同一个目录，2026-10 才分开——旧的 `sdk/` 实际做的全是系统自身的事，名字却承诺了
第三方工具。分开之后，两边各归其位。

## 计划提供什么

工具集计划包含下面几部分（**均未开始**）：

| 内容 | 作用 |
| --- | --- |
| **系统根目录** | 一组与 POSIX 形状一致的头文件，让已有的 C 代码能直接编译 |
| **库与启动文件** | 静态库、程序入口目标文件、链接脚本 |
| **编译器包装** | 一个封装好的编译器入口，自动带上正确的参数 |
| **目标定义** | 描述 BORUIX 程序的目标平台（关闭位置无关、显式声明线程局部存储） |
| **构建配置模板** | 让用户的构建工具自动采用上述设置 |

这些东西合起来的效果是：**用户只需要一条命令**，不必自己拼装参数。

## 为什么现在还不能用

**内容尚未落地，所以不假装它已经存在。**

这个仓库目前只有这份文档，没有 `include/`、没有库文件、没有编译器包装——上面表格里的每一项都是
**计划**，不是现状。

这也是刻意的纪律：在内容真正可用之前，这个目录**不被任何脚本引用、不承载任何构建产物**。做一
个空壳仓库、让外部开发者下载之后发现什么都没有，比暂时不提供更糟。

## 当前可用的替代方案

如果你想现在就为 BORUIX 写程序：

- **写 C 程序**：[`libc`](https://github.com/BRX-Boruix/libc) 提供了带 C 接口的标准库实现
- **写 Rust 程序**：[`libsys`](https://github.com/BRX-Boruix/libsys) 是用户态与内核之间的接口层，
  编译目标为 `x86_64-unknown-none`
- **看一个完整例子**：若干独立的用户态程序仓库，如 [`threaddemo`](https://github.com/BRX-Boruix/threaddemo)、
  [`synce2e`](https://github.com/BRX-Boruix/synce2e)

这些目前需要配合系统源码树使用——把它们放在源码树的同级目录下，按各仓库的构建说明编译。
**这正是本工具集将来要消除的不便。**

## 许可

MIT License，版权归 Yang Borui 所有。详见 [LICENSE](LICENSE)。

---

# English

[简体中文](#sdk) | **English**

> **This project is at the planning stage and is not usable yet.** The repository holds only this
> document — no working tools or artifacts.

BORUIX's **third-party development toolchain** — the goal is to let a developer cross-compile
programs that run on BORUIX from an ordinary computer, **without cloning the system source**.

---

## The problem it addresses

As things stand, writing a program for this system means obtaining the whole system source first:
compiler configuration, headers, linker scripts, and target definitions are scattered through the
source tree, and the build flow assumes you are working inside it.

That is fine for **the system's own developers** and unfriendly to **someone who just wants to write a
program** — they must download an entire operating system and work out its internal layout before
compiling "hello, world".

This project aims to pull that path out on its own: **a self-contained toolchain for third parties**,
install it and compile, with no need to understand the system's internals.

It also turns "supporting third-party programs" from something needing inside knowledge into something
you can do by following the documentation.

## How it differs from the system tools

The project has two easily confused directories, distinguished by **who they serve**:

| Directory | Serves | Contains |
| --- | --- | --- |
| [`tools/`](https://github.com/BRX-Boruix/tools) | The system's own developers | Kernel build, image packaging, VM acceptance |
| **`sdk/` (this repository)** | **Third-party developers** | Everything needed to cross-compile one program |

The rule in one line: **only things meant for third parties belong here**.

The two were once a single directory, separated in 2026-10 — the old `sdk/` did nothing but the
system's own work while its name promised third-party tools. Split apart, each now fits its name.

## What it plans to provide

The toolchain is planned to include the following (**none of it started**):

| Item | Purpose |
| --- | --- |
| **A sysroot** | A set of POSIX-shaped headers so existing C code compiles as-is |
| **Libraries and startup files** | The static library, the program entry object, and a linker script |
| **A compiler wrapper** | A packaged compiler entry point that supplies the right flags automatically |
| **Target definitions** | A description of the BORUIX program target (position independence off, thread-local storage declared explicitly) |
| **A build config template** | So the user's build tool adopts those settings automatically |

Together these mean **the user needs one command** instead of assembling flags by hand.

## Why it is not usable yet

**Nothing is in place, so it is not pretended to be.**

The repository holds only this document — no `include/`, no library files, no compiler wrapper. Every
row in the table above is a **plan**, not a present state.

That is deliberate discipline too: until the contents are genuinely usable, this directory is
**referenced by no script and carries no build artifacts**. Publishing an empty shell that external
developers download only to find nothing inside would be worse than not offering it yet.

## What to use in the meantime

If you want to write a program for BORUIX today:

- **Writing C** — [`libc`](https://github.com/BRX-Boruix/libc) provides a standard library implementation with C interfaces
- **Writing Rust** — [`libsys`](https://github.com/BRX-Boruix/libsys) is the interface layer between user space and the kernel,
  with `x86_64-unknown-none` as the build target
- **A complete example** — several standalone user-space program repositories such as
  [`threaddemo`](https://github.com/BRX-Boruix/threaddemo) and [`synce2e`](https://github.com/BRX-Boruix/synce2e)

These currently need the system source tree alongside them — place them as siblings of the source tree
and follow each repository's build instructions. **That inconvenience is exactly what this toolchain
will remove.**

## License

MIT License, copyright Yang Borui. See [LICENSE](LICENSE).
