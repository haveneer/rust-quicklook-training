# tokio block producer/consumer with a zero-copy block parser

* `cargo bench` show comparison between 3 parsing methods (naive, zero-copy, iterator only)
* `cargo bench --features iai` benches the same methods using iai-callgrind (don't forget to [install tools](../../benches/environment))

## Testing tools

* Test server using

```
nc localhost 9000 | hexdump -C
```

as a fake reader

* Test client using

```
cat /dev/random | nc -l 9000 (fake server)
```

as a fake server

* To enable [tokio-console](https://github.com/tokio-rs/console), compile/run as:

```
RUST_LOG=info RUSTFLAGS="--cfg tokio_unstable" cargo run --bin <BIN>
```

## Examples

### `main_sender`

* send fake blocks in sequence to clients connected to port 9000
* provide an HTTP on port 8080 (http://localhost:8080/stats) that returns a JSON describing counts on sent and built
  blocks

### `main_reader`

* Connect to a server
* Read chunks of data to fill a local buffer.
* Once filled, decodes blocks within and refill buffer when necessary

### `main_reader2`

* As `main_reader` using [tokio Streams](https://tokio.rs/tokio/tutorial/streams)
* The chunk size is defined by tokio

### `main_reader3`

* Split filling buffer and decoding blocks in two tasks
* Both share a buffer
* If the buffer is fully consumed by the decoding task, it waits for a notification (
  using [Notify](https://docs.rs/tokio/latest/tokio/sync/struct.Notify.html) primitive)  