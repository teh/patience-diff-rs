# What is this?

This is an implementation of
[Bram Cohen's patience-diff](https://bramcohen.livejournal.com/73318.html). It's intended to
be used as a library, but there is also a small command line utility:

The library hasn't seen much testing yet so I'm calling it a 0.1.x - use at your own risk.

There is [another implementation](https://crates.io/crates/patience-diff). This
package has no relation to that, and I hadn't seen it until I tried publishing
with the same name.

## API:

```rust
use patience_diff_rs::{patience_diff, Hunk, Range};

fn main() {}
    let before = vec!["x", "y", "c", "z", "0"];
    let after = vec!["x", "b", "y", "z", "1"];
    assert_eq!(
        patience_diff(&before, &after),
        [
            Hunk {
                remove: Range { start: 1, end: 1 },
                insert: Range { start: 1, end: 2 }
            },
            Hunk {
                remove: Range { start: 2, end: 3 },
                insert: Range { start: 3, end: 3 }
            },
            Hunk {
                remove: Range { start: 4, end: 5 },
                insert: Range { start: 4, end: 5 }
            }
        ]
    );
}
```


## Example output:

```plain
$ cargo run -- src/testdata/before.c src/testdata/after.c
    Finished dev [unoptimized + debuginfo] target(s) in 0.05s
     Running `target/debug/pdiff src/testdata/before.c src/testdata/after.c`
--- src/testdata/before.c	2020-02-07T15:27:43.828861414+00:00
+++ src/testdata/after.c	2020-02-06T15:23:53.584403913+00:00
@@ -2,6 +2,10 @@
     x += 1
 }

+void functhreehalves() {
+    x += 1.5
+}
+
 void func2() {
     x += 2
 }
```

