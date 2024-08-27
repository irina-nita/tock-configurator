Tock Configurator
=================

The Tock Configurator offers a **TUI** (Terminal User Interface) similar to Linux's `menuconfig` to configure Tock (capsules and kernel resources). It also provides a generator to create the `main.rs` based on the configuration.

![](demo.gif)

The workspace(TODO: change this word if we make it a crate) has the following crates(TODO: change this word if we make the modules):

- [`configurator/`](crates/Configurator.md): The **TUI** menu used for visually configuring a platform. This part of the configuration process is meant to be as agnostic as possible to the **Tock**-specific implementations. The application saves the configuration into a `JSON` file named `.config.json`.

- [`parse/` and `parse-macros/`](crates/Parse.md): Parse is the *"glue"* crate for the configurator and the generator. This crates deals with parsing the configuration components into **Rust** code, providing helper types and procedural macros.

- [`generator/`](crates/Generator.md): The crate that can be exported in order to fully parse a configuration file into a `main.rs` file.

- [`chips/`](crates/Chips.md): Supported chips crates, based on the traits provided by the `parse` crate

# Building and Installation

```shell
$ cargo install --path configurator # install the configurator binary
```

# Usage

```shell
$ tock-configurator # run the configuration menu TUI app
```

After configuring the platform, you should see a file named `.config.json`, similar
to this:

```json
{
  "TYPE": "Microbit",
  "CAPSULES": {
    "ALARM": {
      "timer": [
        "Rtc"
      ]
    },
    "CONSOLE": {
      "uart": [
        "Uart0"
      ],
      "baud_rate": 112500
    }
  },
  "SCHEDULER": "Cooperative",
  "PROCESS_COUNT": 0,
  "STACK_SIZE": 2304,
  "SYSCALL_FILTER": "None"
}
```

The crate `tock-generator` deals with parsing the configuration file into
`Rust` code. Add it as a dependency in your crate.

```rust
use tock_generator::{TockMain, Nrf52833};

let tock_main = TockMain::from_json(Nrf52833::default(), ".config.json")?;
tock_main.write_to_file("main.rs")?;
```

**Currently work in progress 🚧**
