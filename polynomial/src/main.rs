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
    println!("Hello, world!");
}
