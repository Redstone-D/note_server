# Component 2: Project Config

## Ownership

Use `notes-project.sty` for the two values required by the Core:

```tex
\NeedsTeXFormat{LaTeX2e}
\ProvidesPackage{notes-project}[YYYY/MM/DD Course project information]

\newcommand*{\notesServerRoot}{https://notes.example.com}
\newcommand*{\notesProjectName}{project-id}

\RequirePackage{notes-common}
```

Keep shared formatting and reference parsing in `notes-common.sty`. Keep Git
repository URLs, deployment secrets, and notes-server registry JSON out of
`notes-project.sty`.

Load optional course-specific notation packages after the Core. Keep their
implementations in separate files rather than growing `notes-project.sty`:

```tex
\RequirePackage{notes-common}
\RequirePackage{qm-notations}
```

When a portable notation package exposes a reference-target hook with
`\providecommand`, set a project-specific destination before loading it:

```tex
\RequirePackage{notes-common}
\newcommand*{\commutatorRef}{https://other.example::quantum-mechanics::ch03::def:commutator}
\RequirePackage{qm-notations}
```

For a portable subject package, make its default target project-qualified, such
as `quantum-mechanics::ch03::def:commutator`. A dependent course can then load
the package without changing the link. Override only when the owning project
or server differs.

Treat subject packages as optional dependencies. Vendor or otherwise track the
needed `.sty` file inside the dependent repository; do not use `../` paths into
a sibling course checkout.

## Identity contract

Keep the following values equal where they represent the same project:

| Location | Value |
|---|---|
| `notes-project.sty` | `\notesProjectName` |
| course `latex_target/manifest.json` | `project` |
| repository checkout beneath `NOTES_SOURCES_ROOT` | folder name |
| server `config/<project>/config.json` | directory name and `project` |
| server `config/<project>/manifest.json` | `project` |
| public URL | first path segment after the host |
| CI workflow | project environment value and deploy path |

Use the public origin without a trailing slash for `\notesServerRoot`.

## Verification

Search all identity-bearing files for placeholders and mismatched project IDs.
Compile a document containing an external `\xref` and inspect the emitted URL.
Do not create server-owned configuration inside a course repository; report the
server-side registration as a separate required action when it is out of scope.
