# planout-rust

Fast parser, interpreter, and API for [Facebook's PlanOut Framework](https://github.com/facebook/planout).

## Status

In active development, iterating on the client API, benchmarks, and ergonomics. Most of the
DSL is implemented, see operators below.

## Getting Started

Construct an [`Experiment`](http://google.com) using the `plan!` macro.

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

- [X] Seq
- [X] Set
- [X] Get
- [X] Product
- [ ] Sum
- [X] Array
- [X] Cond
- [X] UniformChoice
- [X] BernoulliTrial
- [ ] WeightedChoice
- [ ] RandomFloat
- [ ] RandomInteger
- [ ] Sample
