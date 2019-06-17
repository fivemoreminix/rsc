![](https://img.shields.io/crates/l/rsc.svg) ![](https://img.shields.io/badge/status-stable-blue.svg)

[Changelog](CHANGELOG.md)

**RSC is a handwritten scientific calculator for turning an equation inside a string into a result.** RSC is designed to be very lightweight and have as few dependencies as possible. It has the goal of just doing a single thing really well, and enabling anyone to extend it with more features.

RSC is also designed with the goal to not become bloated software. **RSC will not receive frequent updates, but that does not mean it is old or unmaintained.** That being said, RSC may still receive internal updates with relation to speed or resource usage.

# Obtaining RSC
If you would like to install RSC as a program onto your computer, there is information [here](https://github.com/asmoaesl/rsc/wiki/Executable).

# Performance
RSC computes next to instantaneously, even with debug builds. The following output is the time (in nanoseconds) the different operations take to process `sqrt((6.1--2.22)^2 + (-24-10.5)^2)`, where "bench_eval" is all of them at once. More info at [lib.rs](https://github.com/asmoaesl/rsc/blob/master/src/lib.rs).
```
PS C:\Users\Luke\Documents\Projects\rsc> cargo bench
    Finished release [optimized] target(s) in 0.02s
     Running target\release\deps\rsc-74a7d2c06ab98eee.exe

running 4 tests
test tests::bench_compute  ... bench:         231 ns/iter (+/- 10)
test tests::bench_eval     ... bench:       6,272 ns/iter (+/- 504)
test tests::bench_parse    ... bench:       1,624 ns/iter (+/- 133)
test tests::bench_tokenize ... bench:       4,302 ns/iter (+/- 1,462)
...
```

# Stability
RSC will not have any major changes to its syntax. It will remain to be consistent for a long time. It is up to forkers to make different tastes of RSC. It will also forever keep the same open-source permissions.

# License
RSC is MIT licensed. RSC will always remain free to modify and use without attribution.

# Example
## Executable
Using RSC as the actual program provides a simple interface for solving expressions. A right arrow shows where the user can input an expression, and upon pressing the Return key, the result of the entered expression is displayed on the next line.
```
PS C:\Users\Luke> rsc
>2+2
4
>a = 2
2
>a^3
8
>b
Compute error: UnrecognizedIdentifier("b")
>(b = a^2) + 3
7
>b
4
>sqrt((6.1--2.22)^2 + (-24-10.5)^2)
35.48904619738322
>|-3|
3
>abs -3
3
```
## Library
RSC is very painless to use. For simple, one-off expression solving:
```rust
extern crate rsc;

use rsc::eval;

fn main() {
    assert!(eval("5^2").unwrap() == 25.0);
    assert!(eval("x = 5").unwrap() == 5.0);
    assert!(eval("x").is_err()); // Previously assigned variables are discarded
}
```
In order to keep variables, you must create a `Computer` instance:
```rs
use rsc::computer::Computer;

fn main() {
    let mut c = Computer::<f64>::default();

    assert!(c.eval("x = 5").unwrap() == 5.0);
    assert!(c.eval("x^2").unwrap() == 25.0);
}
```

Much more information can be found in [the documentation](https://docs.rs/rsc/).
## Debug
RSC can be run with the `ast` flag and show the internal expression that was created by the parser. This is most commonly used for entertainment purposes 😛.
```rust
PS C:\Users\Luke> rsc ast
>(a = 2)^3
Pow(
    Assignment(
        "a",
        Constant(
            2.0
        )
    ),
    Constant(
        3.0
    )
)
8
```
# Related Projects
* [rscplot](https://github.com/asmoaesl/rscplot): A graphing calculator dependent on RSC for solving expressions.
