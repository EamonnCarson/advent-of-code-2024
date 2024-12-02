use crate::global_imports::*;

pub struct Day01;
impl Day for Day01 {
    fn star1(&self, _input: String) -> String {
        log!("Logs");
        log!("More logs");
        log!("Even more logs");
        log!("Wow I cannot believe there are more logs");
        let x = 3;
        let y = debug!(x) + 1;

        y.to_string()
    }
    fn star2(&self, input: String) -> String {
        debug!(input)
    }
}
