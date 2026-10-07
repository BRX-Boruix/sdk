# sdk

BORUIX 的第三方开发工具集：不克隆系统源码即可交叉编译出能在 BORUIX 上运行的程序。

[English](README.en.md)

**状态：可用（2026-10 起）。** 仓内已有：`boruix_std/`（`boruix_std` 门面 crate +
`boruix_std_macros` + `cargo-boruix` 包装器）、`x86_64-unknown-boruix.json`（目标定义）、
`examples/`（`wclite`、`three-lines`、`cbreadth`）。

**诚实边界**：

- 目标定义的上游化（`3P2-4`）只做到**本地 Tier-3 形态**；进入上游 rustc 是外部流程，**未完成**。
- 动态库（`cdylib`）不被 cargo 接受，故 `.so` 生态（`3P5-2`）不在此列。
- 此前本 README 写「处于规划阶段、尚不可用、仓库只有这份说明文档」——**那是过期的**：工具与示例
  均已落地并验收，见 `docs/TODO/3p.md` 的 `3P1-*` / `3P2-*` 各项。

## 它要解决什么问题

目前为这个系统写程序需要先拿到完整系统源码：编译器配置、头文件、链接脚本、目标定义都散在源码
树里，构建流程也假定你在源码目录下工作。这对只想写个程序的人不友好。

本项目的目标是把这条路径独立出来：一套自包含的工具，装上就能编译，不需要了解系统内部结构。

## 与系统工具的区别

与 [`tools`](https://github.com/BRX-Boruix/tools) 的区别在服务对象：tools 服务系统自身的开发者
（编译内核、打包镜像、验收），本仓库服务第三方开发者（交叉编译一个程序所需的一切）。

两者曾是同一个目录，2026 年 10 月分开——旧名字承诺了第三方工具，实际做的全是系统自身的事。
划分规则：只放「给第三方的东西」。

## 计划提供什么

以下各项均为计划，均未开始：

- **系统根目录**——与 POSIX 形状一致的头文件，让已有 C 代码直接编译
- **库与启动文件**——静态库、程序入口目标文件、链接脚本
- **编译器包装**——封装好的编译器入口，自动带上正确的参数
- **目标定义**——描述 BORUIX 程序的目标平台（关闭位置无关、显式声明线程局部存储）
- **构建配置模板**——让用户的构建工具自动采用上述设置

## 当前可用的替代方案

- **写 C 程序**：[`libc`](https://github.com/BRX-Boruix/libc) 提供带 C 接口的标准库实现
- **写 Rust 程序**：[`libsys`](https://github.com/BRX-Boruix/libsys) 是用户态与内核的接口层，
  编译目标 `x86_64-unknown-none`
- **完整例子**：[`threaddemo`](https://github.com/BRX-Boruix/threaddemo)、
  [`synce2e`](https://github.com/BRX-Boruix/synce2e) 等独立程序仓库

这些目前需要配合系统源码树使用。这正是本工具集将来要消除的不便。

## 许可

MIT License，版权归 Yang Borui 所有。详见 [LICENSE](LICENSE)。
