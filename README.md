# [ataraxiac](https://gitlab.com/AugmentedFourth/ataraxiac): compiler for the ataraxia programming language

[![version: v0.0.0](https://img.shields.io/badge/version-v0.0.0-37a988.svg "version: v0.0.0")](https://gitlab.com/AugmentedFourth/ataraxiac)
[![status: pre-alpha](https://img.shields.io/badge/status-pre--alpha-444444.svg "status: pre-alpha")](https://gitlab.com/AugmentedFourth/ataraxiac)
[![license: UPL v1+](https://img.shields.io/badge/license-UPL_v1+-3f779d.svg "license: UPL v1+")](https://opensource.org/licenses/UPL)
[![implementation: Rust](https://img.shields.io/badge/implementation-Rust-b7410e.svg "implementation: Rust")](https://www.rust-lang.org)
[![rustc: nightly](https://img.shields.io/badge/rustc-nightly-d185bd.svg "rustc: nightly")](https://github.com/rust-lang/rust)
[![documentation: CC BY-SA 4.0](https://img.shields.io/badge/documentation-CC_BY--SA_4.0-c6be5f.svg "documentation: CC BY-SA 4.0")](https://gitlab.com/AugmentedFourth/ataraxia)

---

## what is ataraxia

**ataraxia** is a
[high-level](https://en.wikipedia.org/wiki/High-level_programming_language),
[statically-strongly typed](https://en.wikipedia.org/wiki/Intuitionistic_type_theory),
[stack-oriented](https://en.wikipedia.org/wiki/Stack-oriented_programming_language)
programming language. It is similar to
[Joy](https://en.wikipedia.org/wiki/Joy_\(programming_language\)),
[Forth](http://www.forth.org/), or
[PostScript](https://en.wikipedia.org/wiki/PostScript), but more similar to
[Rust](https://www.rust-lang.org) in terms of typechecking and handling of
data.

More about the language, including a specification, [can be found here](https://gitlab.com/AugmentedFourth/ataraxia).

## using ataraxiac

```bash
ataraxiac --help
```

## building ataraxiac

```bash
git clone https://gitlab.com/AugmentedFourth/ataraxiac.git
cd ataraxiac
make native  # or `make debug` for fast-compilation debug builds with symbols
./target/release/ataraxiac  # or `./target/debug/ataraxiac`
```
