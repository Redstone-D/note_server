# LaTeX Authoring and Reference Guide

This project-neutral template compiles with pdfLaTeX. It requires no Python,
JSON reference index, or remote AUX files.

## Start and build a document

Copy the nearest starter and keep two-digit filenames:

```text
lec01.tex  -> latex_target/lec01.pdf
tut01.tex  -> latex_target/tut01.pdf
```

Every document loads the two-layer library:

```tex
\documentclass[12pt]{article}
\usepackage{notes-project}
```

- `notes-common.sty` is reusable and project-neutral.
- `notes-project.sty` supplies the public server root and project ID.

Build from VS Code with **Build LaTeX project**, or run:

```sh
latexmk -pdf lec01.tex
```

Auxiliary files go to `.build/`; the PDF goes to `latex_target/`.

## Numbered environments

Ordinary `\section` headings provide the first number. Each content type
has an independent counter, so a section can contain both Definition 2.1 and
Proposition 2.1.

| Short form | Full form | Reference kind |
|---|---|---|
| `dfn` | `definition` | `def` |
| `thm` | `theorem` | `thm` |
| `prop` | `proposition` | `prop` |
| `lem` | `lemma` | `lem` |
| `cor` | `corollary` | `cor` |
| `rem` | `remark` | `rem` |
| `ex` | `example` | `ex` |
| `exc` | `exercise` | `exc` |
| `conv` | `convention` | `conv` |

Give an item an optional printed title and a stable semantic ID:

```tex
\begin{dfn}[Sample Mean]{sample-mean}
  Definition text.
\end{dfn}
```

This creates `def:sample-mean`. If the ID is omitted, a plain-text title is
converted to lowercase kebab-case. Prefer an explicit ID when the title
contains LaTeX or when a shorter name avoids an overfull heading.

IDs appear as small gray monospace text by default. Use `\notesHideIDs` or
`\notesShowIDs` in the preamble to control them.

## Proofs

```tex
\begin{proof}
  ...
\end{proof}

\begin{proof}[Uniqueness]
  ...
\end{proof}
```

An untitled proof prints “Proof.” A titled proof prints “Pf. Uniqueness.”
Proof environments do not take semantic IDs.

## References in the current PDF

```tex
See \xref{def:sample-mean}.
Apply the \xref[sample-mean definition]{def:sample-mean}.
```

The optional text makes ordinary words clickable. The starred form,
`\xref*{def:sample-mean}`, prints the same reference without a link.

## References to other PDFs

Reference forms progress from the current project to an explicit server:

| Scope | Syntax |
|---|---|
| Current project | `\xref{lec01::def:sample-mean}` |
| Another project on the same server | `\xref{other-project::lec01::def:item}` |
| Another server and project | `\xref{https://other.example::other-project::lec01::def:item}` |

Declare a frequently used document once:

```tex
\xrefuse{lec01}
See \xref{def:sample-mean}.
```

A local reference displays its current number via `cleveref`. A
cross-PDF reference displays its semantic name because it does not fetch
remote metadata. The link targets the stable semantic destination embedded in
the other PDF.

## Exercises and source citations

For publicly hosted tutorial work, cite the source and write the original
answer without reproducing a full copyrighted question:

```tex
\begin{exc}{author-book-exercise-id}
  \emph{Source:} Author, \emph{Book Title}, edition, Exercise 1.2.3.
\end{exc}

% Write your answer below.
```

## Publication contract

Only `latex_target/` is deployable. It must contain a valid
`manifest.json`; built documents may be flat:

```text
latex_target/
├── manifest.json
├── lec01.pdf
└── tut01.pdf
```

The manifest project ID, `\notesProjectName`, repository checkout name,
server registration, and public URL segment must agree. Server-side
configuration is deliberately outside this LaTeX template.
