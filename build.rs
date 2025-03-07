use std::env;

fn main() {
    let target = env::var("TARGET").unwrap();
    let host_triple = env::var("HOST").unwrap();

    if host_triple == target {
        println!("cargo:rustc-cfg=native");
    }

    println!("cargo:rustc-check-cfg=cfg(thumb)");
    if target.starts_with("thumbv") {
        println!("cargo:rustc-cfg=thumb");
    }

    println!("cargo::rustc-check-cfg=cfg(cortex_m)");
    println!("cargo::rustc-check-cfg=cfg(armv6m)");

    println!("cargo::rustc-check-cfg=cfg(armv7m)");
    println!("cargo::rustc-check-cfg=cfg(armv7em)");

    println!("cargo::rustc-check-cfg=cfg(armv8m)");
    println!("cargo::rustc-check-cfg=cfg(armv8m_base)");
    println!("cargo::rustc-check-cfg=cfg(armv8m_main)");

    println!("cargo::rustc-check-cfg=cfg(native)");
    println!("cargo::rustc-check-cfg=cfg(has_fpu)");

    if target.starts_with("thumbv6m-") {
        println!("cargo:rustc-cfg=cortex_m");
        println!("cargo:rustc-cfg=armv6m");
    } else if target.starts_with("thumbv7m-") {
        println!("cargo:rustc-cfg=cortex_m");
        println!("cargo:rustc-cfg=armv7m");
    } else if target.starts_with("thumbv7em-") {
        println!("cargo:rustc-cfg=cortex_m");
        println!("cargo:rustc-cfg=armv7m");
        println!("cargo:rustc-cfg=armv7em");
    } else if target.starts_with("thumbv8m.base") {
        println!("cargo:rustc-cfg=cortex_m");
        println!("cargo:rustc-cfg=armv8m");
        println!("cargo:rustc-cfg=armv8m_base");
    } else if target.starts_with("thumbv8m.main") {
        println!("cargo:rustc-cfg=cortex_m");
        println!("cargo:rustc-cfg=armv8m");
        println!("cargo:rustc-cfg=armv8m_main");
    }

    if target.ends_with("-eabihf") {
        println!("cargo:rustc-cfg=has_fpu");
    }
}
