// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// xfail-fast
// aux-build:xc_conditions_2.rs

extern mod xc_conditions_2;
use xcc = xc_conditions_2;

pub fn main() {
    xcc::oops::cond.trap(|_| 1).inside(|| xcc::oops::cond.raise(1));
}
