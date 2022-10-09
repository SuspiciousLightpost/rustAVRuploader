use std::ffi::OsStr;
use std::process::Command;
use std::str;
use std::env;

const STANDERD_FILE_NAME: &str = "main";
const MICROCONTROLLER_UNIT: &str = "atmega2560";

const DUDE: &str = "avrdude";
const GCC: &str = "avrgcc";
const OBJCOPY: &str = "avr-objcopy";

fn main() {
    let args: Vec<String> = env::args().collect();

    let com_port: &str = &args[1].as_str();
    let file_name: &str = if args.len() < 3 { STANDERD_FILE_NAME } else { &args[2].as_str() };

    if !std::path::Path::new(&format!("{}.hex", file_name)).exists() {
        /* Compiling the c file to out file */
        run_command(GCC, vec!["-O2", "-Wall", &format!("-mmcu={}", MICROCONTROLLER_UNIT), &format!("{}.c", file_name), "-o", &format!("{}.out", file_name)]);

        /* Compiling the out file to hex file */
        run_command(OBJCOPY, vec!["-O", "ihex", &format!("{}.out", file_name), &format!("{}.hex", file_name)]);
    };
    
    /* Sending a reset signal to the board */
    reset_port(com_port);

    /* Uploading the hex file to the board */
    run_command(DUDE, vec!["-cstk500v2", "-pm2560", "-v", "-q", "-D", &format!("-Uflash:w:{}.hex:i", file_name), "-b115200", &format!("-P{}", com_port)])
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

fn run_command(program_name: &str, command_args: impl IntoIterator<Item = impl AsRef<OsStr>>) {
    let output = match Command::new(program_name)
        .args(command_args)
        .output() {
            Ok(output) => output,
            Err(e) => panic!("{e}"),
        };

    let output_string = str::from_utf8(&output.stderr).unwrap();
    println!("{}", output_string)
}