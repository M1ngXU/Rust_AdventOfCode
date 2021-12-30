use std::collections::LinkedList;
use ming_xu::file_manager;

pub fn run() -> (usize, usize) {
    let mut matrix = file_manager::get_matrix(2021, 11, "");
    let mut ticks = 0;
    let mut flashes = 0;

    while !matrix.values().iter().all(| &v | v == 0) {
        let mut flashed= LinkedList::new();
        matrix.modify_values(&mut | (c, &v) | {
            let new_value = (v + 1) % 10;
            if new_value == 0 {
                flashed.push_back(c);
                if ticks < 100 {
                    flashes += 1;
                }
            }
            new_value
        });
        while let Some(current) = flashed.pop_back() {
            matrix.modify_adjacent_values(&current, &mut | (c, &v) | {
                if v > 0 {
                    let new_value = (v + 1) % 10;
                    if new_value == 0 {
                        flashed.push_back(c);
                        if ticks < 100 {
                            flashes += 1;
                        }
                    }
                    new_value
                } else {
                    0
                }
            })
        }
        ticks += 1;
    }
    (flashes, ticks)
}