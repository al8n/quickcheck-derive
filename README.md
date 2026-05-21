<div align="center">
<h1>quickcheck-derive</h1>
</div>
<div align="center">

A `#[derive(Arbitrary)]` proc-macro that emits a **native**
[`quickcheck::Arbitrary`] implementation — both `arbitrary` **and** a real
`shrink` — for structs and enums.

</div>

Unlike bridges that route generation through `arbitrary::Unstructured` byte
buffers, this derive produces a genuine quickcheck impl that calls
`Arbitrary::arbitrary` / `Arbitrary::shrink` directly on the fields.

This crate does **not** depend on `quickcheck` itself (only as a dev-dependency
for its own tests). The generated code refers to *your* quickcheck via the
`crate` attribute, defaulting to `::quickcheck`, so consumers bring their own.

## Usage

```rust
use quickcheck_derive::Arbitrary;

#[derive(Clone, Debug, Arbitrary)]
struct Point {
    x: i32,
    y: i32,
}
```

The generated impl is wrapped in an anonymous `const _: () = { … };` for hygiene
and provides:

```rust,ignore
fn arbitrary(g: &mut Gen) -> Self;
fn shrink(&self) -> Box<dyn Iterator<Item = Self>>;
```

`shrink` shrinks **one field at a time**, holding the others at their current
value, and chaining the resulting iterators.

## Attribute surface

All attributes live under the `quickcheck` path: `#[quickcheck(...)]`.

### Container (on the struct/enum)

| Attribute | Meaning |
|-----------|---------|
| `crate = "path::to::quickcheck"` | Base path for the emitted `Arbitrary` / `Gen`. Default `::quickcheck`. |
| `bound = "P: Bound, Q: Other"` | **Repeatable.** If *any* `bound` is present, the generated `where` clause = the type's own predicates **plus exactly** these. If none is present, infers `Param: Arbitrary` per generic **type** param. |
| `with = "fn"` | Override generation of the whole value: `fn(&mut Gen) -> Self`. |
| `shrink = "fn"` | Override shrinking of the whole value: `fn(&Self) -> Box<dyn Iterator<Item = Self>>`. |

`with` / `shrink` are independent. `with` without `shrink` ⇒ shrink is empty.
`shrink` without `with` ⇒ generation is still field/variant-derived.

> **Note:** explicit `bound` *replaces* the inferred `Arbitrary` bounds. Since
> `quickcheck::Arbitrary: Clone + 'static`, you usually need to include `'static`
> (and `Clone`) in a custom bound, e.g. `bound = "T: Clone + Default + 'static"`.

### Field (struct fields, and fields of struct/tuple variants)

| Attribute | Meaning |
|-----------|---------|
| `with = "fn"` | Generate this field via `fn(&mut Gen) -> FieldT`. |
| `shrink = "fn"` | Shrink this field via `fn(&FieldT) -> Box<dyn Iterator<Item = FieldT>>`. |
| `default` | Generate via `Default::default()`; the field is held constant when shrinking. |

Shrink rule per field: `shrink` attr → that fn; plain → `Arbitrary::shrink`;
`with`-without-`shrink` **or** `default` → held constant (not shrunk).

### Variant (enum variants)

| Attribute | Meaning |
|-----------|---------|
| `skip` | Exclude from `arbitrary` selection. A value that *is* this variant shrinks to empty. If **every** variant is `skip`, a `compile_error!` is produced. |
| `with = "fn"` | Generate the whole `Self` value as this variant: `fn(&mut Gen) -> Self`. Takes **precedence** over the variant's field attributes. |
| `shrink = "fn"` | Shrink a value of this variant. `with`-without-`shrink` ⇒ empty for that variant. |

`arbitrary` picks uniformly among the non-skipped variants via `g.choose`.

## Codegen summary

* **Struct** — `arbitrary` builds the struct literal (each field per its rule);
  `shrink` clones `self`, assigns one shrunk field at a time, and chains.
* **Enum** — `arbitrary` does `match *g.choose(&[indices]).unwrap()` over the
  non-skipped variants; `shrink` matches the current variant and rebuilds it
  explicitly with one field shrunk and the others cloned.

## License

Licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
* MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.

[`quickcheck::Arbitrary`]: https://docs.rs/quickcheck/latest/quickcheck/trait.Arbitrary.html
