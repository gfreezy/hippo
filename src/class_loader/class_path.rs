use std::env;
use std::fs::read_dir;
use std::fs::File;
use std::io;
use std::io::Error;
use std::io::ErrorKind;
use std::io::Read;
use std::path::Path;
use std::path::PathBuf;
use tracing::trace;

#[derive(Debug)]
enum Entry {
    Dir { path: PathBuf },
    Wildcard { path_vec: Vec<PathBuf> },
    Zip { path: PathBuf },
}

impl Entry {
    fn new(path: &str) -> Entry {
        if path.ends_with('*') {
            trace!(path, "Entry::new Wildcard");
            let len = path.len();
            let base_path = &path[..len - 1];
            trace!(base_path);
            let path_vec: Vec<PathBuf> = read_dir(base_path)
                .unwrap_or_else(|_| panic!("read_dir: {}", base_path))
                .map(|x| x.unwrap())
                .map(|x| x.path())
                .filter(|x| {
                    x.extension()
                        .map(|x| x.to_str().unwrap() == "jar")
                        .unwrap_or(false)
                })
                .collect();
            trace!("path_vec {:?}", path_vec);

            Entry::Wildcard { path_vec }
        } else if path.ends_with(".jar") {
            trace!("Entry::new Zip {}", path);
            Entry::Zip {
                path: Path::new(path).to_owned(),
            }
        } else {
            trace!("Entry::new Dir {}", path);
            Entry::Dir {
                path: Path::new(path).to_owned(),
            }
        }
    }

    fn read_class(&self, class_file_name: &str) -> Result<Vec<u8>, io::Error> {
        match self {
            Entry::Dir { path } => {
                trace!("read class {} using Dir", class_file_name);
                let filepath = Path::new(path).join(class_file_name);
                tracing::debug!(?filepath, "read_class");
                let mut file = File::open(filepath)?;
                let meta = file.metadata()?;
                let mut buf = Vec::<u8>::with_capacity(meta.len() as usize);
                file.read_to_end(&mut buf)?;
                Ok(buf)
            }
            Entry::Wildcard { path_vec } => {
                trace!("read class {} using Wildcard", class_file_name);
                path_vec
                    .iter()
                    .map(|x| Entry::new(x.to_str().unwrap()))
                    .map(|x| x.read_class(class_file_name))
                    .find(|x| x.is_ok())
                    .unwrap_or_else(|| Err(Error::new(ErrorKind::Other, "Class not found")))
            }
            Entry::Zip { path } => {
                trace!("read class {} using Zip", class_file_name);
                let file = File::open(path)?;
                let mut zip = zip::ZipArchive::new(file)?;
                let mut file = zip.by_name(&class_file_name)?;
                let mut buf = Vec::<u8>::with_capacity(file.size() as usize);
                file.read_to_end(&mut buf).expect("read file content");
                Ok(buf)
            }
        }
    }
}

#[derive(Debug)]
pub struct ClassPath {
    boot: Entry,
    user: Entry,
}

impl ClassPath {
    pub fn read_class(&self, name: &str) -> Result<Vec<u8>, io::Error> {
        let class_file_name = name.to_owned() + ".class";
        tracing::debug!(%class_file_name, "read_class");

        self.boot
            .read_class(&class_file_name)
            .or_else(|_x| self.user.read_class(&class_file_name))
    }

    pub fn new(jre_opt: Option<String>, cp_opt: Option<String>) -> ClassPath {
        let jre = get_jre(jre_opt);
        let boot = parse_boot_classpath(&jre);
        let user = parse_user_classpath(cp_opt);
        ClassPath { user, boot }
    }
}

fn parse_boot_classpath(jre: &str) -> Entry {
    let jre_lib_path = Path::new(jre)
        .join("lib")
        .join("*")
        .to_str()
        .unwrap()
        .to_owned();
    Entry::new(&jre_lib_path)
}

fn parse_user_classpath(cp_opt: Option<String>) -> Entry {
    let cp = cp_opt.unwrap_or_else(|| ".".to_owned());
    Entry::new(&cp)
}

fn exists(path: &str) -> bool {
    Path::new(path).exists()
}

fn get_jre(jre_opt: Option<String>) -> String {
    match jre_opt {
        Some(ref jre) if exists(jre) => jre.to_string(),
        _ => {
            if exists("./jre") {
                println!("use jre in current dir");
                "./jre".to_string()
            } else {
                match env::var_os("JAVA_HOME") {
                    Some(java_home) => Path::new(&java_home)
                        .join("jre")
                        .to_str()
                        .unwrap()
                        .to_string(),
                    None => panic!("Can not find JRE folder"),
                }
            }
        }
    }
}
