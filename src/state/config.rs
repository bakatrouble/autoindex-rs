use std::env;
use std::path::{Path, PathBuf};
use axum::http::Uri;
use serde::Serialize;

#[derive(Serialize)]
pub struct ConfigResponse {
    root: Option<String>,
    default_mode: DefaultMode,
}

#[derive(Serialize, Clone, Copy)]
pub enum DefaultMode {
    #[serde(rename = "list")]
    List,
    #[serde(rename = "gallery")]
    Gallery,
}

#[derive(Serialize, Clone)]
pub struct Config {
    pub files_root: PathBuf,
    pub subdomains_root: PathBuf,
    pub hostname: Option<String>,
    pub expose_base_path: bool,
    pub default_mode: DefaultMode,
    pub dev: Option<String>,
    pub enable_chroot: bool,
    pub bind: String,
}

impl Config {
    pub fn new() -> Self {
        let files_root = PathBuf::from(
            env::var("FILES_ROOT").unwrap_or("files/root".into())
        )
            .canonicalize()
            .unwrap();
        let subdomains_root = PathBuf::from(
            env::var("SUBDOMAINS_ROOT").unwrap_or("files/subdomains".into())
        )
            .canonicalize()
            .unwrap();
        let hostname = env::var("HOSTNAME").ok();
        let expose_base_path = env::var("EXPOSE_BASE_PATH").ok().is_some();
        let default_mode = match env::var("DEFAULT_MODE").unwrap_or("gallery".into()).as_str() {
            "gallery" => DefaultMode::Gallery,
            _ => DefaultMode::List,
        };
        let dev = env::var("DEV").ok();
        let enable_chroot = env::var("ENABLE_CHROOT").ok().is_some();
        let bind = env::var("BIND").unwrap_or("[::]:8474".into());

        Self {
            files_root,
            subdomains_root,
            hostname,
            expose_base_path,
            default_mode,
            dev,
            enable_chroot,
            bind,
        }
    }

    pub fn get_config_response(&self, uri: &Uri) -> ConfigResponse {
        let default_mode = self.default_mode;
        if !self.expose_base_path {
            return ConfigResponse {
                root: None,
                default_mode,
            };
        }

        ConfigResponse {
            root: self.get_root_path(uri).map(|(path, _)| {
                path.to_string_lossy().into()
            }),
            default_mode,
        }
    }

    pub fn get_root_path(&self, uri: &Uri) -> Option<(PathBuf, bool)> {
        if self.hostname.is_none() {
            return Some((self.files_root.clone(), true));
        }

        let hostname = self.hostname.clone().unwrap();
        let host = uri.host().unwrap_or("").strip_suffix(hostname.as_str()).unwrap_or("");
        if host.is_empty() || !host.ends_with(".") {
            return Some((self.files_root.clone(), true));
        }

        let host = &host[..host.len()-1];  // strip final dot
        if let Some(entry) = self.subdomains_root.read_dir().ok()?
            .filter_map(Result::ok)
            .find(|e| {
                let filename = e.file_name();
                filename == host || filename == format!(".{host}").as_str()
            })
        {
            Some((
                self.subdomains_root.clone().join(host),
                !entry.file_name().to_str()?.starts_with(".")
            ))
        } else {
            None
        }
    }

    pub fn clean_path<P: AsRef<Path>>(path: P) -> PathBuf {
        use std::path::Component;
        let path = path.as_ref();

        let components = path.components().peekable();
        let mut cleaned = PathBuf::from("");
        let mut component_count = 0;

        for component in components {
            match component {
                Component::Prefix(..) => unreachable!(),
                Component::ParentDir if component_count > 0 => {
                    cleaned.pop();
                    component_count -= 1;
                }
                Component::Normal(component) => {
                    cleaned.push(component);
                    component_count += 1;
                }
                Component::RootDir | Component::CurDir | Component::ParentDir => {}
            }
        }

        cleaned
    }
}
