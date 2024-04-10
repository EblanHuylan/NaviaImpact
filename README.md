# NaviaImpact

A Server emulator for the game [`Genshin Impact`](https://genshin.hoyoverse.com/en/)
![screenshot](https://git.xeondev.com/reversedrooms/NaviaImpact/raw/branch/master/screenshot.png)

## Installation

### From Source

#### Requirements

- [Rust](https://www.rust-lang.org/tools/install)

#### Building

```sh
git clone https://git.xeondev.com/reversedrooms/NaviaImpact.git
cd NaviaImpact
cargo build --bin gameserver --release
cargo build --bin sdkserver --release
```
### From Pre-built Binaries

Navigate to the [Releases](https://git.xeondev.com/reversedrooms/NaviaImpact/releases)
page and download the latest release for your platform.

## Usage

To begin using the server, you need to run both the SDK server and the game server.

```sh
gameserver.exe
sdkserver.exe
```

If you installed from pre-built binaries, navigate to the directory where you downloaded
the binaries and either a) double-click on the following executable names or b)
run the following in a terminal:

```sh
start gameserver.exe
start sdkserver.exe
```

## Connecting

Get GI 4.5.0 client from somewhere,
put patch [version.dll](https://git.xeondev.com/reversedrooms/hk4e-patch/releases/download/0.1.0/version.dll)
file in your game folder, it will redirect game traffic and patch cryptography.
Replace [mihoyonet.dll](https://autopatchhk.yuanshen.com/client_app/download/pc_zip/20231030132335_iOEfPMcbrXpiA8Ca/ScatteredFiles/GenshinImpact_Data/Plugins/mihoyonet.dll) in `GenshinImpact_Data\Plugins`

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss
what you would like to change, and why.

## Bug Reports

If you find a bug, please open an issue with as much detail as possible. If you
can, please include steps to reproduce the bug.

Bad issues such as "This doesn't work" will be closed immediately, be _sure_ to
provide exact detailed steps to reproduce your bug. If it's hard to reproduce, try
to explain it and write a reproducer as best as you can.
