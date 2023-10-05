use rust_test::split_work;

#[cfg(test)]
mod different_types_test {
    use super::*;
    #[test]
    fn int_array() {
        let input = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let threshold = 1;
        assert_eq!(split_work(input, |x: i32| x * x, threshold), &[1, 4, 9, 16, 25, 36, 49, 64, 81, 100]);
    }

    #[test]
    fn str_array() {
        let input = vec!['a', 'a'];
        let f = |num: char| -> bool {
            num.is_ascii()
        };
        let threshold = 1;
        assert_eq!(split_work(input, f, threshold), &[true, true]);
    }

    #[test]
    fn float_array() {
        let input = vec![0.2, 33.213, 1.3, -123.2, 0.00123];
        let f = |num: f64| -> f64 {
            num.sinh()
        };
        let threshold = 1;
        assert_eq!(split_work(input, f, threshold), &[0.201336002541094, 132798335557537.0, 1.698382437292616, -1.5997428390842235e53, 0.0012300003101445235]);
    }
    #[test]
    fn array_array() {
        let input = vec![[123, 1], [1, 4123], [123, 421], [-1, 213], [0, 0]];
        let f = |num: [i32; 2]| -> f64 {
            ((num[0].pow(2) + num[1].pow(2)) as f64 ).powf(0.5)
        };
        let threshold = 1;
        assert_eq!(split_work(input, f, threshold), &[123.00406497347964, 4123.000121270918, 438.60004559963284, 213.00234740490538, 0.0]);
    }

    #[test]
    fn empty_array() {
        let input = vec![];
        let f = |num: f64| -> f64 {
            num * num
        };
        let threshold = 1;
        assert_eq!(split_work(input, f, threshold), &[]);
    }

    #[test]
    fn many_threads() {
        assert_eq!(split_work(vec![100;100], |x: i32| x * x, 1), vec![10000; 100]);
    }
}