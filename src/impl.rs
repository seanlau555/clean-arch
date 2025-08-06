struct Temperature {
    degree_f: f64,
}

impl Temperature {
    fn freezing() -> Self {
        Self { degree_f: 32.0 }
    }

    fn boiling() -> Self {
        // Without $self in first parameter -> associated function
        Self { degree_f: 212.0 }
    }

    fn show_temp(&self) {
        // Have $self in first parameter -> method
        println!("{} degrees F", self.degree_f);
    }
}

fn main() {
    let hot = Temperature { degree_f: 100.0 };
    hot.show_temp();
    // Temperature::show_temp(hot);

    let cold = Temperature::freezing();
    cold.show_temp();

    let boil = Temperature::boiling();
    boil.show_temp();
}
