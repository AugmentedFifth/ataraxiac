# ataraxiac: compiler for the ataraxia programming language

![](https://img.shields.io/badge/version-v0.0.0-37a988.svg)
![](https://img.shields.io/badge/status-pre--alpha-444444.svg)
[![](https://img.shields.io/badge/license-UPL_v1+-3f779d.svg)](https://opensource.org/licenses/UPL)
![](https://img.shields.io/badge/implementation-Rust-b7410e.svg)
![](https://img.shields.io/badge/rustc-nightly-d185bd.svg)

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

More about the language, including a specification, [can be found here](#TODO).

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