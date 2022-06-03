fn main() {
    fn exponentiate(base: f64, exponent: f64) -> f64 {
        (base.ln() * exponent).exp()
    }
    print!("{}", exponentiate(2.5, 3.2));
}
