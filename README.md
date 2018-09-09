**RSC is a handwritten scientific calculator for turning an equation inside a string into a result.** RSC is designed to be very lightweight and have as few dependencies as possible. It has the goal of just doing a single thing really well, and enabling anyone to extend it with more features.

RSC is also designed with the goal to not become bloated software. **RSC will not receive frequent updates, but that does not mean it is old or unmaintained.** That being said, RSC may still receive internal updates with relation to speed or resource usage.

The abbreviation can be interpreted in two majorly different ways:
* Rust Scientific Calculations library
* Rust Scientific Calculator

# Size
**RSC is ~300 lines of code**.

# Performance
RSC computes instantaneously, even with debug builds.

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
