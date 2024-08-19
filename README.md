# Problem

Suppose you have foo.did, and its contents are like this:

```
type Hello = ...;

service : (Hello) {
   jump : (Request) -> (Response);
}
```

Furthermore, suppose you have a blob that was constructed (possibly by someone
else) like so:

```rust
let blob = candid::Encode(&Hello { ... }).unwrap();
```

How would you take foo.did and this blob, and print it out nicely?

## Related Work

When you do

```bash
dfx canister call --candid foo.did hello '(record { ... })'
```

dfx reads foo.did, and uses that to "nicely" print out the response. That is, it
does not print opaque field IDs, but rather field names, according to the
definition of Response in foo.did. That is sort of what we are trying to do
here. The difference is that our type is not associated with an individual
method, but rather, with the service as a whole.


# How src/lib.rs was "written":

```bash
didc bind --target rs foo.did > src/lib.rs
```

# Running This Proof-of-Concept

```
cargo run
```

All this does is compile src/main.rs and run it. The only other source files are
foo.did and src/lib.rs, but lib.rs hardly counts, since it isn't hand-written.
Anyway, main.rs uses foo.did via include_str!. Therefore, to understand how to
solve the problem described in the first section ☝️, "simply" read src/main.rs.