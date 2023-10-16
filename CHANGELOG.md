# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.5] - 2023-10-13
### :sparkles: New Features
- [`32c3150`](https://github.com/SARDONYX-sard/dar-to-oar/commit/32c31505303d1e04b52615c1f26e6147b09d3705) - **front**: add experimental customJS system *(commit by @SARDONYX-sard)*
- [`9e41f60`](https://github.com/SARDONYX-sard/dar-to-oar/commit/9e41f60e30e4b985b0ca0c8c87233c7277a907d9) - **core**: add sentinel in converter *(commit by @SARDONYX-sard)*
- [`ae852d0`](https://github.com/SARDONYX-sard/dar-to-oar/commit/ae852d0757a0a7515856bf09ffbabf2c9c9a0a6e) - implement DAR hinder & OAR remover *(commit by @SARDONYX-sard)*

### :bug: Bug Fixes
- [`92563f2`](https://github.com/SARDONYX-sard/dar-to-oar/commit/92563f2c6ff50c4c6dc47bbd1a0d165874be242b) - **front**: fix design *(commit by @SARDONYX-sard)*
- [`330041f`](https://github.com/SARDONYX-sard/dar-to-oar/commit/330041f1de0b8bd0cb59fb063e5a2d622b822c0f) - **front**: fix problem with navigation focus not changing color after pressing `alt+->` *(commit by @SARDONYX-sard)*
- [`7231c65`](https://github.com/SARDONYX-sard/dar-to-oar/commit/7231c65515febf95637110203bb67421c4fb5bda) - **ci**: remove draft option in release *(commit by @SARDONYX-sard)*
- [`8af1069`](https://github.com/SARDONYX-sard/dar-to-oar/commit/8af1069a9c3d51724f53c8d74c63c764ddb61226) - **core-test**: revert to dyn read file *(commit by @SARDONYX-sard)*

### :recycle: Refactors
- [`20c3c59`](https://github.com/SARDONYX-sard/dar-to-oar/commit/20c3c59109bc08dd90c8b180f7f817cab17e7acc) - **front**: remove unused import *(commit by @SARDONYX-sard)*

### :wrench: Chores
- [`85542ea`](https://github.com/SARDONYX-sard/dar-to-oar/commit/85542ea5820c0810be1c7a4a3e42e22943cbf523) - **bug-report**: add version selectors *(commit by @SARDONYX-sard)*

## [0.1.4] - 2023-10-09
### :sparkles: New Features
- [`20a64c4`](https://github.com/SARDONYX-sard/dar-to-oar/commit/20a64c485b02956647b299e6ba30e5b36f02b8e6) - add dev build ci & new form help text in GUI *(PR [#8](https://github.com/SARDONYX-sard/dar-to-oar/pull/8) by [@SARDONYX-sard](https://github.com/SARDONYX-sard))*

### :wrench: Chores
- [`0b0af17`](https://github.com/SARDONYX-sard/dar-to-oar/commit/0b0af17571a05d4cd9d7512312e3b2bfa383338d) - add license files *(commit by [@SARDONYX-sard](https://github.com/SARDONYX-sard))*


## [0.1.3] - 2023-10-08
### :sparkles: New Features
- [`7d3605c`](https://github.com/SARDONYX-sard/dar-to-oar/commit/7d3605c168310ebbc6f1d0d74382cbca1d7105f3) - impl parallel walk dir(but this is slow) *(commit by [@SARDONYX-sard](https://github.com/SARDONYX-sard))*
- [`b94c041`](https://github.com/SARDONYX-sard/dar-to-oar/commit/b94c041f3120ffbcf9c83abfe64dc270759fb220) - **core**: returns Err instead of unwrap *(commit by [@SARDONYX-sard](https://github.com/SARDONYX-sard))*
- [`dc1ede7`](https://github.com/SARDONYX-sard/dar-to-oar/commit/dc1ede732155e534b4c2a80050904ff77546ed0f) - **core-dar-syntax**: support empty `_condition.txt` *(commit by [@SARDONYX-sard](https://github.com/SARDONYX-sard))*
- [`7f818b4`](https://github.com/SARDONYX-sard/dar-to-oar/commit/7f818b48a8e16dadb9926f53ac9a0a9d387bbd4a) - **frontend**: implement new GUI Design *(commit by [@SARDONYX-sard](https://github.com/SARDONYX-sard))*
- [`1300486`](https://github.com/SARDONYX-sard/dar-to-oar/commit/1300486aad17b9fbc0d02affbced47efa4aae8f9) - **backend**: return the default converter to single *(commit by [@SARDONYX-sard](https://github.com/SARDONYX-sard))*
- [`118d272`](https://github.com/SARDONYX-sard/dar-to-oar/commit/118d2729a128d5065934ec7307302577205701bf) - **front**: remove css hook *(commit by [@SARDONYX-sard](https://github.com/SARDONYX-sard))*
- [`78cf04f`](https://github.com/SARDONYX-sard/dar-to-oar/commit/78cf04f6d37bef00274043e025aef4189df25077) - **front**: add parallel mode checkbox *(commit by [@SARDONYX-sard](https://github.com/SARDONYX-sard))*
- [`711d412`](https://github.com/SARDONYX-sard/dar-to-oar/commit/711d4124206e404c6beca03f5f6fc9fad2c35245) - **cli**: change to bool arg *(commit by [@SARDONYX-sard](https://github.com/SARDONYX-sard))*

### :bug: Bug Fixes
- [`504b793`](https://github.com/SARDONYX-sard/dar-to-oar/commit/504b793551aac5ecbdefa7e54664565f8e554d95) - **ci**: fix tag name *(commit by [@SARDONYX-sard](https://github.com/SARDONYX-sard))*
- [`b5370d0`](https://github.com/SARDONYX-sard/dar-to-oar/commit/b5370d0e1782c446c010750fe26b1edc1c0d1d32) - **core**: support "0X" prefix & eof condition *(commit by [@SARDONYX-sard](https://github.com/SARDONYX-sard))*
- [`3a10598`](https://github.com/SARDONYX-sard/dar-to-oar/commit/3a1059838e13781fb13e875432bdff88430ce6da) - **core**: add `IsActorValueLessThan` condition *(commit by [@SARDONYX-sard](https://github.com/SARDONYX-sard))*

### :recycle: Refactors
- [`4d55fd6`](https://github.com/SARDONYX-sard/dar-to-oar/commit/4d55fd60f9be8fdc5763b4d2556fc64cb774359d) - **core-dar-syntax**: remove redundant stmt *(commit by [@SARDONYX-sard](https://github.com/SARDONYX-sard))*


## [0.1.2] - 2023-10-07
### :zap: Performance Improvements
- [`3ab1c35`](https://github.com/SARDONYX-sard/dar-to-oar/commit/3ab1c35aa69a6c95fb548f747f69bafb98c5b63e) - **front**: implement `useDynStyle` hook *(commit by [@SARDONYX-sard](https://github.com/SARDONYX-sard))*

### :recycle: Refactors
- [`4d310e9`](https://github.com/SARDONYX-sard/dar-to-oar/commit/4d310e9df68b3c8f66194760db10da1515584800) - **front**: sort imports *(commit by [@SARDONYX-sard](https://github.com/SARDONYX-sard))*
- [`d0aeca3`](https://github.com/SARDONYX-sard/dar-to-oar/commit/d0aeca324645c55633fdb91e30f38e2975aa74cd) - **front**: merge state to `useDynStyle` *(commit by [@SARDONYX-sard](https://github.com/SARDONYX-sard))*
- [`564598c`](https://github.com/SARDONYX-sard/dar-to-oar/commit/564598cf06cdef10c0906339fb4d72dcfdb51330) - **tauri**: change to a simplified stmt *(commit by [@SARDONYX-sard](https://github.com/SARDONYX-sard))*

### :wrench: Chores
- [`4490178`](https://github.com/SARDONYX-sard/dar-to-oar/commit/4490178193487fecb52d83605b47430b62924a28) - **github**: add feature request issue template *(commit by [@SARDONYX-sard](https://github.com/SARDONYX-sard))*


[0.1.2]: https://github.com/SARDONYX-sard/dar-to-oar/compare/0.1.1...0.1.2
[0.1.3]: https://github.com/SARDONYX-sard/dar-to-oar/compare/0.1.2...0.1.3
[0.1.4]: https://github.com/SARDONYX-sard/dar-to-oar/compare/0.1.3...0.1.4
[0.1.5]: https://github.com/SARDONYX-sard/dar-to-oar/compare/0.1.4...0.1.5
