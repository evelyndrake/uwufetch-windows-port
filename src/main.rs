#![allow(unused_imports)]
#![allow(non_camel_case_types)]
#![allow(deprecated)]
use winver::WindowsVersion;
use sysinfo::{
    Disks, Networks, System,
};
use regex::Regex;
use wmi::{COMLibrary, WMIConnection, Variant};
use serde::Deserialize;
use std::{collections::HashMap, io::Write};
use whoami;
use console::{style, Style, Term};
use dialoguer::{theme::ColorfulTheme, MultiSelect};
use displayz::{query_displays, refresh, Resolution};
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::io::prelude::*;
use std::path::Path;
use std::env;
#[derive(Deserialize, Debug)]

struct Win32_VideoController {
    name: String,
}

struct OS {
    name: String,
    version: String,
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

fn uwu_letter_replace(text: &str, dont_replace: &bool) -> String {
    
    if dont_replace == &true {
        return text.to_string();
    }
    let replacements = [
        ("r", "w"),
        ("l", "w"),
        ("R", "W"),
        ("L", "W"),
        // ("o", "owo"),
        // ("u", "uwu"),
        ("O", "Owo"),
        ("U", "Uwu"),
        ("ew", "euwu"),
        ("Ne", "Nye"),
        ("ne", "nye")
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

fn setup_disks(replace: &bool) -> Vec<String> {
    let disks = Disks::new_with_refreshed_list();
    let mut disk_strings = Vec::new();
    for disk in &disks {
        let disk_name = disk.name();

        // Remove quotes from disk name
        let disk_name = disk_name.to_string_lossy();
        // if disk_name == "" {
        //     continue;
        // }
        // Get drive letter from mount point
        let disk_mount_point = disk.mount_point().to_string_lossy();
        // Remove last 2 chars
        let disk_mount_point = &disk_mount_point[..disk_mount_point.len() - 2];
        let mut disk_name_string = disk_name.to_string();
        if disk_name == "" {
            disk_name_string = "Local Disk".to_string();
        }
        let disk_name = format!("{} ({})",disk_name_string.replace("\"", ""), disk_mount_point);

        let disk_size = disk.total_space();
        let disk_size_gb = disk_size as f64 / 1024.0 / 1024.0 / 1024.0;
        let disk_used = disk.available_space();
        let disk_used_gb = disk_used as f64 / 1024.0 / 1024.0 / 1024.0;
        let disk_free_gb = disk_size_gb - disk_used_gb;
        let disk_string = format!("{}: {:.2} GB / {:.2} GB", disk_name, disk_free_gb, disk_size_gb);
        disk_strings.push(uwu_letter_replace(disk_string.as_str(), replace));
    }
    disk_strings
}

fn setup_network_adapters(replace: &bool) -> Vec<String> {
    let networks = Networks::new_with_refreshed_list();
    let mut network_strings = Vec::new();
    for (interface_name, data) in &networks {
        let data_recieved_string = format!("{:.2} MB ↓", data.total_received() as f64 / 1024.0 / 1024.0);
        let data_transmitted_string = format!("{:.2} MB ↑", data.total_transmitted() as f64 / 1024.0 / 1024.0);
        let data = format!("{} / {}", data_recieved_string, data_transmitted_string);
        network_strings.push(format!("{}: {}", uwu_letter_replace(interface_name.as_str(), replace), data));
    }
    network_strings
}

fn return_ascii_line_by_line(filename: &String) -> io::Result<Vec<String>> {
    let mut ascii_file_path = format!("./res/ascii/{}.txt", filename);
    let file = File::open(&ascii_file_path).or_else(|_| {
        // Fallback to user's home folder /uwufetch/windows.txt
        ascii_file_path = format!("./uwufetch/windows.txt");
        File::open(&ascii_file_path)
    });

    match file {
        Ok(file) => {
            let reader = BufReader::new(file);
            let mut lines = Vec::new();
            for line in reader.lines() {
                let mut line = line?;
                replace_placeholders(&mut line);
                lines.push(line);
            }
            Ok(lines)
        }
        Err(_) => {
            Err(io::Error::new(io::ErrorKind::Other, "Failed to open file"))
        }
    }

}

fn replace_placeholders(line: &mut String) {
    let replacements = vec![
        ("{NORMAL}", "\x1B[0m"),
        ("{BOLD}", "\x1b[1m"),
        ("{BLACK}", "\x1b[30m"),
        ("{RED}", "\x1b[31m"),
        ("{GREEN}", "\x1b[32m"),
        ("{SPRING_GREEN}", "\x1b[38;5;120m"),
        ("{YELLOW}", "\x1b[33m"),
        ("{BLUE}", "\x1b[34m"),
        ("{MAGENTA}", "\x1b[0;35m"),
        ("{CYAN}", "\x1b[36m"),
        ("{WHITE}", "\x1b[37m"),
        ("{PINK}", "\x1b[38;5;201m"),
        ("{LPINK}", "\x1b[38;5;213m"),
        ("{BLOCK}", "█"),
        ("{BLOCK_VERTICAL}", "█")
    ];

    for (placeholder, replacement) in replacements {
        *line = line.replace(placeholder, replacement);
    }
    
}

// TODO: Clean up and split into multiple files
fn main() {
    let args: Vec<String> = env::args().collect();

    // Check if config file in working directory exists
    let mut config_file = Path::new("./uwufetch.cfg");
    if !config_file.exists() {
        // Check if config file in home directory exists
        config_file = Path::new("./uwufetch/uwufetch.cfg");
    }

    if config_file.exists() {
        // Do nothing
    } else {
        // Create config file
        let mut file = File::create(config_file).unwrap();
        file.write_all(b"false\nfalse\nfalse").unwrap();
    }
    // Parse config file and load bools into defaults
    let config = std::fs::read_to_string(config_file).unwrap();
    let defaults = config.lines().map(|line| line.parse::<bool>().unwrap()).collect::<Vec<bool>>();
    
    // Disable letter replacement if true
    let replace = &defaults[2];

    // If run with t flag
    if args.len() > 1 && args[1] == "-c" {
        // let defaults = &[false, false];
        let options =&[
            "No user@hostname display",
            "No artwork",
            "No letter replacement for arbitrary text",
        ];
        let selections = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Configure (space to select, enter to continue)")
        .items(&options[..])
        .defaults(&defaults[..])
        .interact()
        .unwrap();
        // Write selections to config file
        let mut file = File::create(config_file).unwrap();
        // If "No colors" is selected, write true to first line
        if selections.contains(&0) {
            file.write_all(b"true\n").unwrap();
        } else {
            file.write_all(b"false\n").unwrap();
        }
        // If "No artwork" is selected, write true to second line
        if selections.contains(&1) {
            file.write_all(b"true\n").unwrap();
        } else {
            file.write_all(b"false\n").unwrap();
        }
        // If "No letter replacement" is selected, write true to third line
        if selections.contains(&2) {
            file.write_all(b"true").unwrap();
        } else {
            file.write_all(b"false").unwrap();
        }
        return;
    }

    let _green = Style::new().green().bold();
    let _red = Style::new().red().bold();
    let _yellow = Style::new().yellow().bold();
    let _blue = Style::new().blue().bold();
    let _magenta = Style::new().magenta().bold();
    let _cyan = Style::new().cyan().bold();
    let _white = Style::new().white().bold();
    let _pink = Style::new().on_magenta().black().bold();
    let _lpink = Style::new().on_magenta().black().bold();
    let _spring_green = Style::new().on_magenta().black().bold();
    let _bold = Style::new().bold();
    let _username = Style::new().magenta().italic();
    let _regular = Style::new().white();
    let sub = Style::new().white().dim();
    let none = Style::new();
    let header = _cyan;

    let ascii_lines = return_ascii_line_by_line(&"windows".to_string()).unwrap();
    
    let term = Term::stdout();
    let space_offset_num = 23;
    let space_offset = " ".repeat(space_offset_num);
    let min_title_length = 12;
    let mut current_line = 0;


    let mut write_spec = |title: &str, text: String, style_type: &Style| {
        let mut current_line_string = "".to_string();
        if defaults[1] == false { // If artwork is enabled
            current_line_string = space_offset.clone();
            if current_line < ascii_lines.len() {
                current_line_string = ascii_lines[current_line].clone() + "    ";
            }
        }
        
        
        current_line += 1;
        let title_string = title.to_string();
        // if the title length is less than the minimum title length, add spaces to the end of the title
        let title_string: String = if title.len() < min_title_length {
            format!("{}{}", title, " ".repeat(min_title_length - title.len()))
        } else {
            title_string
        };
        term.write_line(
            format!("{}{}\x1B[0m{}", current_line_string, style_type.apply_to(style(title_string)), style(text).white()).as_str(),
        ).unwrap();
        
    };

    let username_plaintext = uwu_letter_replace(whoami::username().as_str(), replace);
    let hostname_plaintext = uwu_letter_replace(whoami::hostname().as_str(), replace);
    let username_string_plaintext = format!("{}@{}", username_plaintext, hostname_plaintext);

    if defaults[0] == false { // If user@hostname display is enabled
        let username_string = format!("{}{}{}",
        style(username_plaintext).magenta().italic(),
        style("@").dim(),
        style(hostname_plaintext).yellow());

        write_spec(username_string.as_str(), "".to_string(), &none);

        write_spec("-".repeat(username_string_plaintext.len()).as_str(), "".to_string(), &_regular);
    }
    

    let os = setup_os();
    write_spec("OS", format!("{} {}", os.name, os.version), &header);


    let sys = System::new_all();
    write_spec("CPUWU", setup_cpu(&sys), &header);

    let gpus = setup_gpu();
    let mut gpu_num = 0;
    write_spec("GPUWU", "".to_string(), &header);
    for gpu in gpus {
        gpu_num += 1;
        write_spec(format!("  {}", gpu_num).as_str(), gpu, &sub);
    }

    write_spec("WAM", setup_ram(&sys), &header);

    write_spec("RESOWUTION", setup_resolution(), &header);

    write_spec("DISKS", "".to_string(), &header);
    let disk_strings = setup_disks(replace);
    let mut disk_num = 0;
    for disk_string in disk_strings {
        disk_num += 1;
        write_spec(format!("  {}", disk_num).as_str(), disk_string, &sub);
    }

    
    write_spec("NETWOWORK", "".to_string(), &header);
    let network_strings: Vec<String> = setup_network_adapters(replace);
    let mut network_num = 0;
    for network_string in network_strings {
        network_num += 1;
        write_spec(format!("  {}", network_num).as_str(), network_string, &sub);
    }
}
