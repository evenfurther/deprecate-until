#![allow(deprecated)]

use deprecate_until::deprecate_until;

#[deprecate_until(since = "0.1", note = "Please ignore", remove = ">= 2.0")]
#[test]
fn ok() {}
