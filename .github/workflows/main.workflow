workflow "Main" {
  on = "push"
  resolves = ["Format", "Clippy", "Build", "Test"]
}

action "Format" {
  uses = "icepuma/rust-action@master"
  args = "cargo fmt -- --check"
}

action "Clippy" {
  uses = "icepuma/rust-action@master"
  args = "cargo clippy -- -Dwarnings"
}

action "Build" {
  uses = "icepuma/rust-action@master"
  args = "cargo build"
}

action "Test" {
  uses = "icepuma/rust-action@master"
  args = "cargo test"
}
