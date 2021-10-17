mod tests {
    use rayon::prelude::*;

    #[test]
    fn test_stable_sort() {
        let mut v = [-5, 4, 1, -3, 2];
        v.par_sort();
        assert_eq!(v, [-5, -3, 1, 2, 4]);
    }

    #[test]
    fn test_custom_unstable_sort() {
        let mut v = [-5i32, 4, 1, -3, 2];
        v.par_sort_unstable_by_key(|k| k.abs());
        assert_eq!(v, [1, 2, -3, 4, -5]);
    }

    #[test]
    fn test_par_iter() {
        let sum = (1..=100)
            .into_par_iter() // simply parallel !
            .map(|n| n * n)
            .sum::<u32>();
        assert_eq!(sum, 338350);
    }
}
