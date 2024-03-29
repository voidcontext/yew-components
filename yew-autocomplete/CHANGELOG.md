# Changelog
All notable changes to this project will be documented in this file. See [conventional commits](https://www.conventionalcommits.org/) for commit guidelines.

- - -
## yew-autocomplete-v0.3.1 - 2023-12-04
#### Bug Fixes
- propagate props updates to internal state (#7) - (c00d044) - Gabor Pihaj

- - -

## yew-autocomplete-v0.3.0 - 2023-11-25
#### Build system
- upgrade nix-rust-utils to v0.8.0 (#3) - (c391428) - Gabor Pihaj
#### Miscellaneous Chores
- **(version)** yew-commons-v0.1.1 - (127c3bb) - Gabor Pihaj
- upgrade: yew to 0.21, nix-rust-utils to v0.10.0 (#5) - (324351f) - Gabor Pihaj

- - -

## yew-autocomplete-v0.2.0 - 2023-07-05
#### Bug Fixes
- don't require exact version of wasm-bindgen - (e97b643) - Gabor Pihaj
#### Refactoring
- replace FnProp with Callback - (2258075) - Gabor Pihaj
- Move more responsibilty into the state: replace dispatcher with a simple Callback - (5d6d4b1) - Gabor Pihaj
- Move more responsibilty into the state: storing items after resolving - (75bddd7) - Gabor Pihaj
- Refactor views into function components - (0c53f49) - Gabor Pihaj

- - -

## yew-autocomplete-v0.1.1 - 2023-06-07
#### Bug Fixes
- clippy warnings - (87f767b) - Gabor Pihaj
- clean up state after selecting an item - (3fdf4bc) - Gabor Pihaj
- search button in bulma view when auto is false - (881149a) - Gabor Pihaj
#### Documentation
- Add autocomplete README + add example in module documentation - (306c6e8) - Gabor Pihaj
- update crate changelogs, since conventional commits weren't used from the beginning - (b2f720c) - Gabor Pihaj
#### Miscellaneous Chores
- bump yew-autocomplete version manually - (fad40b4) - Gabor Pihaj

- - -

## yew-autocomplete-v0.1.0 - 2023-06-06

Initial release, with the following functionality:

- Autocomplete component
  - multi select option
  - configurable non automatic completion / search
  - show / hide selected items
- 2 view implementation
  - plain using simple html list
  - bulma css

- - -

Changelog generated by [cocogitto](https://github.com/cocogitto/cocogitto).