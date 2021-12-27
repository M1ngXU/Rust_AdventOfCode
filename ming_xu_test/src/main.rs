#[cfg(test)]
mod grid {
    mod sparse {
        use ming_xu::grid;
        use ming_xu::grid::Sparse;

        fn get_cut() -> Sparse<i64> {
            grid::Sparse::new(2, 1)
        }

        #[test]
        fn default_value() {
            assert_eq!(get_cut().default(), 1);
        }

        #[test]
        fn get_inserted_cell() {
            let mut cut = get_cut();
            cut.insert(&vec![ 0, 0 ], 1);
            assert_eq!(cut.get_unwrapped(&vec![ 0, 0 ]), 1);
        }

        #[test]
        fn get_not_inserted_cell() {
            assert_eq!(get_cut().get(&vec![ 0, 0 ]), None);
        }

        #[test]
        fn only_insert_if_no_value() {
            let mut cut = get_cut();
            cut.insert(&vec![ 0, 0 ], 1);
            cut.insert(&vec![ 0, 0 ], 2);
            assert_eq!(cut.get_unwrapped(&vec![ 0, 0 ]), 1);
        }

        #[test]
        fn replace_unset_value() {
            let mut cut = get_cut();
            cut.replace(vec![ 0, 0 ], 1);
            assert_eq!(cut.get_unwrapped(&vec![ 0, 0 ]), 1);
        }

        #[test]
        fn replace_set_value() {
            let mut cut = get_cut();
            cut.insert(&vec![ 0, 0 ], 2);
            cut.replace(vec![ 0, 0 ], 1);
            assert_eq!(cut.get_unwrapped(&vec![ 0, 0 ]), 1);
        }

        #[test]
        fn modify_value() {
            let mut cut = get_cut();
            cut.insert(&vec![ 0, 0 ], 3);
            *cut.get_unwrapped_mut(&vec![ 0, 0 ]) -= 1;
            assert_eq!(cut.get_unwrapped(&vec![ 0, 0 ]), 2);
        }

        #[test]
        fn get_or_insert() {
            let mut cut = get_cut();
            cut.insert(&vec![ 0, 0 ], 2);
            assert_eq!(*cut.get_mut_or_insert_default(&vec![ 0, 1 ]), 1);
            assert_eq!(*cut.get_mut_or_insert_default(&vec![ 0, 0 ]), 2);
            assert_eq!(*cut.get_mut_or_insert_value(&vec![ 0, 1 ], 5), 1);
        }
    }
}

#[cfg(test)]
mod a_star {
    use ming_xu::a_star;
}