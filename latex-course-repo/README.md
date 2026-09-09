# Generic LaTeX Course Repository Template

This directory is a project-neutral template for a course source repository
published by the notes server. Copy its contents—including dotfiles—into a new
repository, then replace these placeholders:

| Placeholder | Replace with |
|---|---|
| `project-id` | Stable lowercase project ID, such as `example-course` |
| `Course Title` | Public course title |
| `COURSE-CODE` | Course code shown in PDFs |
| `https://notes.example.com` | Public notes-server root |
| `https://github.com/OWNER/REPOSITORY.git` | Course repository URL |

## What belongs here

The template contains only source-repository concerns:

- reusable LaTeX formatting and semantic references;
- project-value placeholders;
- lecture and tutorial starters;
- local and VS Code build configuration;
- the deployable `latex_target/manifest.json`;
- an optional GitHub Actions trigger template.

It intentionally contains no notes-server `config/<project>/config.json`.
Registering a project and defining server-side configuration are responsibilities
of the notes-server application. After creating a course repository, register
the same project ID, title, repository URL, and deployment script separately in
the application.

## Quick start

1. Copy the whole directory into an empty course repository.
2. Replace every placeholder listed above.
3. Rename `.github/workflows/deploy.yml.template` to `deploy.yml` if
   GitHub-triggered deployment is wanted.
4. Open `lec01.tex` or `tut01.tex` and run **Build LaTeX project**.
5. Confirm the PDFs appear in `latex_target/`.
6. Register the project separately in the notes-server application.

Read `LATEX_AUTHORING_GUIDE.md` for environments and cross-references.
Read `README.txt` for a plain-text setup specification suitable for
copying into another program or coding-agent prompt.

## Resulting layout

```text
course-repository/
├── .github/workflows/deploy.yml.template
├── .gitignore
├── .latexmkrc
├── .vscode/
├── LATEX_AUTHORING_GUIDE.md
├── README.md
├── README.txt
├── notes-common.sty
├── notes-project.sty
├── lec01.tex
├── tut01.tex
└── latex_target/
    └── manifest.json
```
