# Component 1: Core

## Ownership

Treat `latex-course-repo/notes-common.sty` as the canonical project-neutral
Core in the notes-server repository. Synchronize it into course repositories
only after comparing local copies and preserving intentional changes.

Keep course titles, course codes, project IDs, repository URLs, and server
registration out of this file.

## Numbered environments

Each environment has an independent counter beneath the current section.

| Short | Full | Semantic kind |
|---|---|---|
| `dfn` | `definition` | `def` |
| `thm` | `theorem` | `thm` |
| `axm` | `axiom` | `axm` |
| `prop` | `proposition` | `prop` |
| `lem` | `lemma` | `lem` |
| `cor` | `corollary` | `cor` |
| `rem` | `remark` | `rem` |
| `eg` | `example` | `eg` |
| `ex` | `exercise` | `ex` |
| `conv` | `convention` | `conv` |

Use `[Printed Title]{semantic-id}`:

```tex
\begin{dfn}[Sample Mean]{sample-mean}
  Definition text.
\end{dfn}
```

Require explicit IDs to use lowercase kebab-case. When the ID is omitted but a
plain title exists, derive the ID from the title. Leave an untitled environment
without an explicit ID unlabeled. Use `\notesShowIDs` and `\notesHideIDs` to
control the printed gray identifier.

Keep an untitled proof headed `Proof.` and a titled proof headed `Pf. Title.`.
Do not assign semantic IDs to proofs.

Prefer the narrative order **definition → theorem → proof** when a result
follows directly from a new concept. Introduce the concept (`dfn`) first, state
the claim (`thm`/`lem`/`prop`/`cor`) second, then prove it. Keep remarks,
examples, and conventions near the theorem they illuminate rather than
scattered across the document. The reader should encounter every term the next
environment uses before meeting that environment.

## References

Use these forms:

| Scope | Syntax |
|---|---|
| Current PDF | `\xref{def:sample-mean}` |
| Another PDF, current project | `\xref{lec01::def:sample-mean}` |
| Another project, current server | `\xref{real-analysis::lec01::def:field}` |
| Explicit server and project | `\xref{https://other.example::project::lec01::def:item}` |

Use `\xref[clickable words]{...}` for custom link text and `\xref*{...}` for
unlinked text. Use `\xrefuse{lec01}` to make short references resolve to one
declared document; require qualification if more than one document is declared.

Resolve local labels with `cleveref`. Resolve non-local references directly to:

```text
<server>/<project>/<document>.pdf#nameddest=notes.<kind>.<id>
```

Do not introduce Python, a JSON reference index, or remote AUX files for this
reference path.

## Linked notation

Before introducing a new notation macro, check the course's loaded notation
packages (`real-analysis-notations.sty`, `qm-notations.sty`, and any others
required by `notes-project.sty`) for an existing command that already covers
the symbol or operator. Reuse the existing macro so cross-document links keep
resolving to a single canonical definition. Define a new macro only when no
existing one applies, and prefer adding it to the owning subject's notation
package rather than inlining a one-off in the current document.

Keep subject-specific symbols out of `notes-common.sty`. Define them in a
course package such as `qm-notations.sty`, and use `\xref` inside the notation
macro when the symbol should link to its definition:

```tex
\providecommand*{\commutatorRef}{quantum-mechanics::ch03::def:commutator}
\newcommand{\comm}[2]{%
  \ensuremath{%
    \xref[\operatorname{comm}]{\commutatorRef}\!\left(#1,\, #2\right)%
  }%
}
```

Expose the destination as a `\providecommand` hook so another project can set
the target before loading the notation package. Accept any normal `\xref`
scope in that hook. The Core expands the mandatory `\xref` argument before
parsing it, so a target macro behaves the same as a literal reference.

Use a project-qualified default, such as
`quantum-mechanics::ch03::def:commutator`, when dependent subjects may reuse
the package. This keeps imported notation linked to the owning subject.

Link only the stable symbol or operator head, not the full expression. Keeping
arguments outside the link avoids nested hyperlinks when an argument contains
another linked symbol. Prefer new semantic commands over replacing standard
LaTeX operators. When replacement is intentional, preserve the existing
signature and use an explicit renewal command so a missing or conflicting
definition fails visibly.

Keep subject modules optional. A dependent repository may vendor or otherwise
track another subject's `.sty` module and load it after `notes-common`. Do not
depend on relative paths into sibling checkouts; they are not portable across
machines or build environments.

## Authoring markers

Use these inline markers while a document is a work in progress. They render in
colored text so unfinished spots stand out at a glance; grep for the macro name
to list every outstanding item.

| Macro | Render | Meaning |
|---|---|---|
| `\todo` | blue bold "TODO" | Spot to revisit within these notes. |
| `\todo{text}` | blue bold "TODO:" then non-bold body | Dated or described TODO within these notes. |
| `\missingprereq{text}` | purple body (no bold) | Material owed from another subject but not yet available. |

Example:

```tex
We will prove this via \missingprereq{Hahn--Banach, functional-analysis::lec05}.
\todo{tighten the bound before week 6}
```

Both macros live in `notes-common.sty` so every course that loads the Core
inherits them. Use them freely during drafting. Suppress them for a release
build with:

```tex
\renewcommand{\todo}[1][]{}
\renewcommand{\missingprereq}[1]{}
```

Keep `\todo` for work the current author owes themselves, and
`\missingprereq` for cross-subject blockers; the color split lets a reader
tell at a glance which items will unblock from inside this document and which
require another repository to move first.

## Verification

Compile a small document containing every changed environment, reference, or
linked notation macro. Check its `.aux` labels and PDF link annotations, and
confirm clickable local and external destinations.
When changing the Core contract, synchronize the canonical template and every
course repository requested by the user, then compare the resulting Core files.
