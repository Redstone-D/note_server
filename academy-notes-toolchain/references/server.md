# Component 4: Server

## Ownership

Keep server registration beneath `config/<project>/` in the notes-server
repository. A GitHub-backed project requires both files:

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

Keep these files out of the course template. Restart the server after adding or
removing a project directory or editing its `config.json`, because project
configuration is loaded once. Central manifest titles are read per request, so
a title-only change in `manifest.json` does not require a restart.

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
