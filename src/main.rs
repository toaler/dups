use std::{env, fs};
use std::path::Path;

struct FileSystemTraversal;

impl FileSystemTraversal {

    fn traverse(&self, path: &Path, visitor: &NodeWriter) {
        visitor.visit(&path);
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.flatten() {
                if let Ok(entry_path) = entry.path().into_os_string().into_string() {

                    if entry.path().is_dir() {
                        self.traverse(&path, &visitor);

                    } else {
                        visitor.visit(&entry.path());
                    }
                }
            }
        }
    }
}

trait Visitable {
    fn visit(&self, path: &Path);
}

struct NodeWriter {

}

impl Visitable for NodeWriter {
    fn visit(&self, path: &Path) {
        println!("Visiting : {}", path.display());
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Please provide a directory path.");
        return;
    }

    let root_directory = &args[1];
    let root = Path::new(root_directory);
    let traverser = FileSystemTraversal;
    let nodewriter = NodeWriter {};

    traverser.traverse(&root, &nodewriter);
}