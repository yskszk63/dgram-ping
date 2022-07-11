# dgram-ping

Very simple ICMP echo with no privilege.

Currently supports Linux only.

## Example

```
$ cat /proc/sys/net/ipv4/ping_group_range
0       2147483647
$ id -g
1000
$ cargo run --example=ping -- 127.0.0.1
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/examples/ping 127.0.0.1`
127.0.0.1: 0 ms
127.0.0.1: 0 ms
127.0.0.1: 0 ms
^C
$
```

## Snippet

```rust
use dgram_ping::Pinger;

...

    let mut pinger = Pinger::open("127.0.0.1")?;
    pinger.ping().await?
```

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
