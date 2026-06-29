use std::{
    path::{Path, PathBuf}
};
use axum_extra::headers::Host;
use serde::{Serialize};
use clap::{Parser, ValueEnum};

#[derive(Serialize)]
pub struct ConfigResponse {
    root: Option<String>,
    default_mode: DefaultMode,
}

#[derive(ValueEnum, Serialize, Clone, Copy, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum DefaultMode {
    #[serde(rename = "list")]
    List,
    #[serde(rename = "gallery")]
    Gallery,
}

#[derive(Parser, Serialize, Clone)]
pub struct Config {
    #[arg(short = 'f', long, default_value = "files/root", env = "FILES_ROOT")]
    pub files_root: PathBuf,
    #[arg(short = 's', long, default_value = "files/subdomains", env = "SUBDOMAIN_ROOT")]
    pub subdomains_root: PathBuf,
    #[arg(long, env = "HOSTNAME")]
    pub hostname: Option<String>,
    #[arg(short = 'e', long, default_value_t = false, env = "EXPOSE_BASE_PATH")]
    pub expose_base_path: bool,
    #[arg(short = 'm', long, default_value = "list", env = "DEFAULT_MODE")]
    pub default_mode: DefaultMode,
    #[arg(long, env = "DEV")]
    pub dev: Option<String>,
    #[arg(short = 'c', long, default_value_t = false, env = "ENABLE_CHROOT")]
    pub enable_chroot: bool,
    #[arg(short = 'b', long, default_value = "[::]:8474", env = "BIND")]
    pub bind: String,
}

impl Config {
    pub fn new() -> Self {
        Config::parse()
    }

    pub fn get_config_response(&self, host: &Host) -> ConfigResponse {
        let default_mode = self.default_mode;
        if !self.expose_base_path {
            return ConfigResponse {
                root: None,
                default_mode,
            };
        }

        ConfigResponse {
            root: self.get_root_path(&host).map(|(path, _)| {
                path.to_string_lossy().into()
            }),
            default_mode,
        }
    }

    pub fn get_root_path(&self, host: &Host) -> Option<(PathBuf, bool)> {
        if self.hostname.is_none() {
            return Some((self.files_root.clone(), true));
        }

        let hostname = self.hostname.clone().unwrap();
        let host = host.hostname().strip_suffix(hostname.as_str()).unwrap_or("");
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
