# DAR to OAR Converter(GUI & CLI)

[![Cargo Lint & Test](https://github.com/SARDONYX-sard/dar-to-oar/actions/workflows/lint-and-test.yaml/badge.svg)](https://github.com/SARDONYX-sard/dar-to-oar/actions/workflows/lint-and-test.yaml)

[![Release Cli](https://github.com/SARDONYX-sard/dar-to-oar/actions/workflows/release-cli.yaml/badge.svg)](https://github.com/SARDONYX-sard/dar-to-oar/actions/workflows/release-cli.yaml)

[![Release GUI](https://github.com/SARDONYX-sard/dar-to-oar/actions/workflows/release-gui.yaml/badge.svg)](https://github.com/SARDONYX-sard/dar-to-oar/actions/workflows/release-gui.yaml)

- What is DAR?
  [Dynamic Animation Replacer](https://www.nexusmods.com/skyrimspecialedition/mods/33746)

- What is OAR?
  [Open Animation Replacer](https://www.nexusmods.com/skyrimspecialedition/mods/92109)

## Features

- [x] DAR to OAR conversion(CLI & GUI applications)
- [x] Implemented sub commands (Remove OAR dir, Unhide DAR files)
- [x] Mapping table complements OAR's readability on GUI
- [x] Localization system(Could be customized)
- [x] Could edit JavaScript & CSS

## Getting Started for User

- Download latest version.
  [dar2oar release](https://github.com/SARDONYX-sard/dar-to-oar/releases)

## Please consider to read wiki

[DAR to OAR Wiki](https://github.com/SARDONYX-sard/dar-to-oar/wiki/)

### GUI

Click g_dar2oar.exe

![convert-page](https://github.com/SARDONYX-sard/dar-to-oar/assets/68905624/b0074c27-d26b-4ce9-b093-3e8ed20205e4)
![settings-page](https://github.com/SARDONYX-sard/dar-to-oar/assets/68905624/d00ab41b-4fd6-4189-bef6-612fdefae384)

### CLI

Example

```shell
./dar2oar convert "./test/data/UNDERDOG Animations" --mapping-file "./test/mapping_tables/UnderDog Animations_v1.9.6_mapping_table.txt" --run-parallel --stdout
```

#### CLI Help

```shell
dar2oar 0.4.0
DAR to OAR Converter CLI

USAGE:
    dar2oar.exe <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    convert       Convert DAR to OAR
    help          Print this message or the help of the given subcommand(s)
    remove-oar    Find and delete `OpenAnimationReplacer` directory
    unhide-dar    Unhide all files in the `DynamicAnimationReplacer` directory by removing the
                      `mohidden` extension
    Finished release [optimized] target(s) in 0.74s
     Running `target\release\dar2oar.exe convert --help`
```

```shell
dar2oar.exe-convert
Convert DAR to OAR

USAGE:
    dar2oar.exe convert [OPTIONS] <SRC>

ARGS:
    <SRC>
            DAR source dir path

OPTIONS:
        --author <AUTHOR>
            Mod author in config.json

        --dst <DST>
            OAR destination dir path(If not, it is inferred from DAR path)

    -h, --help
            Print help information

        --hide-dar
            After conversion, add ".mohidden" to all DAR files to hide them(For MO2 user)

        --log-level <LOG_LEVEL>
            Log level

            trace | debug | info | warn | error

            [default: error]

        --log-path <LOG_PATH>
            Output path of log file

            [default: ./convert.log]

        --mapping-1person-file <MAPPING_1PERSON_FILE>
            Path to section name table(For _1st_person)

        --mapping-file <MAPPING_FILE>
            Path to section name table

            - See more details
            https://github.com/SARDONYX-sard/dar-to-oar/wiki#what-is-the-mapping-file

        --name <NAME>
            Mod name in config.json & directory name(If not, it is inferred from DAR path)

        --run-parallel
            Use multi thread

            [Note] More than twice the processing speed can be expected, but the concurrent
            processing results in thread termination timings being out of order, so log writes will
            be out of order as well, greatly reducing readability of the logs.

        --stdout
            Log output to stdout as well
    Finished release [optimized] target(s) in 0.72s
     Running `target\release\dar2oar.exe remove-oar --help`
```

```shell
dar2oar.exe-remove-oar
Find and delete `OpenAnimationReplacer` directory

USAGE:
    dar2oar.exe remove-oar [OPTIONS] <TARGET_PATH>

ARGS:
    <TARGET_PATH>
            Path containing the "OpenAnimationReplacer" directory

OPTIONS:
    -h, --help
            Print help information

        --log-level <LOG_LEVEL>
            Log level

            trace | debug | info | warn | error

            [default: error]

        --log-path <LOG_PATH>
            Output path of log file

            [default: ./convert.log]

        --stdout
            Log output to stdout as well
    Finished release [optimized] target(s) in 0.70s
     Running `target\release\dar2oar.exe unhide-dar --help`
```

```shell
dar2oar.exe-unhide-dar
Unhide all files in the `DynamicAnimationReplacer` directory by removing the `mohidden` extension

USAGE:
    dar2oar.exe unhide-dar [OPTIONS] <DAR_DIR>

ARGS:
    <DAR_DIR>
            DAR directory containing files with ".mohidden" extension

OPTIONS:
    -h, --help
            Print help information

        --log-level <LOG_LEVEL>
            Log level

            trace | debug | info | warn | error

            [default: error]

        --log-path <LOG_PATH>
            Output path of log file

            [default: ./convert.log]

        --stdout
            Log output to stdout as well
```

## License

- MIT OR Apache-2.0: Without the source of this application, we could not have
  created this Rust converter. Thank you.

- Original Application: MIT License Copyright (c) 2023 Allison Payne
