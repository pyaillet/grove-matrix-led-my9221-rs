<p align="center">
    <a href="https://github.com/pyaillet/grove-matrix-led-my9221-rs/actions/workflows/ci.yml"><img src="https://github.com/pyaillet/grove-matrix-led-my9221-rs/actions/workflows/ci.yml/badge.svg?branch=main" alt="Build status" /></a>
    <a href="https://crates.io/crates/grove-matrix-led-my9221"><img src="https://img.shields.io/crates/v/grove-matrix-led-my9221.svg" alt="Crates.io"></a>
    <a href="https://docs.rs/grove-matrix-led-my9221"><img src="https://docs.rs/grove-matrix-led-my9221/badge.svg" alt="Docs.rs"></a>
</p>

# Grove RGB Matrix Led Rust Driver

Rust driver for [Grove RGB Matrix Led with my-9221 Driver](https://wiki.seeedstudio.com/Grove-RGB_LED_Matrix_w-Driver/)

## Example

You can use the example provided for the [stm32f3-discovery board](https://www.st.com/en/evaluation-tools/stm32-discovery-kits.html)

```sh
# Install Cargo embed
cargo install cargo-embed

# Flash the board
cargo embed --chip STM32F303VCTx --release --example stm32f3-discovery-example --target thumbv7em-none-eabihf

```

## Example result

A simple smiley:
![Smiley fixed image](assets/Smiley.jpg)

Picture changing:
![Creepers](assets/Creepers.gif)
