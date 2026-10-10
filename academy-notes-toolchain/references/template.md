# Component 3: Template

## Ownership

Use `latex-course-repo/` as the reusable course repository template. Keep it
project-neutral and include dotfiles when copying it.

The template owns:

```text
.github/workflows/deploy.yml.template
.gitignore
.latexmkrc
.vscode/
LATEX_AUTHORING_GUIDE.md
README.md
README.txt
latex_target/manifest.json
lec01.tex
notes-common.sty
notes-project.sty
tut01.tex
```

Do not add notes-server `config/<project>/` files to this template.
Create a subject-specific package such as `qm-notations.sty` only when a course
needs reusable notation beyond the project-neutral Core.
Make such packages optional and portable: use project-qualified default
reference targets and allow dependent courses to vendor and load them.

## Instantiate a course

1. Copy the complete template, including `.github`, `.vscode`, `.gitignore`,
   and `.latexmkrc`.
2. Replace the project ID, course code, title, repository URL, branch, and
   public server origin wherever the template marks them.
3. Keep source and output names zero-padded, such as `lec01.tex` and
   `latex_target/lec01.pdf`.
4. Keep `notes-common.sty` unchanged unless updating the shared Core. Put course
   identity and optional notation-package loading in `notes-project.sty`; keep
   notation implementations in their own `.sty` file.
5. Rename `.github/workflows/deploy.yml.template` to `deploy.yml` only when
   GitHub-triggered deployment is required, then configure `NOTES_DEPLOY_KEY`
   as a repository Actions secret.

## Build and publication layout

Use `.latexmkrc` to place auxiliary files in `.build/` and final PDFs in
`latex_target/`:

```sh
latexmk -pdf lec01.tex
latexmk -pdf tut01.tex
```

Only `latex_target/` is copied into the public sandbox. A Git-based deployment
still clones or pulls the complete repository into the private sources area.
Keep the common flat publication layout:

```text
latex_target/
├── manifest.json
├── lec01.pdf
└── tut01.pdf
```

Use schema version `1` in the course manifest:

```json
{
  "schema_version": 1,
  "project": "project-id",
  "title": "Course Title"
}
```

The template intentionally tracks deployable PDFs while ignoring `.build/`
and accidental root-level PDFs.

## Authoring and CI rules

Keep the authoring guide synchronized with the Core API. For public tutorial
work, cite copyrighted exercise sources and publish the student's own answer
without copying the full question unless permission allows it.

Document course notation packages using the linked-operator pattern from the
Core reference: expose overridable semantic targets, link only the operator
head, and avoid silently replacing standard LaTeX commands.

When one subject depends on another subject's notation, track the dependency
inside the course repository rather than loading it through a sibling-relative
path. Only source packages needed to compile belong in the dependent repo; the
notes server still publishes only `latex_target/`.

The course CI workflow triggers `POST /deploy/<project>` and verifies a public
PDF. It does not compile LaTeX. Build and commit the current files beneath
`latex_target/` before relying on server-side Git synchronization.

## Verification

Search for unreplaced placeholders. Compile both starters, validate the
manifest, inspect generated PDFs, and check that the workflow names the correct
branch, project, server, secret, and verification file.
