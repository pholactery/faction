# Faction

Because of the shared key nature, if the tests run in parallel, it is possible that
one test will be adding to a key while another one expects it to contain a fixed value.

This is dirty and horrible and awful, but _for now_ tests must be run single-threaded:

```
 cargo test -- -test-threads=1
```
