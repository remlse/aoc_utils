//! The module providing [solution].

/// A declarative macro for creating the tests of an aoc puzzle.
#[macro_export]
macro_rules! solution {
    // both parts unsolved
    ($package:ident; TODO; TODO;) => {
        static INPUT: &str = include_str!("../input/input.txt");
        #[test]
        fn test_part1() {
            assert_eq!($package::part1(INPUT), $crate::fail::Fail);
        }
        #[test]
        fn test_part2() {
            assert_eq!($package::part2(INPUT), $crate::fail::Fail);
        }
    };
    // part1 solved
    ($package:ident; $res_part1:expr; TODO;) => {
        static INPUT: &str = include_str!("../input/input.txt");
        #[test]
        fn test_part1() {
            assert_eq!($package::part1(INPUT), $res_part1);
        }
        #[test]
        fn test_part2() {
            assert_eq!($package::part2(INPUT), $crate::fail::Fail);
        }
    };
    // both parts solved
    ($package:ident; $res_part1:expr; $res_part2:expr;) => {
        static INPUT: &str = include_str!("../input/input.txt");
        #[test]
        fn test_part1() {
            assert_eq!($package::part1(INPUT), $res_part1);
        }
        #[test]
        fn test_part2() {
            assert_eq!($package::part2(INPUT), $res_part2);
        }
    };

    // variants for ignoring
    // ignore part1
    ($package:ident; ignore $res_part1:expr; $res_part2:expr;) => {
        static INPUT: &str = include_str!("../input/input.txt");
        #[test]
        #[ignore]
        fn test_part1() {
            assert_eq!($package::part1(INPUT), $res_part1);
        }
        #[test]
        fn test_part2() {
            assert_eq!($package::part2(INPUT), $res_part2);
        }
    };
    // ignore part2
    ($package:ident; $res_part1:expr; ignore $res_part2:expr;) => {
        static INPUT: &str = include_str!("../input/input.txt");
        #[test]
        fn test_part1() {
            assert_eq!($package::part1(INPUT), $res_part1);
        }
        #[test]
        #[ignore]
        fn test_part2() {
            assert_eq!($package::part2(INPUT), $res_part2);
        }
    };
    // ignore both parts
    ($package:ident; ignore $res_part1:expr; ignore $res_part2:expr;) => {
        static INPUT: &str = include_str!("../input/input.txt");
        #[test]
        #[ignore]
        fn test_part1() {
            assert_eq!($package::part1(INPUT), $res_part1);
        }
        #[test]
        #[ignore]
        fn test_part2() {
            assert_eq!($package::part2(INPUT), $res_part2);
        }
    };
}
