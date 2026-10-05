//! boruix_std —— BORUIX 用户态程序的「熟名」表层（3P2-1）。
//!
//! 目标：让用户不必知道 `#![no_std]` / `#![no_main]` / `user_main` 这些底层约定。
//!
//! ```ignore
//! use boruix_std::println;
//!
//! #[boruix_std::main]
//! fn main() {
//!     println!("hello, {}", 42);
//! }
//! ```
//!
//! 组成：
//! 1. **入口属性** `#[boruix_std::main]`（re-export 自 `boruix_std_macros`）。为什么不是
//!    「裸 `fn main`」：rustc 明确拒绝——实测报错 `using `fn main` requires the standard
//!    library`，并要求「自己声明平台入口（通常加 `#[no_mangle]`）」。无 std 时这是硬约束，
//!    属性宏是行业同款做法（`#[tokio::main]` / `#[cortex_m_rt::entry]`）。
//! 2. `print!` / `println!`：经 libsys 的 `write(1, ...)` 直写，**无堆分配**。
//! 3. libsys 熟名薄封装（fs / process / thread / sync / 时间 / 参数环境）。
//!
//! **panic handler 不在本 crate**：由 libsys 提供（它已按 `target_os = "none"|"boruix"`
//! 门禁定义了 `#[panic_handler]`）。同一程序只能有一个 panic handler，本 crate 再定义一份
//! 会与之冲突。
// 包装器（cargo-boruix）经**目标域** rustflags 向目标侧所有 crate 注入 `#![no_std]`，
// 本 crate 自己也声明了一份——于是会有一条 `unused_attributes` 重复声明告警。
// 保留本 crate 自己的声明（它必须能独立作为 no_std 库编译），显式放行该告警。
#![allow(unused_attributes)]
#![no_std]

pub use boruix_std_macros::main;
pub use libsys;
pub use libsys::Error;

/// 进程参数与环境（熟名）。
pub use libsys::{args, cmdline, env, var};

/// 时间：单调时钟与休眠。
pub use libsys::{now, sleep};

// ---------- 输出 ----------

pub mod io {
    use core::fmt;

    /// 标准输出句柄：`core::fmt::Write` 的薄实现，直接写 fd 1。
    pub struct Stdout;

    impl fmt::Write for Stdout {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            // 写失败（如 fd 1 已关）在此语境下无补救手段：如实忽略返回值，绝不 panic——
            // 那会把「输出失败」升级成「程序崩溃」。
            let _ = libsys::write(libsys::STDOUT, s.as_bytes());
            Ok(())
        }
    }
}

/// 写标准输出，不换行。与 `std::print!` 同形。
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {{
        use core::fmt::Write as _;
        let _ = core::write!($crate::io::Stdout, $($arg)*);
    }};
}

/// 写标准输出并换行。与 `std::println!` 同形。
#[macro_export]
macro_rules! println {
    () => { $crate::print!("\n") };
    ($($arg:tt)*) => {{
        use core::fmt::Write as _;
        let _ = core::write!($crate::io::Stdout, $($arg)*);
        let _ = core::write!($crate::io::Stdout, "\n");
    }};
}

// ---------- 熟名薄封装 ----------

pub mod fs {
    pub use libsys::{
        DirEntry, OpenFlags, Permissions, StatInfo, chdir, chmod, chown, close, dup2, fstat,
        ftruncate, getcwd, mkdir, open, pread, pwrite, read, read_dir, read_to_end, readlink,
        rename, stat, symlink, unlink, write,
    };
}

pub mod process {
    pub use libsys::{
        PsEntry, WAIT_ANY, WaitResult, exec, exec_path, exit, getpid, gettid, kill, ps,
        waitpid_any, yield_now,
    };
}

pub mod thread {
    pub use libsys::{thread_exit, thread_join, thread_spawn};
}

pub mod sync {
    pub use libsys::{sync_create, sync_delete, sync_wait, sync_wake};
}