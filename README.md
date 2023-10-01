<p align="center">
  <img src="https://github.com/dhirajgagrai/goti/assets/22605432/53ab63fc-66ba-4e18-9159-f77025e360ce" width="480" height="352" />
</p>

# goti
This was created as part of Project Aegis. It will be included in the Project Aegis repository when WASM compatibility is added.

## Usage

### Binaries
Download the binaries from [release page](https://github.com/dhirajgagrai/goti/releases).
Give the necessary file permissions to the executable and start it.

Example for aarch64-appple-darwin:
```bash
chmod a+x goti_aarch64-apple-darwin
./goti_aarch64-apple-darwin
```

### Build from source
Install [Rust](https://www.rust-lang.org/tools/install), and then run the following commands:
```bash
git clone https://github.com/dhirajgagrai/goti.git
cd goti

# Compile
cargo build --release

# Run
cargo run --release
```

### Cross-compilation
Use the Docker Compose configuration to cross-compile for different platforms.
Currently, Docker images are only available for compiling to x86_64 Windows and x86_64 Linux targets.
```bash
# For x86_64-pc-windows-gnu
docker-compose up x86_64-pc-windows-gnu

# For x86_64-unknown-linux-gnu
docker-compose up x86_64-unknown-linux-gnu
```

Executables will be generated under target/\<architecture\>/release directory.

## Help
When starting the app, there is a noticeable delay in visual rendering.
Although the x and y positions of `Goti` begin to change, it's position inside window gets updated after a delay.
This results in the app skipping the first few frames, and the position jumps to the updated location after the delay,
causing an initial lack of smoothness.

To mitigate this issue, I've introduced a delay in calculating the x and y positions using `thread::sleep` in the `update()` function inside the `goti.rs` file:
```rust
thread::sleep(time::Duration::from_micros(9999));
```

I would appreciate any suggestions for a more elegant way to address this.
