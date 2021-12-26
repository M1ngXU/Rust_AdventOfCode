#[cfg(test)]
mod y2021 {
    mod d5;

    #[test]
    fn results() {
        assert_eq!(d5::run(), (5147, 16925));
    }
}