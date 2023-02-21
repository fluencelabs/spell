# Changelog

## [0.4.0](https://github.com/fluencelabs/spell/compare/spell-v0.3.6...spell-v0.4.0) (2023-02-21)


### ⚠ BREAKING CHANGES

* **storage:** bump SQLite module to 0.17.1 ([#64](https://github.com/fluencelabs/spell/issues/64))

### Bug Fixes

* **storage:** bump SQLite module to 0.17.1 ([#64](https://github.com/fluencelabs/spell/issues/64)) ([386edad](https://github.com/fluencelabs/spell/commit/386edad08b84c2e915b5a8a669d2fffc5a21021c))

## [0.3.6](https://github.com/fluencelabs/spell/compare/spell-v0.3.5...spell-v0.3.6) (2023-02-21)


### Features

* improve cli logs command [fixes DXJ-296] ([#61](https://github.com/fluencelabs/spell/issues/61)) ([e7cfd53](https://github.com/fluencelabs/spell/commit/e7cfd53503ef6b878284613b23a09c9ee1d67cea))


### Bug Fixes

* **installation-spell:** workaround for failed registration ([#63](https://github.com/fluencelabs/spell/issues/63)) ([30ae917](https://github.com/fluencelabs/spell/commit/30ae917eb22ff570133d07ba7ff53b261bab43e5))

## [0.3.5](https://github.com/fluencelabs/spell/compare/spell-v0.3.4...spell-v0.3.5) (2023-02-17)


### Features

* **installation-spell:** implement get_logs ([#59](https://github.com/fluencelabs/spell/issues/59)) ([81611c5](https://github.com/fluencelabs/spell/commit/81611c5dea0561c51fbc51657aafc2261ab1ea5d))

## [0.3.4](https://github.com/fluencelabs/spell/compare/spell-v0.3.3...spell-v0.3.4) (2023-02-16)


### Bug Fixes

* **spell:** remove 'prepare' ([#57](https://github.com/fluencelabs/spell/issues/57)) ([3a39c61](https://github.com/fluencelabs/spell/commit/3a39c61c51ec2eb7f14bc67a47a1f56b1119506e))

## [0.3.3](https://github.com/fluencelabs/spell/compare/spell-v0.3.2...spell-v0.3.3) (2023-02-16)


### Features

* **deals:** implement deal_install ([#55](https://github.com/fluencelabs/spell/issues/55)) ([d96fa78](https://github.com/fluencelabs/spell/commit/d96fa78bcdec408a53487a60f6a8eb749ccd3092))

## [0.3.2](https://github.com/fluencelabs/spell/compare/spell-v0.3.1...spell-v0.3.2) (2023-02-14)


### Bug Fixes

* **config:** upload module config as a string ([#51](https://github.com/fluencelabs/spell/issues/51)) ([6ce60d1](https://github.com/fluencelabs/spell/commit/6ce60d10804a9cb18af9c78ecebf37a335167516))

## [0.3.1](https://github.com/fluencelabs/spell/compare/spell-v0.3.0...spell-v0.3.1) (2023-02-13)


### Features

* Add Installation Spell [#34](https://github.com/fluencelabs/spell/issues/34) ([b957008](https://github.com/fluencelabs/spell/commit/b95700881e77805d2a17c18d21568943f2a51f2e))
* **workers:** Implement worker deployment though spells ([#39](https://github.com/fluencelabs/spell/issues/39)) ([29952b9](https://github.com/fluencelabs/spell/commit/29952b99b83f0fc817d7c095744341a9ade9eaad))

## [0.3.0](https://github.com/fluencelabs/spell/compare/spell-v0.2.0...spell-v0.3.0) (2023-02-08)


### ⚠ BREAKING CHANGES

* **api:** accept init_data ⊤ (any) in Spell.install ([#35](https://github.com/fluencelabs/spell/issues/35))

### Features

* **api:** accept init_data ⊤ (any) in Spell.install ([#35](https://github.com/fluencelabs/spell/issues/35)) ([61982ef](https://github.com/fluencelabs/spell/commit/61982efd736dca1085236067da6be4048b5d4578))

## Changelog

### ⚠ BREAKING CHANGES

* **errors:** SpellValueT::get_error -> SpellValueT::take_error to get rid of `.clone()`

### Features

* **kv:** add exists method [NET-301] ([#20](https://github.com/fluencelabs/spell/issues/20)) ([360d5ea](https://github.com/fluencelabs/spell/commit/360d5eade111fae5ce4a8835f33dada464c9fc32))
* add serializable to TriggerConfig ([#8](https://github.com/fluencelabs/spell/issues/8)) ([e1b0e28](https://github.com/fluencelabs/spell/commit/e1b0e2855b23d0457c92245b1a5f7c24b5cb6ac2))
* **errors:** SpellValueT::get_error -&gt; SpellValueT::take_error ([#21](https://github.com/fluencelabs/spell/issues/21)) ([531d856](https://github.com/fluencelabs/spell/commit/531d856a91ac60bd3e72841dc86feafbd6f7cb46))
* set `absent` flag when key is not in KV ([#19](https://github.com/fluencelabs/spell/issues/19)) ([a62e03e](https://github.com/fluencelabs/spell/commit/a62e03e6c681add41b67d6461fd729cd324955f4))

### Bug Fixes

* add doc-comment for spell-distro::modules ([#9](https://github.com/fluencelabs/spell/issues/9)) ([501432f](https://github.com/fluencelabs/spell/commit/501432fd7774a2a77211b77f475fc05d9ae2f2b8))
