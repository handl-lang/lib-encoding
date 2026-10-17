# lib-encoding

`encoding` — the codec port plus the `Encoder`/`Decoder`/`Encoding`/`EncodingRegistry` traits that `lib-base64` and `lib-hex` implement.

Part of the stdlib → `lib-*` split (ADR-0128): history from `stdlib/modules/encoding` (the Rust trait crate) and `stdlib/handl-packages/utils/encoding.hnl` (the registered interface).

## Layout

```text
src/index.hnl   the interface (op declarations + handles)
src/{lib,encoding,error,traits}.rs   the traits
src/ops.rs      the Rust atoms (`encoding.encode`/`decode`)
pkg.hnl         { modules: ["encoding"] }
ci.sh           bupha entry
```

**Transitional** — Phase 7 rewrites the ops as `ffi.call`.
