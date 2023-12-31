# DAR to OAR Converter(GUI & CLI)

[![Cargo Lint & Test](https://github.com/SARDONYX-sard/dar-to-oar/actions/workflows/lint-and-test.yaml/badge.svg)](https://github.com/SARDONYX-sard/dar-to-oar/actions/workflows/lint-and-test.yaml)

[![Release Cli](https://github.com/SARDONYX-sard/dar-to-oar/actions/workflows/release-cli.yaml/badge.svg)](https://github.com/SARDONYX-sard/dar-to-oar/actions/workflows/release-cli.yaml)

[![Release GUI](https://github.com/SARDONYX-sard/dar-to-oar/actions/workflows/release-gui.yaml/badge.svg)](https://github.com/SARDONYX-sard/dar-to-oar/actions/workflows/release-gui.yaml)

- What is DAR?
  [Dynamic Animation Replacer](https://www.nexusmods.com/skyrimspecialedition/mods/33746)

- What is OAR?
  [Open Animation Replacer](https://www.nexusmods.com/skyrimspecialedition/mods/92109)

## Getting Started for User

- Download latest version.
  [dar2oar release](https://github.com/SARDONYX-sard/dar-to-oar/releases)

## Please consider to read wiki

[DAR to OAR Wiki](https://github.com/SARDONYX-sard/dar-to-oar/wiki/)

### GUI

Click g_dar2oar.exe

![convert-page](https://github.com/SARDONYX-sard/dar-to-oar/assets/68905624/336cef29-9810-4bbb-b893-0086ff113b75)
![settings-page](https://github.com/SARDONYX-sard/dar-to-oar/assets/68905624/6dd655d7-db34-471b-8df9-defa5fca3445)

### CLI

Example

```shell
./dar2oar --src "./data/Smooth Moveset" --mapping-file "./settings/mapping_table.txt"
```

- options help

```shell
DAR to OAR Converter CLI

Usage: dar2oar.exe [OPTIONS] --src <SRC>

Options:
      --src <SRC>
          DAR source dir path
      --dist <DIST>
          OAR destination dir path(If not, it is inferred from src)
      --name <NAME>
          mod name in config.json & folder name(If not, it is inferred from src)
      --author <AUTHOR>
          mod author in config.json
      --mapping-file <MAPPING_FILE>
          path to section name table
      --mapping-1person-file <MAPPING_1PERSON_FILE>
          path to section name table(For _1st_person)
      --log-level <LOG_LEVEL>
          log_level trace | debug | info | warn | error [default: error]
      --log-path <LOG_PATH>
          Output path of log file [default: ./convert.log]
      --run-parallel
          use multi thread(Probably effective for those with long DAR syntax. Basically single-threaded is faster.)
  -h, --help
          Print help
  -V, --version
          Print version
```

## License

- MIT OR Apache-2.0: Without the source of this application, we could not have
  created this Rust converter. Thank you.

- Original Application: MIT License Copyright (c) 2023 Allison Payne
