use crate::settings::Settings;


pub fn print_toolchain_list() {
    println!("Configured toolchains:\n");

    let settings = Settings::from_home().unwrap();
    let kits = settings.kits;
    for (kit_name, kit) in kits.kits.into_iter() {
        let compiler = kits.compilers.get(&kit.compiler).unwrap();
        let cmake = kits.cmake.get(&kit.cmake).unwrap();
        println!("{} ({})", kit.name, kit_name);
        println!("    Compiler ({})", kit.compiler);
        println!("        Name: {}", compiler.name);
        println!("        Path: {}", compiler.path);
        println!("    CMake ({})", kit.cmake);
        println!("        Name: {}", cmake.name);
        println!("        Path: {}", cmake.path);
        println!();
    }
}