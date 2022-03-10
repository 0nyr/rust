# Concurrency

### Useful links

##### guide

[concurrency, scope, channels, mutex](https://blog.rust-lang.org/2015/04/10/Fearless-Concurrency.html)

[std::Mutex | doc](https://doc.rust-lang.org/std/sync/struct.Mutex.html)

Result (call `cargo run` to build and run the code):

```shell
(base) onyr@aezyr:~/Documents/code/rust/concurrency/concurrent_vect_use$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/concurrent_vect_use`
Launching the program...
initial counter: Counter { last_value: 100, updating_threads: [] }
Counter { last_value: 400, updating_threads: [0, 0, 0, 0, 1, 1, 1, 2, 0, 2, 1, 2, 2, 1, 2, 0, 1, 2, 0, 0, 2, 1, 2, 1, 2, 2, 0, 0, 1, 1] }
```
