# DAR to OAR Converter(GUI & CLI)

- What is DAR?
  [Dynamic Animation Replacer](https://www.nexusmods.com/skyrimspecialedition/mods/33746)

- What is OAR?
  [Open Animation Replacer](https://www.nexusmods.com/skyrimspecialedition/mods/92109)

## Getting Started for User

- Download latest version.

Click this.
[dar2oar release](https://github.com/SARDONYX-sard/dar-to-oar/releases)

### GUI

Click g_dar2oar.exe

![convert-page](https://github.com/SARDONYX-sard/dar-to-oar/assets/68905624/45cf20e7-c8c5-4b24-aeb0-e9aa67fe0a08)
![settings-page](https://github.com/SARDONYX-sard/dar-to-oar/assets/68905624/31d430ed-399c-40b5-912a-a6d046dba869)

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
          [default: ./convert.log]
  -h, --help
          Print help
  -V, --version
          Print version
```

## What is the mapping file?

You can rename the DAR priority folder name to a specific name corresponding to
the priority, instead of using the section name, by passing the path in the
correspondence table as shown below.

### If a mapping file is passed

Sample

- mapping_table.txt

```txt
8000000  Combat
8000001
8000002
// This is a line comment. // to line breaks are ignored.
8000005
8000005  Female
8001000
8001000  Unarmed
8001010
8001010  Sword
```

parse to(As you can see, if there is no corresponding priority folder name, a
sequential number will be added at the end.)

```txt
8000000  Combat
8000001  Combat_1
8000002  Combat_2
8000005  Female
8001000  Unarmed
8001010  Sword
```

result

```txt
Smooth Moveset
    ├─1hm
    ├─2hm
    ├─Axe
    │  └─XPMSE
    ├─Axe+Shield
    │  └─XPMSE
    ├─Axe,
    ├─Base
    ├─Claw
    │  └─XPMSE
    ├─Claw+shield
    │  └─XPMSE
    ├─Dagger
    │  └─XPMSE
    ├─Dagger+Shield
    │  └─XPMSE
    ├─Dual
    │  └─XPMSE
    ├─Dual_1
    │  └─XPMSE
    ├─Dual_2
    ├─Female
    ├─Giant
    ├─Giant_1
    ├─Halberd
    ├─Hammer,
    ├─Javelin
    ├─Javelin+Shield
    ├─Katana
    │  └─XPMSE
    ├─LargeShield
    ├─Mace
    │  └─XPMSE
    ├─Mace+Shield
    │  └─XPMSE
    ├─Non_1
    ├─Non_2
    ├─Pike
    ├─Pike,
    ├─Pike,_1
    ├─Pike_1
    ├─QuarterStaff
    ├─QuarterStaff_1
    ├─Rapier
    │  └─XPMSE
    ├─Rapier+shield
    │  └─XPMSE
    ├─Scythe
    ├─Spear
    ├─Spear+Shield
    │  └─XPMSE
    ├─Spear+Shield_1
    ├─Sword
    │  └─XPMSE
    ├─Sword+Shield
    │  └─XPMSE
    ├─Sword+Shield_1
    │  └─XPMSE
    ├─Sword+Shield_2
    ├─Sword_1
    │  └─XPMSE
    ├─Sword_2
    ├─Unarmed
    ├─Unarmed_1
    │  └─XPMSE
    └─Unarmed_2
```

### If no correspondence table is passed to --mapping_file

The name of the priority folder is used.

- Sample result

```txt
└─Smooth Moveset
    ├─8000001
    ├─8000005
    ├─8001000
    ├─8001010
    │  └─XPMSE
    ├─8001020
    │  └─XPMSE
    ├─8001040
    │  └─XPMSE
    ├─8001041
    ├─8001050
    │  └─XPMSE
    ├─8001055
    │  └─XPMSE
    ├─8001060
    ..........
    ├─8003010
    │  └─XPMSE
    └─8213000
```

## For Developer

requirements Building the CLI

- Rustup latest

Building the GUI

- Rustup latest
- Node.js (LTS 18)

- GUI

```bash
npm run dev # dev
npm run build # release
```

- CLI

```bash
cargo run # dev
cargo build --release # release
```

Open [http://localhost:3000](http://localhost:3000) with your browser to see the
result.
