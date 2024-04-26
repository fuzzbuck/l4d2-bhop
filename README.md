Hobby project, hold space to automatically BHop in left4dead2q

Press & hold "P" to toggle bhop ON and OFF
check console for toggle status

# Compiling
`cargo build --release`

# Running
`cargo run --release`

# Offsets

at 26/04/2024 offsets are;
```rust
const PLAYER_OFFSET: u32 = 0x724B58;
const MFLAGS_OFFSET: u32 = 0xF0;
const PROCESS_NAME: &str = "left4dead2.exe";
```
