# optional_take

[![Crates.io](https://img.shields.io/crates/d/optional_take?label=crates.io)][crates.io/optional_take]

## optional_take::io

Adapter for [`std::io::Take`][std::io::take] that accepts `Option<u64>` for `limit`.

### Examples

```rust
use optional_take::io::Takable;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let read = b"hello, world";
    let mut take = read.take_optional(Some(5));
    let mut buf = [0; 10];

    let len = take.read(&mut buf)?;
    assert_eq!(len, 5);
    assert_eq!(std::str::from_utf8(&buf[..len]).unwrap(), "hello");

    assert_eq!(take.read(&mut buf).unwrap(), 0);

    take.set_limit(None);
    let len = take.read(&mut buf)?;
    assert_eq!(len, 7);
    assert_eq!(std::str::from_utf8(&buf[..len]).unwrap(), ", world");

    Ok(())
}
```

# License

`optional_take` is distributed under the terms of both the MIT license and the
Apache License (Version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT) for
details.

[std::io::take]: https://doc.rust-lang.org/std/io/struct.Take.html
[crates.io/optional_take]: https://crates.io/crates/optional_take
