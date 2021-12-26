#[cfg(test)]
mod sparse {
    mod two_dimensional {
        use ming_xu::grid;
        use ming_xu::grid::Sparse2d;

        fn get_cut() -> Sparse2d<i64> {
            grid::Sparse2d::new(1)
        }

        #[test]
        fn default_value() {
            assert_eq!(get_cut().default(), 1);
        }

        #[test]
        fn contains_not_inserted_row() {
            assert!(!get_cut().contains_row(0));
        }

        #[test]
        fn get_inserted_cell() {
            let mut cut = get_cut();
            cut.insert(0, 0, 1);
            assert_eq!(cut.get_unwrapped(0, 0), 1);
        }

        #[test]
        fn get_not_inserted_cell() {
            assert_eq!(get_cut().get(0, 0), None);
        }

        #[test]
        fn only_insert_if_no_value() {
            let mut cut = get_cut();
            cut.insert(0, 0, 1);
            cut.insert(0, 0, 2);
            assert_eq!(cut.get_unwrapped(0, 0), 1);
        }

        #[test]
        fn replace_unset_value() {
            let mut cut = get_cut();
            cut.replace(0, 0, 1);
            assert_eq!(cut.get_unwrapped(0, 0), 1);
        }

        #[test]
        fn replace_set_value() {
            let mut cut = get_cut();
            cut.insert(0, 0, 2);
            cut.replace(0, 0, 1);
            assert_eq!(cut.get_unwrapped(0, 0), 1);
        }

        #[test]
        fn modify_value() {
            let mut cut = get_cut();
            cut.insert(0, 0, 3);
            *cut.get_unwrapped_mut(0, 0) -= 1;
            assert_eq!(cut.get_unwrapped(0, 0), 2);
        }

        #[test]
        fn get_or_insert() {
            let mut cut = get_cut();
            cut.insert(0, 0, 2);
            assert_eq!(*cut.get_mut_or_insert_default(0, 1), 1);
            assert_eq!(*cut.get_mut_or_insert_default(0, 0), 2);
            assert_eq!(*cut.get_mut_or_insert_value(0, 1, 5), 1);
        }
    }
}