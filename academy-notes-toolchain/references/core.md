# Component 1: Core

## Ownership

Treat `latex-course-repo/notes-common.sty` as the canonical project-neutral
Core in the notes-server repository. Synchronize it into course repositories
only after comparing local copies and preserving intentional changes.

Keep course titles, course codes, project IDs, repository URLs, and server
registration out of this file.

## Concepts

The Core introduces nine concepts that the rest of the toolchain and every
course document build on. Operational catalogues for each concept live in
sibling reference files; this page is a glossary.

### Linkable block

A container that renders a labeled statement ("Theorem 2.1", "Definition
3.2"), bumps a section-scoped counter, and acts as a target for cross
references. The eleven linkable kinds are `dfn`, `thm`, `axm`, `hypo`,
`prop`, `lem`, `cor`, `rem`, `eg`, `ex`, and `conv`. A twelfth block,
`proof`, exists for proofs but is not a linkable block — it has no counter
and no semantic ID. Full catalogue in [blocks.md](blocks.md).

### Semantic kind

The short tag classifying a linkable block's role: `def`, `thm`, `axm`,
`hypo`, `prop`, `lem`, `cor`, `rem`, `eg`, `ex`, `conv`. The kind appears
in cross-reference paths (`\xref{...::thm:example}`) and in the printed
gray marker.

### Semantic ID

The stable, lowercase-kebab-case identifier attached to a linkable block.
Survives section renumbering, reordering, and insertion. Powers the
`\label` for the local reference and the `\hypertarget` named destination
for direct PDF jumps. Full form of the printed marker is `kind:id`.

### Document

A single `.tex` source and the `.pdf` it produces, published under a
project. Identified by its filename stem (e.g., `lec01`, `tut03`). Appears
as the third segment of a cross reference's scope path.

### Project

A course-sized collection of documents on a notes server. Identified by a
project ID — the value of `\notesProjectName` on the LaTeX side and the
server-side `config/<project>/` directory name on the publication side.
Appears as the second segment of a cross reference's scope path and as the
first URL segment after the host.

### Notes server

The publication service that hosts projects. Identified by a base URL — the
value of `\notesServerRoot`. Appears as the first segment of a cross
reference's scope path when the reference is explicitly cross-server. See
[Component 4: Server](server.md) for the implementation side.

### Cross reference

A citation from one place to a linkable block. Scoped within a PDF, across
PDFs in one project, across projects, or across servers. One macro covers
all scopes: `\xref`. Full catalogue in
[cross-references.md](cross-references.md).

### Cross reference redirect

A tombstone that replaces a linkable block when the concept it defined has
been moved. Occupies the original block's semantic kind and semantic ID so
every existing cross reference pointing at that destination still resolves.
Steps the original kind's counter so numbering of subsequent blocks stays
stable. Renders a visually distinct "Redirect" marker whose body is a
clickable cross reference to the current location. See
[blocks.md](blocks.md) for the macro.

### Notation package

A per-subject `.sty` file loaded from `notes-project.sty` — for example
`real-analysis-notations.sty`, `qm-notations.sty`. Defines macros for
symbols and operators, each macro potentially carrying an internal cross
reference to its defining linkable block. The Core itself is subject-neutral;
notation packages are vendored per course. Authoring the package itself:
[notation-packages.md](notation-packages.md). Using the macros in a
document: [authoring.md](authoring.md).

### Authoring marker

An inline work-in-progress annotation that renders colored and is
grep-friendly: `\todo`, `\todo{text}`, `\missingprereq{text}`. Not counted,
not a cross-reference target. Suppressable for a release build. Catalogue in
[authoring.md](authoring.md).

## How the concepts fit together

A **document** holds **linkable blocks**. Each block declares a **semantic
kind** and an optional **semantic ID**; together they produce a stable local
label and a named PDF destination. A **project** is a set of documents, a
**notes server** is a set of projects, and a **cross reference** points at
one named destination using as many of those three scope segments as it
needs plus the `kind:id` pair. A **notation package** exports macros that
may internally wrap a cross reference, so that writing a symbol in a
document becomes a clickable path to the block where the symbol is defined.
**Authoring markers** sit outside this content graph — they are annotations
on the drafting process, not reference targets.

## Verification

When changing a concept listed on this page, synchronize the canonical
`notes-common.sty`, update the catalogue page that owns the concept's
operational detail, and compile a document that exercises the change.
Confirm every clickable local and external destination resolves. Follow the
component-specific verification note in the catalogue page that was touched.
