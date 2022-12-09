test day:
  watchexec -c -- cargo test --bin {{day}} -- --nocapture

run day:
  watchexec -c -- cargo run --bin {{day}} -- --nocapture
