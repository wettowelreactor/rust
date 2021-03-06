// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// pretty-expanded FIXME #23616

fn f(i: int, called: &mut bool) {
    assert_eq!(i, 10);
    *called = true;
}

fn g(f: fn(int, v: &mut bool), called: &mut bool) {
    f(10, called);
}

pub fn main() {
    let mut called = false;
    let h = f;
    g(h, &mut called);
    assert_eq!(called, true);
}
