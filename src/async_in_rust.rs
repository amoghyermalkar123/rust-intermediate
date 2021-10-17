/*

Futures are inert in Rust and make progress only when polled. Dropping a future stops it 
from making further progress.

Async is zero-cost in Rust, which means that you only pay for what you use. 
Specifically, you can use async without heap allocations and dynamic dispatch, 
which is great for performance! This also lets you use async in constrained environments, 
such as embedded systems.

No built-in runtime is provided by Rust. Instead, runtimes are provided by community 
maintained crates.
Both single- and multithreaded runtimes are available in Rust, which have different 
strengths and weaknesses.

*/