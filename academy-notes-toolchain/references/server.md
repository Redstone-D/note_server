# Component 4: Server

Terms used on this page — **project**, **notes server**, **document** — are
defined in [core.md](core.md).

## Ownership

Project registration lives beneath `config/<project>/` in the notes-server
repository and nowhere else. The course repository never ships server
configuration. A GitHub-backed project requires both files:

```text
config/<project>/
├── config.json
└── manifest.json
```

Use schema version `1`:

```json
{
  "schema_version": 1,
  "project": "project-id",
  "script": "github-sync",
  "script_config": {
    "repo": "https://github.com/OWNER/REPOSITORY.git"
  }
}
```

```json
{
  "schema_version": 1,
  "project": "project-id",
  "title": "Course Title"
}
```

Restart the server after adding or removing a project directory or editing
its `config.json`, because project configuration is loaded once. Central
manifest titles are read per request, so a title-only change in
`manifest.json` does not require a restart.

## Register a project

Steps to add a new project to a running notes server:

1. Create `config/<project-id>/` on the notes server. The directory name is
   the project ID; it must match the course's `\notesProjectName` and the
   `project` field inside both JSON files.
2. Drop in `config.json` and `manifest.json` using the shapes above. For
   `github-sync`, set `script_config.repo` to the course's git clone URL.
3. Restart the notes server so the new `config.json` is picked up.
4. Trigger the first deploy with
   `POST /deploy/<project-id>` carrying the `x-api-key` header. The server
   clones the repository, materializes `$NOTES_SOURCES_ROOT/<project-id>/`,
   and synchronizes `latex_target/` into `$NOTES_ROOT/<project-id>/`.
5. On the course side, set `\notesProjectName` to the same project ID and
   `\notesServerRoot` to this notes server's public origin in
   `notes-project.sty`. See [project-config.md](project-config.md).
6. On the course repository CI, set the `NOTES_DEPLOY_KEY` secret to match
   the server's environment variable so push-to-deploy works. For the
   course-side workflow file, secret binding, and trigger shape, see
   [template.md](template.md) under "Authoring and CI rules" — the
   server-side contract is simply the API key match plus the
   `POST /deploy/<project>` endpoint documented below.

## Identity contract

Keep the project ID equal across every location that references the project:

| Location | Value |
|---|---|
| server `config/<project>/` directory | directory name |
| server `config/<project>/config.json` | `project` |
| server `config/<project>/manifest.json` | `project` |
| server `$NOTES_SOURCES_ROOT/<project>/` checkout | folder name (clone target) |
| server `$NOTES_ROOT/<project>/` sandbox | folder name (synchronization destination) |
| course `notes-project.sty` | `\notesProjectName` |
| public URL | first path segment after the host |
| course CI workflow | project environment value and deploy path |

Mismatches silently break deploys or render under the wrong path. Audit
these together whenever a project is added, renamed, or migrated.

## Runtime paths and synchronization

Recognize these environment variables:

| Variable | Default | Purpose |
|---|---|---|
| `NOTES_CONFIG_ROOT` | `./config` | central project registrations |
| `NOTES_SOURCES_ROOT` | `./sources` | Git checkouts and folder-sync sources |
| `NOTES_ROOT` | `./published` | served project sandboxes |
| `NOTES_DEPLOY_KEY` | none | API key expected in `x-api-key` |
| `NOTES_SITE_TITLE` | `Notes` | site index title |

For `github-sync`, clone a missing repository into
`$NOTES_SOURCES_ROOT/<project>` or run `git pull --ff-only` in an existing
checkout. Then synchronize
`$NOTES_SOURCES_ROOT/<project>/latex_target/` into
`$NOTES_ROOT/<project>/` with deletion enabled. Never expect files outside
`latex_target/` to be published.

## HTTP behavior

- `GET /` lists registered projects using central manifests.
- `GET /<project>` renders the published file tree.
- `GET /<project>/<path>` serves a published file.
- `POST /deploy/<project>` requires `x-api-key`, rejects concurrent deployment
  of the same project, and dispatches the configured synchronization script.

The landing-page tree lists root files and one level of grouped files. It lists
only `.pdf`, `.html`, and `.png`, omits dotfiles and unsafe names, and ignores
nested directories beneath a group. These listing rules are not access
control: a direct static URL can return any canonicalized file beneath the
published project sandbox. Therefore keep only intentionally public files in
`latex_target/`.

Do not place symlinks in `latex_target/`. The current walker obtains metadata
through symlinks before checking the file type, so a symlink may be listed and
a broken symlink may make the tree walk fail. Direct serving canonicalizes the
requested path and rejects paths that resolve outside the project sandbox.

## Deployment workflow

Configure the course repository secret `NOTES_DEPLOY_KEY` to match the server
environment. Treat a successful deploy request as incomplete until the course
landing page and a representative PDF both respond successfully.

## Verification

Run Rust tests after server changes. Validate project IDs and repository URLs,
confirm the expected checkout name, and inspect both the source
`latex_target/` and published sandbox when diagnosing missing files. Test API
authentication and avoid exposing secret values in logs or command output.
