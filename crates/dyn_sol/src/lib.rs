use std::path::Path;

extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro]
pub fn aoc_imports(_item: TokenStream) -> TokenStream {
    let mut mods = Vec::new();
    for i in 1..50 {
        let fname = format!("src/aoc2022_{:02}.rs", i);
        let mod_str = format!("mod aoc2022_{:02};", i);
        if Path::new(fname.as_str()).exists() {
            mods.push(mod_str);
        }
    }

    mods.join("\n").parse().unwrap()
}

#[proc_macro]
pub fn aoc_select(_item: TokenStream) -> TokenStream {
    let mut arms = Vec::new();
    arms.push("match args.solver.as_str() {".to_string());
    for i in 1..50 {
        let fname = format!("src/aoc2022_{:02}.rs", i);
        let arm = format!("\"{:02}\" => aoc2022_{:02}::run(input),", i, i);
        if Path::new(fname.as_str()).exists() {
            arms.push(arm);
        }
    }
    arms.push("_ => panic!(\"Unknown solver: {}\", args.solver),".to_string());
    arms.push("}".to_string());

    arms.join("\n").parse().unwrap()
}
