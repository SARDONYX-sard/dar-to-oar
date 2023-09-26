# DAR to OAR Converter

## Warning

Currently we are managing to work in CLI mode.

The GUI is currently not producing the expected output due to an unknown bug.

## Getting Started for User

- Example

```shell
./dar2oar cli --src "./data/Smooth Moveset" --mapping-file "./settings/mapping_table.txt"
```

- options help

```shell
Usage: dar2oar.exe [COMMAND]

Commands:
  cli   run with cli mode
  help  Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version

run with cli mode

Usage: dar2oar.exe cli [OPTIONS] --src <SRC>

Required:
      --src <SRC>                    DAR source dir path

Options:
      --dist <DIST>                  OAR destination dir path
      --name <NAME>                  mod name in config.json & folder name - If not, it is extracted from the mod name in src
      --author <AUTHOR>              mod author in config.json
      --mapping-file <MAPPING_FILE>  path to section name table
  -h, --help                         Print help
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

### If no correspondence table is passed to --mapping_file: The name of the priority folder is used

the name of the priority folder is used.

Sample

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

- Develop

```bash
npm run dev
```

Open [http://localhost:3000](http://localhost:3000) with your browser to see the
result.
