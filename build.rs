// Copyright (c) 2021 - devs of `reusable-fmt`
// SPDX-License-Identifier: WTFPL

use rustc_version::{version_meta, Channel};

fn main() {
    match version_meta().unwrap().channel {
        Channel::Nightly | Channel::Dev => println!("cargo:rustc-cfg=nightly"),
        _ => ()
    }
}
