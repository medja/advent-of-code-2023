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
mod day_13;
mod day_14;
mod day_15;
mod day_16;
mod day_17;
mod day_18;
mod day_19;
mod day_20;
mod day_21;
mod day_22;
mod day_23;
mod day_24;

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
        day(13, "Point of Incidence", day_13::part_a, day_13::part_b),
        day(14, "Parabolic Reflector Dish", day_14::part_a, day_14::part_b),
        day(15, "Lens Library", day_15::part_a, day_15::part_b),
        day(16, "The Floor Will Be Lava", day_16::part_a, day_16:: part_b),
        day(17, "Clumsy Crucible", day_17::part_a, day_17::part_b),
        day(18, "Lavaduct Lagoon", day_18::part_a, day_18::part_b),
        day(19, "Aplenty", day_19::part_a, day_19::part_b),
        day(20, "Pulse Propagation", day_20::part_a, day_20::part_b),
        day(21, "Step Counter", day_21::part_a, day_21::part_b),
        day(22, "Sand Slabs", day_22::part_a, day_22::part_b),
        day(23, "A Long Walk", day_23::part_a, day_23::part_b),
        day(24, "Never Tell Me The Odds", day_24::part_a, day_24::part_b),
    }
}
