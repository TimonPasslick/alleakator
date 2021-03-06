# alleakator

![alt=crocodile](https://images.emojiterra.com/google/android-pie/128px/1f40a.png)

`alleakator` provides a global allocator that leaks memory on purpose.

## Motivation

If you have a step based program that only runs for a short time, leaking memory is okay - and the fastest computations are the ones that aren't done.

This crate avoids the runtime overhead of freeing memory by just not doing it.

## Usage

Add this to your main.rs:

```rust
use alleakator::Alleakator;

#[global_allocator]
static GLOBAL: Alleakator<YourAllocator> = Alleakator<YourAllocator>;
```

This works even if you don't use the standard library.

## Implementation

All function calls are forwarded to a parent allocator that you can specify except `dealloc`, which is just empty.

## Contributing

Contributions are welcome.

## License

[MIT](https://choosealicense.com/licenses/mit/)

## Trivia

The image shows a crocodile.
