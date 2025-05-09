use std::iter::zip;

#[allow(dead_code)]
pub fn min_moves_to_seat(mut seats: Vec<i32>, mut students: Vec<i32>) -> i32 {
    seats.sort_unstable();
    students.sort_unstable();
    zip(seats, students).fold(0, |acc, (seat, student)| acc + (seat - student).abs())
}
