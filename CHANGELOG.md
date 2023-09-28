# Changelog

## [0.5.23](https://github.com/fluencelabs/spell/compare/spell-v0.5.22...spell-v0.5.23) (2023-09-28)


### Bug Fixes

* **installation-spell:** migrate deal.logs to subnets [NET-570] ([#181](https://github.com/fluencelabs/spell/issues/181)) ([9c39bfb](https://github.com/fluencelabs/spell/commit/9c39bfb107ed135d2b8fa5c6178a490700f4ba4f))

## [0.5.22](https://github.com/fluencelabs/spell/compare/spell-v0.5.21...spell-v0.5.22) (2023-09-25)


### Features

* **installation-spell:** add worker removal for direct deploy [NET-546] ([#179](https://github.com/fluencelabs/spell/issues/179)) ([6c5e490](https://github.com/fluencelabs/spell/commit/6c5e490b89fd1a0f42a56b05667abe0a8b23fb46))

## [0.5.21](https://github.com/fluencelabs/spell/compare/spell-v0.5.20...spell-v0.5.21) (2023-09-12)


### Features

* **installation-spell:** change ipfs configuration ([#175](https://github.com/fluencelabs/spell/issues/175)) ([0f75fa9](https://github.com/fluencelabs/spell/commit/0f75fa9dde43549aa0caa10fb8ee2bf6fc7c88b5))


### Bug Fixes

* **installation-spell:** dedup ipfs multiaddrs ([#177](https://github.com/fluencelabs/spell/issues/177)) ([ca326b0](https://github.com/fluencelabs/spell/commit/ca326b0c1750f8171394290522eee6dea3f96cb9))

## [0.5.20](https://github.com/fluencelabs/spell/compare/spell-v0.5.19...spell-v0.5.20) (2023-09-06)


### Features

* bump spell 0.5.19 ([#171](https://github.com/fluencelabs/spell/issues/171)) ([a41cff5](https://github.com/fluencelabs/spell/commit/a41cff5ae8d3420ae55cd7f1953f5ee49bb95c27))

## [0.5.19](https://github.com/fluencelabs/spell/compare/spell-v0.5.18...spell-v0.5.19) (2023-09-06)


### Features

* **installation-spell:** pass dummy deal id for direct deploy [NET-520] ([#168](https://github.com/fluencelabs/spell/issues/168)) ([37eeb42](https://github.com/fluencelabs/spell/commit/37eeb42ae6c2ff59f57f478ff6b70358de93ac94))
* **installation-spell:** upload to ipfs on relay ([#169](https://github.com/fluencelabs/spell/issues/169)) ([5fc64fa](https://github.com/fluencelabs/spell/commit/5fc64fae9e509ecf4c90ffbbd7122b7a58c83c37))

## [0.5.18](https://github.com/fluencelabs/spell/compare/spell-v0.5.17...spell-v0.5.18) (2023-09-04)


### Features

* **installation-spell:** add statuses and tests [NET-504] ([#159](https://github.com/fluencelabs/spell/issues/159)) ([63ffeb7](https://github.com/fluencelabs/spell/commit/63ffeb72e0a823cd1202a00e212b9fd6be0d19d9))


### Bug Fixes

* **deps:** update marine-rs-sdk and sqlite-wasm-connector ([#165](https://github.com/fluencelabs/spell/issues/165)) ([aa9e8e9](https://github.com/fluencelabs/spell/commit/aa9e8e975c2bf7f6efc15744443412d81916e569))
* **installation-spell:** fix bug on multiple services installaiton [NET-519] ([#160](https://github.com/fluencelabs/spell/issues/160)) ([68dabc5](https://github.com/fluencelabs/spell/commit/68dabc53918d2cd5d8a1dc23429dc0bd61acd293))
* **installation-spell:** remove get_trigger [NET-533] ([#166](https://github.com/fluencelabs/spell/issues/166)) ([8352de8](https://github.com/fluencelabs/spell/commit/8352de8e3ccb6c5c46601db72d86c20715dfca60))

## [0.5.17](https://github.com/fluencelabs/spell/compare/spell-v0.5.16...spell-v0.5.17) (2023-07-27)


### Features

* **mailbox:** get_mailbox returns in FIFO order [NET-508] ([#153](https://github.com/fluencelabs/spell/issues/153)) ([715981f](https://github.com/fluencelabs/spell/commit/715981fc0f7d41fc5f3d660da678dbef21211fad))
* **mailbox:** store and return init_peer_id and timestamp [NET-510] ([#154](https://github.com/fluencelabs/spell/issues/154)) ([333e404](https://github.com/fluencelabs/spell/commit/333e404913e54f66672ef22b292f8d4a8af275b8))

## [0.5.16](https://github.com/fluencelabs/spell/compare/spell-v0.5.15...spell-v0.5.16) (2023-07-14)


### Features

* rotate logs and mailbox [NET-496 NET-497] ([#145](https://github.com/fluencelabs/spell/issues/145)) ([ab5e5b7](https://github.com/fluencelabs/spell/commit/ab5e5b76d0904ed89e0f495b1d9e3dd4f3c92994))

## [0.5.15](https://github.com/fluencelabs/spell/compare/spell-v0.5.14...spell-v0.5.15) (2023-06-12)


### Features

* **installation-spell:** add dag upload to IpfsClient ([#139](https://github.com/fluencelabs/spell/issues/139)) ([2a261a4](https://github.com/fluencelabs/spell/commit/2a261a44274458921878535b0e7e16a93e7d7bd4))

## [0.5.14](https://github.com/fluencelabs/spell/compare/spell-v0.5.13...spell-v0.5.14) (2023-06-06)


### Features

* **script:** allow to overwrite spell script ([#128](https://github.com/fluencelabs/spell/issues/128)) ([3a23274](https://github.com/fluencelabs/spell/commit/3a232742d6a95b54f4e08328922aa3dd1c8da6b8))


### Bug Fixes

* **deal:** print subnet id correctly ([#130](https://github.com/fluencelabs/spell/issues/130)) ([ea01a42](https://github.com/fluencelabs/spell/commit/ea01a429cdfcb2e28679fcb657b5d077fe0c9812))
* **deps:** update dependency @fluencelabs/aqua-ipfs to v0.5.13 ([#110](https://github.com/fluencelabs/spell/issues/110)) ([159e1b7](https://github.com/fluencelabs/spell/commit/159e1b77683e1c76b1fb4320b9c9fb3e3fe866da))

## [0.5.13](https://github.com/fluencelabs/spell/compare/spell-v0.5.12...spell-v0.5.13) (2023-05-28)


### Features

* **deps:** update registry to 0.8.6 ([#126](https://github.com/fluencelabs/spell/issues/126)) ([87daa5a](https://github.com/fluencelabs/spell/commit/87daa5a8a12d35c6ea1169bdb2df0c94268432b5))

## [0.5.12](https://github.com/fluencelabs/spell/compare/spell-v0.5.11...spell-v0.5.12) (2023-05-05)


### Features

* **installation-spell:** use new blueprint ([#124](https://github.com/fluencelabs/spell/issues/124)) ([fae1552](https://github.com/fluencelabs/spell/commit/fae1552ee4dab071ff419474b191130095ed8e35))


### Bug Fixes

* **installation-spell:** better error reporting ([#121](https://github.com/fluencelabs/spell/issues/121)) ([f5d2f4d](https://github.com/fluencelabs/spell/commit/f5d2f4d90f924e6df53ba386a021889e3d85ff89))

## [0.5.11](https://github.com/fluencelabs/spell/compare/spell-v0.5.10...spell-v0.5.11) (2023-04-20)


### Bug Fixes

* **installation-spell:** fix compile errors in installation spell ([#117](https://github.com/fluencelabs/spell/issues/117)) ([7a88db1](https://github.com/fluencelabs/spell/commit/7a88db102da3ec0843dd6c8d5a89e071838342f3))

## [0.5.10](https://github.com/fluencelabs/spell/compare/spell-v0.5.9...spell-v0.5.10) (2023-04-13)


### Features

* **deal:** rate limit registry calls to 12h ([#108](https://github.com/fluencelabs/spell/issues/108)) ([71f43ed](https://github.com/fluencelabs/spell/commit/71f43ed36fe010df3e29200520feca783db65c2b))
* **sqlite:** bump sqlite up to v0.18.1 ([#107](https://github.com/fluencelabs/spell/issues/107)) ([71a735b](https://github.com/fluencelabs/spell/commit/71a735bf18d35f6f691cb67b0541c53578cdc11d))


### Bug Fixes

* **deps:** update dependency @fluencelabs/registry to v0.8.3 ([#101](https://github.com/fluencelabs/spell/issues/101)) ([540469a](https://github.com/fluencelabs/spell/commit/540469a471afd2df53b781716b189d7a998fd26b))
* **deps:** update rust crate marine-sqlite-connector to 0.8.0 ([#105](https://github.com/fluencelabs/spell/issues/105)) ([bbfc536](https://github.com/fluencelabs/spell/commit/bbfc53651664868244b45f223e7ede0f4a8c9025))

## [0.5.9](https://github.com/fluencelabs/spell/compare/spell-v0.5.8...spell-v0.5.9) (2023-04-07)


### Bug Fixes

* **direct-hosting:** do not set alias "worker-spell" directly ([#97](https://github.com/fluencelabs/spell/issues/97)) ([ef27f55](https://github.com/fluencelabs/spell/commit/ef27f55987ffa703d02d77fbfda0ee758dd92b3d))

## [0.5.8](https://github.com/fluencelabs/spell/compare/spell-v0.5.7...spell-v0.5.8) (2023-04-07)


### Bug Fixes

* **cli:** create worker-spell if not exists ([#96](https://github.com/fluencelabs/spell/issues/96)) ([9162af8](https://github.com/fluencelabs/spell/commit/9162af86dad5f5eb2ffd8b93cdc9c5e4d694213b))
* run PeerSpell.list test on worker ([#92](https://github.com/fluencelabs/spell/issues/92)) ([6d2fa89](https://github.com/fluencelabs/spell/commit/6d2fa89ff6e1f50dc34f734f18f5b839c12f851f))

## [0.5.7](https://github.com/fluencelabs/spell/compare/spell-v0.5.6...spell-v0.5.7) (2023-03-27)


### Features

* add get_logs for deal workers [NET-425] ([#90](https://github.com/fluencelabs/spell/issues/90)) ([195f8fd](https://github.com/fluencelabs/spell/commit/195f8fdd5f6af8fe5dae67369945e1a41581e9a1))

## [0.5.6](https://github.com/fluencelabs/spell/compare/spell-v0.5.5...spell-v0.5.6) (2023-03-21)


### Bug Fixes

* update rust toolchain ([#88](https://github.com/fluencelabs/spell/issues/88)) ([7ba22bf](https://github.com/fluencelabs/spell/commit/7ba22bf92d254da3de587359685ec00e8ad33e15))

## [0.5.5](https://github.com/fluencelabs/spell/compare/spell-v0.5.4...spell-v0.5.5) (2023-03-21)


### Features

* **installation-spell:** deploy spells [NET-410] ([#85](https://github.com/fluencelabs/spell/issues/85)) ([16276d9](https://github.com/fluencelabs/spell/commit/16276d91af3f223c475f43441eae32bee7760c95))

## [0.5.4](https://github.com/fluencelabs/spell/compare/spell-v0.5.3...spell-v0.5.4) (2023-03-01)


### Bug Fixes

* **cli:** use worker_id to resolve Spell ([#77](https://github.com/fluencelabs/spell/issues/77)) ([b42a881](https://github.com/fluencelabs/spell/commit/b42a8812fe46d4422c604aa53662742e27f24c58))

## [0.5.3](https://github.com/fluencelabs/spell/compare/spell-v0.5.2...spell-v0.5.3) (2023-02-27)


### Bug Fixes

* create worker for direct hosting ([#76](https://github.com/fluencelabs/spell/issues/76)) ([5e48e37](https://github.com/fluencelabs/spell/commit/5e48e377b2a69e8e3a667fbd40d96cbb1a93663d))
* **installation-spell:** update registry ([#74](https://github.com/fluencelabs/spell/issues/74)) ([9e4081c](https://github.com/fluencelabs/spell/commit/9e4081c54498a1556b41c8fcb3e7b34f863748e3))

## [0.5.2](https://github.com/fluencelabs/spell/compare/spell-v0.5.1...spell-v0.5.2) (2023-02-24)


### Bug Fixes

* **spells:** Fix connection leak ([#70](https://github.com/fluencelabs/spell/issues/70)) ([2ddd625](https://github.com/fluencelabs/spell/commit/2ddd6250cd4abb296155d99fb9edf5e839b1b450))

## [0.5.1](https://github.com/fluencelabs/spell/compare/spell-v0.5.0...spell-v0.5.1) (2023-02-24)


### Bug Fixes

* **deal_spell:** add alias as early as possible ([#68](https://github.com/fluencelabs/spell/issues/68)) ([3836a2b](https://github.com/fluencelabs/spell/commit/3836a2b6b1a665f9599e828f357a4e8535088176))

## [0.5.0](https://github.com/fluencelabs/spell/compare/spell-v0.4.0...spell-v0.5.0) (2023-02-24)


### ⚠ BREAKING CHANGES

* **storage:** bump SQLite module to 0.18.0 to resolve mem leak. ([#66](https://github.com/fluencelabs/spell/issues/66))

### Bug Fixes

* **storage:** bump SQLite module to 0.18.0 to resolve mem leak. ([#66](https://github.com/fluencelabs/spell/issues/66)) ([fe0771d](https://github.com/fluencelabs/spell/commit/fe0771dfd4b4f741709ec0dcf44596192f04a3e4))

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
