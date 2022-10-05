use std::process::Command;
use std::process::Output;
use std::str;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let com_port: &str = &args[1].as_str();
    let file_dir: &str = &args[2].as_str();

    reset_port(com_port);
    
    let Output = match Command::new("avrdude")
        .args(["-cstk500v2", "-pm2560", "-v", "-q", "-D", &format!("-Uflash:w:{}.hex:i", file_dir), "-b115200", &format!("-P{}", com_port)])
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