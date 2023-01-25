struct Fern {
    size: f64,
    growth_rate: f64,
}

impl Fern {
    fn grow(&mut self) {
        self.size *= 1.0 + self.growth_rate;
    }
}

fn run_simiulation(fern: &mut Fern, days: usize) {
    for _ in 0..days {
        fern.grow();
    }
}

fn main() {
    let mut fern = Fern {
        size: 1.0,
        growth_rate: 0.001,
    };

    run_simiulation(&mut fern, 1000);
    println!("final fern size: {}", fern.size);
}
