pub mod benchmarker;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn c_interop_works_const() {
        let tuple1 = benchmarker::tuple {a: 1, b: 2};
        let tuple2 = benchmarker::tuple {a: 3, b: 4};
        let result = unsafe { benchmarker::tuple_add(&tuple1, &tuple2) };
        assert_eq!(result.a, 4);
        assert_eq!(result.b, 6);
    }

    #[test]
    fn c_interop_works_non_const() {
        let mut tuple1 = benchmarker::tuple {a: 1, b: 2};
        let tuple2 = benchmarker::tuple {a: 3, b: 4};
        unsafe { benchmarker::inline_tuple_add(&mut tuple1, &tuple2) };
        assert_eq!(tuple1.a, 4);
        assert_eq!(tuple1.b, 6);
    }
}
