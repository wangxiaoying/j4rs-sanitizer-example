# j4rs-sanitizer-example

#### Run test
`cargo run --example test`

#### Run init jvm with sanitizer enabled
`RUSTFLAGS=-Zsanitizer=address cargo run --example init_naive_jvm -Zbuild-std --target x86_64-unknown-linux-gnu`

#### Run init jvm (loading `test.jar`) with sanitizer enabled
`RUSTFLAGS=-Zsanitizer=address cargo run --example init_jvm -Zbuild-std --target x86_64-unknown-linux-gnu`

#### Do not init jvm with sanitizer enabled
`RUSTFLAGS=-Zsanitizer=address cargo run --example no_j4rs -Zbuild-std --target x86_64-unknown-linux-gnu`
