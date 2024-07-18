#![allow(unused_imports)]
#![allow(non_camel_case_types)]
use winver::WindowsVersion;
use sysinfo::{
    Disks, Networks, System,
};
use regex::Regex;
use wmi::{COMLibrary, WMIConnection, Variant};
use serde::Deserialize;
use std::collections::HashMap;
use whoami;
struct OS {
    name: String,
    version: String,
}
use displayz::{query_displays, refresh, Resolution};

#[derive(Deserialize, Debug)]
struct Win32_VideoController {
    name: String,
}



fn uwu_hw(hwname: &mut String) {
    let replacements = [
        ("lenovo", "LenOwO"),
        ("cpu", "CPUwU"),
        ("core", "Cowe"),
        ("gpu", "GPUwU"),
        ("graphics", "Gwaphics"),
        ("corporation", "COwOpowation"),
        ("nvidia", "NyaVIDIA"),
        ("mobile", "Mwobile"),
        ("intel", "Inteww"),
        ("celeron", "Celewon"),
        ("radeon", "Radenyan"),
        ("geforce", "GeFOwOce"),
        ("raspberry", "Nyasberry"),
        ("broadcom", "Bwoadcom"),
        ("motorola", "MotOwOwa"),
        ("proliant", "ProLinyant"),
        ("poweredge", "POwOwEdge"),
        ("apple", "Nyapple"),
        ("electronic", "ElectrOwOnic"),
        ("processor", "Pwocessow"),
        ("microsoft", "MicOwOsoft"),
        ("ryzen", "Wyzen"),
        ("advanced", "Adwanced"),
        ("micro", "Micwo"),
        ("devices", "Dewices"),
        ("inc.", "Nyanc."),
        ("lucienne", "Lucienyan"),
        ("tuxedo", "TUWUXEDO"),
        ("aura", "Uwura"),
    ];

    for &(original, uwuified) in &replacements {
        let re = Regex::new(&format!("(?i){}", original)).unwrap(); // `(?i)` enables case-insensitive matching
        *hwname = re.replace_all(hwname, uwuified).to_string();
    }
}

fn uwu_letter_replace(text: &str) -> String {
    let replacements = [
        ("r", "w"),
        ("l", "w"),
        ("R", "W"),
        ("L", "W"),
    ];

    let mut uwuified_text = text.to_string();
    for &(original, replacement) in &replacements {
        let re = Regex::new(original).unwrap(); // Compile the regex
        uwuified_text = re.replace_all(&uwuified_text, replacement).to_string(); // Perform the replacement
    }

    uwuified_text // Return the modified string
}


fn setup_os() -> OS {
    let version = WindowsVersion::detect().unwrap();
    let os = OS {
        name: "MicOwOsoft WinyandOwOws".to_string(),
        version: version.to_string(),
    };
    os
}

fn setup_cpu(system: &System) -> String {
    let cpu = system.cpus()[0].brand().to_string();
    let mut uwu_cpu = cpu.clone();
    uwu_hw(&mut uwu_cpu);
    uwu_cpu
}

fn setup_gpu() -> Vec<String> {
    let com_con = COMLibrary::new().unwrap();
    let wmi_con = WMIConnection::new(com_con.into()).unwrap();
    
    let results: Vec<Win32_VideoController> = wmi_con.query().unwrap();
    let mut gpus = Vec::new();
    for result in results {
        let mut gpu = result.name;
        uwu_hw(&mut gpu);
        gpus.push(gpu);
    }
    gpus
}

fn setup_ram(system: &System) -> String {
    // Get memory usage using sysinfo
    let total_ram = system.total_memory();
    let total_ram_gb = total_ram as f64 / 1024.0 / 1024.0 / 1024.0;
    format!("{:.2} GB", total_ram_gb);
    let used_ram = system.used_memory();
    let used_ram_gb = used_ram as f64 / 1024.0 / 1024.0 / 1024.0;
    format!("{:.2} GB", used_ram_gb);
    format!("{:.2} GB / {:.2} GB", used_ram_gb, total_ram_gb)
}

fn setup_resolution() -> String {
    let display_set = query_displays();
    if let Ok(display_set) = display_set {
        if let Some(settings) = display_set.primary().settings() {
            let res = (*settings).borrow().resolution;
            format!("{}x{}", res.width, res.height)
        } else {
            "Unknown".to_string()
        }
    } else {
        "Unknown".to_string()
    }
}

fn setup_disks() -> Vec<String> {
    let disks = Disks::new_with_refreshed_list();
    let mut disk_strings = Vec::new();
    for disk in &disks {
        let disk_name = disk.name();
        if disk_name == "" {
            continue;
        }
        // Remove quotes from disk name
        let disk_name = disk_name.to_string_lossy();
        if disk_name == "" {
            continue;
        }
        let disk_name = disk_name.replace("\"", "");
        let disk_size = disk.total_space();
        let disk_size_gb = disk_size as f64 / 1024.0 / 1024.0 / 1024.0;
        let disk_used = disk.available_space();
        let disk_used_gb = disk_used as f64 / 1024.0 / 1024.0 / 1024.0;
        let disk_free_gb = disk_size_gb - disk_used_gb;
        let disk_string = format!("{}: {:.2} GB / {:.2} GB", disk_name, disk_free_gb, disk_size_gb);
        disk_strings.push(uwu_letter_replace(disk_string.as_str()));
    }
    disk_strings
}

fn setup_network_adapters() -> Vec<String> {
    let networks = Networks::new_with_refreshed_list();
    let mut network_strings = Vec::new();
    for (interface_name, data) in &networks {
        let data_recieved_string = format!("{:.2} MB", data.total_received() as f64 / 1024.0 / 1024.0);
        let data_transmitted_string = format!("{:.2} MB", data.total_transmitted() as f64 / 1024.0 / 1024.0);
        let data = format!("{} / {}", data_recieved_string, data_transmitted_string);
        network_strings.push(format!("{}: {}", uwu_letter_replace(interface_name.as_str()), data));
    }
    network_strings // Add this line to return the vector
}

fn main() {
    let sys = System::new_all();
    println!("Username is: {}", uwu_letter_replace(whoami::username().as_str()));
    println!("Hostname is: {}", uwu_letter_replace(whoami::hostname().as_str()));
    let os = setup_os();
    println!("OS is: {} {}", os.name, os.version);
    println!("CPU is: {}", setup_cpu(&sys));
    let gpus = setup_gpu();
    let mut gpu_num = 0;
    for gpu in gpus {
        gpu_num += 1;
        println!("GPU {}: {}", gpu_num, gpu);
    }
    println!("RAM is: {}", setup_ram(&sys));
    println!("Resolution is: {}", setup_resolution());
    println!("Disks: (used/total)");
    let disk_strings = setup_disks();
    for disk_string in disk_strings {
        println!("{}", disk_string);
    }
    let network_strings: Vec<String> = setup_network_adapters();
    println!("Network Adapters (down/up):");
    for network_string in network_strings {
        println!("{}", network_string);
    }
    // println!("OS is: {}", get_os());
}
