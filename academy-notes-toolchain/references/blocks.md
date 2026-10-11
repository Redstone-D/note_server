# Linkable Blocks

A linkable block is defined in [core.md](core.md). This page is the operational
catalogue: which blocks exist, how to write one, and when to pick each.

## Catalogue

Each block has an independent counter beneath the current section.

| Short | Full | Semantic kind |
|---|---|---|
| `dfn` | `definition` | `def` |
| `thm` | `theorem` | `thm` |
| `axm` | `axiom` | `axm` |
| `hypo` | `hypothesis` | `hypo` |
| `prop` | `proposition` | `prop` |
| `lem` | `lemma` | `lem` |
| `cor` | `corollary` | `cor` |
| `rem` | `remark` | `rem` |
| `eg` | `example` | `eg` |
| `ex` | `exercise` | `ex` |
| `conv` | `convention` | `conv` |

Each short form is an alias; the full environment name is also available.

The `proof` block exists for proofs but is not a linkable block — it carries
no counter and no semantic ID.

## Syntax

Use `[Printed Title]{semantic-id}`:

```tex
\begin{dfn}[Sample Mean]{sample-mean}
  Definition text.
\end{dfn}
```

Explicit semantic IDs must be lowercase kebab-case. When the ID is omitted
but a plain title exists, the ID is derived from the title. An untitled block
without an explicit ID is left unlabeled.

Use `\notesShowIDs` and `\notesHideIDs` to control the printed gray `[kind:id]`
marker.

## Proof convention

Keep an untitled proof headed `Proof.` and a titled proof headed `Pf. Title.`.
Do not assign semantic IDs to proofs. Use [cross references](cross-references.md)
to point at the surrounding theorem, lemma, or proposition.

## Picking between `axm` and `hypo`

Both are foundational-ish posits, but they signal different intent.

Use `axm` when the course takes a statement as foundational and reasons
downstream from it — the statement is accepted; the course does not plan to
revisit whether it holds.

Use `hypo` for a named posit whose truth or applicability is being
investigated — material like Riemann's hypothesis, the ergodic hypothesis,
the exponential-time hypothesis, the canonical commutation relations when a
chapter is deriving their consequences, or any assumption whose downstream
results are explicitly conditional on it.

Reserve `rem` for commentary that does not need a counter.

## Cross-reference redirects

When a block moves to another document, replace the original site with a
redirect rather than deleting the environment. See [core.md](core.md) for
the concept.

```tex
\redirect[Original Title]{old-kind}{old-id}{new-xref-target}

% Example
\redirect[Sample Mean]{def}{sample-mean}{lec03::def:sample-mean}
```

Argument shape:

| Position | Meaning |
|---|---|
| `[Original Title]` | Optional. Displayed in italic after "Redirect" so a reader can see what moved. |
| `{old-kind}` | One of `def`, `thm`, `axm`, `hypo`, `prop`, `lem`, `cor`, `rem`, `eg`, `ex`, `conv`. Proofs are not linkable, so they cannot be redirected. |
| `{old-id}` | The original block's semantic ID. The redirect occupies `old-kind:old-id` as its hypertarget and label. |
| `{new-xref-target}` | Any valid `\xref` scope — see [cross-references.md](cross-references.md). |

Effects:

- Steps the counter corresponding to `old-kind` so subsequent numbering is
  preserved.
- Creates `\hypertarget{notes.<old-kind>.<old-id>}` so any existing
  `\xref{...::<old-kind>:<old-id>}` from another document still resolves.
- Creates `\label{<old-kind>:<old-id>}` so local `\cref` still works.
- Prints the gray `[old-kind:old-id]` marker under `\notesShowIDs`, matching
  the convention for every other linkable block.
- Renders the whole marker in italic black text with the word "Redirect"
  underlined, followed by the optional `(Title)` in parentheses and a line
  "Moved to \<clickable new reference\>."

Use `\redirect` instead of deleting a block whenever other documents might
cite the moved concept.

Compile a document containing every block type touched by the change. Verify:

- Each counter advances under the current section as expected.
- Each labeled block generates a `\label` and a named PDF destination
  matching the printed gray marker.
- `\cref` resolves to the correct printed form for every kind
  (singular and `\Cref` plural).
