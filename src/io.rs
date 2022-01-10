// Copyright (c) 2022 Na-x4
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fmt::Debug;
use std::io::{self, BufRead, Read, Take};

pub struct OptionalTake<T>(Option<OptionalTakeImpl<T>>);

enum OptionalTakeImpl<T> {
    Read(T),
    Take(Take<T>),
}

impl<T> OptionalTake<T> {
    pub fn from_read(read: T, limit: Option<u64>) -> Self
    where
        T: Read,
    {
        let mut optional_take = OptionalTake(Some(OptionalTakeImpl::Read(read)));
        optional_take.set_limit(limit);
        optional_take
    }

    pub fn limit(&self) -> Option<u64> {
        match self.0.as_ref().unwrap() {
            OptionalTakeImpl::Read(_) => None,
            OptionalTakeImpl::Take(take) => Some(take.limit()),
        }
    }

    pub fn set_limit(&mut self, limit: Option<u64>)
    where
        T: Read,
    {
        match limit {
            Some(limit) => {
                if self.is_take() {
                    self.as_mut_take().unwrap().set_limit(limit);
                } else {
                    let read = match self.0.take().unwrap() {
                        OptionalTakeImpl::Read(read) => read,
                        _ => unreachable!(),
                    };

                    self.0 = Some(OptionalTakeImpl::Take(read.take(limit)));
                }
            }
            None => {
                if !self.is_read() {
                    let read = match self.0.take().unwrap() {
                        OptionalTakeImpl::Take(take) => take.into_inner(),
                        _ => unreachable!(),
                    };

                    self.0 = Some(OptionalTakeImpl::Read(read));
                }
            }
        }
    }

    pub fn into_inner(self) -> T {
        match self.0.unwrap() {
            OptionalTakeImpl::Read(read) => read,
            OptionalTakeImpl::Take(take) => take.into_inner(),
        }
    }

    pub fn get_ref(&self) -> &T {
        match self.0.as_ref().unwrap() {
            OptionalTakeImpl::Read(read) => read,
            OptionalTakeImpl::Take(take) => take.get_ref(),
        }
    }

    pub fn get_mut(&mut self) -> &mut T {
        match self.0.as_mut().unwrap() {
            OptionalTakeImpl::Read(read) => read,
            OptionalTakeImpl::Take(take) => take.get_mut(),
        }
    }
}

impl<T> OptionalTake<T> {
    fn is_read(&self) -> bool {
        match self.0 {
            Some(OptionalTakeImpl::Read(_)) => true,
            _ => false,
        }
    }

    fn is_take(&self) -> bool {
        match self.0 {
            Some(OptionalTakeImpl::Take(_)) => true,
            _ => false,
        }
    }

    fn as_mut_take(&mut self) -> Option<&mut Take<T>> {
        match self.0.as_mut() {
            Some(OptionalTakeImpl::Take(take)) => Some(take),
            _ => None,
        }
    }
}

impl<T: Read> Read for OptionalTake<T> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let read: &mut dyn Read = match self.0.as_mut().unwrap() {
            OptionalTakeImpl::Read(read) => read,
            OptionalTakeImpl::Take(take) => take,
        };

        read.read(buf)
    }
}

impl<T: BufRead> BufRead for OptionalTake<T> {
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        let read: &mut dyn BufRead = match self.0.as_mut().unwrap() {
            OptionalTakeImpl::Read(read) => read,
            OptionalTakeImpl::Take(take) => take,
        };

        read.fill_buf()
    }

    fn consume(&mut self, amt: usize) {
        let read: &mut dyn BufRead = match self.0.as_mut().unwrap() {
            OptionalTakeImpl::Read(read) => read,
            OptionalTakeImpl::Take(take) => take,
        };

        read.consume(amt)
    }
}

impl<T: Debug> Debug for OptionalTake<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("OptionalTake")
            .field(self.0.as_ref().unwrap())
            .finish()
    }
}

impl<T: Debug> Debug for OptionalTakeImpl<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OptionalTakeImpl::Read(ref read) => f.debug_tuple("Read").field(read).finish(),
            OptionalTakeImpl::Take(ref take) => take.fmt(f),
        }
    }
}

pub trait Takable: Read {
    fn take_optional(self, limit: Option<u64>) -> OptionalTake<Self>
    where
        Self: Sized,
    {
        OptionalTake::from_read(self, limit)
    }
}

impl<T> Takable for T where T: Read {}
