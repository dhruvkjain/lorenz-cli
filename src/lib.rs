use std::{process, error::Error};

const SIGMA: f64 = 10.0;
const RHO: f64 = 28.0;
const BETA: f64 = 8.0 / 3.0;

pub fn generate_lorenz_points() -> Vec<(f64, f64)> {
    let dt = 0.01;
    let num_steps = 10000;

    let mut x = 0.0;
    let mut y = 1.0;
    let mut z = 1.05;
    let mut points = Vec::with_capacity(num_steps);

    for _ in 0..num_steps {
        let (dx, dy, dz) = lorenz(x, y, z, SIGMA, RHO, BETA).unwrap_or_else(|err| {
            eprintln!("{:?}", err);
            process::exit(1);
        });
        x += dx * dt;
        y += dy * dt;
        z += dz * dt;
        points.push((x, z));
    }
    points
}

pub fn lorenz (x:f64, y:f64, z:f64, sigma:f64, rho:f64, beta:f64) 
-> Result<(f64, f64, f64),Box<dyn Error>>
{
    let dx = sigma*(y-x);
    let dy = x*(rho-z) - y;
    let dz = x*y - beta*z;

    Ok((dx, dy, dz)) 
}