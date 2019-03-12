# swaylock-blur

A small Rust program that runs [swaylock](https://github.com/swaywm/swaylock) and sets the image to a blurred screenshot of the desktop.

Now with multimonitor support!

Note: Only works on Sway.

## Installation

Requires:
- grim
- ffmpeg
- swaylock
- sway

## Usage

The blur intensity can be set with the `--blur-sigma=uint` flag.

Swaylock flags can be passed through by adding `--` followed by the flags.

### Prebuilt binaries:

Run the following to download the correct binary for your system from the releases tab into `$CARGO_HOME/bin`: (currently only Linux-x86_64 is available)

```
bash <(curl https://raw.githubusercontent.com/japaric/trust/c268696ab9f054e1092f195dddeead2420c04261/install.sh) -f --git cjbassi/swaylock-blur
```

### From source:

```
cargo install --git https://github.com/cjbassi/swaylock-blur
```

### Arch Linux

Available in the AUR using `swaylock-blur-bin`.
