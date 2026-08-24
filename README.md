# Rust bindings for ARM Neural Graphics

[![Actions Status](https://github.com/Traverse-Research/arm-neural-graphics-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/Traverse-Research/arm-neural-graphics-rs/actions)
[![MSRV](https://img.shields.io/badge/rustc-1.74.0+-ab6000.svg)](https://blog.rust-lang.org/2023/11/16/Rust-1.74.0.html)
[![Lines of code](https://tokei.rs/b1/github/Traverse-Research/arm-neural-graphics-rs)](https://github.com/Traverse-Research/arm-neural-graphics-rs)
![MIT](https://img.shields.io/badge/license-MIT-blue.svg)
[![Contributor Covenant](https://img.shields.io/badge/contributor%20covenant-v1.4%20adopted-ff69b4.svg)](./CODE_OF_CONDUCT.md)

[![Banner](banner.png)](https://traverseresearch.nl)

Rust bindings for [ARM's Neural Graphics SDK for game engines] (based on [AMD's FidelityFX SDK], forked from our [`fidelityfx-rs` bindings]).

[ARM's Neural Graphics SDK for game engines]: https://github.com/arm/neural-graphics-sdk-for-game-engines
[AMD's FidelityFX SDK]: https://github.com/GPUOpen-LibrariesAndSDKs/FidelityFX-SDK
[`fidelityfx-rs` bindings]: https://github.com/Traverse-Research/fidelityfx-rs

| Crates | | |
| - | - | - |
| [`arm-neural-graphics-sys`](./sys) | Raw C-based bindings into the ARM Neural Graphics SDK | [![Latest version](https://img.shields.io/crates/v/arm-neural-graphics-sys.svg?logo=rust)](https://crates.io/crates/arm-neural-graphics-sys) [![Documentation](https://docs.rs/arm-neural-graphics-sys/badge.svg)](https://docs.rs/arm-neural-graphics-sys) |
| [`arm-neural-graphics-bin`](./bin) | Prebuilt binaries from official release | [![Latest version](https://img.shields.io/crates/v/arm-neural-graphics-bin.svg?logo=rust)](https://crates.io/crates/arm-neural-graphics-bin) [![Documentation](https://docs.rs/arm-neural-graphics-bin/badge.svg)](https://docs.rs/arm-neural-graphics-bin) |

