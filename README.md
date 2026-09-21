# sphered

the SpherePOP ASCII DSL — an evaluator that follows inward and returns
outward, where every symbol is a distinct evaluator motion and every
`<(...)>` is an admissible, witnessed transition. no dependencies.

## the alphabet

| symbol | motion |
|---|---|
| `( )` | enter / return — pure nested evaluation |
| `<( )>` | open a bounded transition — a staged frame |
| `?` | admit — predicate on pre-state and proposal |
| `~` | transform — pure change to the staged store |
| `!` | verify — predicate on before-and-after |
| `@` | attribute — provenance for the pending record |
| `>` | commit — atomically install the staged store and record |

`#` (persist), `|` (bind) are extensions, kept out of the core proofs.

## well-formedness

a transaction's clauses must match `?* ~* !+ @+ >` — admissions, then
transforms, then at least one verification, then at least one
attribution, then exactly one commit, last. the four negative cases
(admission after transform, verification after commit, no verification,
two commits) are rejected with a reason.

## refusal

a predicate that returns the reserved atom `x` refuses the frame: the
transition stops, no witness is emitted, and `refused, no witness` is the
record. refusal is not absence.

## run

```bash
cargo run -- --eval '<( event ? allowed ~ process ! chk @ sensor > done )>'
cargo test
```

## the spine

`#Commit = #ValidWitness` — no state change without a witness, no witness
without a state change. representation is not referent; proposal is not
realization; evaluation is not exposure; persistence is not truth.

*sphered · the spherepop ascii dsl · follow inward, return outward · the
constellation · fine touch from within · vaked.dev*
