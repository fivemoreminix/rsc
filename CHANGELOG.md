# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 2.0 - 2019-06-21
### Added
* Real named functions! Functions are no longer tokens, and can now be created in a `Computer`, similar to variables.
```rust
let mut map = HashMap::<String, &'a Fn(f64) -> f64>::new();
map.insert("sqrt".to_owned(), &|n| n.sqrt());
```
* RSC is fully generic, now! Types that can support addition, subtraction, and a couple functions necessary in the `Num` trait can be lexed, parsed, and computed with no changes to the RSC source code.
* Getting the previous answer with the new `ans` variable. `ans` does not exist until you've run a calculation on a Computer already.
* Factorial: `5! = 120`

## [1.2.1] - 2017-06-20
### Removed
* Tests from lib.rs removed so it can compile on stable compiler branches.

*Versions prior to 1.2.1 had no changelog recordings, unfortunately.*
