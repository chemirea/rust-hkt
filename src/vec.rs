use crate::type_classes::{Functor, HKT};

derive_hkt!(Vec);

impl<Base, Target> Functor<Target> for Vec<Base> {
    fn fmap<F>(&self, f: F) -> Vec<Target>
    where
        F: Fn(&Base) -> Target,
    {
        let mut result = Vec::with_capacity(self.len());

        for v in self {
            result.push(f(v));
        }

        result
    }
}

#[cfg(test)]
mod test {
    use crate::vec::*;

    // Helper function for test
    // テスト用のヘルパー関数
    fn double_fn() -> impl Fn(&isize) -> isize {
        |x| x * 2
    }

    #[test]
    fn vec_map_test() {
        let numbers = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let numbers = numbers.fmap(&double_fn());
        assert_eq!(vec![0, 2, 4, 6, 8, 10, 12, 14, 16, 18], numbers);
    }
}
