# Changelog

## [0.2.0](https://github.com/fluencelabs/spell/compare/spell-v0.1.2...spell-v0.2.0) (2023-01-10)


### ⚠ BREAKING CHANGES

* **errors:** SpellValueT::get_error -> SpellValueT::take_error to get rid of `.clone()`

### Features

* **errors:** SpellValueT::get_error -&gt; SpellValueT::take_error ([#21](https://github.com/fluencelabs/spell/issues/21)) ([531d856](https://github.com/fluencelabs/spell/commit/531d856a91ac60bd3e72841dc86feafbd6f7cb46))
* **kv:** add exists method [NET-301] ([#20](https://github.com/fluencelabs/spell/issues/20)) ([360d5ea](https://github.com/fluencelabs/spell/commit/360d5eade111fae5ce4a8835f33dada464c9fc32))
* set `absent` flag when key is not in KV ([#19](https://github.com/fluencelabs/spell/issues/19)) ([a62e03e](https://github.com/fluencelabs/spell/commit/a62e03e6c681add41b67d6461fd729cd324955f4))

## 0.1.2 (2023-01-10)


### ⚠ BREAKING CHANGES

* **errors:** SpellValueT::get_error -> SpellValueT::take_error to get rid of `.clone()`

### Features

* **errors:** SpellValueT::get_error -&gt; SpellValueT::take_error ([#21](https://github.com/fluencelabs/spell/issues/21)) ([531d856](https://github.com/fluencelabs/spell/commit/531d856a91ac60bd3e72841dc86feafbd6f7cb46))
* **kv:** add exists method [NET-301] ([#20](https://github.com/fluencelabs/spell/issues/20)) ([360d5ea](https://github.com/fluencelabs/spell/commit/360d5eade111fae5ce4a8835f33dada464c9fc32))
* set `absent` flag when key is not in KV ([#19](https://github.com/fluencelabs/spell/issues/19)) ([a62e03e](https://github.com/fluencelabs/spell/commit/a62e03e6c681add41b67d6461fd729cd324955f4))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * fluence-spell-dtos bumped from 0.1.3 to 0.1.4
