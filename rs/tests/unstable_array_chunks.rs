#![cfg_attr(feature = "nightly", feature(array_chunks))]

#[cfg(test)]
mod tests {
    #[cfg(nightly)]
    #[test]
    fn test_array_chunks_infer() {
        let v: &[i32] = &[0, 1, 2, 3, 4, -4];
        let c = v.array_chunks();
        for &[a, b, c] in c {
            assert_eq!(a + b + c, 3);
        }

        let v2: &[i32] = &[0, 1, 2, 3, 4, 5, 6];
        let total = v2.array_chunks().map(|&[a, b]| a * b).sum::<i32>();
        assert_eq!(total, 2 * 3 + 4 * 5);
    }
}
