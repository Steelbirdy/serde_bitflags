#[macro_export]
#[doc(hidden)]
macro_rules! __bitflags {
    {
        #[repr($repr_name:ident)]
        $( #[$outer_meta:meta] )*
        $( pub $( ($vis:vis) )? )? enum $enum_name:ident {
            $(
                $( #[$inner_meta:meta] )*
                $variant:ident = $value:expr,
            )+
        } ($flag_name:ident)
    } => {
        #[repr($repr_name)]
        $( #[$outer_meta] )*
        #[derive(Debug, Copy, Clone, PartialEq)]
        $(pub $( ($vis) )? )? enum $flag_name {
            $(
                $( #[$inner_meta] )*
                $variant = $value,
            )+
        }

        #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, $crate::lib::Serialize, $crate::lib::Deserialize)]
        #[serde(from = "__BFVec<<Self as __BFBitFlags<_>>::Flag>", into = "__BFVec<<Self as __BFBitFlags<_>>::Flag>")]
        $(pub $( ($vis) )? )? struct $enum_name($repr_name);

        $crate::__bitflags_impl!($enum_name, $repr_name, $flag_name; $($variant = $value),+);
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! __bitflags_impl {
    ($enum_name:ident, $repr_name:ident, $flag_name:ident; $($variant:ident = $value:expr),+) => {
        impl $crate::lib::AsPrimitive<$repr_name> for $flag_name {
            #[inline]
            fn as_(self) -> $repr_name {
                self as $repr_name
            }
        }

        impl $enum_name {
            #![allow(non_upper_case_globals)]

            $(
                pub const $variant: Self = Self($value);
            )+

            const MAX: $repr_name = 0 $( + $value )+;
            const MIN: $repr_name = 0;

            const FLAGS: &'static [Self] = &[ $( Self::$variant, )+ ];
        }

        impl $crate::BitFlags<$repr_name> for $enum_name {
            type Flag = $flag_name;

            fn all() -> Self {
                Self(Self::MAX)
            }

            fn none() -> Self {
                Self(Self::MIN)
            }

            fn flags() -> &'static [Self] {
                Self::FLAGS
            }

            fn contains(&self, flag: &Self) -> bool {
                self.0 & flag.0 != 0
            }
        }

        impl $crate::lib::From<$crate::lib::Vec<$flag_name>> for $enum_name {
            fn from(flags: $crate::lib::Vec<$flag_name>) -> Self {
                let value = flags
                    .into_iter()
                    .map(|f| f.as_())
                    .reduce(|a, b| a|b)
                    .unwrap_or(0);

                Self(value)
            }
        }

        impl $crate::lib::From<$enum_name> for $crate::lib::Vec<$flag_name> {
            fn from(flags: $enum_name) -> Self {
                vec![$($flag_name::$variant),+]
                    .into_iter()
                    .filter(|&f| flags.0 & f.as_() != 0)
                    .collect()
            }
        }

        impl $crate::lib::Sub<Self> for $enum_name {
            type Output = Self;

            fn sub(self, rhs: Self) -> Self::Output {
                Self(self.0 & !rhs.0)
            }
        }

        impl $crate::lib::Sub<$flag_name> for $enum_name {
            type Output = Self;

            fn sub(self, rhs: $flag_name) -> Self::Output {
                Self(self.0 & !rhs.as_())
            }
        }

        impl $crate::lib::SubAssign<Self> for $enum_name {
            fn sub_assign(&mut self, rhs: Self) {
                self.0 &= !rhs.0;
            }
        }

        impl $crate::lib::SubAssign<$flag_name> for $enum_name {
            fn sub_assign(&mut self, rhs: $flag_name) {
                self.0 &= !rhs.as_();
            }
        }

        $crate::__binop_impl!(
            [
                (BitAnd : bitand => &),
                (BitOr : bitor => |),
                (BitXor : bitxor => ^),
            ] for $enum_name, $flag_name, $repr_name
        );

        $crate::__binop_assign_impl!(
            [
                (BitAndAssign : bitand_assign => &=),
                (BitOrAssign : bitor_assign => |=),
                (BitXorAssign : bitxor_assign => ^=),
            ] for $enum_name, $flag_name, $repr_name
        );
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! __binop_impl {
    ([$( ($trait_name:ident : $fn_name:ident => $op:tt), )+] for $enum_name:ident, $flag_name:ident, $repr_name:ident) => {
        $(
            $crate::__binop_impl! { ($trait_name : $fn_name => $op) for $enum_name, $flag_name, $repr_name }
        )+
    };
    { ($trait_name:ident : $fn_name:ident => $op:tt) for $enum_name:ident, $flag_name:ident, $repr_name:ident } => {
        impl $crate::lib::$trait_name<Self> for $enum_name {
            type Output = Self;

            fn $fn_name(self, rhs: Self) -> Self {
                Self(self.0 $op rhs.0)
            }
        }

        impl $crate::lib::$trait_name<$flag_name> for $enum_name {
            type Output = Self;

            fn $fn_name(self, rhs: $flag_name) -> Self {
                Self(self.0 $op rhs.as_())
            }
        }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! __binop_assign_impl {
    ([$( ($trait_name:ident : $fn_name:ident => $op:tt), )+] for $enum_name:ident, $flag_name:ident, $repr_name:ident) => {
        $(
            $crate::__binop_assign_impl! { ($trait_name : $fn_name => $op) for $enum_name, $flag_name, $repr_name }
        )+
    };
    { ($trait_name:ident : $fn_name:ident => $op:tt) for $enum_name:ident, $flag_name:ident, $repr_name:ident } => {
        impl $crate::lib::$trait_name<Self> for $enum_name {
            fn $fn_name(&mut self, rhs: Self) {
                self.0 $op rhs.0;
            }
        }

        impl $crate::lib::$trait_name<$flag_name> for $enum_name {
            fn $fn_name(&mut self, rhs: $flag_name) {
                self.0 $op rhs.as_();
            }
        }
    };
}
