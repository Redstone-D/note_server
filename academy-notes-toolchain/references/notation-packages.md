# Notation Packages

A notation package is defined in [core.md](core.md). This page is the operational
catalogue: how to author one, where it lives, and how its macros carry cross
references back to their defining linkable blocks.

For the companion authoring rule — when writing a document, prefer the
macro over the raw symbol — see [authoring.md](authoring.md).

## Where it lives

A notation package is a per-subject `.sty` file loaded from
`notes-project.sty`, after `notes-common.sty`:

```tex
\RequirePackage{notes-common}
\RequirePackage{real-analysis-notations}
```

Keep subject-specific symbols out of `notes-common.sty`. The Core is
project-neutral and shared across courses; notation packages are vendored
per course.

Treat subject notation packages as optional portable modules. A dependent
course should vendor the file into its own repository and `\RequirePackage`
it by name. Do not use `../` paths into a sibling checkout — they are not
portable across machines or build environments.

## Linking a macro to its defining block

A macro should carry an `\xref` to the linkable block that defines the symbol
or operator. The target is a destination like `project::doc::kind:id` — see
[cross-references.md](cross-references.md) for scope syntax.

Expose the destination as a `\providecommand` hook so a dependent project can
retarget the macro before loading the package:

```tex
\providecommand*{\commutatorRef}{quantum-mechanics::ch03::def:commutator}
\newcommand{\comm}[2]{%
  \ensuremath{%
    \xref[\operatorname{comm}]{\commutatorRef}\!\left(#1,\, #2\right)%
  }%
}
```

Any valid `\xref` scope is accepted in the hook. The Core expands the
mandatory `\xref` argument before parsing it, so a target macro behaves the
same as a literal reference.

Prefer a project-qualified default (such as
`quantum-mechanics::ch03::def:commutator`) when a dependent subject may reuse
the package. The imported notation stays linked to the owning subject
unless a downstream course deliberately overrides the hook.

## What to link

Link only the stable symbol or operator head, not the full expression.
Keeping arguments outside the link avoids nested hyperlinks when an argument
itself contains another linked symbol.

Prefer new semantic commands over replacing standard LaTeX operators. When a
replacement is intentional, preserve the existing signature and use an
explicit renewal command so a missing or conflicting definition fails
visibly.

## Verification

Compile a document that exercises every notation macro the package exports.
Verify the clickable head lands on the owning block, that the hook override
redirects correctly in a project that sets a different target, and that no
nested link annotations appear in the generated PDF.
