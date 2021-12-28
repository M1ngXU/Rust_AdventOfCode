#[cfg(test)]
mod y2021 {
    mod d15;

    #[test]
    fn results() {
        assert_eq!(d15::run(), (562, 2874));
    }
}