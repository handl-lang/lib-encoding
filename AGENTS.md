# lib-encoding

`encoding` — the codec **port**: `Encoder`/`Decoder`/`Encoding`/
`EncodingRegistry` traits that `lib-base64` and `lib-hex` implement.

## Layout

| path | role |
| --- | --- |
| `src/index.hnl` | the interface — op declarations + handles |
| `pkg.hnl` | manifest — `modules: ["encoding"]` |
| `scripts/hnl-gate.sh` | `.hnl` gate |
| `workflows/gate.yaml` | CI — shared `handl-gates` campaign |

## Contract

- Contract/trait repo — declare the codec surface here; concrete
  encodings are sibling packages (`lib-base64`, `lib-hex`).
- Implementors declare `encoding` as a git dep on `branch:main`.

## Verification

```bash
HNL=../host-cli/target/debug/hnl scripts/hnl-gate.sh
```

<!-- myco:agent-awareness -->

## Agent awareness

- Session start / any work discussion: run `myco status --prompt`
  before proposing next steps — know the board first.
- User describes new work → offer to capture it (`myco todo add`,
  `issue new`, `proposal new`). Don't wait to be asked.
- Picking up a task: `myco resume <ID>` is the briefing.
- Wrapping up: `myco handoff <ID>` anything resumable; keep statuses
  honest (claimed → in-progress → blocked/done).

<!-- /myco:agent-awareness -->
