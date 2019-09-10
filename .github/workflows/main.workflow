workflow "Main" {
  on = "push"
  resolves = ["Test"]
}

action "Format" {
  uses = "icepuma/rust-action@master"
  args = "cargo fmt -- --check"
}

action "Clippy" {
  uses = "icepuma/rust-action@master"
  needs = ["Format"]
  args = "cargo clippy -- -Dwarnings"
}

action "Build" {
  uses = "icepuma/rust-action@master"
  needs = ["Clippy"]
  args = "cargo build"
}

action "Test" {
  uses = "icepuma/rust-action@master"
  needs = ["Build"]
  args = "cargo test"
}
