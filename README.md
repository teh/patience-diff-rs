# What is this?

This is an implementation of
[Bram Cohen's patience-diff](https://bramcohen.livejournal.com/73318.html). It's intended to
be used as a library, but there is also a small command line utility:

The library hasn't seen much testing yet so I'm calling it a 0.1.0 - use at your own risk.


## Example output:

```plain
$ cargo run -- src/testdata/before.c src/testdata/after.c
    Finished dev [unoptimized + debuginfo] target(s) in 0.05s
     Running `target/debug/pdiff src/testdata/before.c src/testdata/after.c`
--- src/testdata/before.c	2020-02-07T15:27:43.828861414+00:00
+++ src/testdata/after.c	2020-02-06T15:23:53.584403913+00:00
@@ -1,6 +1,10 @@
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

