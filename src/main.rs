use std::process::Command;
use std::str;

fn main() {
    {
        let mut port  = match serialport::new("COM3", 1200)
        .stop_bits(serialport::StopBits::One)
        .data_bits(serialport::DataBits::Eight)
        .parity(serialport::Parity::None)
        .open() {
            Ok(output) => output,
            Err(e) => panic!("Dit is cringe {e}")
        };
        
        port.write_data_terminal_ready(true).unwrap();
        port.write_data_terminal_ready(false).unwrap();
    }
    

    let swag = match Command::new("./avrdude")
        .args(["-cstk500v2", "-pm2560", "-v", "-q", "-D", "-Uflash:w:C:/Users/sebas/Desktop/school/MCP1/LedBlink.hex:i", "-b115200", "-PCOM3"])
        .output() {
            Ok(output) => output,
            Err(e) => panic!("Dit is best wel cringe {e}"),
        };

    let cool = str::from_utf8(&swag.stderr).unwrap();
    println!("{}", cool)
}