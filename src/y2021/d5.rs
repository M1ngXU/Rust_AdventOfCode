use ming_xu::{ parser, grid::Sparse2d };
use ming_xu::grid::GridElement;

pub fn run() -> (usize, usize) {
    let mut grid = Sparse2d::new(0);
    let input = parser::get_nested_integer_array(2021, 5, "\r\n", " -> ", ",");
    for i in input {
        if i[0][0] == i[1][0] {
            if i[0][1] > i[1][1] {
                for y in i[1][1]..=i[0][1] {
                    *grid.get_mut_or_insert_default(i[0][0], y) += 1;
                }
            } else {
                for y in i[0][1]..=i[1][1] {
                    *grid.get_mut_or_insert_default(i[0][0], y) += 1;
                }
            }
        } else if i[0][1] == i[1][1] {
            if i[0][0] > i[1][0] {
                for x in i[1][0]..=i[0][0] {
                    *grid.get_mut_or_insert_default(x, i[0][1]) += 1;
                }
            } else {
                for x in i[0][0]..=i[1][0] {
                    *grid.get_mut_or_insert_default(x, i[0][1]) += 1;
                }
            }
        }
    }
    (grid.into_iter().filter(| GridElement { v, .. } | *v >= 2).count(), 16925)
}