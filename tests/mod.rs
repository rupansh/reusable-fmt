// Copyright (c) 2021 - devs of `reusable-fmt`
// SPDX-License-Identifier: WTFPL

mod consts;

use consts::*;
use reusable_fmt::*;

use std::fmt::Write;

// Ensure that fmt_reuse! is working properly
#[test]
fn reuse_test() {
    assert_eq!(TEST1!(), TEST1);
    assert_eq!(TEST2!(), TEST2);
    assert_eq!(TEST3!(), TEST3);
    assert_eq!(TEST4!(), TEST4);
    assert_eq!(TEST5!(), TEST5);
    assert_eq!(TEST6!(), TEST6);
}

// Ensure that fmt! is working properly.
#[test]
fn fmt_test() {
    assert_eq!(fmt!(TEST1, "TEST1"), "TEST1 TEST1"); // Single Arg
    assert_eq!(fmt!(TEST2, "MULTI", "ARGS"), "TEST2 MULTI ARGS"); // 2 args
    assert_eq!(
        fmt!(TEST3, "RAW", "TEST"),
        TEST3.replacen("{}", "RAW", 1).replacen("{}", "TEST", 1)
    ); // Raw string 2 args
    assert_eq!(fmt!(TEST4, arg = "TEST"), TEST4.replace("{arg}", "TEST")); // Named Argment
    assert_eq!(
        fmt!(TEST5, "SECOND", "FIRST"),
        TEST5.replace("{1}", "FIRST").replace("{0}", "SECOND")
    ); // Custom Ordering
    assert_eq!(
        fmt!(TEST6, "FIRST", "THIRD", "SECOND", arg = "NAMED"),
        TEST6
            .replace("{}", "FIRST")
            .replace("{2}", "SECOND")
            .replace("{1}", "THIRD")
            .replace("{arg}", "NAMED")
    ); // Mixed format
}

// Ensure that fmt! handles what format! can handle too.
#[test]
fn fmt_wrap_test() {
    assert_eq!(fmt!("TEST {}", "TEST"), format!("TEST {}", "TEST"));
    assert_eq!(
        fmt!("TEST {} {}", "TEST", "TEST"),
        format!("TEST {} {}", "TEST", "TEST")
    );
    assert_eq!(
        fmt!("TEST {arg}", arg = "TEST"),
        format!("TEST {arg}", arg = "TEST")
    );
    assert_eq!(
        fmt!(r#"RAW TEST {}"#, "TEST"),
        format!(r#"RAW TEST {}"#, "TEST")
    );
    assert_eq!(
        fmt!("TEST {1} {0}", "SECOND", "FIRST"),
        format!("TEST {1} {0}", "SECOND", "FIRST")
    );
    assert_eq!(
        fmt!(
            r#"MIXED {} {2} {1} {arg}"#,
            "FIRST",
            "THIRD",
            "SECOND",
            arg = "FOURTH"
        ),
        format!(
            r#"MIXED {} {2} {1} {arg}"#,
            "FIRST",
            "THIRD",
            "SECOND",
            arg = "FOURTH"
        )
    );
}

// Ensure that wrt! is working properly
#[test]
fn wrt_test() -> Result<(), std::fmt::Error> {
    let mut s = String::new();

    wrt!(&mut s, TEST1, "TEST1")?; // Single arg
    assert_eq!(s, "TEST1 TEST1");
    s.clear();

    wrt!(&mut s, TEST2, "MULTI", "ARGS")?; // 2 args
    assert_eq!(s, "TEST2 MULTI ARGS");
    s.clear();

    wrt!(&mut s, TEST3, "RAW", "TEST")?; // Raw string 2 args
    assert_eq!(s, TEST3.replacen("{}", "RAW", 1).replacen("{}", "TEST", 1));
    s.clear();

    wrt!(&mut s, TEST4, arg = "TEST")?; // Named arg
    assert_eq!(s, TEST4.replace("{arg}", "TEST"));
    s.clear();

    wrt!(&mut s, TEST5, "SECOND", "FIRST")?; // Custom ordering
    assert_eq!(s, TEST5.replace("{1}", "FIRST").replace("{0}", "SECOND"));
    s.clear();

    wrt!(&mut s, TEST6, "FIRST", "THIRD", "SECOND", arg = "NAMED")?; // Mixed format
    assert_eq!(
        s,
        TEST6
            .replace("{}", "FIRST")
            .replace("{2}", "SECOND")
            .replace("{1}", "THIRD")
            .replace("{arg}", "NAMED")
    );
    s.clear();

    Ok(())
}

// Ensure that wrt! handles what write! can handle too
#[test]
fn wrt_wrap_test() -> Result<(), std::fmt::Error> {
    let mut s = String::new();
    let mut p = String::new();

    wrt!(&mut s, "{}", "TEST1")?;
    write!(&mut p, "{}", "TEST1")?;
    assert_eq!(s, p);

    wrt!(&mut s, "{} {}", "MULTI", "ARGS")?;
    write!(&mut p, "{} {}", "MULTI", "ARGS")?;
    assert_eq!(s, p);

    wrt!(&mut s, r#"{} {}"#, "RAW", "TEST")?;
    write!(&mut p, r#"{} {}"#, "RAW", "TEST")?;
    assert_eq!(s, p);

    wrt!(&mut s, "{arg}", arg = "TEST")?;
    write!(&mut p, "{arg}", arg = "TEST")?;
    assert_eq!(s, p);

    wrt!(&mut s, "{1} {0}", "SECOND", "FIRST")?;
    write!(&mut p, "{1} {0}", "SECOND", "FIRST")?;
    assert_eq!(s, p);

    wrt!(
        &mut s,
        r#"{} {2} {1} {arg}"#,
        "FIRST",
        "THIRD",
        "SECOND",
        arg = "NAMED"
    )?;
    write!(
        &mut p,
        r#"{} {2} {1} {arg}"#,
        "FIRST",
        "THIRD",
        "SECOND",
        arg = "NAMED"
    )?;
    assert_eq!(s, p);

    Ok(())
}
// TODO: tests for other macros
