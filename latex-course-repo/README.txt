GENERIC LATEX COURSE REPOSITORY SETUP SPEC
==========================================

Replace these values before using the template:

    project id: project-id
    course code: COURSE-CODE
    course title: Course Title
    repository: https://github.com/OWNER/REPOSITORY.git
    branch: master
    notes server root: https://notes.example.com

FILES
-----

notes-common.sty
    Shared, project-neutral formatting, environments, semantic IDs, and direct
    PDF cross-reference logic. Do not put course values in this file.

notes-project.sty
    The two project-specific values used by the shared package: public server
    root and project ID.

lec01.tex and tut01.tex
    Empty lecture and tutorial starters. Copy them to lec02.tex, tut02.tex,
    and so on, keeping two-digit zero padding.

.latexmkrc and .vscode/
    Build configuration. Auxiliary files go to .build/ and finished PDFs go to
    latex_target/.

latex_target/manifest.json
    Source-owned public metadata. Its project value must match notes-project.sty
    and the project ID registered by the server.

LATEX_AUTHORING_GUIDE.md
    Environment, semantic-ID, proof, exercise, and cross-PDF reference syntax.

HOW TO BUILD
------------

Open a .tex file in VS Code and run "Build LaTeX project", or run:

    latexmk -pdf lec01.tex
    latexmk -pdf tut01.tex

Only latex_target/ is deployed. Keep the published layout flat unless a course
intentionally needs named groups:

    latex_target/manifest.json
    latex_target/lec01.pdf
    latex_target/tut01.pdf

SERVER-SIDE FOLLOW-UP
---------------------

The LaTeX template does not provide or own server configuration files.
Separately register the course in the notes-server application using the same:

    project id
    display title
    Git repository URL
    deployment script

The notes-server application is responsible for its own configuration schema,
examples, validation, credentials, and restart/deployment procedure.
