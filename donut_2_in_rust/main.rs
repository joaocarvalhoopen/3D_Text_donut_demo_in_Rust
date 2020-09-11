// Author of the port to Rust: Joao Nuno Carvalho
// Date: 2020.09.11
// Original code in Javascript made by Andy Sloane.
// See [Have a donut - obfuscated c donut](https://www.a1k0n.net/2006/09/15/obfuscated-c-donut.html)
// And this project page on github user joaocarvalhoopen.
// License: For my part of the port is MIT Open License but the original code
// in C and in Javascript doesn't have a license written, that I could find.


use std::f32::consts::PI;
use std::{thread, time};
use slice_fill::SliceExt;

const THETA_SPACING: f32 = 0.07;
const PHI_SPACING:   f32 = 0.02;

const R1: f32 = 1.0;
const R2: f32 = 2.0; // 2.0; 
const K2: f32 = 10.0; // 5.0;

const SCREEN_WIDTH:  f32 = 80.0;
const SCREEN_HEIGHT: f32 = 50.0;

// Calculate K1 based on screen size: the maximum x-distance occurs
// roughly at the edge of the torus, which is at x=R1+R2, z=0.  we
// want that to be displaced 3/8ths of the width of the screen, which
// is 3/4th of the way from the center to the side of the screen.
// screen_width*3/8 = K1*(R1+R2)/(K2+0)
// screen_width*K2*3/(8*(R1+R2)) = K1
// const K1: f32 = SCREEN_WIDTH * K2 * 3.0 / (8.0 * (R1 + R2));

const K1: f32 = SCREEN_WIDTH * K2 * 2.0 / (8.0 * (R1 + R2));

fn lin_pos(x: usize, y: usize) -> usize {
    return y * (SCREEN_WIDTH as usize) + x; 
}

fn render_frame(a: & f32, b: & f32,
    zBuffer: & mut Vec<f32>, output: & mut Vec<char>) {

    let lux = ['.', ',', '-', '~', ':', ';', '=', '!', '*', '#', '$', '@'];

    // Pre-compute sines and cosines of A and B
    let cos_A: f32 = a.cos();
    let sin_A: f32 = a.sin();
    let cos_B: f32 = b.cos();
    let sin_B: f32 = b.sin();

    output.fill(' ');  // Space
    zBuffer.fill(0.0);

    // Theta goes around the cross-sectional circle of a torus
    let mut theta: f32 = 0.0;
    while theta < 2.0 * PI {
        // Pre-compute sines and cosines of theta
        let cos_theta: f32 = theta.cos();
        let sin_theta: f32 = theta.sin();

        // Phi goes around the center of revolution of a torus
        let mut phi: f32 = 0.0;
        while phi < 2.0 * PI {
            // Pre-compute sines and cosines of phi
            let cos_phi: f32 = phi.cos();
            let sin_phi: f32 = phi.sin();
    
            // The x,y coordinate of the circle, before revolving (factored
            // out of the above equations)
            let circle_x: f32 = R2 + R1 * cos_theta;
            let circle_y: f32 = R1 * sin_theta;

            // Final 3D (x,y,z) coordinate after rotations, directly from
            // our math above
            let x: f32 = circle_x * (cos_B * cos_phi + sin_A * sin_B * sin_phi)
                - circle_y * cos_A * sin_B; 
            let y: f32 = circle_x * (sin_B * cos_phi - sin_A * cos_B * sin_phi)
                + circle_y * cos_A * cos_B;
            let z: f32 = K2 + cos_A * circle_x * sin_phi + circle_y * sin_A;
            let ooz: f32 = 1.0 / z;  // "one over z"
      
            // x and y projection.  note that y is negated here, because y
            // goes up in 3D space but down on 2D displays.
            let xp: usize = (SCREEN_WIDTH / 2.0 + K1 * ooz * x) as usize;
            let yp: usize = (SCREEN_HEIGHT / 2.0 - K1 * ooz * y) as usize;
      
            // calculate luminance.  ugly, but correct.
            let L: f32 = cos_phi * cos_theta * sin_B - cos_A * cos_theta * sin_phi -
                sin_A * sin_theta + cos_B * (cos_A * sin_theta - cos_theta * sin_A * sin_phi);
            // L ranges from -sqrt(2) to +sqrt(2).  If it's < 0, the surface
            // is pointing away from us, so we won't bother trying to plot it.
            if L > 0.0 {
                // test against the z-buffer.  larger 1/z means the pixel is
                // closer to the viewer than what's already plotted.
                if     xp < SCREEN_WIDTH as usize 
                    && yp < SCREEN_HEIGHT as usize
                    && ooz > zBuffer[lin_pos(xp, yp)] {
                    zBuffer[lin_pos(xp, yp)] = ooz;
                    let luminance_index: usize = (L * 8.0) as usize;
                    // luminance_index is now in the range 0..11 (8*sqrt(2) = 11.3)
                    // now we lookup the character corresponding to the
                    // luminance and plot it in our output:
                    output[lin_pos(xp, yp)] = lux[luminance_index];
                }
            }
            phi += PHI_SPACING;
        }
        theta += THETA_SPACING;
    }

    // now, dump output[] to the screen.
    // bring cursor to "home" location, in just about any currently-used
    // terminal emulation mode
    print!("\x1b[H");
    for j in 0..SCREEN_HEIGHT as usize {
        for i in 0..SCREEN_WIDTH as usize {
            print!("{}", output[lin_pos(i,j)]);
        }
        print!("\n");
    }
}


#[warn(unstable_name_collisions)]
fn main() {
    let mut a: f32 = 0.0_f32;
    let mut b: f32 = 0.0_f32;

    let mut zBuffer: Vec<f32>  = vec![0.0; (SCREEN_WIDTH * SCREEN_HEIGHT) as usize];
    let mut output:  Vec<char> = vec![' '; (SCREEN_WIDTH * SCREEN_HEIGHT) as usize];

    println!("\x1b[2J");
    loop {
        render_frame(& a, & b, & mut zBuffer, & mut output);
        println!("donut_2_in_rust");
        a += 0.04;
        b += 0.02;
        thread::sleep(time::Duration::from_millis(10)); // 10
    }
}
