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
   - **Core** — read [references/core.md](references/core.md) for shared LaTeX
     environments, semantic IDs, proofs, and `\xref`.
   - **Project Config** — read
     [references/project-config.md](references/project-config.md) for per-course
     LaTeX identity and consistency rules.
   - **Template** — read [references/template.md](references/template.md) for
     repository scaffolding, builds, manifests, and course-side CI.
   - **Server** — read [references/server.md](references/server.md) for project
     registration, synchronization, routes, and deployment.
3. Make the smallest change in the owning component. Update dependent
   components only when their contract changes.
4. Verify in proportion to the change and report generated artifacts.

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
