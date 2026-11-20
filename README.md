# lib-encoding

`encoding` — the codec port plus the `Encoder`/`Decoder`/`Encoding`/`EncodingRegistry` traits that `lib-base64` and `lib-hex` implement.

## Layout

```text
src/index.hnl            the interface (op declarations + handles)
pkg.hnl                  package manifest
scripts/hnl-gate.sh      the gate
ci.sh                    bupha entry
```

## Gate

```bash
HNL=../host-cli/target/debug/hnl scripts/hnl-gate.sh
```

## License

MIT.
