# Changelog

Notable changes to this project should be documented in this file.
Make sure it is up to date before performing a release.

This project follows the [Keep a Changelog](https://keepachangelog.com/en/2.0.0/) format wherever that is reasonable.

The "title" of each release should be its first line.
A title is required for publishing a github release, so all versions should have one.

Most changes include the relevant [jj](https://jj-vcs.dev) change ids in parens. An example of a change id is wuoxvnsw.

## Unreleased

## v0.1.1 - 2026-10-20
Implement `PrettyPlease` for `TraitItem` and `ImplItem`.

### Added
- Implement `PrettyPlease` for [`TraitItem`] and [`ImplItem`]. (nzyrmtwv)

[`TraitItem`]: https://docs.rs/syn/3/syn/enum.TraitItem.html
[`ImplItem`]: https://docs.rs/syn/3/syn/enum.ImplItem.html

### Changed
- Always require the `syn/parsing` and `syn/printing` features (mxwsykzm)
  - These are enabled by default, so shouldn't meaningfully increase compile times.

## v0.1.0 - 2026-10-04
Initial release.

Tested against prettyplease v0.3.0.
