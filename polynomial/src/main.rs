use std::f64::consts::FRAC_PI_2;

struct Polynomial<const N: usize> {
    coefficient: [f64; N],
}

impl<const N: usize> Polynomial<N> {
    fn new(coefficient: [f64; N]) -> Polynomial<N> {
        Polynomial { coefficient }
    }

    fn eval(&self, x: f64) -> f64 {
        let mut sum = 0.0;
        for i in (0..N).rev() {
            sum = self.coefficient[i] + x * sum;
        }

        sum
    }
}

fn main() {
    let sine_poly = Polynomial::new([0.0, 1.0, 0.0, -1.0 / 6.0, 0.0, 1.0 / 120.0]);

    assert_eq!(sine_poly.eval(0.0), 0.0);
    assert!((sine_poly.eval(FRAC_PI_2) - 1.).abs() <0.005);
}
