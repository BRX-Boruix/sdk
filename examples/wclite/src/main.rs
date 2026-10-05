//! wclite —— 一个小而真实的 Rust 项目：统计输入文本的字节数 / 词数 / 行数。
//!
//! **同一份源码同时构建宿主（std）与 BORUIX**：平台差异全部收在文件末尾的两个 `cfg`
//! 分支里（各自取第一个参数、各自打印），文件本身**逐字节相同**。核心逻辑 `count()` 与
//! 平台完全无关。
//!
//! 构建：
//! ```text
//!   宿主:   cargo build --release
//!   BORUIX: cargo boruix build --release      (sysroot 经 BORUIX_SYSROOT 指定)
//! ```

/// 无参数时统计的默认文本（让程序在任何环境都能给出可复核的输出）。
const DEFAULT_TEXT: &[u8] = b"the quick brown fox\njumps over the lazy dog\n";

// ---------- 与平台无关的核心逻辑（本文件主体）----------

/// 统计 (字节数, 词数, 行数)。词 = 由空白分隔的非空字节序列。
fn count(text: &[u8]) -> (usize, usize, usize) {
    let mut words = 0usize;
    let mut lines = 0usize;
    let mut in_word = false;
    for &b in text {
        if b == b'\n' {
            lines += 1;
        }
        let is_space = b == b' ' || b == b'\t' || b == b'\n' || b == b'\r';
        if is_space {
            in_word = false;
        } else if !in_word {
            in_word = true;
            words += 1;
        }
    }
    (text.len(), words, lines)
}

// ---------- 入口一：宿主（std）----------

#[cfg(not(target_os = "boruix"))]
fn main() {
    let arg = std::env::args().nth(1);
    let text: &[u8] = match &arg {
        Some(s) => s.as_bytes(),
        None => DEFAULT_TEXT,
    };
    let (b, w, l) = count(text);
    println!("bytes={} words={} lines={}", b, w, l);
}

// ---------- 入口二：BORUIX ----------

#[cfg(target_os = "boruix")]
#[boruix_std::main]
fn main() {
    // BORUIX 的 ABI 把整条命令行放在 argv[0] 且**不拆词**（见 docs/abi/syscall-abi.md §4），
    // 拆词是用户程序的职责——libsys 提供单点实现。**经 shell 派生时该串已剥掉程序名**，
    // 故首个词就是第一个参数（这与宿主 POSIX argv 的 [0]=程序名不同——差异属平台层，
    // 收在本分支内）。
    let text: &[u8] = match boruix_std::args().nth(0) {
        Some(a) => a,
        None => DEFAULT_TEXT,
    };
    let (b, w, l) = count(text);
    boruix_std::println!("bytes={} words={} lines={}", b, w, l);
}