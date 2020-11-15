# miniremotery

Minimal Rust wrapper around the [`Remotery`](https://github.com/Celtoys/Remotery) real-time CPU profiler.
GPU profiling is not supported.

Inspired by (seemingly outdated and abandoned) [`remotery`](https://docs.rs/remotery/0.1.2/remotery/),
which this is a slightly updated version of.

# How to use
1) Call `Remotery::initialize` at application startup.
## Example
```rust
use miniremotery::Remotery;
use strum::EnumMessage;

let profiler = Remotery::initialize().unwrap_or_else(
    |error| panic!(
        "Remotery initialization error: {}"
        ,error.get_detailed_message().unwrap()
    )
);
// Remotery is finalized when `profiler` goes out of scope.
```
2) Add calls to `Remotery::scope` to code scopes you want to profile.
## Example
```rust
use miniremotery::{ Remotery, rmtSampleFlags };

fn does_something_complicated() {
    let _scope = Remotery::scope("does_something_complicated", rmtSampleFlags::RMTSF_None);

    // Some expensive calculations.

    // Remotery gathers timing data when `_scope` goes out of scope.
}
```
3) Open `vis/index.html` when the profiled application is running to view real-time profile data.

Refer to [`Remotery`](https://github.com/Celtoys/Remotery) repo for more info.