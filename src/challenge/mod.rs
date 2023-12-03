use crate::solution::{build, Challenges};

mod day_01;
mod day_02;
mod day_03;

pub fn challenges() -> Challenges {
    build! {
        day(1, "Trebuchet?!", day_01::part_a, day_01::part_b),
        day(2, "Cube Conundrum", day_02::part_a, day_02::part_b),
        day(3, "Gear Ratios", day_03::part_a, day_03::part_b),
    }
}
