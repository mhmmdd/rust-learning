# Run
cargo run hello\
cargo run status\
cargo run hi

# Install Raspberry Pi Remote Debugging

## Install Rust
`$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`\
Logout and login\
`$ rustc --version`\
rustc 1.44.1 (c7087fe00 2020-06-17)

## Install gdbserver
`$ sudo apt-get update && sudo apt-get install -y curl gdb g++-multilib lib32stdc++6 libssl-dev libncurses5-dev`
`$ sudo apt-get install gdb-multiarch`\
`$ sudo apt-get install gdbserver`

## Build project
$ cargo build --package rust-learning --target-dir ./target_remote

## Open the debug session
$ gdbserver --multi :9999 /home/pi/rust/rust-learning/target_remote/debug/rust-learning 