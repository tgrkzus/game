extern crate worldgen;

use worldgen::MacroWorld;

pub fn simulate(mw: &mut MacroWorld, ticks: u32) {
    for i in 0..ticks {
        // Nothing yet
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
