extern crate goblin;
extern crate pdb;

use std::env;
use std::fs::File;
use std::io::{Read, Write};

use goblin::Object;
use pdb::{PDB, SymbolData};
use pdb::FallibleIterator;

const NAMES: [(&str, &str); 9] = [
    ("?UpdateTimeAndHandleMaxTickRate@UEngine", "UENGINE_UPDATETIMEANDHANDLEMAXTICKRATE"),
    ("?Tick@FSlateApplication", "FSLATEAPPLICATION_TICK"),
    ("?OnKeyDown@FSlateApplication", "FSLATEAPPLICATION_ONKEYDOWN"),
    ("?OnKeyUp@FSlateApplication", "FSLATEAPPLICATION_ONKEYUP"),
    ("?OnRawMouseMove@FSlateApplication", "FSLATEAPPLICATION_ONRAWMOUSEMOVE"),
    ("?GetControlRotation@AController", "ACONTROLLER_GETCONTROLROTATION"),
    ("?execForcedUnCrouch@AMyCharacter", "AMYCHARACTER_EXECFORCEDUNCROUCH"),
    ("?Tick@AMyCharacter", "AMYCHARACTER_TICK"),
    ("FApp::DeltaTime", "FAPP_DELTATIME"),
];

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: {} <exe> <pdb>", args[0]);
    }
    let exe = &args[1];
    let pdb = &args[2];
    let mut exe = File::open(exe.as_str()).expect("Couldn't open exe");
    let pdb = File::open(pdb.as_str()).expect("Couldn't open pdb");
    let mut binary = Vec::new();
    exe.read_to_end(&mut binary).unwrap();
    let pe = match Object::parse(&binary).expect("Couldn't parse exe") {
        Object::PE(pe) => pe,
        _ => panic!("Exe is not a PE")
    };

    let mut pdb = PDB::open(&pdb).expect("Couldn't read pdb");
    let table = pdb.global_symbols().expect("Couldn't find global symbol table");
    let mut iter = table.iter();
    let mut consts = Vec::new();
    while let Some(symbol) = iter.next().expect("Error getting next symbol") {
        let symbol_data = symbol.parse().expect("Error parsing symbol");

        let (segment, offset) = match symbol_data {
            SymbolData::PublicSymbol { function: true, segment, offset, .. } => (segment, offset),
            SymbolData::DataSymbol { segment, offset, .. } => (segment, offset),
            _ => continue
        };
        let name = match symbol.name() {
            Ok(name) => name.to_string(),
            Err(e) => { eprintln!("Error getting symbol name: {}", e); continue }
        };
        for &(start, _) in &NAMES {
            if name.starts_with(start) {
                match pe.sections.get((segment as usize).wrapping_sub(1)) {
                    Some(section) => consts.push((name.clone(), section.virtual_address + offset)),
                    None => eprintln!("Error getting section")
                }
            }
        }
    }

    if consts.len() != NAMES.len() {
        panic!("Did not find all names. Only got {:?}\nof {:?}", consts, NAMES);
    }

    let mut s = String::new();
    for (name, addr) in consts {
        let name = NAMES.iter()
            .filter(|&&(start, _)| name.starts_with(start))
            .map(|&(_, name)| name)
            .next().unwrap();
        s += &format!("pub const {}: usize = {:#x};\n", name, addr)
    }
    println!("{}", s);
    let mut file = File::create("../rtil/src/native/windows/consts.rs").unwrap();
    file.write_all(s.as_bytes()).unwrap();
}
