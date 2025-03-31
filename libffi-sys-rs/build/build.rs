mod common;
mod msvc;
mod not_msvc;

fn main() {
    let target_env = std::env::var("CARGO_CFG_TARGET_ENV").unwrap();

    if cfg!(feature = "system") {
        match target_env.as_str() {
            "msvc" => msvc::probe_and_link(),
            _ => not_msvc::probe_and_link()
        }
    } else {
        match target_env.as_str() {
            "msvc" => msvc::build_and_link(),
            _ => not_msvc::build_and_link()
        }
    }
}
