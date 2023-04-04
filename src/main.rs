use coordinate::Coordinate;
use ir_sir::IrSir;

mod euler;
mod coordinate;
mod ir_sir;
mod error;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let ir_sir = IrSir::new(0.4, 0.2, 0.3, 0.5, 0.1);
    let step_size: f64 = args[1].trim().parse().unwrap();

    let output = ir_sir.estimate(Coordinate::new(0., 1.5, 1., 1.), 50., step_size).unwrap();

    for output in output.iter() {
        println!("{:?}", output);
    }
}
