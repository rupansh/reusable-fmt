// Copyright (c) 2021 - devs of `reusable-fmt`
// SPDX-License-Identifier: WTFPL

use reusable_fmt::*;

pub const TEST1: &str = "TEST1 {}";
pub const TEST2: &str = "TEST2 {} {}";
pub const TEST3: &str = r#"
TEST3
RAW STRING
{} {}
"#;
pub const TEST4: &str = "TEST4 NAMED {arg}";
pub const TEST5: &str = "TEST5 CUSTOM ORDER {1} {0}";
pub const TEST6: &str = r#"
TEST6 MIXED
DEF {}
CUST ORD {2} {1}
NAMED {arg}
"#;

fmt_reuse! {
    TEST1 = "TEST1 {}";
    TEST2 = "TEST2 {} {}";
    TEST3 = r#"
TEST3
RAW STRING
{} {}
"#;
    TEST4 = "TEST4 NAMED {arg}";
    TEST5 = "TEST5 CUSTOM ORDER {1} {0}";
    TEST6 = r#"
TEST6 MIXED
DEF {}
CUST ORD {2} {1}
NAMED {arg}
"#;
}

