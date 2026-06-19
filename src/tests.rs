// src/tests.rs
use super::*;
use crate::utils::generate_temp_name;
use std::fs;
use std::io::{ErrorKind, Read, Write};
use std::path::PathBuf;

// Helper: a temporary directory that is cleaned up after the test.
struct TestDir(PathBuf);
impl TestDir {
    fn new(test_name: &str) -> Self {
        let dir = generate_temp_name(&format!("fmtest_{}_", test_name));
        fs::create_dir(&dir).unwrap();
        TestDir(dir)
    }
    fn path(&self) -> &Path {
        &self.0
    }
    fn join<P: AsRef<Path>>(&self, child: P) -> PathBuf {
        self.0.join(child)
    }
}
impl Drop for TestDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

// ──── create_file ────
#[test]
fn create_file_success() {
    let dir = TestDir::new("create_file");
    let f = dir.join("new.txt");
    FileManager::create_file(&f).unwrap();
    assert!(f.exists());
    assert!(f.is_file());
}

#[test]
fn create_file_already_exists() {
    let dir = TestDir::new("create_file_dup");
    let f = dir.join("dup.txt");
    FileManager::create_file(&f).unwrap();
    let err = FileManager::create_file(&f).unwrap_err();
    assert!(matches!(err, FileError::AlreadyExists(_)));
}

// ──── write / append / read ────
#[test]
fn write_and_read_file() {
    let dir = TestDir::new("write_read");
    let f = dir.join("hello.txt");
    FileManager::write_file(&f, b"world").unwrap();
    let s = FileManager::read_file(&f).unwrap();
    assert_eq!(s, "world");
}

#[test]
fn append_file() {
    let dir = TestDir::new("append");
    let f = dir.join("log.txt");
    FileManager::append_file(&f, b"a").unwrap();
    FileManager::append_file(&f, b"b").unwrap();
    assert_eq!(FileManager::read_file(&f).unwrap(), "ab");
}

#[test]
fn read_bytes() {
    let dir = TestDir::new("read_bytes");
    let f = dir.join("bin.dat");
    let data = vec![0, 1, 2, 253, 254, 255];
    FileManager::write_file(&f, &data).unwrap();
    let out = FileManager::read_bytes(&f).unwrap();
    assert_eq!(out, data);
}

// ──── copy / move ────
#[test]
fn copy_file_success() {
    let dir = TestDir::new("copy_file");
    let src = dir.join("src.txt");
    let dst = dir.join("dst.txt");
    FileManager::write_file(&src, b"copy me").unwrap();
    FileManager::copy_file(&src, &dst).unwrap();
    assert!(dst.exists());
    assert_eq!(FileManager::read_file(&dst).unwrap(), "copy me");
}

#[test]
fn copy_directory_recursive() {
    let dir = TestDir::new("copy_dir");
    let src = dir.join("src_dir");
    FileManager::create_directory(&src).unwrap();
    FileManager::write_file(src.join("f1.txt"), b"one").unwrap();
    FileManager::create_directory(src.join("sub")).unwrap();
    FileManager::write_file(src.join("sub/f2.txt"), b"two").unwrap();

    let dst = dir.join("dst_dir");
    FileManager::copy_directory_recursive(&src, &dst).unwrap();
    assert!(dst.is_dir());
    assert_eq!(FileManager::read_file(dst.join("f1.txt")).unwrap(), "one");
    assert_eq!(
        FileManager::read_file(dst.join("sub/f2.txt")).unwrap(),
        "two"
    );
}

#[test]
fn copy_directory_target_exists() {
    let dir = TestDir::new("copy_dir_exists");
    let src = dir.join("src");
    FileManager::create_directory(&src).unwrap();
    let dst = dir.join("dst");
    FileManager::create_directory(&dst).unwrap();
    let err = FileManager::copy_directory_recursive(&src, &dst).unwrap_err();
    assert!(matches!(err, FileError::AlreadyExists(_)));
}

#[test]
fn move_file_rename() {
    let dir = TestDir::new("move_file");
    let old = dir.join("old.txt");
    let new = dir.join("new.txt");
    FileManager::write_file(&old, b"data").unwrap();
    FileManager::move_file(&old, &new).unwrap();
    assert!(!old.exists());
    assert!(new.exists());
    assert_eq!(FileManager::read_file(&new).unwrap(), "data");
}

// ──── directory operations ────
#[test]
fn create_and_delete_directory() {
    let dir = TestDir::new("create_del_dir");
    let sub = dir.join("subdir");
    FileManager::create_directory(&sub).unwrap();
    assert!(sub.is_dir());
    FileManager::delete_directory(&sub).unwrap();
    assert!(!sub.exists());
}

#[test]
fn delete_directory_recursive() {
    let dir = TestDir::new("delete_recursive");
    let sub = dir.join("parent");
    FileManager::create_directory_recursive(sub.join("child")).unwrap();
    FileManager::write_file(sub.join("child/grandson.txt"), b"x").unwrap();
    assert!(sub.exists());
    FileManager::delete_directory_recursive(&sub).unwrap();
    assert!(!sub.exists());
}

#[test]
fn create_directory_recursive() {
    let dir = TestDir::new("create_recursive");
    let deep = dir.join("a").join("b").join("c");
    FileManager::create_directory_recursive(&deep).unwrap();
    assert!(deep.is_dir());
}

// ──── existence / type / listing ────
#[test]
fn exists_is_file_is_dir() {
    let dir = TestDir::new("exists");
    let f = dir.join("f");
    FileManager::write_file(&f, b"").unwrap();
    assert!(FileManager::exists(&f));
    assert!(FileManager::is_file(&f));
    assert!(!FileManager::is_directory(&f));
    assert!(FileManager::is_directory(dir.path()));
    assert!(!FileManager::is_file(dir.path()));
}

#[test]
fn list_directory() {
    let dir = TestDir::new("list");
    FileManager::write_file(dir.join("a.txt"), b"").unwrap();
    FileManager::write_file(dir.join("b.txt"), b"").unwrap();
    FileManager::create_directory(dir.join("sub")).unwrap();
    let entries = FileManager::list_directory(dir.path()).unwrap();
    let mut names: Vec<String> = entries
        .iter()
        .map(|p| p.file_name().unwrap().to_str().unwrap().to_owned())
        .collect();
    names.sort();
    assert_eq!(names, vec!["a.txt", "b.txt", "sub"]);
}

// ──── metadata / size ────
#[test]
fn file_size_and_metadata() {
    let dir = TestDir::new("fsize");
    let f = dir.join("f");
    FileManager::write_file(&f, b"12345").unwrap();
    assert_eq!(FileManager::file_size(&f).unwrap(), 5);
    let meta = FileManager::metadata(&f).unwrap();
    assert!(meta.is_file());
}

#[test]
fn file_size_on_directory() {
    let dir = TestDir::new("fsize_dir");
    let d = dir.join("d");
    FileManager::create_directory(&d).unwrap();
    let err = FileManager::file_size(&d).unwrap_err();
    assert!(matches!(err, FileError::NotAFile(_)));
}

// ──── canonicalize / working directory ────
#[test]
fn canonicalize_and_cwd() {
    let dir = TestDir::new("canon");
    let rel = dir.join("relative.txt");
    FileManager::write_file(&rel, b"").unwrap();
    let abs = FileManager::canonicalize_path(&rel).unwrap();
    assert!(abs.is_absolute());
    assert_eq!(abs.file_name().unwrap(), "relative.txt");

    let original = FileManager::current_working_directory().unwrap();
    FileManager::set_current_working_directory(dir.path()).unwrap();
    assert_ne!(FileManager::current_working_directory().unwrap(), original);
    FileManager::set_current_working_directory(&original).unwrap();
}

// ──── temporary files / directories ────
#[test]
fn temp_file_creation_and_persist() {
    let tmp = FileManager::create_temp_file().unwrap();
    let path = tmp.path().to_path_buf();
    assert!(path.exists());
    tmp.file().write_all(b"hello").unwrap();
    let mut f = tmp.persist().unwrap();
    assert!(path.exists());
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();
    assert_eq!(s, "hello");
    fs::remove_file(&path).unwrap();
}

#[test]
fn temp_file_drop_removes_file() {
    let path;
    {
        let tmp = FileManager::create_temp_file().unwrap();
        path = tmp.path().to_path_buf();
        assert!(path.exists());
    }
    assert!(!path.exists());
}

#[test]
fn temp_dir_creation_and_persist() {
    let mut tmp = FileManager::create_temp_directory().unwrap();
    let p = tmp.path().to_path_buf();
    assert!(p.is_dir());
    FileManager::write_file(p.join("inner.txt"), b"data").unwrap();
    tmp.persist();
    drop(tmp);
    assert!(p.exists());
    fs::remove_dir_all(&p).unwrap();
}

#[test]
fn temp_dir_drop_removes_dir() {
    let p;
    {
        let tmp = FileManager::create_temp_directory().unwrap();
        p = tmp.path().to_path_buf();
        assert!(p.exists());
    }
    assert!(!p.exists());
}

// ──── error conversion ────
#[test]
fn error_kind_mapping() {
    let dir = TestDir::new("error_kind");

    let e = FileManager::read_file(dir.join("no_such_file")).unwrap_err();
    assert!(matches!(e, FileError::NotFound(_)));

    FileManager::write_file(dir.join("exists.txt"), b"").unwrap();
    let e = FileManager::create_file(dir.join("exists.txt")).unwrap_err();
    assert!(matches!(e, FileError::AlreadyExists(_)));

    let sub = dir.join("nonempty");
    FileManager::create_directory(&sub).unwrap();
    FileManager::write_file(sub.join("child"), b"").unwrap();
    let e = FileManager::delete_directory(&sub).unwrap_err();
    assert!(matches!(e, FileError::DirectoryNotEmpty(_)));
}