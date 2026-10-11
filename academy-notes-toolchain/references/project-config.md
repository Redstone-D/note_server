# Component 2: Project Config

Terms used on this page — **project**, **notes server**, **notation
package** — are defined in [core.md](core.md).

This component is strictly the LaTeX side of the course repository. For the
server-side project registration (`config/<project>/` on the notes server,
first-deploy workflow, identity contract across every location that holds
the project ID) see [Component 4: Server](server.md).

## Ownership

Use `notes-project.sty` for the two values the Core requires in order to
identify this project and the notes server that will publish it:

```tex
\NeedsTeXFormat{LaTeX2e}
\ProvidesPackage{notes-project}[YYYY/MM/DD Course project information]

\newcommand*{\notesServerRoot}{https://notes.example.com}
\newcommand*{\notesProjectName}{project-id}

\RequirePackage{notes-common}
```

Keep shared formatting and reference parsing in `notes-common.sty`. Keep
notes-server registry JSON, Git repository URLs, and deployment secrets out
of `notes-project.sty` — all three live on the server side.

Use the notes server's public origin without a trailing slash for
`\notesServerRoot`. The `\notesProjectName` value must match the project ID
the server was registered with; see the identity contract in
[server.md](server.md).

## Loading notation packages

Load any notation packages this project needs after the Core. Keep their
implementations in separate files rather than growing `notes-project.sty`:

```tex
\RequirePackage{notes-common}
\RequirePackage{qm-notations}
```

When a notation package exposes a cross-reference-target hook via
`\providecommand`, set a project-specific destination before loading it:

```tex
\RequirePackage{notes-common}
\newcommand*{\commutatorRef}{https://other.example::quantum-mechanics::ch03::def:commutator}
\RequirePackage{qm-notations}
```

See [notation-packages.md](notation-packages.md) for the authoring side of
these packages and how the hook pattern works. Keep in mind:

- Vendor a notation package's `.sty` into any course that uses it. Do not
  use `../` paths into a sibling course checkout — they are not portable.
- Prefer a project-qualified default target in the hook (such as
  `quantum-mechanics::ch03::def:commutator`) so a dependent course inherits
  the correct link without overriding anything. Override only when the
  owning project or notes server differs.

## Verification

Compile a document that exercises a cross-project or cross-server `\xref`
and inspect the emitted URL to confirm it matches
`\notesServerRoot`/`\notesProjectName`. Verifying the project is actually
registered on the notes server is a server-side concern — see
[server.md](server.md).
