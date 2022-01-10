// Copyright (c) 2022 Na-x4
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/// Adapter for [`std::io::Take`] that accepts `Option<u64>` for `limit`.
///
/// # Examples
///
/// ```
/// use optional_take::io::Takable;
/// use std::io::{self, Read};
///
/// fn main() -> io::Result<()> {
///     let read = b"hello, world";
///     let mut take = read.take_optional(Some(5));
///     let mut buf = [0; 10];
///
///     let len = take.read(&mut buf)?;
///     assert_eq!(len, 5);
///     assert_eq!(std::str::from_utf8(&buf[..len]).unwrap(), "hello");
///
///     assert_eq!(take.read(&mut buf).unwrap(), 0);
///
///     take.set_limit(None);
///     let len = take.read(&mut buf)?;
///     assert_eq!(len, 7);
///     assert_eq!(std::str::from_utf8(&buf[..len]).unwrap(), ", world");
///
///     Ok(())
/// }
/// ```
///
/// [`std::io::Take`]: std::io::Take
pub mod io;
