# Cross References

A cross reference is defined in [core.md](core.md). This page is the operational
catalogue: the one macro to use, its scopes, variants, and how it resolves.

Use `\xref` for every citation, inside or outside the current PDF. The scope
is inferred from the segment count — no choice of macro at the call site.

## Scopes

| Scope | Syntax |
|---|---|
| Current PDF | `\xref{def:sample-mean}` |
| Another PDF, current project | `\xref{lec01::def:sample-mean}` |
| Another project, current server | `\xref{real-analysis::lec01::def:field}` |
| Explicit server and project | `\xref{https://other.example::project::lec01::def:item}` |

Segments are separated by `::`. The last segment is always `kind:id`
(the semantic kind and semantic ID defined in [core.md](core.md)).

## Variants

- `\xref[clickable words]{...}` — custom link text in place of the generated label.
- `\xref*{...}` — unlinked rendering (same visible form, no hyperlink).
- `\xrefuse{lec01}` — declare one document so bare short references resolve
  to it. Require explicit qualification if more than one document is declared.

See [authoring.md](authoring.md) for the "use descriptive prose, never the
printed number as the link text" rule when choosing the clickable-words
argument.

## Resolver

A non-local cross reference resolves to:

```text
<server>/<project>/<document>.pdf#nameddest=notes.<kind>.<id>
```

The named destination `notes.<kind>.<id>` is created on the destination side
by the semantic-ID machinery. The Core does not require a JSON index or a
remote AUX file to resolve any reference.

## Verification

Compile a document containing a representative local and non-local cross
reference. Inspect the generated PDF annotations and confirm each clickable
target lands on the right block. Compile twice when a reference points at a
block that is defined later in the same document, so that the label pass
completes before the link pass reads it.
