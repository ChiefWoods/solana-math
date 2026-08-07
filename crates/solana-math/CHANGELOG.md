# Changelog

## [0.2.1] - 2026-08-07
### Fixed

- Use no-std by default

## [0.2.0] - 2026-08-06
### Added

- Add brine-fp support

## [0.1.6] - 2026-08-06
### Changed

- Depend on git \`quasar-lang\` (master) so optional Quasar integrations can enable \`idl-build\`.

## [0.1.5] - 2026-08-06
### Added

- Add optional `anchor` and `quasar` features with `SafeMathError` → `ProgramError` conversions. The former `zeropod` feature is folded into `quasar`.

## [0.1.4] - 2026-08-05
### Added

- Add optional `zeropod` feature for POD integer safe math.

### Changed

- Rename crate from `solana-safe-math` to `solana-math`.

## [0.1.3] - 2026-06-08
- Bumped `rust_decimal` to 1.42.0.

## [0.1.2] - 2026-05-31
- Added top-level crate and error module documentation comments.

## [0.1.1]
- Replaced `cargo-husky` with `husky-rs` for local Git hook management.

## [0.1.0]
- Initial release.
