use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
use num_traits::{Zero, One, real::Real, Num, NumCast};

macro_rules! replace_expr {
    ($_t:tt $sub:ident) => {$sub}
}

macro_rules! vector_type {
    ($type_name:ident, $dim_count:literal, $($dim_index:literal:$dim:ident),+) => {
        #[derive(Copy, Clone, Debug, Default, Eq)]
        pub struct $type_name<T> {
            $(pub $dim: T),+
        }

        impl<T> $type_name<T> {
            // Create a vector from the elements `x, y`.
            pub fn new($($dim: T),+) -> Self {
                Self { $($dim),+ }
            }

            pub fn numcast<D>(self) -> Option<$type_name<D>>
            where
                T: NumCast,
                D: NumCast,
            {
                Some($type_name::new(
                    $(if let Some($dim) = D::from(self.$dim) { $dim } else { return None },)+
                ))
            }
        }

        impl<T> $type_name<T>
        where
            T: Copy,
        {
            pub fn broadcast(value: T) -> Self {
                Self { $($dim: value),+ }
            }

            pub fn zero() -> Self
            where
                T: Zero,
            {
                Self::broadcast(T::zero())
            }

            pub fn one() -> Self
            where
                T: One,
            {
                Self::broadcast(T::one())
            }

            pub fn into_array(&self) -> [T; $dim_count] {
                [$(self.$dim),+]
            }

            pub fn dot(self, other: Self) -> T
            where
                T: Zero + AddAssign + Mul<Output = T>
            {
                let mut result = T::zero();
                $(result += self.$dim * other.$dim;)+
                result
            }

            pub fn magnitude_squared(self) -> T
            where
                T: Copy + Zero + AddAssign + Mul<Output = T>,
            {
                self.dot(self)
            }

            pub fn magnitude(self) -> T
            where
                T: Zero + AddAssign + Mul + Real,
            {
                self.magnitude_squared().sqrt()
            }

            pub fn apply<F>(&mut self, f: F)
            where
                F: Fn(T) -> T,
            {
                $(self.$dim = f(self.$dim);)+
            }

            pub fn min(self, other: Self) -> Self
            where
                T: Ord,
            {
                Self {
                    $($dim: self.$dim.min(other.$dim),)+
                }
            }

            pub fn max(self, other: Self) -> Self
            where
                T: Ord,
            {
                Self {
                    $($dim: self.$dim.max(other.$dim),)+
                }
            }

            pub fn ceil(self) -> Self
            where
                T: Real,
            {
                Self {
                    $($dim: self.$dim.ceil(),)+
                }
            }

            pub fn floor(self) -> Self
            where
                T: Real,
            {
                Self {
                    $($dim: self.$dim.floor(),)+
                }
            }

            pub fn sum(self) -> T
            where
                T: Zero + AddAssign,
            {
                let mut result = T::zero();
                $(result += self.$dim;)+
                result
            }

            pub fn sqrt(self) -> Self
            where
                T: Real,
            {
                Self {
                    $($dim: self.$dim.sqrt(),)+
                }
            }

            pub fn map<F, U>(self, f: F) -> $type_name<U>
            where
                F: Fn(T) -> U,
            {
                $type_name::<U> {
                    $($dim: f(self.$dim),)+
                }
            }
        }

        impl<T> PartialEq for $type_name<T>
        where
            T: PartialEq,
        {
            fn eq(&self, other: &Self) -> bool {
                $(self.$dim.eq(&other.$dim)) &&+
            }
        }

        impl<T> Add for $type_name<T>
        where
            T: Add<Output = T>,
        {
            type Output = Self;

            fn add(self, rhs: Self) -> Self::Output {
                Self {
                    $($dim: self.$dim + rhs.$dim,)+
                }
            }
        }

        impl<T> AddAssign for $type_name<T>
        where
            T: AddAssign,
        {
            fn add_assign(&mut self, rhs: Self) {
                $(self.$dim += rhs.$dim;)+
            }
        }

        impl<T> Sub for $type_name<T>
        where
            T: Sub<Output = T>,
        {
            type Output = Self;

            fn sub(self, rhs: Self) -> Self::Output {
                Self {
                    $($dim: self.$dim - rhs.$dim,)+
                }
            }
        }

        impl<T> SubAssign for $type_name<T>
        where
            T: SubAssign,
        {
            fn sub_assign(&mut self, rhs: Self) {
                $(self.$dim -= rhs.$dim;)+
            }
        }

        impl<T> Mul<T> for $type_name<T>
        where
            T: Mul<Output = T> + Copy,
        {
            type Output = Self;

            fn mul(self, rhs: T) -> Self::Output {
                Self {
                    $($dim: self.$dim * rhs,)+
                }
            }
        }

        impl<T> MulAssign<T> for $type_name<T>
        where
            T: MulAssign + Copy,
        {
            fn mul_assign(&mut self, rhs: T) {
                $(self.$dim *= rhs;)+
            }
        }

        impl<T> Div<T> for $type_name<T>
        where
            T: Div<Output = T> + Copy,
        {
            type Output = Self;

            fn div(self, rhs: T) -> Self::Output {
                Self {
                    $($dim: self.$dim / rhs,)+
                }
            }
        }

        impl<T> DivAssign<T> for $type_name<T>
        where
            T: DivAssign + Copy,
        {
            fn div_assign(&mut self, rhs: T) {
                $(self.$dim /= rhs;)+
            }
        }

        impl<T> From<$type_name<T>> for ($(replace_expr!($dim T)),+) {
            fn from(vector: $type_name<T>) -> Self {
                ($(vector.$dim),+)
            }
        }

        impl<T> From<$type_name<T>> for [T; $dim_count] {
            fn from(vector: $type_name<T>) -> Self {
                [$(vector.$dim),+]
            }
        }

        impl<T> From<($(replace_expr!($dim T)),+)> for $type_name<T> {
            fn from(src: ($(replace_expr!($dim T)),+)) -> Self {
                let ($($dim),+) = src;
                Self {
                    $($dim,)+
                }
            }
        }

        impl<T> From<[T; $dim_count]> for $type_name<T>
        where
            T: Copy + Num,
        {
            fn from(array: [T; $dim_count]) -> Self {
                Self {
                    $($dim: array[$dim_index],)+
                }
            }
        }
    }
}

vector_type!(Vector2, 2, 0:x, 1:y);
vector_type!(Vector3, 3, 0:x, 1:y, 2:z);
vector_type!(Vector4, 4, 0:x, 1:y, 2:z, 3:w);
