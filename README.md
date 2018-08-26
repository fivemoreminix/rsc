**RSC is a handwritten scientific calculator for use in Rust projects that require turning an equation inside a string into a result.** RSC is designed to be very lightweight and have as few dependencies as possible. It has the goal of just doing a single thing really well, and enabling anyone to extend it with more features.

# Size
**RSC 0.1 is less than 250 lines of code** (including comments and whitespace).

# Performance
RSC 0.1 computes instantaneously, but does make a couple allocations during parsing. The goal is to try and cut down allocations at 1.0.

# Stable
When RSC reaches stable, it will rarely change. Only performance improvements will likely appear after it has become stable.
*You can probably expect RSC 1.0 in about a week.*

# License
RSC is MIT licensed. Feel free to make a suggestion for a different license. Just file an issue.

# Use
I have not yet written any documentation, but the library is so simple that it really doesn't need any yet. Check out `src/main.rs` for an example usage.
