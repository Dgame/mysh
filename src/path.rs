use crate::pool::Pool;
use std::collections::HashMap;
use std::ffi::OsString;
use std::io;
use std::path::{Path, PathBuf};

fn read_files<P: AsRef<Path>>(dir: P) -> io::Result<HashMap<String, PathBuf>> {
    use std::fs;

    let mut files = HashMap::new();
    for entry in fs::read_dir(dir)? {
        if let Ok(entry) = entry {
            let path = entry.path();
            if path.is_file() {
                let name = path
                    .file_stem()
                    .expect("Unable to stem filename")
                    .to_str()
                    .expect("Unable to stringify stem")
                    .to_owned();
                files.insert(name, path);
            }
        }
    }

    Ok(files)
}

pub struct OsPath {
    path: Option<OsString>,
    files: HashMap<String, PathBuf>,
}

impl OsPath {
    pub fn load() -> Self {
        use log::debug;
        use std::env;
        use std::sync::mpsc;

        let (tx, rx) = mpsc::channel();
        let mut pool = Pool::new(tx, rx);
        let mut path_files = HashMap::new();
        if let Some(path) = env::var_os("PATH") {
            for path in path
                .to_str()
                .to_owned()
                .expect("Unable to access PATH")
                .split(':')
            {
                let path: String = path.to_owned();
                pool.send(move |sender| {
                    if let Ok(files) = read_files(&path) {
                        sender.send(Some(files)).expect("Unable to send on channel");
                    } else {
                        sender.send(None).expect("Unable to send on channel");
                    }
                });

                if let Some(Some(files)) = pool.receive() {
                    path_files.extend(files.into_iter());
                }
            }

            debug!(
                "#1 Warte auf {} laufende und {} wartende Jobs",
                pool.get_pending(),
                pool.get_waiting()
            );

            while pool.is_running() {
                if let Some(Some(files)) = pool.receive() {
                    path_files.extend(files.into_iter());
                }

                debug!(
                    "#2 Warte auf {} laufende und {} wartende Jobs",
                    pool.get_pending(),
                    pool.get_waiting()
                );
            }

            Self {
                path: Some(path),
                files: path_files,
            }
        } else {
            Self {
                path: None,
                files: path_files,
            }
        }
    }

    pub fn needs_refresh(&mut self) -> bool {
        use std::env;

        env::var_os("PATH") != self.path
    }

    pub fn refresh(&self) -> Self {
        Self::load()
    }

    pub fn count(&self) -> usize {
        self.files.len()
    }

    pub fn contains(&self, name: &str) -> bool {
        self.files.contains_key(name)
    }
}
