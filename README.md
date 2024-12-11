<div align="center">

# Hyperion

[![Crates.io](https://img.shields.io/crates/v/hyperion.svg)](https://crates.io/crates/hyperion)
![crate license](https://img.shields.io/github/license/wendivoid/hyperion)
[![Docs.rs](https://docs.rs/hexx/badge.svg)](https://docs.rs/hyperion)
[![dependency status](https://deps.rs/crate/hyperion/0.1.0/status.svg)](https://deps.rs/crate/hyperion)

[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)

</div>

Hyperion is a highly Generic crate for working with lindenmayer systems (LSystems). Inspired by The [Algorithmic beauty of Plants](https://en.wikipedia.org/wiki/The_Algorithmic_Beauty_of_Plants). Made to work with all types of lssytems including:

- Stochasic
- Contextual
- Parametric

## Usage

As an example well implement the origin LSystem for the growth of algea.

Hyperion is a framework for working with generic LSystems the first step is to create an enum for grammar that makes up the lsystem.

**NOTE:** This doesn't necessarily have to be an enum. &'static str/String could also work.

```rust
use hyperion::{Rule, LSystemBuilder};

// By Default Alphabet is implemented for any `Copy + PartialEq` type.
#[derive(Clone, Copy, PartialEq)]
pub enum Algea {
    A,
    B,
}

/// Create an LSystem by passing in an Axiom and Rules.
use Algea::*;
let lsys = LSystemBuilder::new([A])
        .rule(Rule::new(A, [A, B]))
        .rule(Rule::new(B, [A]))
        .build();

// To Evauluate the LSystem call .`sample(generation)`. 
lsys.sample(4);
// returns [A, B, A, A, B, A, B, A]


```

More examples can be found in the `tests` folder.

# License

All code in this repository is dual-licensed under either:

- MIT License ([LICENSE-MIT](LICENSE-MIT) or [MIT](http://opensource.org/licenses/MIT))
- Apache License, Version 2.0 ([LICENSE-APACHE](./LICENSE_APACHE) or [APACHE](http://www.apache.org/licenses/LICENSE-2.0))

at your option. This means you can select the license you prefer.

## Your contributions
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
