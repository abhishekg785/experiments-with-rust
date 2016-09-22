// Copyright 2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(stmt_expr_attributes)]

use std::mem::size_of;

enum Ei8 {
    Ai8 = -1,
    Bi8 = 0
}

enum Eu8 {
    Au8 = 0,
    Bu8 = 0x80
}

enum Ei16 {
    Ai16 = -1,
    Bi16 = 0x80
}

enum Eu16 {
    Au16 = 0,
    Bu16 = 0x8000
}

enum Ei32 {
    Ai32 = -1,
    Bi32 = 0x8000
}

enum Eu32 {
    Au32 = 0,
    Bu32 = 0x8000_0000
}

enum Ei64 {
    Ai64 = -1,
    Bi64 = 0x8000_0000
}

pub fn main() {
    assert_eq!(size_of::<Ei8>(), 1);
    assert_eq!(size_of::<Eu8>(), 1);
    assert_eq!(size_of::<Ei16>(), 2);
    assert_eq!(size_of::<Eu16>(), 2);
    assert_eq!(size_of::<Ei32>(), 4);
    assert_eq!(size_of::<Eu32>(), 4);
    #[cfg(target_pointer_width = "64")]
    assert_eq!(size_of::<Ei64>(), 8);
    #[cfg(target_pointer_width = "32")]
    assert_eq!(size_of::<Ei64>(), 4);
}
