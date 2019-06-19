/// All magic happens there
#[macro_export]
macro_rules! __hidden_handling_cases {
    // hide this case from the API
    (else $e: expr) => {$e};

    ($cond : expr => $if_true: expr;
        $(elif $cond2: expr => $if_t2:expr;)*
        else $if_false:expr
    ) => {
        [__hidden_handling_cases!($($cond2 => $if_t2; )elif * else $if_false), $if_true][$cond as bool as usize]
    };
}

#[macro_export]
macro_rules! const_if {
    ($cond: expr => $if_true:expr;$if_false: expr) => {
        [$if_false, $if_true][(!!$cond)  as usize]
    };

    // delegate to private macro
    ($cond : expr => $if_true: expr;
        $(elif $cond2: expr => $if_t2:expr;)*
        else $if_false:expr
    ) => {
        __hidden_handling_cases!($cond => $if_true; $(elif $cond2 => $if_t2;)* else $if_false)
    };
}

mod tests {
    const fn min(x: i32, y: i32) -> i32 {
        const_if!(x < y => x;y)
    }

    #[test]
    fn test_min() {
        assert_eq!(min(2, 3), 2);
    }


    const fn is_zero(i: i32) -> &'static str {
        const_if!(
            i == 0 => "Yes";
            else "No"
        )
    }

    const fn compare_to_42(i: i32) -> &'static str {
        const_if!(
            i < 42 => "Lesser";
            elif i == 42 => "Equals";
            else "Bigger"
        )
    }

    const fn int_to_str(i: i32) -> &'static str {
        const_if!(
            i == 0 => "Zero";
            elif i == 1 => "One";
            elif i == 2 => "Two";
            elif i == 3 => "Three";
            elif i == 4 => "Four";
            elif i == 5 => "Five";
            elif i == 6 => "Six";
            elif i == 7 => "Seven";
            elif i == 8 => "Eight";
            elif i == 9 => "Nine";
            elif i == 10 => "Ten";
            else "Unknown"
        )
    }

    #[test]
    fn test_multiple_elif() {
        assert_eq!(int_to_str(0), "Zero");
        assert_eq!(int_to_str(5), "Five");
        assert_eq!(int_to_str(10), "Ten");
        assert_eq!(int_to_str(11), "Unknown");
    }

    #[test]
    fn test_if_else() {
        assert_eq!(is_zero(0), "Yes");
        assert_eq!(is_zero(42), "No");
    }

    #[test]
    fn test_if_elif_else() {
        assert_eq!(compare_to_42(0), "Lesser");
        assert_eq!(compare_to_42(42), "Equals");
        assert_eq!(compare_to_42(1337), "Bigger");
    }

    #[test]
    fn test_expanded() {
        let i = 6;
        assert_eq!(
            [
                [
                    [
                        [
                            [
                                [
                                    [
                                        [
                                            [
                                                [["Unknown", "Ten"][(i == 10) as usize], "Nine"]
                                                    [(i == 9) as usize],
                                                "Eight",
                                            ][(i == 8) as usize],
                                            "Seven",
                                        ][(i == 7) as usize],
                                        "Six",
                                    ][(i == 6) as usize],
                                    "Five",
                                ][(i == 5) as usize],
                                "Four",
                            ][(i == 4) as usize],
                            "Three",
                        ][(i == 3) as usize],
                        "Two",
                    ][(i == 2) as usize],
                    "One",
                ][(i == 1) as usize],
                "Zero",
            ][(i == 0) as usize],
            "Six"
        );
    }
}