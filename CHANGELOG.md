# Changelog

## Unreleased

## 0.1.0

- Initial release.
- `#[derive(quickcheck_derive::Arbitrary)]` emitting a native
  `quickcheck::Arbitrary` impl (`arbitrary` + `shrink`) for structs and enums.
- Container attributes: `crate`, `bound` (repeatable), `with`, `shrink`.
- Field attributes: `with`, `shrink`, `default`.
- Variant attributes: `skip`, `with`, `shrink`.
