use hotaru::http::*;
use hotaru::prelude::*;
use notes_server::project::{ProjectManifest, ProjectTree};
use notes_server::projects::{CONFIG_ROOT, PROJECTS};
use notes_server::registry::REGISTRY;
use std::collections::{HashMap, HashSet};
use std::sync::{LazyLock, Mutex};

LServer!(
    APP = Server::new()
        .binding("127.0.0.1:3010")
        .single_protocol(ProtocolBuilder::new(
            HTTP::server(HttpSafety::default())
        ))
        .build()
);

// TODO: move to Registry if non-HTTP deploy triggers are added
static DEPLOYING: LazyLock<Mutex<HashSet<String>>> =
    LazyLock::new(|| Mutex::new(HashSet::new()));

fn main() {
    run_server!(APP);
}

middleware! {
    pub ApiKeyAuth<HTTP> {
        let provided = req.header_str("x-api-key").map(|s| s.to_string());
        let expected = std::env::var("NOTES_DEPLOY_KEY").ok();

        let authorized = match (expected.as_deref(), provided.as_deref()) {
            (Some(e), Some(p)) => e == p,
            _ => false,
        };

        if !authorized {
            req.response = text_response("unauthorized")
                .status(StatusCode::UNAUTHORIZED);
            return Ok(req);
        }

        next(req).await
    }
}

endpoint! {
    APP.url("/<str:project>"),
    pub course_landing<HTTP> {
        let project = req.param("project").unwrap_or_default();

        if !PROJECTS.contains_key(project.as_str()) {
            return text_response("not found").status(StatusCode::NOT_FOUND);
        }

        let sandbox = match REGISTRY.sandbox_for(&project) {
            Ok(p) => p,
            Err(_) => return text_response("not found").status(StatusCode::NOT_FOUND),
        };

        let manifest_path = CONFIG_ROOT.join(&project).join(ProjectManifest::FILE_NAME);
        let manifest = match ProjectManifest::read(&manifest_path) {
            Ok(m) => m,
            Err(_) => return text_response("manifest missing")
                .status(StatusCode::NOT_FOUND),
        };

        let tree = match ProjectTree::walk(&sandbox) {
            Ok(t) => t,
            Err(_) => return text_response("tree read failed")
                .status(StatusCode::INTERNAL_SERVER_ERROR),
        };

        let groups_value = tree.render_with_urls(&project);

        akari_render!(
            "course.html",
            title = manifest.title,
            project = project,
            groups = groups_value
        )
    }
}

endpoint! {
    APP.url("/<str:project>/<**path:rest>"),
    pub course_static<HTTP> {
        let project = req.param("project").unwrap_or_default();
        let rest = req.param("rest").unwrap_or_default();

        if rest.is_empty() {
            return text_response("not found").status(StatusCode::NOT_FOUND);
        }

        let bytes = match REGISTRY.read_project_file(&project, &rest) {
            Ok(b) => b,
            Err(_) => return text_response("not found").status(StatusCode::NOT_FOUND),
        };

        let start_line = HttpStartLine::new_response(HttpVersion::Http11, StatusCode::OK);
        let mut meta = HttpMeta::new(start_line, HashMap::new());
        meta.set_content_type(HttpContentType::from_file_name(&rest));
        HttpResponse::new(meta, HttpBody::Binary(bytes))
    }
}

endpoint! {
    APP.url("/deploy/<str:project>"),
    middleware = [ApiKeyAuth],
    config = [HttpSafety::default().with_allowed_method(HttpMethod::POST)],
    pub deploy_project<HTTP> {
        let project = req.param("project").unwrap_or_default();

        let cfg = match PROJECTS.get(project.as_str()) {
            Some(c) => c.clone(),
            None => return text_response("unknown project")
                .status(StatusCode::NOT_FOUND),
        };

        {
            let mut deploying = DEPLOYING.lock().unwrap();
            if !deploying.insert(project.clone()) {
                return text_response("deploy in progress")
                    .status(StatusCode::CONFLICT);
            }
        }

        let event = akari::Value::new_dict();
        let result = REGISTRY
            .dispatch(&project, &cfg.script, cfg.script_config.clone(), event)
            .await;

        DEPLOYING.lock().unwrap().remove(&project);

        match result {
            Ok(()) => text_response("ok"),
            Err(e) => text_response(format!("deploy failed: {e}"))
                .status(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}
