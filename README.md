# trulza_win-rs
This is an external Windows hack made to help new users into memory reading/writing in Rust Language. Everything should be easy to read.

### Includes:
- Memory Injection/Reading/Writing
- Helpers like KeyPress and ThreadSleep
- Features:
  - Bhop
  - NoFlash
  - Glow
  - Radar
  - SkinChanger
  - TriggerBot

### Problems:
- Can't read Structs (for example: `mem::read::<Vector3>(localplayer + of::velocity);` won't work)
- SkinChanger: Knives not working and FPS spikes
- No PatternScan (you have to manually add offsets)

### Credits:
  - Got the memory from [navewindre](https://github.com/navewindre/rust-external), modified and updated by me
  - SkinChanger references from [Spec122/skinsX](https://github.com/Spec122/skinsX) and [0xf1a/xSkins](https://github.com/0xf1a/xSkins)
  - Offsets from [hazedumper](https://github.com/frk1/hazedumper)

### Important:
I'm not responsible for any bans that might occur on your account while using this software. Use at your own risk.

## Installation
1. Install [RustUp from the official site](https://www.rust-lang.org/tools/install)
2. Get the "nightly-i686-pc-windows-msvc" toolchain by typing `rustup default nightly-i686` in a Terminal.
3. Clone and unzip this repository, inside it *(you should see: src, Cargo.lock, Cargo.toml)* open a Terminal and type `cargo build --release`
4. It should compile executable located in "-/target/release"