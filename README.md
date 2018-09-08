**RSC is a handwritten scientific calculator for use in Rust projects that require turning an equation inside a string into a result.** RSC is designed to be very lightweight and have as few dependencies as possible. It has the goal of just doing a single thing really well, and enabling anyone to extend it with more features.

# Size
**RSC 0.1 is less than 300 lines of code**.

# Performance
RSC 0.1 computes instantaneously, even with debug builds.

# Stable
When RSC reaches stable, it will rarely change. Only performance improvements will likely appear after it has become stable.
*You can probably expect RSC 1.0 in about a week.*

# License
RSC is MIT licensed. RSC will always remain free to modify and use without attribution.

# Example
```
PS C:\Users\lukew> rsc
>2+2
4
>3(3)
9
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
>cos(7.2/6.5)
0.44672732786461816
>sin cos tan 2
-0.5449592372801408
>sin ( cos ( tan ( 2 ) ) )
-0.5449592372801408
```
## Errors
```
>oops
Lexer error: InvalidIdentifier("oops")
>3.3.1
Lexer error: InvalidNumber("3.3.1")
>2+
Parser error: ExpectedFactor(None)
>2(3
Parser error: ExpectedClosingParenthesis
```
