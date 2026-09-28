# sdk

BORUIX's third-party development toolchain (planned): cross-compile programs that run on BORUIX without cloning the system source.

[简体中文](README.md)

**This project is at the planning stage and is not usable yet.** The repository holds only this
document — no working tools or artifacts.

## The problem it addresses

Writing a program for this system today means obtaining the whole system source first: compiler
configuration, headers, linker scripts and target definitions are scattered through the source tree,
and the build flow assumes you work inside it. That is unfriendly to someone who just wants to write
a program.

This project aims to pull that path out on its own: a self-contained toolchain — install it, compile,
with no need to understand the system's internals.

## How it differs from the system tools

The difference from [`tools`](https://github.com/BRX-Boruix/tools) is who each serves: tools serves
the system's own developers (kernel build, image packaging, acceptance), this repository serves
third-party developers (everything needed to cross-compile one program).

They were once a single directory, separated in 2026-10 — the old name promised third-party tools
while the work was entirely the system's own. The rule: only things meant for third parties belong here.

## What it plans to provide

Every item below is a plan; none is started:

- **A sysroot** — POSIX-shaped headers so existing C code compiles as-is
- **Libraries and startup files** — the static library, the program entry object, a linker script
- **A compiler wrapper** — a packaged compiler entry that supplies the right flags automatically
- **Target definitions** — the BORUIX program target (position independence off, thread-local storage declared explicitly)
- **A build config template** — so the user's build tool adopts those settings automatically

## What to use in the meantime

- **Writing C** — [`libc`](https://github.com/BRX-Boruix/libc) provides a standard library with C interfaces
- **Writing Rust** — [`libsys`](https://github.com/BRX-Boruix/libsys) is the user-space interface to the kernel, target `x86_64-unknown-none`
- **A complete example** — standalone program repositories such as [`threaddemo`](https://github.com/BRX-Boruix/threaddemo) and [`synce2e`](https://github.com/BRX-Boruix/synce2e)

These currently need the system source tree alongside them. That inconvenience is exactly what this
toolchain will remove.

## License

MIT License, copyright Yang Borui. See [LICENSE](LICENSE).
