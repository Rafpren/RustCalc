fn main() {
    // Ce script ne doit s'exécuter que si l'on compile pour Windows
    #[cfg(target_os = "windows")]
    {
        let mut res = winres::WindowsResource::new();
        // Spécifie le chemin vers ton fichier .ico
        res.set_icon("icon.ico");
        // Tu peux aussi ajouter des métadonnées visibles dans les propriétés du fichier
        res.set("ProductName", "RustCalc");
        res.set("CompanyName", "Rafpren");
        res.compile().unwrap();
    }
}