---
name: academy-notes-toolchain
description: "Maintain the Academy Notes Toolchain across its four components: the project-neutral LaTeX Core, per-course Project Config, reusable course Template, and publication Server. Use when creating or migrating a course repository; editing notes-common.sty or notes-project.sty; changing numbered environments, semantic IDs, or cross-PDF references; updating latex-course-repo; registering a server project; configuring GitHub deployment; diagnosing build, link, synchronization, or publication failures; or synchronizing toolchain behavior across course repositories."
---

# Academy Notes Toolchain

Maintain a consistent path from LaTeX source to linked PDFs published by the
notes server. Treat the toolchain as four components with explicit ownership.

## Route the task

1. Inspect repository instructions, working-tree status, and the files named by
   the user before editing.
2. Read the reference for every affected component:
   - **Core** — start with [references/core.md](references/core.md) for the
     concept vocabulary (linkable block, semantic kind, semantic ID, cross
     reference, notation package, authoring marker). Then open the catalogue
     for the concept actually touched:
     - [references/blocks.md](references/blocks.md) — the linkable-block
       catalogue, syntax, proof convention, picking between kinds.
     - [references/cross-references.md](references/cross-references.md) —
       `\xref` scopes, variants, resolver.
     - [references/notation-packages.md](references/notation-packages.md) —
       authoring per-subject notation packages and linking their macros.
   - **Project Config** — read
     [references/project-config.md](references/project-config.md) for per-course
     LaTeX identity and consistency rules.
   - **Template** — read [references/template.md](references/template.md) for
     repository scaffolding, builds, manifests, and course-side CI.
   - **Server** — read [references/server.md](references/server.md) for project
     registration, synchronization, routes, and deployment.
3. If the task touches `.tex` files or notation, also read
   [references/authoring.md](references/authoring.md) for subject-neutral
   writing conventions (macro-over-raw-symbol, semantic IDs, narrative order,
   one concept per definition, direct derivation over black-box invocation).
4. Make the smallest change in the owning component. Update dependent
   components only when their contract changes.
5. Verify in proportion to the change and report generated artifacts.

## Preserve component boundaries

- Keep `notes-common.sty` project-neutral and identical wherever the Core is
  intentionally synchronized.
- Put the public server root and project ID in `notes-project.sty`. Let it load
  optional course-specific notation packages after `notes-common`, while
  keeping the notation implementations in separate `.sty` files.
- Treat subject notation packages as optional portable modules. Let dependent
  courses vendor them, and use project-qualified default references so links
  continue to target the owning subject.
- Keep server-side `config/<project>/` files out of the reusable course
  template and course repositories.
- Publish only the contents of `latex_target/`; never rely on the server seeing
  source `.tex`, build intermediates, or repository documentation.
- Keep semantic IDs stable when rewriting prose or renumbering sections.
- Preserve unrelated user changes. Compare before copying a shared file.
- Do not commit, push, or trigger a deployment unless explicitly requested.

## Validate changes

- For Core, Project Config, or Template changes, compile representative `.tex`
  documents with `latexmk -pdf` and inspect the final logs for errors,
  undefined references, and overfull boxes.
- For semantic-reference changes, compile twice when necessary and verify the
  generated labels and named PDF destinations.
- For Server changes, run the relevant Rust tests and validate representative
  `config.json` and `manifest.json` files against the implemented schema.
- Before handoff, run `git diff --check`, inspect `git status --short`, and
  distinguish pre-existing changes from changes made for the task.
