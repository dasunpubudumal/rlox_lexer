# rlox_lexer

![workflow](https://github.com/dasunpubudumal/rlox_lexer/actions/workflows/rust.yml/badge.svg)
[![Project Status: WIP – Initial development is in progress, but there has not yet been a stable, usable release suitable for the public.](https://www.repostatus.org/badges/latest/active.svg)](https://www.repostatus.org/#wip)

Lexer writen in Rust for [Lox Language](https://craftinginterpreters.com/the-lox-language.html).

### Notes

1. Debug logs in tests: 

```shell
RUST_LOG=debug cargo test -- --nocapture
```

### Development

At WIP level:

- Each feature needs to be tested in `tests/lexer_test.rs`.

## References

1. [Borrowing Rules for `self`](https://users.rust-lang.org/t/borrowing-rules-about-self/69451/2)

