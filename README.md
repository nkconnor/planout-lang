_Warning: this is in early development and not in suitable shape for deployment._

# Under Construction ~~planout-rust~~

![Build](https://github.com/nkconnor/planout-rust/workflows/Rust/badge.svg)
![Crate](https://img.shields.io/badge/crates.io-json_macros%20=%20%220.1.3%22-brightgreen.svg)

Fast parser, interpreter, and API for [Facebook's PlanOut Framework](https://github.com/facebook/planout).

This project is in active development, iterating on the client API, benchmarks, and ergonomics. Most of the
DSL is implemented, see operators below. Please raise an issue with bugs or suggestions; and contributors
are welcome to open a PR!

## Getting Started

Construct an [`Experiment`](http://google.com) using the `plan!` macro. This will use the `planout.js` compiler to generate an intermediate rep that is parsed into an internal AST at compile time.

```rust
use planout::*;

let bid_pricing = plan!(r#"
  if (country == 'US') {
    algorithm = 'EVEN'
  } else {
    algorithm = 'ASAP'
  }
"#);

let parameters = bid_pricing.evaluate(variables!({
    "user_id" => 30
}));

parameters.get("algorithm")?
```

Alternatively, namespaces in the flavor of a `Meta` also implement `Experiment`:

![](http://facebook.github.io/planout/static/namespace_diagram.png)

```rust
let user_mapping = Segment::variable("user_id").size(5000);

Meta::segment(user_mapping)
     .plan(bid_pricing)
```

Evaluating an experiment has no side-effects or logging,
however, it returns an `Assignment` which implements `serde::Serialize`. You can log this to a file as JSON (in the
style of [Facebook logging](http://facebook.github.io/planout/docs/logging.html));
or send it anywhere else.

## Operators

- [x] Seq
- [x] Set
- [x] Get
- [x] Product
- [ ] Sum
- [x] Array
- [x] Cond
- [x] UniformChoice
- [x] BernoulliTrial
- [ ] WeightedChoice
- [ ] RandomFloat
- [ ] RandomInteger
- [ ] Sample

## License

Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in `sharded` by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

License: MIT OR Apache-2.0
