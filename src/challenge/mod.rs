use crate::solution::{build, Challenges};

mod day_01;

pub fn challenges() -> Challenges {
    build! {
        day(1, "Trebuchet?!", day_01::part_a, day_01::part_b),
    }
}
