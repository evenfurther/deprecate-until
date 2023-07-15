#![deny(warnings)]

use deprecate_until::deprecate_until;

#[deprecate_until(remove = ">= 1.0")]
fn f1() {}

#[deprecate_until(remove = ">= 1.0", note = "Remove me")]
fn f2() {}

#[deprecate_until(remove = ">= 1.0", since = "1.0.0", note = "Remove me")]
fn f3() {}

fn main() {
    f1();
    f2();
    f3();
}
