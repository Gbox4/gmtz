// Gabe Banks 1/21/22 <https://gabebanks.net>

use std::{env, process::exit};

fn print_help() {
    println!("gmtz - get the GMT offset of a timezone.\nUSAGE: gmtz TIMEZONE");
    exit(1);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        print_help();
    }

    let tz = &args[1].to_uppercase();

    const TZ_COUNT: usize = 32;
    // Data from https://publib.boulder.ibm.com/tividd/td/TWS/SC32-1274-02/en_US/HTML/SRF_mst273.htm
    const TZ_CODES: [&str; TZ_COUNT] = ["GMT", "UTC", "ECT", "EET", "ART", "EAT", "MET", "NET", "PLT", "IST", "BST", "VST", "CTT", "JST", "ACT", "AET", "SST", "NST", "MIT", "HST", "AST", "PST", "PNT", "MST", "CST", "EST", "IET", "PRT", "CNT", "AGT", "BET", "CAT"];
    const TZ_OFFSETS: [&str; TZ_COUNT] = ["GMT", "GMT", "GMT+1:00", "GMT+2:00", "GMT+2:00", "GMT+3:00", "GMT+3:30", "GMT+4:00", "GMT+5:00", "GMT+5:30", "GMT+6:00", "GMT+7:00", "GMT+8:00", "GMT+9:00", "GMT+9:30", "GMT+10:00", "GMT+11:00", "GMT+12:00", "GMT-11:00", "GMT-10:00", "GMT-9:00", "GMT-8:00", "GMT-7:00", "GMT-7:00", "GMT-6:00", "GMT-5:00", "GMT-5:00", "GMT-4:00", "GMT-3:30", "GMT-3:00", "GMT-3:00", "GMT-1:00"];

    let mut i = 0;
    while TZ_CODES[i] != tz {
        i = i + 1;
        if i == TZ_COUNT {
            println!("error: invalid timezone {}", tz);
            exit(1);
        }
    }

    println!("{}", TZ_OFFSETS[i]);
}