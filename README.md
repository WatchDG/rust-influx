# influx

```rust
let mut influx = Influx::new("127.0.0.1:8086".into());
influx.connect().unwrap();
let ping = influx.ping().unwrap();
influx.disconnect().unwrap();
```