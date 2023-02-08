# Changelog

### ⚠ BREAKING CHANGES

* **errors:** SpellValueT::get_error -> SpellValueT::take_error to get rid of `.clone()`

### Features

* **kv:** add exists method [NET-301] ([#20](https://github.com/fluencelabs/spell/issues/20)) ([360d5ea](https://github.com/fluencelabs/spell/commit/360d5eade111fae5ce4a8835f33dada464c9fc32))
* add serializable to TriggerConfig ([#8](https://github.com/fluencelabs/spell/issues/8)) ([e1b0e28](https://github.com/fluencelabs/spell/commit/e1b0e2855b23d0457c92245b1a5f7c24b5cb6ac2))
* **errors:** SpellValueT::get_error -&gt; SpellValueT::take_error ([#21](https://github.com/fluencelabs/spell/issues/21)) ([531d856](https://github.com/fluencelabs/spell/commit/531d856a91ac60bd3e72841dc86feafbd6f7cb46))
* set `absent` flag when key is not in KV ([#19](https://github.com/fluencelabs/spell/issues/19)) ([a62e03e](https://github.com/fluencelabs/spell/commit/a62e03e6c681add41b67d6461fd729cd324955f4))

### Bug Fixes

* add doc-comment for spell-distro::modules ([#9](https://github.com/fluencelabs/spell/issues/9)) ([501432f](https://github.com/fluencelabs/spell/commit/501432fd7774a2a77211b77f475fc05d9ae2f2b8))
