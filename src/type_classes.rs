pub trait HKT<Target> {
    // $HKT<Base>
    type Base;

    // HKTT == HKT<Target>
    type HKTT;
}

macro_rules! derive_hkt {
    ($t:ident) => {
        impl<Base, Target> HKT<Target> for $t<Base> {
            type Base = Base;
            type HKTT = $t<Target>;
        }
    };
}

pub trait Functor<Target>: HKT<Target> {
    fn fmap<F>(&self, f: F) -> Self::HKTT
    where
        F: Fn(&Self::Base) -> Target;
}

pub trait Applicative<Target>: Functor<Target> {
    fn apure(v: Self::Base) -> Self::HKTT
    where
        // BaseとTargetは同じ型
        // Base and Target are the same type
        Self: HKT<Target, Base = Target>;

    // <*>
    fn seq<F>(&self, f: <Self as HKT<F>>::HKTT) -> <Self as HKT<Target>>::HKTT
    where
        Self: HKT<F>,
        F: Fn(&<Self as HKT<F>>::Base) -> Target;
}

pub trait Monad<Target>: Applicative<Target> {
    // mreturn == apure
    fn mreturn(v: Target) -> Self::HKTT
    where
        // BaseとTargetは同じ型
        // Base and Target are the same type
        Self: HKT<Target, Base = Target>,
    {
        Self::apure(v)
    }

    // >>=
    fn bind<F>(&self, f: F) -> <Self as HKT<Target>>::HKTT
    where
        F: Fn(&Self::Base) -> <Self as HKT<Target>>::HKTT;

    // 高階型がT型でかつBaseになる型も同じ高階型のとき、
    // 同じ文脈の二つのスタックを一つにする
    fn join<T>(&self) -> T
    where
        Self: HKT<Target, HKTT = T, Base = T>,
        T: Clone,
    {
        self.bind(|x| x.clone())
    }
}
