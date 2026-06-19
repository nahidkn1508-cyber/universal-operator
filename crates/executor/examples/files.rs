use executor::FileManager;

fn main() {
    let manager = FileManager::new();

    manager.create_folder("demo").unwrap();

    manager.create_file("demo/hello.txt").unwrap();

    manager
        .write_file("demo/hello.txt", "Universal AI Operator\n")
        .unwrap();

    println!("Done");
}
