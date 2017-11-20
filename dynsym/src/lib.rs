extern crate object;
extern crate cpp_demangle;
extern crate memmap;

use std::path::Path;
use std::fs::File;

use object::{ElfFile, Object};
use cpp_demangle::{Symbol, DemangleOptions};
use memmap::Mmap;

pub fn iter<P: AsRef<Path>>(path: P) -> Vec<(String, usize)> {
    let file = File::open(path).unwrap();
    let file = unsafe { Mmap::map(&file) }.unwrap();
    let file = ElfFile::parse(&*file).unwrap();
    file.dynamic_symbols().into_iter()
        .flat_map(|sym| sym.name()
            .and_then(|name| Symbol::new(name).ok())
            .and_then(move |symbol| {
                let options = DemangleOptions { no_params: false };
                symbol.demangle(&options).ok()
            })
            .map(|name| (name, sym.address() as usize)))
        .collect()
}
