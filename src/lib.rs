#![feature(bool_to_option)]

#[cfg(test)]
mod y2021 {
    mod d5;
    mod d11;
    mod d12;
    mod d15;
    mod d21;

    #[test]
    fn d5() {
        assert_eq!(d5::run(), (5147, 16925));
    }

    #[test]
    fn d11() {
        assert_eq!(d11::run(), (1634, 210));
    }

    #[test]
    fn d12() {
        assert_eq!(d12::run(), (4775, 152480));
    }

    #[test]
    fn d15() {
        assert_eq!(d15::run(), (562, 2874));
    }

    #[test]
    fn d21() {
        assert_eq!(d21::run(), 301304993766094);
    }
}