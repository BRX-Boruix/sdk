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

/// 进程环境（熟名）。
pub use libsys::{env, var};

// ---------- 入口参数：由属性宏接住，用户零参数取用 ----------
//
// 为什么需要这一层：`#[boruix_std::main]` 生成的 `user_main(argc, argv)` 若把两个参数丢掉，
// 用户就再也拿不到命令行（3P2-3 端点验收正是这样暴露的）。故入口先把它们存进本 crate 的
// 静态槽，用户经下面两个零参数访问器取用。

use core::sync::atomic::{AtomicIsize, AtomicUsize, Ordering};

static ENTRY_ARGC: AtomicIsize = AtomicIsize::new(0);
static ENTRY_ARGV: AtomicUsize = AtomicUsize::new(0);

/// 由入口属性宏在调用用户 `main` **之前**写入。不是给用户直接调的。
pub fn __set_entry_args(argc: isize, argv: *const *const u8) {
    ENTRY_ARGV.store(argv as usize, Ordering::Release);
    ENTRY_ARGC.store(argc, Ordering::Release);
}

/// 整条命令行（**未拆词**；本系统 ABI 把整条命令放 argv[0]，拆词是用户程序职责）。
///
/// 无命令行时返回 `None`。
pub fn cmdline() -> Option<&'static [u8]> {
    let argc = ENTRY_ARGC.load(Ordering::Acquire);
    let argv = ENTRY_ARGV.load(Ordering::Acquire) as *const *const u8;
    if argv.is_null() {
        return None;
    }
    // SAFETY: 指针由入口属性宏写入，指向内核按 ABI 放置的入口参数块；该内存位于进程初始栈，
    // 生命周期覆盖整个进程（故此处以 'static 表达）。
    unsafe { libsys::cmdline(argc, argv) }
}

/// 拆词后的参数迭代器（首个词是程序名/首词）。无命令行时为空迭代器。
pub fn args() -> libsys::Words<'static> {
    match cmdline() {
        Some(line) => libsys::split_words(line),
        None => libsys::split_words(b""),
    }
}

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