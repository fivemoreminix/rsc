**RSC is a handwritten scientific calculator for turning an equation inside a string into a result.** RSC is designed to be very lightweight and have as few dependencies as possible. It has the goal of just doing a single thing really well, and enabling anyone to extend it with more features.

RSC is also designed with the goal to not become bloated software. **RSC will not receive frequent updates, but that does not mean it is old or unmaintained.** That being said, RSC may still receive internal updates with relation to speed or resource usage.

The abbreviation can be interpreted in two majorly different ways:
* Rust Scientific Calculations library
* Rust Scientific Calculator

# Obtaining RSC
If you would like to install RSC as a program onto your computer, you can either build the package or download it from the releases. More information [here](https://github.com/asmoaesl/rsc/wiki/Executable).

# Size
RSC is ~350 lines of code. Executable RSC is about 387KiB debug; 266KiB release (on Windows).

# Performance
RSC computes instantaneously, even with debug builds. The following output is the time (in nanoseconds) the different operations take to process `sqrt((6.1--2.22)^2 + (-24-10.5)^2)`, where "bench_eval" is all of them at once. More info at [lib.rs](https://github.com/asmoaesl/rsc/blob/master/src/lib.rs).
```rs
PS C:\Users\Luke\Documents\Projects\rsc\target\release> cargo bench
    Finished release [optimized] target(s) in 0.01s
     Running deps\rsc-d36a195311b11bb0.exe

running 4 tests
test tests::bench_compute  ... bench:         174 ns/iter (+/- 30)
test tests::bench_eval     ... bench:       5,678 ns/iter (+/- 300)
test tests::bench_parse    ... bench:       1,480 ns/iter (+/- 149)
test tests::bench_tokenize ... bench:       3,774 ns/iter (+/- 352)
...
```

# Stability
RSC will not have any major changes to its syntax. It will remain to be consistent for a long time. It is up to forkers to make different tastes of RSC. It will also forever keep the same open-source permissions.

# License
RSC is MIT licensed. RSC will always remain free to modify and use without attribution.

# Example
```
PS C:\Users\lukew> rsc
>2+3*4
14
>2+3(4)
14
>(3)(4)
12
>sqrt 4
2
>sqrt(4)
2
>sqrt 4 + 2
4
>sin(6.5/9.7)
0.6210667900937665
>sin cos tan 2
-0.5449592372801408
```
## Debug
RSC can be run with the `ast` flag and show the internal expression that was created by the parser. This is most commonly used for entertainment purposes 😛.
```
PS C:\Users\lukew> rsc ast
>pi*2^2
BinOp(
    Star,
    Constant(
        3.141592653589793
    ),
    Pow(
        Constant(
            2.0
        ),
        Constant(
            2.0
        )
    )
)
12.566370614359172
```
## Errors
```
PS C:\Users\lukew> rsc
>oops
Lexer error: InvalidIdentifier("oops")
>3.3.1
Lexer error: InvalidNumber("3.3.1")
>2+
Parser error: ExpectedFactor(None)
>2(3
Parser error: ExpectedClosingParenthesis
```
