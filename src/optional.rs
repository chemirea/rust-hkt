use crate::type_classes::{Applicative, Functor, Monad, HKT};

derive_hkt!(Option);

impl<Base, Target> Functor<Target> for Option<Base> {
    fn fmap<F>(&self, f: F) -> Option<Target>
    where
        F: Fn(&Base) -> Target,
    {
        match self {
            Some(v) => Some(f(v)),
            None => None,
        }
    }
}

impl<Base, Target> Applicative<Target> for Option<Base> {
    fn apure(v: Target) -> Self::HKTT {
        Some(v)
    }

    fn seq<F>(&self, f: <Self as HKT<F>>::HKTT) -> <Self as HKT<Target>>::HKTT
    where
        F: Fn(&<Self as HKT<F>>::Base) -> Target,
    {
        match self {
            Some(v) => match f {
                Some(f) => Some(f(v)),
                None => None,
            },
            None => None,
        }
    }
}

impl<Base, Target> Monad<Target> for Option<Base> {
    fn bind<F>(&self, f: F) -> Option<Target>
    where
        F: Fn(&Self::Base) -> Option<Target>,
    {
        match self {
            Some(v) => f(v),
            None => None,
        }
    }
}

#[cfg(test)]
mod test {
    // Helper function for test
    // テスト用のヘルパー関数
    mod helper {
        pub fn double_fn() -> impl Fn(&isize) -> isize {
            |x| x * 2
        }

        pub fn safe_div_by(x: isize) -> impl Fn(&isize) -> Option<isize> {
            move |n| match x {
                0 => None,
                y => Some(n / y),
            }
        }
    }

    use crate::optional::*;

    #[test]
    fn option_map_test() {
        let number1 = Some(10);
        let number2 = number1.fmap(helper::double_fn());

        assert_eq!(Some(20), number2);
        println!("{:?}", number1);
    }

    #[test]
    fn option_pure_test() {
        let number: Option<isize> = Option::<isize>::apure(10);

        assert_eq!(Some(10), number);
    }

    #[test]
    fn option_seq_test() {
        let f = &helper::double_fn();
        let f = Some(f);

        let result = Some(10).seq(f);
        assert_eq!(Some(20), result);
    }

    #[test]
    fn option_bind_test() {
        let div_by_2 = &helper::safe_div_by(2);
        let div_by_0 = &helper::safe_div_by(0);

        let result = Option::mreturn(20).bind(div_by_2);
        assert_eq!(Some(10), result);

        let result = Option::mreturn(20).bind(div_by_0);
        assert_eq!(None, result);

        let result = Option::mreturn(20).bind(div_by_2).bind(div_by_2);
        assert_eq!(Some(5), result);

        let result = Option::mreturn(20).bind(div_by_2).bind(div_by_0);
        assert_eq!(None, result);

        let result = Option::mreturn(20).bind(div_by_0).bind(div_by_2);
        assert_eq!(None, result);

        let result = Option::mreturn(20).bind(div_by_0).bind(div_by_0);
        assert_eq!(None, result);
    }
}
