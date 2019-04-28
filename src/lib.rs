/// All magic happens there
#[macro_export]
macro_rules! const_if {
    ($cond: expr => $if_true:expr;$if_false: expr) => {
        [$if_false, $if_true][$cond as usize]
    };

    ($cond : expr => $if_true: expr; elif $cond2: expr => $if_t2:expr;  else $if_false:expr ) => {
        [const_if!($cond2 => $if_t2;$if_false) , $if_true][$cond as usize]
    };

    ($cond : expr => $if_true: expr; elif $cond2: expr => $if_t2:expr; elif $cond3: expr => $if_t3: expr; else $if_false:expr ) => {
        [const_if!($cond2 => $if_t2;elif $cond3 => $if_t3; else $if_false) , $if_true][$cond as usize]
    };
    ($cond : expr => $if_true: expr; elif $cond2: expr => $if_t2:expr; elif $cond3: expr => $if_t3: expr; elif $cond4: expr => $if_t4: expr; else $if_false:expr ) => {
        [const_if!($cond2 => $if_t2;elif $cond3 => $if_t3;elif $cond4 => $if_t4; else $if_false) , $if_true][$cond as usize]
    };

    ($cond : expr => $if_true: expr; elif $cond2: expr => $if_t2:expr; elif $cond3: expr => $if_t3: expr; elif $cond4: expr => $if_t4: expr;elif $cond5: expr => $if_t5: expr; else $if_false:expr ) => {
        [const_if!($cond2 => $if_t2;elif $cond3 => $if_t3;elif $cond4 => $if_t4;elif $cond5 => $if_t5; else $if_false) , $if_true][$cond as usize]
    };
    ($cond : expr => $if_true: expr; elif $cond2: expr => $if_t2:expr; elif $cond3: expr => $if_t3: expr; elif $cond4: expr => $if_t4: expr;elif $cond5: expr => $if_t5: expr;elif $cond6: expr => $if_t6:expr; else $if_false:expr ) => {
        [const_if!($cond2 => $if_t2;elif $cond3 => $if_t3;elif $cond4 => $if_t4;elif $cond5 => $if_t5;elif $cond6 => $if_t6; else $if_false) , $if_true][$cond as usize]
    };

    ($cond : expr => $if_true: expr; elif $cond2: expr => $if_t2:expr; elif $cond3: expr => $if_t3: expr; elif $cond4: expr => $if_t4: expr;elif $cond5: expr => $if_t5: expr;elif $cond6: expr => $if_t6:expr;elif $cond7: expr => $if_t7:expr; else $if_false:expr ) => {
        [const_if!($cond2 => $if_t2;elif $cond3 => $if_t3;elif $cond4 => $if_t4;elif $cond5 => $if_t5;elif $cond6 => $if_t6;elif $cond7 => $if_t7; else $if_false) , $if_true][$cond as usize]
    };
    ($cond : expr => $if_true: expr; elif $cond2: expr => $if_t2:expr; elif $cond3: expr => $if_t3: expr; elif $cond4: expr => $if_t4: expr;elif $cond5: expr => $if_t5: expr;elif $cond6: expr => $if_t6:expr;elif $cond7: expr => $if_t7:expr;elif $cond8: expr => $if_t8: expr;else $if_false:expr ) => {
        [const_if!($cond2 => $if_t2;elif $cond3 => $if_t3;elif $cond4 => $if_t4;elif $cond5 => $if_t5;elif $cond6 => $if_t6;elif $cond7 => $if_t7;elif $cond8 => $if_t8; else $if_false) , $if_true][$cond as usize]
    };
    ($cond : expr => $if_true: expr; elif $cond2: expr => $if_t2:expr; elif $cond3: expr => $if_t3: expr; elif $cond4: expr => $if_t4: expr;elif $cond5: expr => $if_t5: expr;elif $cond6: expr => $if_t6:expr;elif $cond7: expr => $if_t7:expr;elif $cond8: expr => $if_t8: expr;elif $cond9: expr => $if_t9: expr;else $if_false:expr ) => {
        [const_if!($cond2 => $if_t2;elif $cond3 => $if_t3;elif $cond4 => $if_t4;elif $cond5 => $if_t5;elif $cond6 => $if_t6;elif $cond7 => $if_t7;elif $cond8 => $if_t8;elif $cond9 => $if_t9; else $if_false) , $if_true][$cond as usize]
    };

    ($cond : expr => $if_true: expr; elif $cond2: expr => $if_t2:expr; elif $cond3: expr => $if_t3: expr; elif $cond4: expr => $if_t4: expr;elif $cond5: expr => $if_t5: expr;elif $cond6: expr => $if_t6:expr;elif $cond7: expr => $if_t7:expr;elif $cond8: expr => $if_t8: expr;elif $cond9: expr => $if_t9: expr;elif $cond10: expr => $if_t10: expr;else $if_false:expr ) => {
        [const_if!($cond2 => $if_t2;elif $cond3 => $if_t3;elif $cond4 => $if_t4;elif $cond5 => $if_t5;elif $cond6 => $if_t6;elif $cond7 => $if_t7;elif $cond8 => $if_t8;elif $cond9 => $if_t9;elif $cond10 => $if_t10; else $if_false) , $if_true][$cond as usize]
    };
    ($cond : expr => $if_true: expr; elif $cond2: expr => $if_t2:expr; elif $cond3: expr => $if_t3: expr; elif $cond4: expr => $if_t4: expr;elif $cond5: expr => $if_t5: expr;elif $cond6: expr => $if_t6:expr;elif $cond7: expr => $if_t7:expr;elif $cond8: expr => $if_t8: expr;elif $cond9: expr => $if_t9: expr;elif $cond10: expr => $if_t10: expr;elif $cond11: expr => $if_t11: expr;else $if_false:expr ) => {
        [const_if!($cond2 => $if_t2;elif $cond3 => $if_t3;elif $cond4 => $if_t4;elif $cond5 => $if_t5;elif $cond6 => $if_t6;elif $cond7 => $if_t7;elif $cond8 => $if_t8;elif $cond9 => $if_t9;elif $cond10 => $if_t10;elif $cond11 => $if_t11; else $if_false) , $if_true][$cond as usize]
    };
    ($cond : expr => $if_true: expr; elif $cond2: expr => $if_t2:expr;
    elif $cond3: expr => $if_t3: expr;
    elif $cond4: expr => $if_t4: expr;
    elif $cond5: expr => $if_t5: expr;
    elif $cond6: expr => $if_t6:expr;
    elif $cond7: expr => $if_t7:expr;
    elif $cond8: expr => $if_t8: expr;
    elif $cond9: expr => $if_t9: expr;
    elif $cond10: expr => $if_t10: expr;
    elif $cond11: expr => $if_t11: expr;
    elif $cond12: expr => $if_t12: expr;
    else $if_false:expr ) => {
        [const_if!($cond2 => $if_t2;
        elif $cond3 => $if_t3;
        elif $cond4 => $if_t4;
        elif $cond5 => $if_t5;
        elif $cond6 => $if_t6;
        elif $cond7 => $if_t7;
        elif $cond8 => $if_t8;
        elif $cond9 => $if_t9;
        elif $cond10 => $if_t10;
        elif $cond11 => $if_t11;
        elif $cond12 => $if_t12;
        else $if_false) , $if_true][$cond as usize]
    };

    ($cond : expr => $if_true: expr; elif $cond2: expr => $if_t2:expr;
    elif $cond3: expr => $if_t3: expr;
    elif $cond4: expr => $if_t4: expr;
    elif $cond5: expr => $if_t5: expr;
    elif $cond6: expr => $if_t6:expr;
    elif $cond7: expr => $if_t7:expr;
    elif $cond8: expr => $if_t8: expr;
    elif $cond9: expr => $if_t9: expr;
    elif $cond10: expr => $if_t10: expr;
    elif $cond11: expr => $if_t11: expr;
    elif $cond12: expr => $if_t12: expr;
    elif $cond13: expr => $if_t13: expr;
    else $if_false:expr ) => {
        [const_if!($cond2 => $if_t2;
        elif $cond3 => $if_t3;
        elif $cond4 => $if_t4;
        elif $cond5 => $if_t5;
        elif $cond6 => $if_t6;
        elif $cond7 => $if_t7;
        elif $cond8 => $if_t8;
        elif $cond9 => $if_t9;
        elif $cond10 => $if_t10;
        elif $cond11 => $if_t11;
        elif $cond12 => $if_t12;
        elif $cond13 => $if_t13;
        else $if_false) , $if_true][$cond as usize]
    };
    ($cond : expr => $if_true: expr; elif $cond2: expr => $if_t2:expr;
    elif $cond3: expr => $if_t3: expr;
    elif $cond4: expr => $if_t4: expr;
    elif $cond5: expr => $if_t5: expr;
    elif $cond6: expr => $if_t6:expr;
    elif $cond7: expr => $if_t7:expr;
    elif $cond8: expr => $if_t8: expr;
    elif $cond9: expr => $if_t9: expr;
    elif $cond10: expr => $if_t10: expr;
    elif $cond11: expr => $if_t11: expr;
    elif $cond12: expr => $if_t12: expr;
    elif $cond13: expr => $if_t13: expr;
    elif $cond14: expr => $if_t14: expr;
    else $if_false:expr ) => {
        [const_if!($cond2 => $if_t2;
        elif $cond3 => $if_t3;
        elif $cond4 => $if_t4;
        elif $cond5 => $if_t5;
        elif $cond6 => $if_t6;
        elif $cond7 => $if_t7;
        elif $cond8 => $if_t8;
        elif $cond9 => $if_t9;
        elif $cond10 => $if_t10;
        elif $cond11 => $if_t11;
        elif $cond12 => $if_t12;
        elif $cond13 => $if_t13;
        elif $cond14 => $if_t14;
        else $if_false) , $if_true][$cond as usize]
    };
    ($cond : expr => $if_true: expr; elif $cond2: expr => $if_t2:expr;
    elif $cond3: expr => $if_t3: expr;
    elif $cond4: expr => $if_t4: expr;
    elif $cond5: expr => $if_t5: expr;
    elif $cond6: expr => $if_t6:expr;
    elif $cond7: expr => $if_t7:expr;
    elif $cond8: expr => $if_t8: expr;
    elif $cond9: expr => $if_t9: expr;
    elif $cond10: expr => $if_t10: expr;
    elif $cond11: expr => $if_t11: expr;
    elif $cond12: expr => $if_t12: expr;
    elif $cond13: expr => $if_t13: expr;
    elif $cond14: expr => $if_t14: expr;
    else $if_false:expr ) => {
        [const_if!($cond2 => $if_t2;
        elif $cond3 => $if_t3;
        elif $cond4 => $if_t4;
        elif $cond5 => $if_t5;
        elif $cond6 => $if_t6;
        elif $cond7 => $if_t7;
        elif $cond8 => $if_t8;
        elif $cond9 => $if_t9;
        elif $cond10 => $if_t10;
        elif $cond11 => $if_t11;
        elif $cond12 => $if_t12;
        elif $cond13 => $if_t13;
        elif $cond14 => $if_t14;
        else $if_false) , $if_true][$cond as usize]
    };

    ($cond : expr => $if_true: expr; elif $cond2: expr => $if_t2:expr;
    elif $cond3: expr => $if_t3: expr;
    elif $cond4: expr => $if_t4: expr;
    elif $cond5: expr => $if_t5: expr;
    elif $cond6: expr => $if_t6:expr;
    elif $cond7: expr => $if_t7:expr;
    elif $cond8: expr => $if_t8: expr;
    elif $cond9: expr => $if_t9: expr;
    elif $cond10: expr => $if_t10: expr;
    elif $cond11: expr => $if_t11: expr;
    elif $cond12: expr => $if_t12: expr;
    elif $cond13: expr => $if_t13: expr;
    elif $cond14: expr => $if_t14: expr;
    elif $cond15: expr => $if_t15: expr;
    else $if_false:expr ) => {
        [const_if!($cond2 => $if_t2;
        elif $cond3 => $if_t3;
        elif $cond4 => $if_t4;
        elif $cond5 => $if_t5;
        elif $cond6 => $if_t6;
        elif $cond7 => $if_t7;
        elif $cond8 => $if_t8;
        elif $cond9 => $if_t9;
        elif $cond10 => $if_t10;
        elif $cond11 => $if_t11;
        elif $cond12 => $if_t12;
        elif $cond13 => $if_t13;
        elif $cond14 => $if_t14;
        elif $cond15 => $if_t15;
        else $if_false) , $if_true][$cond as usize]
    };
    ($cond : expr => $if_true: expr; elif $cond2: expr => $if_t2:expr;
    elif $cond3: expr => $if_t3: expr;
    elif $cond4: expr => $if_t4: expr;
    elif $cond5: expr => $if_t5: expr;
    elif $cond6: expr => $if_t6:expr;
    elif $cond7: expr => $if_t7:expr;
    elif $cond8: expr => $if_t8: expr;
    elif $cond9: expr => $if_t9: expr;
    elif $cond10: expr => $if_t10: expr;
    elif $cond11: expr => $if_t11: expr;
    elif $cond12: expr => $if_t12: expr;
    elif $cond13: expr => $if_t13: expr;
    elif $cond14: expr => $if_t14: expr;
    elif $cond15: expr => $if_t15: expr;
    elif $cond16: expr => $if_t16: expr;
    else $if_false:expr ) => {
        [const_if!($cond2 => $if_t2;
        elif $cond3 => $if_t3;
        elif $cond4 => $if_t4;
        elif $cond5 => $if_t5;
        elif $cond6 => $if_t6;
        elif $cond7 => $if_t7;
        elif $cond8 => $if_t8;
        elif $cond9 => $if_t9;
        elif $cond10 => $if_t10;
        elif $cond11 => $if_t11;
        elif $cond12 => $if_t12;
        elif $cond13 => $if_t13;
        elif $cond14 => $if_t14;
        elif $cond15 => $if_t15;
        elif $cond16 => $if_t16;
        else $if_false) , $if_true][$cond as usize]
    };

    ($cond : expr => $if_true: expr; elif $cond2: expr => $if_t2:expr;
    elif $cond3: expr => $if_t3: expr;
    elif $cond4: expr => $if_t4: expr;
    elif $cond5: expr => $if_t5: expr;
    elif $cond6: expr => $if_t6:expr;
    elif $cond7: expr => $if_t7:expr;
    elif $cond8: expr => $if_t8: expr;
    elif $cond9: expr => $if_t9: expr;
    elif $cond10: expr => $if_t10: expr;
    elif $cond11: expr => $if_t11: expr;
    elif $cond12: expr => $if_t12: expr;
    elif $cond13: expr => $if_t13: expr;
    elif $cond14: expr => $if_t14: expr;
    elif $cond15: expr => $if_t15: expr;
    elif $cond16: expr => $if_t16: expr;
    elif $cond17: expr => $if_t17: expr;
    else $if_false:expr ) => {
        [const_if!($cond2 => $if_t2;
        elif $cond3 => $if_t3;
        elif $cond4 => $if_t4;
        elif $cond5 => $if_t5;
        elif $cond6 => $if_t6;
        elif $cond7 => $if_t7;
        elif $cond8 => $if_t8;
        elif $cond9 => $if_t9;
        elif $cond10 => $if_t10;
        elif $cond11 => $if_t11;
        elif $cond12 => $if_t12;
        elif $cond13 => $if_t13;
        elif $cond14 => $if_t14;
        elif $cond15 => $if_t15;
        elif $cond16 => $if_t16;
        elif $cond17 => $if_t17;
        else $if_false) , $if_true][$cond as usize]
    };
    ($cond : expr => $if_true: expr; elif $cond2: expr => $if_t2:expr;
    elif $cond3: expr => $if_t3: expr;
    elif $cond4: expr => $if_t4: expr;
    elif $cond5: expr => $if_t5: expr;
    elif $cond6: expr => $if_t6:expr;
    elif $cond7: expr => $if_t7:expr;
    elif $cond8: expr => $if_t8: expr;
    elif $cond9: expr => $if_t9: expr;
    elif $cond10: expr => $if_t10: expr;
    elif $cond11: expr => $if_t11: expr;
    elif $cond12: expr => $if_t12: expr;
    elif $cond13: expr => $if_t13: expr;
    elif $cond14: expr => $if_t14: expr;
    elif $cond15: expr => $if_t15: expr;
    elif $cond16: expr => $if_t16: expr;
    elif $cond17: expr => $if_t17: expr;
    elif $cond18: expr => $if_t18: expr;
    else $if_false:expr ) => {
        [const_if!($cond2 => $if_t2;
        elif $cond3 => $if_t3;
        elif $cond4 => $if_t4;
        elif $cond5 => $if_t5;
        elif $cond6 => $if_t6;
        elif $cond7 => $if_t7;
        elif $cond8 => $if_t8;
        elif $cond9 => $if_t9;
        elif $cond10 => $if_t10;
        elif $cond11 => $if_t11;
        elif $cond12 => $if_t12;
        elif $cond13 => $if_t13;
        elif $cond14 => $if_t14;
        elif $cond15 => $if_t15;
        elif $cond16 => $if_t16;
        elif $cond17 => $if_t17;
        elif $cond18 => $if_t18;
        else $if_false) , $if_true][$cond as usize]
    };
    ($cond : expr => $if_true: expr; elif $cond2: expr => $if_t2:expr;
    elif $cond3: expr => $if_t3: expr;
    elif $cond4: expr => $if_t4: expr;
    elif $cond5: expr => $if_t5: expr;
    elif $cond6: expr => $if_t6:expr;
    elif $cond7: expr => $if_t7:expr;
    elif $cond8: expr => $if_t8: expr;
    elif $cond9: expr => $if_t9: expr;
    elif $cond10: expr => $if_t10: expr;
    elif $cond11: expr => $if_t11: expr;
    elif $cond12: expr => $if_t12: expr;
    elif $cond13: expr => $if_t13: expr;
    elif $cond14: expr => $if_t14: expr;
    elif $cond15: expr => $if_t15: expr;
    elif $cond16: expr => $if_t16: expr;
    elif $cond17: expr => $if_t17: expr;
    elif $cond18: expr => $if_t18: expr;
    elif $cond19: expr => $if_t19: expr;
    else $if_false:expr ) => {
        [const_if!($cond2 => $if_t2;
        elif $cond3 => $if_t3;
        elif $cond4 => $if_t4;
        elif $cond5 => $if_t5;
        elif $cond6 => $if_t6;
        elif $cond7 => $if_t7;
        elif $cond8 => $if_t8;
        elif $cond9 => $if_t9;
        elif $cond10 => $if_t10;
        elif $cond11 => $if_t11;
        elif $cond12 => $if_t12;
        elif $cond13 => $if_t13;
        elif $cond14 => $if_t14;
        elif $cond15 => $if_t15;
        elif $cond16 => $if_t16;
        elif $cond17 => $if_t17;
        elif $cond18 => $if_t18;
        elif $cond19 => $if_t19;
        else $if_false) , $if_true][$cond as usize]
    };

    ($cond : expr => $if_true: expr; elif $cond2: expr => $if_t2:expr;
    elif $cond3: expr => $if_t3: expr;
    elif $cond4: expr => $if_t4: expr;
    elif $cond5: expr => $if_t5: expr;
    elif $cond6: expr => $if_t6:expr;
    elif $cond7: expr => $if_t7:expr;
    elif $cond8: expr => $if_t8: expr;
    elif $cond9: expr => $if_t9: expr;
    elif $cond10: expr => $if_t10: expr;
    elif $cond11: expr => $if_t11: expr;
    elif $cond12: expr => $if_t12: expr;
    elif $cond13: expr => $if_t13: expr;
    elif $cond14: expr => $if_t14: expr;
    elif $cond15: expr => $if_t15: expr;
    elif $cond16: expr => $if_t16: expr;
    elif $cond17: expr => $if_t17: expr;
    elif $cond18: expr => $if_t18: expr;
    elif $cond19: expr => $if_t19: expr;
    elif $cond20: expr => $if_t20: expr;
    else $if_false:expr ) => {
        [const_if!($cond2 => $if_t2;
        elif $cond3 => $if_t3;
        elif $cond4 => $if_t4;
        elif $cond5 => $if_t5;
        elif $cond6 => $if_t6;
        elif $cond7 => $if_t7;
        elif $cond8 => $if_t8;
        elif $cond9 => $if_t9;
        elif $cond10 => $if_t10;
        elif $cond11 => $if_t11;
        elif $cond12 => $if_t12;
        elif $cond13 => $if_t13;
        elif $cond14 => $if_t14;
        elif $cond15 => $if_t15;
        elif $cond16 => $if_t16;
        elif $cond17 => $if_t17;
        elif $cond18 => $if_t18;
        elif $cond19 => $if_t19;
        elif $cond20 => $if_t20;
        else $if_false) , $if_true][$cond as usize]
    };
    ($cond : expr => $if_true: expr; elif $cond2: expr => $if_t2:expr;
    elif $cond3: expr => $if_t3: expr;
    elif $cond4: expr => $if_t4: expr;
    elif $cond5: expr => $if_t5: expr;
    elif $cond6: expr => $if_t6:expr;
    elif $cond7: expr => $if_t7:expr;
    elif $cond8: expr => $if_t8: expr;
    elif $cond9: expr => $if_t9: expr;
    elif $cond10: expr => $if_t10: expr;
    elif $cond11: expr => $if_t11: expr;
    elif $cond12: expr => $if_t12: expr;
    elif $cond13: expr => $if_t13: expr;
    elif $cond14: expr => $if_t14: expr;
    elif $cond15: expr => $if_t15: expr;
    elif $cond16: expr => $if_t16: expr;
    elif $cond17: expr => $if_t17: expr;
    elif $cond18: expr => $if_t18: expr;
    elif $cond19: expr => $if_t19: expr;
    elif $cond20: expr => $if_t20: expr;
    elif $cond21: expr => $if_t21: expr;
    else $if_false:expr ) => {
        [const_if!($cond2 => $if_t2;
        elif $cond3 => $if_t3;
        elif $cond4 => $if_t4;
        elif $cond5 => $if_t5;
        elif $cond6 => $if_t6;
        elif $cond7 => $if_t7;
        elif $cond8 => $if_t8;
        elif $cond9 => $if_t9;
        elif $cond10 => $if_t10;
        elif $cond11 => $if_t11;
        elif $cond12 => $if_t12;
        elif $cond13 => $if_t13;
        elif $cond14 => $if_t14;
        elif $cond15 => $if_t15;
        elif $cond16 => $if_t16;
        elif $cond17 => $if_t17;
        elif $cond18 => $if_t18;
        elif $cond19 => $if_t19;
        elif $cond20 => $if_t20;
        elif $cond21 => $if_t21;
        else $if_false) , $if_true][$cond as usize]
    };

    ($cond : expr => $if_true: expr; elif $cond2: expr => $if_t2:expr;
    elif $cond3: expr => $if_t3: expr;
    elif $cond4: expr => $if_t4: expr;
    elif $cond5: expr => $if_t5: expr;
    elif $cond6: expr => $if_t6:expr;
    elif $cond7: expr => $if_t7:expr;
    elif $cond8: expr => $if_t8: expr;
    elif $cond9: expr => $if_t9: expr;
    elif $cond10: expr => $if_t10: expr;
    elif $cond11: expr => $if_t11: expr;
    elif $cond12: expr => $if_t12: expr;
    elif $cond13: expr => $if_t13: expr;
    elif $cond14: expr => $if_t14: expr;
    elif $cond15: expr => $if_t15: expr;
    elif $cond16: expr => $if_t16: expr;
    elif $cond17: expr => $if_t17: expr;
    elif $cond18: expr => $if_t18: expr;
    elif $cond19: expr => $if_t19: expr;
    elif $cond20: expr => $if_t20: expr;
    elif $cond21: expr => $if_t21: expr;
    elif $cond22: expr => $if_t22: expr;
    else $if_false:expr ) => {
        [const_if!($cond2 => $if_t2;
        elif $cond3 => $if_t3;
        elif $cond4 => $if_t4;
        elif $cond5 => $if_t5;
        elif $cond6 => $if_t6;
        elif $cond7 => $if_t7;
        elif $cond8 => $if_t8;
        elif $cond9 => $if_t9;
        elif $cond10 => $if_t10;
        elif $cond11 => $if_t11;
        elif $cond12 => $if_t12;
        elif $cond13 => $if_t13;
        elif $cond14 => $if_t14;
        elif $cond15 => $if_t15;
        elif $cond16 => $if_t16;
        elif $cond17 => $if_t17;
        elif $cond18 => $if_t18;
        elif $cond19 => $if_t19;
        elif $cond20 => $if_t20;
        elif $cond21 => $if_t21;
        elif $cond22 => $if_t22;
        else $if_false) , $if_true][$cond as usize]
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
    fn test_elif() {

        assert_eq!(int_to_str(5), "Five");

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