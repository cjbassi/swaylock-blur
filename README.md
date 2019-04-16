# swaylock-blur

A small Rust program that runs [swaylock](https://github.com/swaywm/swaylock) and sets the image to a blurred screenshot of the desktop.

Now with multimonitor support!

Note: Only works on Sway.

## Usage

The blur intensity can be set with the `--blur-sigma=uint` flag.

Swaylock flags can be passed through by adding `--` followed by the flags.

## Installation

Requires:
- grim
- ffmpeg
- swaylock
- sway

### Prebuilt binaries:

Note: (currently only a binary for Linux-x86_64 is available)

Run the following to download the correct binary for your system from the releases tab into `$CARGO_HOME/bin`, courtesy of [japaric/trust](https://github.com/japaric/trust):

```bash
bash <(curl -LSfs https://japaric.github.io/trust/install.sh) \
  -f --git cjbassi/swaylock-blur
```

### From source:

```bash
cargo install --git https://github.com/cjbassi/swaylock-blur
```

### Arch Linux

Available in the AUR using `swaylock-blur-bin`.
