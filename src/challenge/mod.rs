use crate::solution::{build, Challenges};

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;
mod day_10;
mod day_11;
mod day_12;

pub fn challenges() -> Challenges {
    build! {
        day(1, "Trebuchet?!", day_01::part_a, day_01::part_b),
        day(2, "Cube Conundrum", day_02::part_a, day_02::part_b),
        day(3, "Gear Ratios", day_03::part_a, day_03::part_b),
        day(4, "Scratchcards", day_04::part_a, day_04::part_b),
        day(5, "If You Give A Seed A Fertilizer", day_05::part_a, day_05::part_b),
        day(6, "Wait For It", day_06::part_a, day_06::part_b),
        day(7, "Camel Cards", day_07::part_a, day_07::part_b),
        day(8, "Haunted Wasteland", day_08::part_a, day_08::part_b),
        day(9, "Mirage Maintenance", day_09::part_a, day_09::part_b),
        day(10, "Pipe Maze", day_10::part_a, day_10::part_b),
        day(11, "Cosmic Expansion", day_11::part_a, day_11::part_b),
        day(12, "Hot Springs", day_12::part_a, day_12::part_b),
    }
}
