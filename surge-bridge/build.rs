use std::env::var;

use cc;

macro_rules! realprint {
    ($($tokens:tt)*) => {
        println!("\x1b[1;32m[SRS-BRG] =>\x1b[0m {}", format!($($tokens)*));
    }
}
macro_rules! fakeprint {
    ($($tokens:tt)*) => {
        println!("\x1b[1;36m[SRS-BRG] =>\x1b[0m {}", format!($($tokens)*));
    }
}

fn main() {
    println!("cargo:rerun-if-changed=cpp/bridge.h");
    println!("cargo:rerun-if-changed=cpp/bridge.cpp");

    realprint!("gathering bridge materials.");
    let mut build = cc::Build::new();
    build
        .warnings(false)
        .cpp(true)
        .std("c++20")
        .flag("-fno-char8_t")       // TODO: ponder about inclusion in sys and removal from here.
        .file("cpp/bridge.cpp");

    realprint!("pulling (and applying) build flags from sys.");
    let binding = var("DEP_SURGE_BFLAGS").unwrap();         // binding as in variable binding.
    binding.split(",").for_each(|f| {
        fakeprint!("new flag: {}", f);
        build.flag(&f);
    });

    realprint!("bridge is being built. please hold.");
    if let Err(e) = build.try_compile("bridge") { panic!("bridge burnt down while building.\n\n{}", e); }   // shortened!
    println!("cargo:rustc-link-lib=static=bridge");

    realprint!("all done!");
}
