use std::process::Command;
use std::str;
use std::env;

const STANDERD_FILE_NAME: &str = "main";

fn main() {
    let args: Vec<String> = env::args().collect();

    let com_port: &str = &args[1].as_str();
    let file_name: &str = if args.len() < 3 { STANDERD_FILE_NAME } else { &args[2].as_str() };

    if !std::path::Path::new(&format!("{}.hex", file_name)).exists() {
        compile_file(file_name);
    };
    
    upload_file(file_name, com_port);
}

fn compile_file(file_name: &str) {
    let compile_out = match Command::new("avr-gcc")
        .args(["-O2", "-Wall", "-mmcu=atmega2560", &format!("{}.c", file_name), "-o", &format!("{}.out", file_name)])
        .output() {
            Ok(compile_out) => compile_out,
            Err(e) => panic!("Cant compile to out file because {e}"),
        };
    let compile_out_string = str::from_utf8(&compile_out.stderr).unwrap();
    println!("{}", compile_out_string);
    
    let compile_hex = match Command::new("avr-objcopy")
        .args(["-O", "ihex", &format!("{}.out", file_name), &format!("{}.hex", file_name)])
        .output() {
            Ok(compile_hex) => compile_hex,
            Err(e) => panic!("Cant compile to out file because {e}"),
        };
    let compile_hex_string = str::from_utf8(&compile_hex.stderr).unwrap();
    println!("{}", compile_hex_string);
}

fn upload_file(file_name: &str, com_port: &str) {
    reset_port(com_port);

    let output = match Command::new("avrdude")
        .args(["-cstk500v2", "-pm2560", "-v", "-q", "-D", &format!("-Uflash:w:{}.hex:i", file_name), "-b115200", &format!("-P{}", com_port)])
        .output() {
            Ok(output) => output,
            Err(e) => panic!("Cant program board because {e}"),
        };

    let output_string = str::from_utf8(&output.stderr).unwrap();
    println!("{}", output_string)
}

fn reset_port(com_port: &str) {
    let mut port  = match serialport::new(com_port, 1200)
        .stop_bits(serialport::StopBits::One)
        .data_bits(serialport::DataBits::Eight)
        .parity(serialport::Parity::None)
        .open() {
            Ok(output) => output,
            Err(e) => panic!("Cant open the serial port because {e}")
        };
        
    port.write_data_terminal_ready(true).unwrap();
    port.write_data_terminal_ready(false).unwrap();
}
