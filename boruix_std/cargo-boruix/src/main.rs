//! cargo-boruix —— 用 sysroot 构建 BORUIX 程序的 cargo 子命令（3P2-2）。
//!
//! 用法：
//! ```text
//! set BORUIX_SYSROOT=<install --prefix 的产物>   (或每次传 --sysroot <dir>)
//! cargo boruix build [--release]
//! ```
//!
//! **它包办了什么**：用户工程里只需要 `Cargo.toml` + `src/main.rs`——不需要
//! `#![no_std]` / `#![no_main]`（经 `-Zcrate-attr` 注入）、不需要 `.cargo/config.toml`
//! （目标定义、链接脚本、TLS 模型、`build-std` 全部由本命令经 `--config` 传入）、
//! 不需要 `build.rs`、不需要自带 `linker.ld`。
//!
//! **为什么用 `--config` 而不是让用户抄一份 .cargo/config.toml**：配置里必须带
//! sysroot 的**绝对路径**；写进仓库就是机器相关的（S01）。由命令在调用点注入，
//! 仓库里的源文件保持机器无关。
//!
//! **为什么注入走目标域**：`-Zcrate-attr` 若经全局 `RUSTFLAGS`，会连带作用于宿主
//! （build script / proc-macro），且与依赖自身已声明的 `#![no_std]` 重复。cargo 的
//! `target.<name>.rustflags` 只作用于目标侧。
use std::env;
use std::path::PathBuf;
use std::process::{exit, Command};

fn die(msg: &str) -> ! {
    eprintln!("cargo-boruix: {}", msg);
    exit(1);
}

fn main() {
    let mut argv: Vec<String> = env::args().collect();
    // cargo 以 `cargo-boruix boruix <子命令> ...` 调用；直接跑时允许省略 "boruix"。
    let rest: Vec<String> = if argv.len() > 1 && argv[1] == "boruix" {
        argv.split_off(2)
    } else {
        argv.split_off(1)
    };

    // ---- 取 sysroot：--sysroot <dir> 优先，其次 BORUIX_SYSROOT ----
    let mut sysroot: Option<String> = env::var("BORUIX_SYSROOT").ok();
    let mut forwarded: Vec<String> = Vec::new();
    let mut it = rest.into_iter();
    while let Some(a) = it.next() {
        if a == "--sysroot" {
            sysroot = it.next();
        } else {
            forwarded.push(a);
        }
    }
    let Some(sysroot) = sysroot else {
        die("缺少 sysroot：传 --sysroot <dir> 或设 BORUIX_SYSROOT（由 `python tools/main.py install --prefix <dir>` 产出）");
    };
    let sysroot = PathBuf::from(sysroot);
    let json = sysroot.join("boruix.json");
    let ld = sysroot.join("lib").join("linker.ld");
    if !json.is_file() || !ld.is_file() {
        die(&format!("sysroot 不完整：缺 {} 或 {}", json.display(), ld.display()));
    }
    // 目标名 = json 文件名（不含扩展名）——cargo 对 `--target <path.json>` 即用该名字。
    let name = json
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("boruix")
        .to_string();

    // ---- 目标域 rustflags：注入两个 crate 属性 + TLS 模型 + 链接脚本 ----
    let flags = [
        "-Zcrate-attr=no_std".to_string(),
        "-Zcrate-attr=no_main".to_string(),
        "-Ztls-model=local-exec".to_string(),
        format!("-Clink-arg=-T{}", ld.display()),
    ];
    let toml_list = flags
        .iter()
        .map(|f| format!("{:?}", f))
        .collect::<Vec<_>>()
        .join(", ");
    let cfg = format!("target.{}.rustflags=[{}]", name, toml_list);

    let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
    let mut cmd = Command::new(cargo);
    // 自定义目标需要 build-std（自定义目标没有预编译 core/alloc）与 json-target-spec。
    // 参数层次：`-Z` 与 `--config` 是**全局**选项（在子命令前）；`--target` 是**子命令级**
    // 选项（必须在子命令后）——实测放在子命令前会报 `unexpected argument '--target' found`。
    cmd.args(["-Z", "json-target-spec", "-Z", "build-std=core,alloc"]);
    cmd.arg("--config").arg(cfg);
    let Some((sub, tail)) = forwarded.split_first() else {
        die("缺少子命令（例如 `cargo boruix build`）");
    };
    cmd.arg(sub);
    cmd.arg("--target").arg(&json);
    cmd.args(tail);
    let status = cmd.status().unwrap_or_else(|e| die(&format!("无法启动 cargo: {}", e)));
    exit(status.code().unwrap_or(1));
}