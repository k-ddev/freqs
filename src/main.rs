/*
    A tiny, basic frequency analysis tool. Given a file, displays a table
    of each byte encountered in the file and the number of times it occurs
    in the file. Uses BufReader in order to function for files of arbitrary
    size.
*/

use std::{
    env,
    fs,
    panic,
    str
};
use std::io::{BufReader, stdout, Write};
use std::io::prelude::*;

fn main() {
    // collect and parse args
    let mut h_flag = false;           // usage
    let mut in_path = None::<String>;  // infile
    let mut out_path = None::<String>; // outfile
    let args: Vec<String> = env::args().collect();
    for (i, argv) in args.iter().enumerate() {
        if i != 0 {
            match argv.as_ref() {
                "-h" => h_flag = true,
                "-o" => out_path = args.get(i + 1).cloned(),
                arg => if let Some(prev) = args.get(i - 1) {
                    if prev != "-o" { in_path = Some(String::from(arg)); }
                },
            };
        }
    }

    // exit early if usage option is specified
    if h_flag { // display usage
        println!("
Usage:
    freqs <path to file>
        performs analysis on target file,
        then prints results as stdout.

    freqs <path to target file> -o <outfile>
        performs analysis on target file,
        then prints results to outfile. if
        o flag is specified with no outfile,
        prints to stdout instead.
");
    } else if in_path == None {
        println!("Not enough arguments. try passing -h");
    } else { // main execution
        panic::set_hook(Box::new(|panic_info| {
            println!("Error: {}\nAborting", panic_info.payload().downcast_ref::<&str>().unwrap());
        }));

        // set up in and out files
        let target = match fs::File::open(in_path.clone().unwrap()) {
                Ok(f) => f,
                Err(_) => panic!("Could not open file. Bad file or path?")
        };

        let out_file = match out_path {
            None => None,
            Some(path) => Some(fs::OpenOptions::new()
                .write(true)
                .append(true)
                .create(true)
                .open(path)
                .unwrap()),
        };

        // set up bufreader, chunks, and byte occurence counts
        const CHUNKSIZE: usize = 1024 * 128;
        let mut reader = BufReader::with_capacity(CHUNKSIZE, target);
        let chunks_total = (fs::metadata(in_path.unwrap()).unwrap().len() / CHUNKSIZE as u64) as u32;
        let mut chunks_done: u32 = 0;
        let mut byte_occurences = [0u32; 256];

        // break file into chunks
        loop {
            // process next chunk if any
            let length_of_chunk = {
                if let Ok(chunk) = reader.fill_buf() {
                    // count occurences of each byte in chunk
                    for byte in chunk.iter() { byte_occurences[*byte as usize] += 1; }

                    // return length of chunk done
                    chunk.len()
                } else { 0 }
            };

            if length_of_chunk == 0 { break; } else {
                // update and display progress
                chunks_done += 1;
                print!("\rprocessed chunk {} / {}", chunks_done, chunks_total);
                stdout().flush();

                // we're done with this chunk
                reader.consume(length_of_chunk);
            }
        }

        println!("\ndone!");

        // turn results into table
        let mut lines = vec![String::from("")];
        for (byte, byte_count) in byte_occurences.iter().enumerate() {
            if *byte_count != 0 {
                lines.push(format!(
                    "  {0: <3}: {1}: {2}",
                    format!("{:x}", byte as u8),
                    byte_count,
                    match byte as u8 {
                        // gross-ass to_string()s on all of these because I got
                        // tired of fucking around trying to get a str from
                        // format!() to live long enough.
                        0x00 => "<NULL>".to_string(),
                        0x01 => "<SOH>".to_string(),
                        0x02 => "<STX>".to_string(),
                        0x03 => "<ETX>".to_string(),
                        0x04 => "<EOT>".to_string(),
                        0x05 => "<ENQ>".to_string(),
                        0x06 => "<ACK>".to_string(),
                        0x07 => "<BEL".to_string(),
                        0x08 => "<BS>".to_string(),
                        0x09 => "<TAB>".to_string(),
                        0x0a => "\\n".to_string(),
                        0x0b => "<VT>".to_string(),
                        0x0c => "<FF>".to_string(),
                        0x0d => "\\r".to_string(),
                        0x0e => "<SO>".to_string(),
                        0x0f => "<SI>".to_string(),
                        0x10 => "<DLE>".to_string(),
                        0x11 => "<DC1>".to_string(),
                        0x12 => "<DC2>".to_string(),
                        0x13 => "<DC3>".to_string(),
                        0x14 => "<DC4>".to_string(),
                        0x15 => "<NAK>".to_string(),
                        0x16 => "<SYN>".to_string(),
                        0x17 => "<ETB>".to_string(),
                        0x18 => "<EM>".to_string(),
                        0x19 => "<SUB>".to_string(),
                        0x1a => "<SUB>".to_string(),
                        0x1b => "<ESC>".to_string(),
                        0x1c => "<FS>".to_string(),
                        0x1d => "<GS>".to_string(),
                        0x1e => "<RS>".to_string(),
                        0x1f => "<US>".to_string(),
                        0x20 => "<space>".to_string(),
                        0x7f => "<DEL>".to_string(),
                        0xa0 => "<non break space>".to_string(),
                        0xad => "<soft hyphen>".to_string(),

                        b => format!("{}", b as char)
                    }
                ));
            }
        }

        // either save table in file, or else print as stdout
        if let Some(mut f) = out_file {
            for line in lines {
                if let Err(e) = writeln!(f, "{}", line) { println!("{}", e); }
            }
        } else { for line in lines { println!("{}", line); } }
    }
}
