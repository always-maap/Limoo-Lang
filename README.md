<div align="center">
  <a href="https://github.com/always-maap/TS-Collection">
    <img width="150px;" src="https://raw.githubusercontent.com/always-maap/Limoo-Lang/master/website/public/logo.png" alt="limoo-lang logo" />
  </a>
  
  <p>Limoo-lang is a demonstration purposes only compiler written in Rust</p>
  
</div>

## Overview

## Examples

Sum
```rust
let sum = fn(a, b) {
  return a + b;
}

sum(5, 2)
```

Fibonacci
```rust
let fibonacci = fn(n) {
  if (n < 2) {
    return n;
  }
  
  return fibonacci(n - 1) + fibonacci(n - 2);
}

fibonacci(10)
```

## License 📄
under [MIT-licensed](./LICENSE).
