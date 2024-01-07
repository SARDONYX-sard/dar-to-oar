# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.0] - 2024-01-07
### :sparkles: New Features
- [`3f9fcb4`](https://github.com/SARDONYX-sard/dar-to-oar/commit/3f9fcb4a3970218ba81bc67f540bef0b2078f74f) - **core-converter**: change error *(commit by [@SARDONYX-sard](https://github.com/SARDONYX-sard))*
- [`ad640b7`](https://github.com/SARDONYX-sard/dar-to-oar/commit/ad640b793852ecc3292dc6cc5dfce22d9e18994b) - **cli**: add elapsed time *(commit by [@SARDONYX-sard](https://github.com/SARDONYX-sard))*
- [`a48c613`](https://github.com/SARDONYX-sard/dar-to-oar/commit/a48c6130806ab7d18ba48853081c90af1885ce60) - **core**: support ident only function call DAR syntax *(commit by [@SARDONYX-sard](https://github.com/SARDONYX-sard))*

### :bug: Bug Fixes
- [`646e9d8`](https://github.com/SARDONYX-sard/dar-to-oar/commit/646e9d8076d713769014eda7396d161e898ddcf1) - **front**: fix to show NaN *(commit by [@SARDONYX-sard](https://github.com/SARDONYX-sard))*
- [`abf37d4`](https://github.com/SARDONYX-sard/dar-to-oar/commit/abf37d45f383df668d04e96c896c4af4e6559f4d) - **core**: fix non parsed remain comments *(commit by [@SARDONYX-sard](https://github.com/SARDONYX-sard))*
- [`a6bfc90`](https://github.com/SARDONYX-sard/dar-to-oar/commit/a6bfc90a49908e49c4ea86133010b80d141a8bc2) - **core**: ignore remain but set warn *(commit by [@SARDONYX-sard](https://github.com/SARDONYX-sard))*
- [`ae73f9a`](https://github.com/SARDONYX-sard/dar-to-oar/commit/ae73f9a73fd47795251c5e8ef231ee9f63d90a62) - **core**: support for the tail comment is complete *(commit by [@SARDONYX-sard](https://github.com/SARDONYX-sard))*
- [`83a81fb`](https://github.com/SARDONYX-sard/dar-to-oar/commit/83a81fb3e6ac4527a09fe0769d86a9a9be35d2e0) - **core**: fix multi \n *(commit by [@SARDONYX-sard](https://github.com/SARDONYX-sard))*
- [`3cf6cf5`](https://github.com/SARDONYX-sard/dar-to-oar/commit/3cf6cf57435e7b4fb44984388f3a28f254ee6029) - **front**: fix progress *(commit by [@SARDONYX-sard](https://github.com/SARDONYX-sard))*
- [`3417ddb`](https://github.com/SARDONYX-sard/dar-to-oar/commit/3417ddbff452ccfc1b5e3d7d663e3cb3744244e6) - **backend**: fix false multithread mode *(commit by [@SARDONYX-sard](https://github.com/SARDONYX-sard))*

### :recycle: Refactors
- [`3eef3ab`](https://github.com/SARDONYX-sard/dar-to-oar/commit/3eef3ab6daeccd640cb2859e911a328623aeae5a) - **tauri**: into macros *(commit by [@SARDONYX-sard](https://github.com/SARDONYX-sard))*
- [`47c5ab6`](https://github.com/SARDONYX-sard/dar-to-oar/commit/47c5ab67326973c0fde0b3cf5da80ceddbbd6754) - **core-test**: remove unnecessary atomic *(commit by [@SARDONYX-sard](https://github.com/SARDONYX-sard))*

### :white_check_mark: Tests
- [`3350239`](https://github.com/SARDONYX-sard/dar-to-oar/commit/335023922f731e06eb8534fc1426b49986a0bd08) - change test movement *(commit by [@SARDONYX-sard](https://github.com/SARDONYX-sard))*
- [`5ed18aa`](https://github.com/SARDONYX-sard/dar-to-oar/commit/5ed18aacba5874adccd65b090ad56e968acf44b1) - **core**: Change to `FormID` for non prefix hex *(commit by [@SARDONYX-sard](https://github.com/SARDONYX-sard))*

### :wrench: Chores
- [`ca5306d`](https://github.com/SARDONYX-sard/dar-to-oar/commit/ca5306dee9e6adec66e48044e3060ec243b88c95) - **release-ci**: rename to `Node.js cache` *(commit by [@SARDONYX-sard](https://github.com/SARDONYX-sard))*
- [`8f3ea14`](https://github.com/SARDONYX-sard/dar-to-oar/commit/8f3ea140f85b04ca67447f3cdba116bbe3a11701) - try debug style *(commit by [@SARDONYX-sard](https://github.com/SARDONYX-sard))*


## [0.2.3] - 2023-11-17
### :sparkles: New Features
- [`8a3ffd4`](https://github.com/SARDONYX-sard/dar-to-oar/commit/8a3ffd47b6218f490f3e9435fe4fb32fcf4c108e) - **front-settings**: change to centering *(commit by [@SARDONYX-sard](https://github.com/SARDONYX-sard))*
- [`2c6095b`](https://github.com/SARDONYX-sard/dar-to-oar/commit/2c6095bf7b2a47ed8181a6ba946851bdeae7ec3c) - **core-error**: change error handling in `parse_dar2oar` *(commit by [@SARDONYX-sard](https://github.com/SARDONYX-sard))*
- [`90cd2a9`](https://github.com/SARDONYX-sard/dar-to-oar/commit/90cd2a905a9fe482b0e5d3b88f4c8486474ce131) - **translation**: add errMsg in tauri_cmd *(commit by [@SARDONYX-sard](https://github.com/SARDONYX-sard))*
- [`7a216db`](https://github.com/SARDONYX-sard/dar-to-oar/commit/7a216db15c6c262fbd496efd71cf0f885050ca20) - **github-yml**: add versions *(commit by [@SARDONYX-sard](https://github.com/SARDONYX-sard))*

### :bug: Bug Fixes
- [`6107305`](https://github.com/SARDONYX-sard/dar-to-oar/commit/6107305165ba4d21f66e30ece3d92c1d91032d05) - **locale-ja**: fix translation *(commit by [@SARDONYX-sard](https://github.com/SARDONYX-sard))*
- [`8f272b0`](https://github.com/SARDONYX-sard/dar-to-oar/commit/8f272b0df6938c2e548b39a8116d5f321190f955) - **tauri**: fix log rotation *(commit by [@SARDONYX-sard](https://github.com/SARDONYX-sard))*
- [`6950905`](https://github.com/SARDONYX-sard/dar-to-oar/commit/6950905c8c8439313ddfb25165b249ea691bdf43) - **core**: fix uncovered remain error *(commit by [@SARDONYX-sard](https://github.com/SARDONYX-sard))*
- [`ad5e9a5`](https://github.com/SARDONYX-sard/dar-to-oar/commit/ad5e9a5f17884d3a8215227ba700e9b5dcbadcae) - **ci-dev**: fix trigger *(commit by [@SARDONYX-sard](https://github.com/SARDONYX-sard))*
- [`0f355a5`](https://github.com/SARDONYX-sard/dar-to-oar/commit/0f355a54526944f5b2019d9fdbda0a5af6c2c710) - **docs**: add feature *(commit by [@SARDONYX-sard](https://github.com/SARDONYX-sard))*
- [`2f158c8`](https://github.com/SARDONYX-sard/dar-to-oar/commit/2f158c8025e0976cad450cebbdc21890c18c6422) - **docs**: remove invalid comma *(commit by [@SARDONYX-sard](https://github.com/SARDONYX-sard))*
- [`8fd0b6d`](https://github.com/SARDONYX-sard/dar-to-oar/commit/8fd0b6d64f5fc1268463df74f753c4daade89ca2) - **front**: fix inconsistencies in notification design *(commit by [@SARDONYX-sard](https://github.com/SARDONYX-sard))*
- [`e601651`](https://github.com/SARDONYX-sard/dar-to-oar/commit/e601651922a975347225c1cae8c268b4e0b27063) - **front**: fix error path & lint *(commit by [@SARDONYX-sard](https://github.com/SARDONYX-sard))*
- [`f9619ea`](https://github.com/SARDONYX-sard/dar-to-oar/commit/f9619eaacd64ae43263c6b478df487e56d085131) - **npm**: fix command *(commit by [@SARDONYX-sard](https://github.com/SARDONYX-sard))*
- [`12ceb19`](https://github.com/SARDONYX-sard/dar-to-oar/commit/12ceb1928561c018a29b7cdf6a71896a302aedb5) - **ci**: fix npm cache *(commit by [@SARDONYX-sard](https://github.com/SARDONYX-sard))*
- [`37f03d9`](https://github.com/SARDONYX-sard/dar-to-oar/commit/37f03d943f0b4725178291b4aafe8e9603ddb34b) - **ci**: fix cond & add build test(CLI) *(commit by [@SARDONYX-sard](https://github.com/SARDONYX-sard))*
- [`82296ec`](https://github.com/SARDONYX-sard/dar-to-oar/commit/82296ec729eadc0f753ed7ae5f50f4fa6ec5d013) - **ci**: add mkdir *(commit by [@SARDONYX-sard](https://github.com/SARDONYX-sard))*
- [`b4be419`](https://github.com/SARDONYX-sard/dar-to-oar/commit/b4be419ae192ce7f86c864adb774e48954b553af) - **ci**: fix compress process *(commit by [@SARDONYX-sard](https://github.com/SARDONYX-sard))*
- [`8676d2c`](https://github.com/SARDONYX-sard/dar-to-oar/commit/8676d2c683a0e0fd1594b2a1799cc4f30a1e4782) - **ci**: fix os branch *(commit by [@SARDONYX-sard](https://github.com/SARDONYX-sard))*
- [`fe1705c`](https://github.com/SARDONYX-sard/dar-to-oar/commit/fe1705c3d62ee69311109f21da42fe8f874e079f) - **ci**: fix file path *(commit by [@SARDONYX-sard](https://github.com/SARDONYX-sard))*
- [`9402d99`](https://github.com/SARDONYX-sard/dar-to-oar/commit/9402d99de1c526f50b3ba6d3c84cc732c46cf02a) - **ci**: fix path *(commit by [@SARDONYX-sard](https://github.com/SARDONYX-sard))*
- [`0c223f2`](https://github.com/SARDONYX-sard/dar-to-oar/commit/0c223f23356f204fce521a8063e39c59e88ba62a) - **ci**: fix os name *(commit by [@SARDONYX-sard](https://github.com/SARDONYX-sard))*
- [`fbf2808`](https://github.com/SARDONYX-sard/dar-to-oar/commit/fbf2808879bc52fb63ab27f4135ba794b06f46cb) - **ci**: fix cache path *(commit by [@SARDONYX-sard](https://github.com/SARDONYX-sard))*
- [`6b58e63`](https://github.com/SARDONYX-sard/dar-to-oar/commit/6b58e633443bee56b3d9467574539ab2658bdf90) - **cargo**: fix repo url *(commit by [@SARDONYX-sard](https://github.com/SARDONYX-sard))*

### :zap: Performance Improvements
- [`66077b8`](https://github.com/SARDONYX-sard/dar-to-oar/commit/66077b869ce78bae3ef8054dd1d4afba403f7d62) - implement filter *(commit by [@SARDONYX-sard](https://github.com/SARDONYX-sard))*

### :recycle: Refactors
- [`09e3e7b`](https://github.com/SARDONYX-sard/dar-to-oar/commit/09e3e7ba6c8e473b415fd2c659803d7419da6070) - **tauri**: change to macro *(commit by [@SARDONYX-sard](https://github.com/SARDONYX-sard))*
- [`67ea522`](https://github.com/SARDONYX-sard/dar-to-oar/commit/67ea522d3f2f8be2261c68ac1efb67249956e134) - **core-logger**: organize log msg *(commit by [@SARDONYX-sard](https://github.com/SARDONYX-sard))*
- [`c6a1d36`](https://github.com/SARDONYX-sard/dar-to-oar/commit/c6a1d363b16ff3a0a087aab180d6bd2a333907fd) - **frontend**: add `as const` *(commit by [@SARDONYX-sard](https://github.com/SARDONYX-sard))*
- [`6b26f0f`](https://github.com/SARDONYX-sard/dar-to-oar/commit/6b26f0f443172476b16eba29224eaf6dbab3818e) - **core**: non use unwrap *(commit by [@SARDONYX-sard](https://github.com/SARDONYX-sard))*
- [`6910a8d`](https://github.com/SARDONYX-sard/dar-to-oar/commit/6910a8deefcc708da7e59edd9cadae76ab69115e) - **front**: refactor by prettier & me *(commit by [@SARDONYX-sard](https://github.com/SARDONYX-sard))*

### :white_check_mark: Tests
- [`db71093`](https://github.com/SARDONYX-sard/dar-to-oar/commit/db710934414d10b1fea69e7cf883fc39cb556fee) - **front**: add lint & test *(commit by [@SARDONYX-sard](https://github.com/SARDONYX-sard))*
- [`9ee01ec`](https://github.com/SARDONYX-sard/dar-to-oar/commit/9ee01ece28c8aa188505b9995e82b9884518af31) - **bench**: fix bench runner *(commit by [@SARDONYX-sard](https://github.com/SARDONYX-sard))*

### :wrench: Chores
- [`0bc56f5`](https://github.com/SARDONYX-sard/dar-to-oar/commit/0bc56f5756547363fe33935a89cc5388a396d588) - **ci**: rename cache runner name *(commit by [@SARDONYX-sard](https://github.com/SARDONYX-sard))*
- [`4c1c877`](https://github.com/SARDONYX-sard/dar-to-oar/commit/4c1c87739de6ad31838c4f1e9cf3619498ea660a) - **ci**: rename build path *(commit by [@SARDONYX-sard](https://github.com/SARDONYX-sard))*


## [0.2.2] - 2023-11-05
### :bug: Bug Fixes
- [`9fb7259`](https://github.com/SARDONYX-sard/dar-to-oar/commit/9fb7259e793e247fd2c250f5eb09969329e09933) - **front**: fix trans key *(commit by [@SARDONYX-sard](https://github.com/SARDONYX-sard))*
- [`765fa8f`](https://github.com/SARDONYX-sard/dar-to-oar/commit/765fa8f7bf4046e4b7358ab5c0dec15d725f7852) - **front**: fix trans `convert-form-mapping-1st-label` *(commit by [@SARDONYX-sard](https://github.com/SARDONYX-sard))*
- [`5f6f04a`](https://github.com/SARDONYX-sard/dar-to-oar/commit/5f6f04a5463818dd8c114464b0dada09b3addc07) - **front**: fix localization *(commit by [@SARDONYX-sard](https://github.com/SARDONYX-sard))*
- [`460eaa2`](https://github.com/SARDONYX-sard/dar-to-oar/commit/460eaa21d7cbe9a5b86605e7139e62a1bbaf976b) - **core**: correct the forgetting to add `negated` *(commit by [@SARDONYX-sard](https://github.com/SARDONYX-sard))*

### :recycle: Refactors
- [`0957ea1`](https://github.com/SARDONYX-sard/dar-to-oar/commit/0957ea1db0f07de56a898c43e0dfab698fc0d81a) - **front-form**: separate code *(commit by [@SARDONYX-sard](https://github.com/SARDONYX-sard))*
- [`58be2dd`](https://github.com/SARDONYX-sard/dar-to-oar/commit/58be2dd8c9826fa80056fa46291e469c131f6c13) - **locale**: rename by BCP-47 standard *(commit by [@SARDONYX-sard](https://github.com/SARDONYX-sard))*


## [0.2.1] - 2023-11-05
### :bug: Bug Fixes
- [`6e38434`](https://github.com/SARDONYX-sard/dar-to-oar/commit/6e38434bd88705932de9a0234ff96aa79919d54d) - **front**: fix trans key *(commit by [@SARDONYX-sard](https://github.com/SARDONYX-sard))*
- [`f4e622f`](https://github.com/SARDONYX-sard/dar-to-oar/commit/f4e622fdcbb6c327e809d4493a1b5eafeaa7b944) - **front**: add default *(commit by [@SARDONYX-sard](https://github.com/SARDONYX-sard))*
- [`a5333b4`](https://github.com/SARDONYX-sard/dar-to-oar/commit/a5333b405ebf2f74af0c50a7ed338828aef5a349) - **front**: fix display error *(commit by [@SARDONYX-sard](https://github.com/SARDONYX-sard))*


## [0.2.0] - 2023-11-05
### :sparkles: New Features
- [`84cdfb2`](https://github.com/SARDONYX-sard/dar-to-oar/commit/84cdfb21136671f8aa49cbd8f6e2d479ea783485) - **front**: support miniSize monitor *(commit by [@SARDONYX-sard](https://github.com/SARDONYX-sard))*
- [`4f8c71e`](https://github.com/SARDONYX-sard/dar-to-oar/commit/4f8c71e5677b712abb6089534002ffb40d6bbc64) - **core**: accept non-numeric PRIORITY dir names *(commit by [@SARDONYX-sard](https://github.com/SARDONYX-sard))*
- [`c7ded3d`](https://github.com/SARDONYX-sard/dar-to-oar/commit/c7ded3dcbf89125a46674506366787aac85ab1e7) - add mpsc channel mode *(commit by [@SARDONYX-sard](https://github.com/SARDONYX-sard))*
- [`20d645a`](https://github.com/SARDONYX-sard/dar-to-oar/commit/20d645a9159cf871e9c6c2ad887877e85ced625e) - change sender to async fn *(commit by [@SARDONYX-sard](https://github.com/SARDONYX-sard))*
- [`5ed6322`](https://github.com/SARDONYX-sard/dar-to-oar/commit/5ed63229968b3784e4a071c3ed5f8aa64dcb2733) - **tauri**: separate logLevel  fn *(commit by [@SARDONYX-sard](https://github.com/SARDONYX-sard))*
- [`1b23e2c`](https://github.com/SARDONYX-sard/dar-to-oar/commit/1b23e2caf0ae6ee68f6a1d6ba98bef70a1d6268e) - **gui**: implement progress-bar *(commit by [@SARDONYX-sard](https://github.com/SARDONYX-sard))*
- [`8ccbc8f`](https://github.com/SARDONYX-sard/dar-to-oar/commit/8ccbc8f1fc89f575c4afe1669a29f13552f82342) - **tauri**: implement rotation gui logger *(commit by [@SARDONYX-sard](https://github.com/SARDONYX-sard))*
- [`7e0cd6f`](https://github.com/SARDONYX-sard/dar-to-oar/commit/7e0cd6fdb9221e670b7a0585c559dd1f27a1d137) - **front**: implement i18n system *(commit by [@SARDONYX-sard](https://github.com/SARDONYX-sard))*
- [`fe897c7`](https://github.com/SARDONYX-sard/dar-to-oar/commit/fe897c7298c96ddecdec2f18a68a0bd97aa79101) - **front**: add i18n data *(commit by [@SARDONYX-sard](https://github.com/SARDONYX-sard))*

### :bug: Bug Fixes
- [`09ef816`](https://github.com/SARDONYX-sard/dar-to-oar/commit/09ef816ead2e51245e4598612489d28b85938001) - **core**: fix remover condition *(commit by [@SARDONYX-sard](https://github.com/SARDONYX-sard))*
- [`844d8d7`](https://github.com/SARDONYX-sard/dar-to-oar/commit/844d8d73786ae86e15bae7f0598743fb2abe74be) - **core**: fix test path *(commit by [@SARDONYX-sard](https://github.com/SARDONYX-sard))*

### :zap: Performance Improvements
- [`1bd6d24`](https://github.com/SARDONYX-sard/dar-to-oar/commit/1bd6d242df4d016741904899f16fc5fd4244ae2b) - use async *(commit by [@SARDONYX-sard](https://github.com/SARDONYX-sard))*

### :recycle: Refactors
- [`9917ebd`](https://github.com/SARDONYX-sard/dar-to-oar/commit/9917ebdb51e9c276e113fb4683c7a24812dcafcd) - **cli**: refactor macro *(commit by [@SARDONYX-sard](https://github.com/SARDONYX-sard))*
- [`b7e0461`](https://github.com/SARDONYX-sard/dar-to-oar/commit/b7e046185fb1f9c916acd0bba218ff1138488085) - **backend**: refactor imports *(commit by [@SARDONYX-sard](https://github.com/SARDONYX-sard))*
- [`3a429f0`](https://github.com/SARDONYX-sard/dar-to-oar/commit/3a429f03429e08ccd75c645f23eaf97d41ca8c69) - **core**: remove unnecessary `clone` *(commit by [@SARDONYX-sard](https://github.com/SARDONYX-sard))*
- [`edd2a4d`](https://github.com/SARDONYX-sard/dar-to-oar/commit/edd2a4d60e528cc04005f3bf2eab99baa36c2da9) - **tauri**: remove unnecessary field `show_progress` *(commit by [@SARDONYX-sard](https://github.com/SARDONYX-sard))*
- [`bb25909`](https://github.com/SARDONYX-sard/dar-to-oar/commit/bb259099e0294b8f324ba0811363b5ee7db66b91) - **core**: change log level *(commit by [@SARDONYX-sard](https://github.com/SARDONYX-sard))*
- [`d20c03a`](https://github.com/SARDONYX-sard/dar-to-oar/commit/d20c03a9e3e130b3a0f8473a3f82ef21cba29710) - **core**: change log level *(commit by [@SARDONYX-sard](https://github.com/SARDONYX-sard))*
- [`9e6edb7`](https://github.com/SARDONYX-sard/dar-to-oar/commit/9e6edb74738e194e5c3650a6c49092d8dcf52475) - **front**: refactor `StyleList` *(commit by [@SARDONYX-sard](https://github.com/SARDONYX-sard))*
- [`1651d43`](https://github.com/SARDONYX-sard/dar-to-oar/commit/1651d43663b9e716bfcb638692e112d6546f5ba3) - **front**: refactor path *(commit by [@SARDONYX-sard](https://github.com/SARDONYX-sard))*
- [`1535e9c`](https://github.com/SARDONYX-sard/dar-to-oar/commit/1535e9cf66c237527dd4996d17d82995175a7f7d) - **front**: separate component *(commit by [@SARDONYX-sard](https://github.com/SARDONYX-sard))*

### :white_check_mark: Tests
- [`77f216b`](https://github.com/SARDONYX-sard/dar-to-oar/commit/77f216b4a713d7e55960e352d0683b956142aae7) - **core**: add ignore *(commit by [@SARDONYX-sard](https://github.com/SARDONYX-sard))*


## [0.1.6] - 2023-10-27
### :sparkles: New Features
- [`b42d5ed`](https://github.com/SARDONYX-sard/dar-to-oar/commit/b42d5ed9c4bc297e180f6fe88139bb1268d8e107) - **front**: add icon & change msg *(commit by [@SARDONYX-sard](https://github.com/SARDONYX-sard))*

### :bug: Bug Fixes
- [`15f6c9e`](https://github.com/SARDONYX-sard/dar-to-oar/commit/15f6c9e183cbca864b24b8441b08252054285bed) - **ci**: change bot email *(commit by [@SARDONYX-sard](https://github.com/SARDONYX-sard))*
- [`bc6425f`](https://github.com/SARDONYX-sard/dar-to-oar/commit/bc6425f00e577c3360fe64ae0187dfb4a00e676c) - **front**: add `overflowX` *(commit by [@SARDONYX-sard](https://github.com/SARDONYX-sard))*
- [`d842b6b`](https://github.com/SARDONYX-sard/dar-to-oar/commit/d842b6b704631ee0a24f265671eb01e68d5e8036) - **core**: wrapping PluginValue in "form" *(commit by [@SARDONYX-sard](https://github.com/SARDONYX-sard))*
- [`68e62d4`](https://github.com/SARDONYX-sard/dar-to-oar/commit/68e62d46b6384b798a4a93eb0a27c63a9bcff8ad) - **ci**: fix commit name *(commit by [@SARDONYX-sard](https://github.com/SARDONYX-sard))*

### :recycle: Refactors
- [`9bacf29`](https://github.com/SARDONYX-sard/dar-to-oar/commit/9bacf29d1e8758cf9fc57db0f874d273971346cb) - **core**: change assert order *(commit by [@SARDONYX-sard](https://github.com/SARDONYX-sard))*
- [`5719787`](https://github.com/SARDONYX-sard/dar-to-oar/commit/5719787a455e8bb1a536f65bedfc861a39434905) - **front**: remove `overflowX` *(commit by [@SARDONYX-sard](https://github.com/SARDONYX-sard))*

### :wrench: Chores
- [`05928a0`](https://github.com/SARDONYX-sard/dar-to-oar/commit/05928a0b27e2a8e0ee5fa4a85f8b453f7063ed84) - **cargo**: organize item *(commit by [@SARDONYX-sard](https://github.com/SARDONYX-sard))*
- [`687a385`](https://github.com/SARDONYX-sard/dar-to-oar/commit/687a38540421e5f44b530f66e5d2ec12547f6684) - update `CHANGELOG.md` *(commit by [@SARDONYX-sard](https://github.com/SARDONYX-sard))*


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

[0.1.6]: https://github.com/SARDONYX-sard/dar-to-oar/compare/0.1.5...0.1.6
[0.2.0]: https://github.com/SARDONYX-sard/dar-to-oar/compare/0.1.6...0.2.0
[0.2.1]: https://github.com/SARDONYX-sard/dar-to-oar/compare/0.2.0...0.2.1
[0.2.2]: https://github.com/SARDONYX-sard/dar-to-oar/compare/0.2.1...0.2.2
[0.2.3]: https://github.com/SARDONYX-sard/dar-to-oar/compare/0.2.2...0.2.3
[0.3.0]: https://github.com/SARDONYX-sard/dar-to-oar/compare/0.2.3...0.3.0