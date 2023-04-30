use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::process;

use goblin::Object;
use pdb::{PDB, PublicSymbol, SymbolData, DataSymbol};
use pdb::FallibleIterator;

const NAMES: &[(&str, &str)] = &[
    ("?Tick@FSlateApplication", "FSLATEAPPLICATION_TICK"),
    ("?OnKeyDown@FSlateApplication", "FSLATEAPPLICATION_ONKEYDOWN"),
    ("?OnKeyUp@FSlateApplication", "FSLATEAPPLICATION_ONKEYUP"),
    ("?OnRawMouseMove@FSlateApplication", "FSLATEAPPLICATION_ONRAWMOUSEMOVE"),
    ("?PumpMessages@FWindowsPlatformMisc@@SAX_N@Z", "FPLATFORMMISC_PUMPMESSAGES"),
    ("?Tick@AMyCharacter", "AMYCHARACTER_TICK"),
    ("?ForcedUnCrouch@AMyCharacter", "AMYCHARACTER_FORCEDUNCROUCH"),
    ("?UpdateTimeAndHandleMaxTickRate@UEngine", "UENGINE_UPDATETIMEANDHANDLEMAXTICKRATE"),
    ("FApp::DeltaTime", "FAPP_DELTATIME"),
    ("?Malloc@FMemory@@SAPAXKI@Z", "FMEMORY_MALLOC"),
    ("?Free@FMemory@@SAXPAX@Z", "FMEMORY_FREE"),
    ("??0FName@@QAE@PB_WW4EFindName@@@Z", "FNAME_FNAME"),
    ("?DrawHUD@AMyHUD@@UAEXXZ", "AMYHUD_DRAWHUD"),
    ("?DrawLine@AHUD@@QAEXMMMMUFLinearColor@@M@Z", "AHUD_DRAWLINE"),
    ("?DrawText@AHUD@@QAEXABVFString@@UFLinearColor@@MMPAVUFont@@M_N@Z", "AHUD_DRAWTEXT"),
    ("?DrawTextureSimple@AHUD@@QAEXPAVUTexture@@MMM_N@Z", "AHUD_DRAWTEXTURESIMPLE"),
    ("?Project@AHUD@@QBE?AUFVector@@U2@@Z", "AHUD_PROJECT"),
    ("?GetTextSize@AHUD@@QBEXABVFString@@AAM1PAVUFont@@M@Z", "AHUD_GETTEXTSIZE"),
    ("GWorld", "GWORLD"),
    ("?SpawnActor@UWorld@@QAEPAVAActor@@PAVUClass@@PBUFVector@@PBUFRotator@@ABUFActorSpawnParameters@@@Z", "UWORLD_SPAWNACTOR"),
    ("?DestroyActor@UWorld@@QAE_NPAVAActor@@_N1@Z", "UWORLD_DESTROYACTOR"),
    ("?StaticClass@AMyCharacter@@SAPAVUClass@@XZ", "AMYCHARACTER_STATICCLASS"),
    ("?GetViewportSize@APlayerController@@QBEXAAH0@Z", "APLAYERCONTROLLER_GETVIEWPORTSIZE"),
    ("?SpawnDefaultController@APawn@@UAEXXZ", "APAWN_SPAWNDEFAULTCONTROLLER"),
    ("?SetActorEnableCollision@AActor@@QAEX_N@Z", "AACTOR_SETACTORENABLECOLLISION"),
    ("?GetAccurateRealTime@UGameplayStatics@@SAXPBVUObject@@AAHAAM@Z", "UGAMEPLAYSTATICS_GETACCURATEREALTIME"),
    ("?CreateTransient@UTexture2D@@SAPAV1@HHW4EPixelFormat@@@Z", "UTEXTURE2D_CREATETRANSIENT"),
    ("?GetRunningPlatformData@UTexture2D@@UAEPAPAUFTexturePlatformData@@XZ", "UTEXTURE2D_GETRUNNINGPLATFORMDATA"),
    ("?UpdateResourceW@UTexture2D@@UAEXXZ", "UTEXTURE2D_UPDATERESOURCE"),
    ("?Lock@FUntypedBulkData@@QAEPAXI@Z", "FUNTYPEDBULKDATA_LOCK"),
    ("?Unlock@FUntypedBulkData@@QBEXXZ", "FUNTYPEDBULKDATA_UNLOCK"),
];

fn get_linux_level_pointer_path() -> String {
    "pub const LEVEL_POINTER_PATH: &[usize] = &[0x4c68838, 0x138, 0x140];".to_string()
}

fn get_windows_level_pointer_path() -> String {
    let res = ureq::get("https://raw.githubusercontent.com/BatedUrGonnaDie/Autosplitters/master/Refunct/Refunct.asl")
        .call()
        .unwrap()
        .into_string()
        .unwrap();
    for line in res.lines() {
        let line = line.trim();
        if !(line.starts_with("int") && line.contains("level")) {
            continue;
        }
        let addr_path = line.split(':').nth(1).unwrap();
        let addr_path = addr_path.split(';').next().unwrap().trim();
        return format!("pub const LEVEL_POINTER_PATH: &[usize] = &[{}];", addr_path);

    }
    panic!("Windows level-pointer-path not found.");
}

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: {} <exe> <pdb>", args[0]);
        process::exit(1);
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

        let offset = match symbol_data {
            SymbolData::Public(PublicSymbol { function: true, offset, .. }) => offset,
            SymbolData::Data(DataSymbol { offset, .. }) => offset,
            _ => continue
        };
        let name = match symbol_data.name() {
            Some(name) => name.to_string(),
            None => { eprintln!("Error getting symbol name"); continue }
        };
        let section = pe.sections.get((offset.section as usize).wrapping_sub(1));
        println!("{:<#10x} {}", section.map(|s| s.virtual_address + offset.offset).unwrap_or(0), name);
        for &(start, _) in NAMES {
            if name.starts_with(start) {
                match section {
                    Some(section) => consts.push((name.clone(), section.virtual_address + offset.offset)),
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
    // writeln!(file, "{}", get_windows_level_pointer_path()).unwrap();

    // Linux level pointer path
    // let mut file = File::create("../rtil/src/native/linux/consts.rs").unwrap();
    // writeln!(file, "{}", get_linux_level_pointer_path()).unwrap();
}
