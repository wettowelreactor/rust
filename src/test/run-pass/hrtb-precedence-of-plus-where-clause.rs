// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// pretty-expanded FIXME #23616

#![feature(unboxed_closures)]

// Test that `F : Fn(int) -> int + Send` is interpreted as two
// distinct bounds on `F`.

fn foo1<F>(f: F)
    where F : FnOnce(int) -> int + Send
{
    bar(f);
}

fn foo2<F>(f: F)
    where F : FnOnce(int) -> int + Send
{
    baz(f);
}

fn bar<F:Send>(f: F) { }

fn baz<F:FnOnce(int) -> int>(f: F) { }

fn main() {}
