#[macro_export]
macro_rules! watson {
    ( $other:expr ) => {
        $crate::language::Value::from($other)
    };
}

#[cfg(test)]
mod test {
    use crate::language::Value::*;

    #[test]
    fn test_watson_int() {
        assert_eq!(watson!(0_i8), Int(0));
        assert_eq!(watson!(127_i8), Int(127));
        assert_eq!(watson!(-32_i8), Int(-32));

        assert_eq!(watson!(0_i16), Int(0));
        assert_eq!(watson!(123_i16), Int(123));
        assert_eq!(watson!(-12345_i16), Int(-12345));

        assert_eq!(watson!(0_i32), Int(0));
        assert_eq!(watson!(123_i32), Int(123));
        assert_eq!(watson!(-12345_i32), Int(-12345));

        assert_eq!(watson!(0_i64), Int(0));
        assert_eq!(watson!(123_i64), Int(123));
        assert_eq!(watson!(-12345_i64), Int(-12345));

        assert_eq!(watson!(0_i128), Int(0));
        assert_eq!(watson!(123_i128), Int(123));
        assert_eq!(watson!(-12345_i128), Int(-12345));

        assert_eq!(watson!(0_isize), Int(0));
        assert_eq!(watson!(123_isize), Int(123));
        assert_eq!(watson!(-12345_isize), Int(-12345));
    }

    #[test]
    fn test_watson_uint() {
        assert_eq!(watson!(0_u8), Uint(0));
        assert_eq!(watson!(255_u8), Uint(255));

        assert_eq!(watson!(0_u16), Uint(0));
        assert_eq!(watson!(255_u16), Uint(255));
        assert_eq!(watson!(12345_u16), Uint(12345));

        assert_eq!(watson!(0_u32), Uint(0));
        assert_eq!(watson!(255_u32), Uint(255));
        assert_eq!(watson!(12345_u32), Uint(12345));

        assert_eq!(watson!(0_u64), Uint(0));
        assert_eq!(watson!(255_u64), Uint(255));
        assert_eq!(watson!(12345_u64), Uint(12345));

        assert_eq!(watson!(0_u128), Uint(0));
        assert_eq!(watson!(255_u128), Uint(255));
        assert_eq!(watson!(12345_u128), Uint(12345));

        assert_eq!(watson!(0_usize), Uint(0));
        assert_eq!(watson!(255_usize), Uint(255));
        assert_eq!(watson!(12345_usize), Uint(12345));
    }

    #[test]
    fn test_watson_float() {
        assert_eq!(watson!(0.0_f32), Float(0.0));
        assert_eq!(watson!(1.25_f32), Float(1.25));
        assert_eq!(watson!(-1.25_f32), Float(-1.25));

        assert_eq!(watson!(0.0_f64), Float(0.0));
        assert_eq!(watson!(1.23e45_f64), Float(1.23e45));
        assert_eq!(watson!(-1.98e-76_f64), Float(-1.98e-76));
    }

    #[test]
    fn test_watson_bool() {
        assert_eq!(watson!(true), Bool(true));
        assert_eq!(watson!(false), Bool(false));
    }
}
