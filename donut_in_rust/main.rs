// Author of the port to Rust: Joao Nuno Carvalho
// Date: 2020.09.11
// Original code of demo.c in C made by Andy Sloane.
// See [Have a donut - obfuscated c donut](https://www.a1k0n.net/2006/09/15/obfuscated-c-donut.html)
// And this project page on github user joaocarvalhoopen.
// License: For my part of the port is MIT Open License but the original code
// in C and in Javascript doesn't have a license written, that I could find.


use std::{thread, time};
use slice_fill::SliceExt;

const max_buf_size: usize = 1760;


#[warn(unstable_name_collisions)]
fn main() {
    let lux = ['.', ',', '-', '~', ':', ';', '=', '!', '*', '#', '$', '@'];

    let mut A: f32 = 0.0_f32;
    let mut B: f32 = 0.0_f32;
    let mut i: f32;
    let mut j: f32;
    let mut z: Vec<f32>  = vec![0.0; max_buf_size];
    let mut b: Vec<char> = vec![' '; max_buf_size];

    println!("\x1b[2J");

    loop {        
        b.fill(32 as char);
        z.fill(0.0);
        j = 0.0_f32;
        while 6.28_f32 > j {
            i = 0.0_f32;
            while 6.28_f32 > i {
                let c = i.sin();
                let d = j.cos();
                let e = A.sin();
                let f = j.sin();
                let g = A.cos();
                let h: f32 = d + 2.0;
                let D: f32 = 1.0 / (c * h * e + f * g + 5.0);
                let l = i.cos();
                let m = B.cos();
                let n = B.sin();
                let t: f32 = c * h * g - f * e;
                let x: i32 = (40.0 + 30.0 * D * (l * h * m - t * n)) as i32;
                let y: i32 = (12.0 + 15.0 * D * (l * h * n + t * m)) as i32;
                let o: i32 = x + 80 * y;
                let N: i32 = (8.0 * ((f * e - c * d * g) * m - c * d * e - f * g - l * d * n)) as i32;
                if (22 > y) && (y > 0) && (x > 0) && (80 > x) && (D > z[o as usize]) {
                  z[o as usize] = D;
                  b[o as usize] = lux[if N > 0 {N as usize} else { 0 }];
                }
                i += 0.02_f32;
            } /*#****!!-*/
            j += 0.07_f32;
        }
        println!("\x1b[H");
        for k in 0..max_buf_size {
            print!("{}", if k % 80 != 0 { b[k as usize] } else {10 as char});
        }
        A += 0.04;
        B += 0.02;
        thread::sleep(time::Duration::from_millis(10));
    }
}
