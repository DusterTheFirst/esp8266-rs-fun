use std::{
    io::{BufRead, BufReader, ErrorKind, Read, Write},
    process::{Command, Stdio},
    thread,
    time::Duration,
};

use color_eyre::eyre::{bail, Context};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
/// Serial monitor and control utility
struct Args {
    /// Serial port connected to the target device
    serial: String,
    /// Executable to parse defmt structures from
    #[structopt(short)]
    elf: String,
    /// Baud rate at which to monitor target device
    #[structopt(long = "speed", default_value = "115000")]
    baud_rate: u32,
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let args = Args::from_args();

    let mut serial_port = serialport::new(&args.serial, args.baud_rate)
        .timeout(Duration::from_secs(1))
        .open()
        .wrap_err("Failed to open serial port")?;

    let mut reader = BufReader::new(serial_port.try_clone()?);

    // Reset
    println!("Resetting...");
    {
        serial_port.write_data_terminal_ready(false)?;
        serial_port.write_request_to_send(true)?;

        thread::sleep(Duration::from_millis(100));

        serial_port.write_request_to_send(false)?;

        println!("Clearing serial buffer...");

        loop {
            let len = match reader.fill_buf() {
                Err(e) => match e.kind() {
                    ErrorKind::TimedOut => break,
                    _ => bail!(e),
                },
                Ok(buf) => buf.len(),
            };

            reader.consume(len);
        }

        println!("Started up");
    }

    drop(reader);

    serial_port.set_timeout(Duration::from_secs(100))?;

    let mut defmt = Command::new("defmt-print")
        .arg("-e")
        .arg(args.elf)
        .stdin(Stdio::piped())
        .stderr(Stdio::inherit())
        .stdout(Stdio::inherit())
        .spawn()
        .wrap_err("Failed to run defmt-print")?;

    let mut stdin = defmt.stdin.take().unwrap();

    println!("Sending ready signal to board");

    serial_port.write(&[0x00])?;

    let mut data = [0u8; 32];
    while let Ok(len) = serial_port.read(&mut data) {
        // println!("{:x?}", &data[..len]);
        stdin.write_all(&data[..len])?;
        stdin.flush()?;
    }

    println!("Finished reading data");
    defmt.kill()?;

    Ok(())
}
