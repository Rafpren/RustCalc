fn main() {
    #[cfg(target_os = "windows")]
    {
        let mut res = winres::WindowsResource::new();
        res.set_icon("icon.ico");
        res.set("ProductName", "RustCalc");
        res.set("CompanyName", "Rafpren");
        res.compile().unwrap();
    }
}
