#![warn(clippy::pedantic)]
#![allow(clippy::non_ascii_literal)]

use askama::Template;
use barcoders::sym::code128::Code128;
use std::str::FromStr;

#[derive(Debug, Template)]
#[template(path = "labels.svg", escape = "none")]
struct Labels {
    barcodes: Vec<Barcode>,
}

#[derive(Debug)]
struct Barcode {
    x: &'static str,
    y: usize,
    s: String,
    lines: Vec<usize>,
}

fn main() -> Result<(), String> {
    let start = std::env::args().nth(1).ok_or("missing starting barcode")?;
    let start = u64::from_str(&start).map_err(|e| format!("starting barcode: {}", e))?;

    let labels = Labels {
        barcodes: (start..99_999_999_999_999)
            .filter(|n| primal::is_prime(*n))
            .take(80)
            .enumerate()
            .map(|(i, barcode)| {
                let x = ["21.6", "169.2", "316.8", "464.4"][i / 20];
                let y = i % 20 * 36 + 36;
                let s = format!("{:014}", barcode);
                // The barcode data is guaranteed to be 14 digits long, and Code128::new does
                // not panic in character set C when given an even number of digits.
                let lines = Code128::new(&format!("Ć{}", s))
                    .expect("cannot fail")
                    .encode()
                    .into_iter()
                    .enumerate()
                    .filter_map(|(i, x)| if x == 1 { Some(i) } else { None })
                    .collect();
                Barcode { x, y, s, lines }
            })
            .collect(),
    };
    println!("{}", labels);
    Ok(())
}
