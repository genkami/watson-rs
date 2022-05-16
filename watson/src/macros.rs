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
    fn test_watson_i64() {
        assert_eq!(watson!(0_i64), Int(0));
        assert_eq!(watson!(123_i64), Int(123));
        assert_eq!(watson!(-12345_i64), Int(-12345));
    }
}
