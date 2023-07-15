#![allow(deprecated)]

use deprecate_until::deprecate_until;

#[deprecate_until(remove = ">= 0.0")]
fn f1() {}

#[deprecate_until(remove = ">= 0.0", note = "Remove me")]
fn f2() {}

#[deprecate_until(remove = ">= 0.0", since = "0.0.0", note = "Remove me")]
fn f3() {}

#[deprecate_until(remove = "x.1")]
fn f4() {}

#[deprecate_until(note = "foo")]
fn f5() {}

#[deprecate_until(remove = "1.x", remove = "2.x")]
fn f6() {}

#[deprecate_until(bogus = "bar")]
fn f7() {}

fn main() {
}
