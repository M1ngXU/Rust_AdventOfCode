#[cfg(test)]
mod y2021 {
    mod d5;
    mod d15;

    #[test]
    fn d5() {
        assert_eq!(d5::run(), (5147, 16925));
    }

    #[test]
    fn d15() {
        assert_eq!(d15::run(), (562, 2874));
    }
}