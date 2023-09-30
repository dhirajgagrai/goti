<div align="center" width="100%">
  <img src="https://github.com/dhirajgagrai/goti/assets/22605432/53ab63fc-66ba-4e18-9159-f77025e360ce" width="480" height="352" />
</div>

# goti
This was created as part of Project Aegis. It will be included in the Project Aegis repository when WASM compatibility is added.

## Usage

### Binaries
Download the binaries from [here](https://github.com/dhirajgagrai/goti/releases).
Give the necessary file permissions to the executable and start it.

Example for the aarch64-appple-darwin release:
```bash
chmod a+x goti_aarch64-apple-darwin
./goti_aarch64-apple-darwin
```

### Build from source
Install [Rust](https://www.rust-lang.org/tools/install) and build using Cargo.
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
```bash
# For windows
docker-compose up windows_cross_compile

# For linux
docker-compose up linux_cross_compile
```

Executables will be generated under target/\<architecture\>/release directory.
