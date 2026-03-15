fn main() {
    if cfg!(target_os = "windows") {
        let mut res = winres::WindowsResource::new();
        res.set_icon("icon.ico"); // Le nom de ton fichier icône
        res.compile().unwrap();
    }
}