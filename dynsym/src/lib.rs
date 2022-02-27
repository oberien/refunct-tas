extern crate object;
extern crate cpp_demangle;
extern crate memmap;

use std::path::Path;
use std::fs::File;

use object::{Object, ObjectSymbol};
use cpp_demangle::{Symbol, DemangleOptions};
use memmap::Mmap;

pub fn iter<P: AsRef<Path>>(path: P) -> Vec<(String, usize)> {
    let file = File::open(path).unwrap();
    let file = unsafe { Mmap::map(&file) }.unwrap();
    let file = object::File::parse(&*file).unwrap();
    file.dynamic_symbols()
        .flat_map(|sym| sym.name_bytes().ok()
            .and_then(|name| Symbol::new(name).ok())
            .and_then(move |symbol| {
                symbol.demangle(&DemangleOptions::new()).ok()
            })
            .or_else(|| sym.name().map(|s| s.to_string()).ok())
            .map(|name| (name, sym.address() as usize)))
        .collect()
}
