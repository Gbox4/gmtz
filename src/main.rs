// Gabe Banks 1/21/22 <https://gabebanks.net>

extern crate chrono;
use chrono::{Duration,Utc};

use std::{env, process::exit};

fn print_help() {
    println!("gmtz - get the current time in another timezone.\nUSAGE: gmtz TIMEZONE\nEXAMPLE: gmtz EST");
    exit(1);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        print_help();
    }

    let tz = &args[1].to_uppercase();

    const TZ_COUNT: usize = 28;
    // Data from https://publib.boulder.ibm.com/tividd/td/TWS/SC32-1274-02/en_US/HTML/SRF_mst273.htm
    const TZ_CODES: [&str; TZ_COUNT] = ["GMT", "UTC", "ECT", "EET", "ART", "EAT", "NET", "PLT", "BST", "VST", "CTT", "JST", "AET", "SST", "NST", "MIT", "HST", "AST", "PST", "PNT", "MST", "CST", "EST", "IET", "PRT", "AGT", "BET", "CAT"];
    const TZ_OFFSETS: [i64; TZ_COUNT] = [0, 0, 1, 2, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, -11, -10, -9, -8, -7, -7, -6, -5, -5, -4, -3, -3, -1];

    let mut i = 0;
    while TZ_CODES[i] != tz {
        i = i + 1;
        if i == TZ_COUNT {
            println!("error: invalid timezone {}", tz);
            exit(1);
        }
    }

    let dt = Utc::now() + Duration::hours(TZ_OFFSETS[i]);
    println!("{}", dt.format("%a %R"));
}

