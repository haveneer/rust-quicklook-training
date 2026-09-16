use std::fs;
use std::path::{Path, PathBuf};

// Wrapper RAII : possède un fichier temporaire et le supprime à la sortie de portée.
struct TempFile {
    path: PathBuf,
}

impl TempFile {
    fn create(path: impl AsRef<Path>, contents: &str) -> std::io::Result<Self> {
        let path = path.as_ref().to_path_buf();
        fs::write(&path, contents)?;
        Ok(Self { path })
    }
}

impl Drop for TempFile {
    fn drop(&mut self) {
        // best effort : le fichier a pu déjà être déplacé/supprimé entre-temps
        let _ = fs::remove_file(&self.path);
    }
}

fn main() {
    let path = std::env::temp_dir().join("rust_quicklook_drop_demo.txt");

    {
        let tmp = TempFile::create(&path, "scratch data").unwrap();
        assert!(path.exists());
        drop(tmp); // drop anticipé : équivalent à la sortie de portée ci-dessous
    }
    assert!(!path.exists());

    let path2 = std::env::temp_dir().join("rust_quicklook_drop_demo2.txt");
    {
        let _tmp = TempFile::create(&path2, "scratch data").unwrap();
        assert!(path2.exists());
    } // <- drop() s'exécute ici, sans intervention explicite
    assert!(!path2.exists());
}

#[test]
fn test() {
    main();
}
