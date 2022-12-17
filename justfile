today := `ls src/bin/*.rs | sort | tail -1 | tr -d src/bin/.rs`

t: 
  watchexec -c -- cargo test --bin {{today}} -- --nocapture

r:
  watchexec -c -- cargo run --release --bin {{today}} -- --nocapture

test day:
  watchexec -c -- cargo test --bin {{day}} -- --nocapture

run day:
  watchexec -c -- cargo run --release --bin {{day}} -- --nocapture
