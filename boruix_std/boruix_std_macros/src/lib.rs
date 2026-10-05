//! `#[main]` 入口属性宏（3P2-1）。
//!
//! **为什么需要属性，而不是"裸 `fn main`"**：rustc 自己的报错是
//! `using `fn main` requires the standard library` —— 没有 std（本系统尚未移植）时，
//! rustc 拒绝为 `fn main` 生成入口，要求"自己声明平台相关的入口符号（通常加 `#[no_mangle]`）"。
//! 因此本系统采用与 `#[tokio::main]` / `#[cortex_m_rt::entry]` 同款的**属性宏**方案：
//! 用户照常写 `fn main`，属性宏在原函数之后补上平台入口 `user_main`。
//!
//! 用法：
//! ```ignore
//! use boruix_std::println;
//!
//! #[boruix_std::main]
//! fn main() {
//!     println!("hello, {}", 42);
//! }
//! ```
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

/// 把用户的 `fn main` 标记为程序入口。
///
/// 生成：原函数 + `#[unsafe(no_mangle)] pub extern "C" fn user_main(_argc, _argv) -> i32`
/// （libsys 的 `_start` 调用 `user_main`，其返回值即 exit code）。
#[proc_macro_attribute]
pub fn main(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let func = parse_macro_input!(item as ItemFn);
    // 入口函数必须叫 `main`：生成的 `user_main` 直接按该名字调用它。
    if func.sig.ident != "main" {
        return syn::Error::new_spanned(
            &func.sig.ident,
            "`#[boruix_std::main]` 只能用于名为 `main` 的函数",
        )
        .to_compile_error()
        .into();
    }
    if !func.sig.inputs.is_empty() {
        return syn::Error::new_spanned(
            &func.sig.inputs,
            "入口 `main` 不接受参数（argc/argv 请用 boruix_std::args() 取）",
        )
        .to_compile_error()
        .into();
    }
    let expanded = quote! {
        #func

        #[unsafe(no_mangle)]
        pub extern "C" fn user_main(argc: isize, argv: *const *const u8) -> i32 {
            // 先把入口参数交给门面保管：否则用户再也拿不到命令行（见 boruix_std::args）。
            ::boruix_std::__set_entry_args(argc, argv);
            main();
            0
        }
    };
    expanded.into()
}