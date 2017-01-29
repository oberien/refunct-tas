use std::process::Command;

#[cfg(unix)]
pub fn pidof() -> u32 {
    let output = Command::new("pidof")
        .arg("Refunct-Linux-Shipping")
        .output()
        .expect("Cannot get pid of Refunct");
    let mut s = String::from_utf8(output.stdout).expect("Output of pidof is not utf8");
    assert_eq!(s.pop(), Some('\n'), "could not get pid of Refunct");
    s.parse().expect("Pidof returned non-number")
}

#[cfg(windows)]
pub fn pidof() -> u32 {
    let output = Command::new("wmic")
        .args(&["process", "where", "Name='Refunct-Win32-Shipping.exe'", "get", "ProcessId"])
        .output()
        .expect("Cannot get pid of Refunct");
    let s = String::from_utf8(output.stdout).expect("Output of pidof is not utf8");
    let mut lines = s.lines();
    assert_eq!(lines.next().map(|s| s.trim()), Some("ProcessId"), "could not get pid of Refunct");
    lines.next().expect("No line containing pid").trim().parse().expect("Pidof returned non-number")
}
