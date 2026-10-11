# Authoring Conventions

These rules apply to any course document written with the Core, regardless of
subject. They are not about toolchain mechanics but about how to use the
toolchain well. Read them before editing `.tex` files.

## Prefer the macro over the raw symbol

If the course's notation package (loaded from `notes-project.sty` — for
example `real-analysis-notations.sty` or `qm-notations.sty`) defines a macro
for a symbol, use the macro, not the raw symbol. The toolchain's cross-PDF
links hinge on these macros resolving through the semantic-reference layer.
A raw symbol produces an output that looks identical in the PDF but drops
the link silently — the reader cannot click through to the owning document.

The rule depends on what the course's own package defines. There is no
universal macro set — `\NN`, `\abs`, `\dist`, `\sup`, `\inner`, etc. are only
available in a document that loads a package providing them. Grep the loaded
notation package(s) first; use whichever macros the course has adopted.

Illustrative examples (shape of the rule, not a universal catalogue):

| Write (if the course's notation package defines it) | Not | Reason |
|---|---|---|
| `\NN`, `\RR`, `\CC` | `\mathbb{N}`, `\mathbb{R}`, `\mathbb{C}` | A notation macro can carry an `\xref` to the owning project. |
| `\abs{x}` | `\|x\|` | The macro-generated absolute value participates in reference tracking and spacing. |
| `\norm{x}` | `\|x\|_V` | Same reason, with a hook for the norm's defining space. |
| `\sup`, `\inf`, `\lim` | `\mathrm{sup}`, `\mathrm{inf}`, `\mathrm{lim}` | Operator macros are spaced correctly and (when the package adds `\xref`) cross-linkable. |
| `\dist{x}{y}` | `d(x,y)` | A named `\dist` can be redefined per course without rewriting documents. |
| `\floor{x}`, `\ceil{x}` | `\lfloor x \rfloor`, `\lceil x \rceil` | Semantic macros survive notation changes. |
| `\inner{x}{y}` | `\langle x, y \rangle` | The inner-product macro links to its defining space. |

Writing a raw symbol instead of the course's corresponding macro is the
single most common silent clickability loss. If the course's notation package
doesn't yet cover a symbol you need, add it to the owning subject's `.sty`
rather than inlining a one-off in the document.

## Semantic IDs on everything citable

Every linkable block accepts an explicit semantic ID:

```tex
\begin{dfn}[Convergent Sequence]{convergent-sequence}
  ...
\end{dfn}
```

Give an ID to every environment that another document might cite. The ID is
stable across section renumbering, reordering, and insertion, while a visible
number like `Definition 2.1` rots the moment the document is shuffled.

In cross-references, never cite the number as the visible text. Use
descriptive prose:

```tex
% Good
See \xref[the definition of a convergent sequence]{def:convergent-sequence}.

% Bad
See \xref[Definition 2.1]{def:convergent-sequence}.
```

The number in the second form is a tripping hazard: the next edit changes it
without changing the source, and the prose silently misleads. Only use
numbered citations when `\cref` or `\Cref` generates them from the label —
those stay in sync automatically.

## Link every concept noun

For every noun in prose that names a concept — a definition, theorem,
lemma, axiom, hypothesis, operator, named construction — do one of two
things on its first mention in the passage:

- If the concept is defined somewhere in the corpus, wrap the noun in an
  `\xref` to the owning block.
- If nothing in the corpus defines it yet, mark it with
  `\missingprereq{<name>, <owning subject>}`.

```tex
% Linked — a definition exists at lec02::def:open-set
An \xref[open set]{lec02::def:open-set} contains an open neighbourhood …

% Missing prerequisite — Hahn–Banach is owed by functional-analysis
We will later invoke \missingprereq{Hahn–Banach, functional-analysis}.
```

The rule applies to the **first** mention of a term in a passage. Repeat
mentions of the same term in the same document do not need re-linking.

A reader scanning the rendered PDF should see a visible signal for every
load-bearing noun — a clickable link to the owning definition, or a purple
`\missingprereq` marker that this concept is still owed from another
repository. Unlinked, unmarked nouns are the main class of silent broken
paths: the author who already knows the concept doesn't notice the gap,
and the reader is left with no breadcrumb back to the source.

If writing a passage and unsure whether a definition exists, search the
corpus (`grep -r 'def:<slug>' .` or scan the owning subject's
`latex_target/`); add the `\xref` if the target exists, otherwise reach for
`\missingprereq` and keep moving.

## Direct derivation over black-box invocation

When a result follows from an earlier theorem in one or two sentences, write
the direct derivation instead of invoking the theorem by name. Future readers
should not need to look up another numbered result to follow this one.

Invoke by name when the dependency is heavyweight or long. Inline the
reasoning otherwise. The deciding question: can a reader who skipped the
referenced result still follow this one in the local prose? If yes, inline.

A common anti-pattern: a one-line proof reduced to "by Theorem 3.4," where
Theorem 3.4 is itself a one-line result. The reader now follows two
indirections for what could have been one sentence.

## Narrative order

Prefer the sequence **definition → theorem → proof** when a result follows
directly from a new concept. Introduce the concept (`dfn`) first, state the
claim (`thm`/`lem`/`prop`/`cor`) second, then prove it. Keep remarks,
examples, and conventions near the theorem they illuminate rather than
scattered across the document. The reader should encounter every term the
next linkable block uses before meeting that block.

## One concept per definition

Each `dfn` introduces exactly one noun. If a definition reads "a sequence is
convergent, and its limit operator is defined by…", split it:

```tex
\begin{dfn}[Convergent Sequence]{convergent-sequence}
  ...
\end{dfn}

\begin{dfn}[Limit Operator]{limit-operator}
  For a \xref{def:convergent-sequence}, define ...
\end{dfn}
```

Grouping two concepts in one `dfn` wastes a semantic ID (only one noun gets
the stable reference) and forces readers to parse a compound definition when
they wanted only the simpler piece. The same rule applies to `thm`, `lem`,
`prop`: one claim per environment.

Exception: a definition that genuinely introduces one object with one name
but names its two components (e.g. "a metric space is a pair `(X, d)` where
`X` is the underlying set and `d` is the metric") is one noun with named
parts, not two concepts.

## Leave a redirect when a block moves

When a concept moves to a **different document**, replace the original site
with `\redirect` — do not delete the block. See [core.md](core.md) and
[blocks.md](blocks.md) for the mechanism.

Scope: `\redirect` is only for cross-document moves. For a block that just
reorders within the same document (e.g. moving a definition from §2 to §5
for better narrative flow), the local label stays valid and no breadcrumb
is needed — `\cref` and `\label` already resolve.

Deleting the block when it moves across documents silently breaks every
`\xref` that still targets the old location and shifts the counter so every
subsequent number changes. `\redirect` fixes both: the hypertarget and
label survive at the old location, the counter still advances at the old
slot, and a reader who follows a stale link lands on a visible breadcrumb
pointing to the current definition.

### When to remove a redirect

Keep `\redirect` indefinitely by default — the breadcrumb has near-zero
cost. Removing one requires confirming **both**:

1. **No remaining inbound citations.** Because the toolchain's cross
   references form a cross-subject web, the audit has to span every course
   that could cite this block, not just the one you're editing. Grep for
   `\xref{...::<old-kind>:<old-id>}` across every dependent course
   checkout you have access to. If the block was moved from a widely-cited
   location, keep the redirect — you cannot grep courses you don't have.
2. **The redirect carries no additional information worth preserving.**
   If the redirect's `[Original Title]` or clickable breadcrumb helps a
   reader orient themselves (for example, after a substantial rename),
   that reader value is reason enough to keep it.

If either condition is uncertain, keep the redirect. Grep for `\\redirect`
to list the current tombstones in a document.

### Optional title

`\redirect[Original Title]{old-kind}{old-id}{new-xref-target}` — the title
in brackets is optional. Include it when the block was renamed during the
move so readers can tell what moved. Omit it when the name is unchanged
(the clickable new-location breadcrumb carries enough context on its own).

## Authoring markers

The Core ships two inline work-in-progress markers. See
[core.md](core.md) for their definition; this section covers *when* and *how*
to use them.

- `\todo` — standalone marker for a spot the current author owes themselves.
  Use when the location matters but the work is self-explanatory
  (e.g. "clean up this proof").
- `\todo{short description}` — same, with a brief note. Use when the next
  action is non-obvious or needs context for future-you.
- `\missingprereq{text}` — material owed by a different subject that is not
  yet available. The text should name both the missing concept and the
  owning subject (e.g. `\missingprereq{Hahn–Banach, functional-analysis}`)
  so a reader can locate the dependency. See "Link every concept noun" for
  the rule that decides between this and `\xref`.

Use `\todo` for work the current author can complete alone, and
`\missingprereq` for cross-subject blockers. The color split lets a reader
see at a glance which items will unblock from inside this document and
which require another repository to move first. Grep for `\\todo` to list
every outstanding item in a document; grep for `\\missingprereq` for the
cross-subject blocker report.

Both macros are suppressable for a release build:

```tex
\renewcommand{\todo}[1][]{}
\renewcommand{\missingprereq}[1]{}
```

## Verification

Before saving a document for a commit, run these checks on every added or
edited passage:

1. **Notation coverage.** For every mathematical symbol, operator, or named
   quantity, confirm it is produced by a macro from the course's notation
   package, not raw LaTeX. Grep for common raw-symbol anti-patterns:
   `\\mathbb`, `\\mathrm{sup|inf|lim|...}`, `\\lfloor`, `\\lceil`,
   `\\langle`, bare `|…|` for absolute values. If a needed macro does not
   exist, add it to the owning subject's notation `.sty` rather than
   inlining a one-off.

2. **Link coverage.** For every concept noun in the passage, confirm the
   first mention is either wrapped in `\xref` with the right target or
   marked with `\missingprereq`. See "Link every concept noun" for the
   affirmative rule. Subsequent mentions of the same term in the same
   document do not need re-linking. A linkable term left bare — neither
   `\xref` nor `\missingprereq` — silently loses the clickable path a
   reader would otherwise follow.

3. **Link accuracy.** For every existing `\xref` in the passage, open the
   target page or `kind:id` and confirm it still names the concept the
   surrounding sentence refers to. A concept that has moved should be
   followed through its redirect to its current location, and the `\xref`
   updated to point directly at the new target rather than relying on the
   tombstone.

4. **Compile and skim.** Build the PDF with `latexmk -pdf`. Confirm no
   `undefined reference` warnings and that every cross-reference link lands
   on the right destination. Also grep once for bare-number citations
   (`\\xref\[.*(Thm|Def|Prop|Lem|Cor) [0-9]`) — printed numbers do not
   belong as `\xref` visible text; descriptive prose does.
