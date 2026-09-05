# Changelog

## [0.4.2] - 2026-09-06
### Changed

- Bumped anchor-lang

## [0.4.1] - 2026-08-07
### Fixed

- Use no-std by default

## [0.4.0] - 2026-08-06
### Added

- Add brine-fp support

## [0.3.3] - 2026-08-06
### Added

- Add optional `quasar` feature with `BasisPointsError` → `ProgramError` conversion. Fold `idl-build` into `anchor` / `quasar` (`anchor-lang/idl-build`, `quasar-lang/idl-build`).

## [0.3.2] - 2026-06-08
- Bumped `anchor-lang` to 1.0.2.
- Bumped `rust_decimal` to 1.42.0.

## [0.3.1] - 2026-05-31
- Added optional `codama` feature with `CodamaType` derive support for `BasisPoints`.

## [0.3.0]
- **Breaking:** `From<BasisPoints> for Decimal` now returns a proportional rate in `0..=1` (divides by `10_000`) instead of the raw basis-points count.
- **Breaking:** `TryFrom<Decimal> for BasisPoints` now expects a proportional rate and multiplies by `10_000` before validation.

## [0.1.3]
- Added checked arithmetic helpers: `checked_add`, `checked_sub`, `checked_mul`, and `checked_div`.

## [0.1.2]
- Added top-level crate and error module documentation comments.

## [0.1.1]
- Replaced `cargo-husky` with `husky-rs` for local Git hook management.

## [0.1.0]
- Initial release.
