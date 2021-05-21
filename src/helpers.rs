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

        #[derive(Debug, Copy, Clone, PartialEq, $crate::__SerdeSerialize, $crate::__SerdeDeserialize)]
        #[serde(from = "__CoreVec<<Self as __BitFlags<_>>::Flag>", into = "__CoreVec<<Self as __BitFlags<_>>::Flag>")]
        $(pub $( ($vis) )? )? struct $enum_name($repr_name);

        $crate::__bitflags_impl!($enum_name, $repr_name, $flag_name; $($variant = $value),+);
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! __bitflags_impl {
    ($enum_name:ident, $repr_name:ident, $flag_name:ident; $($variant:ident = $value:expr),+) => {
        impl $crate::__NumTraitsAsPrimitive<$repr_name> for $flag_name {
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
        }

        impl $crate::BitFlags<$repr_name> for $enum_name {
            type Flag = $flag_name;

            fn contains(&self, flag: Self) -> bool {
                self.0 & flag.0 != 0
            }
        }

        impl $crate::__CoreFrom<$crate::__CoreVec<$flag_name> for $enum_name {
            fn from(flags: $crate::__CoreVec<$flag_name>::Flag>) -> Self {
                let value = flags
                    .into_iter()
                    .map(|f| f.as_())
                    .reduce(|a, b| a|b)
                    .unwrap_or(0);

                Self(value)
            }
        }

        impl $crate::__CoreFrom<$enum_name> for $crate::__CoreVec<$flag_name> {
            fn from(flags: $enum_name) -> Self {
                vec![$($flag_name::$variant),+]
                    .into_iter()
                    .filter(|&f| flags.0 & f.as_() != 0)
                    .collect()
            }
        }

        $crate::__binop_impl!(
            [
                (__OpsBitAnd : bitand => &),
                (__OpsBitOr : bitor => |),
                (__OpsBitXor : bitxor => ^),
            ] for $enum_name, $flag_name, $repr_name
        );

        $crate::__binop_assign_impl!(
            [
                (__OpsBitAndAssign : bitand_assign => &=),
                (__OpsBitOrAssign : bitor_assign => |=),
                (__OpsBitXorAssign : bitxor_assign => ^=),
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
        impl $crate::$trait_name<Self> for $flag_name {
            type Output = $enum_name;

            fn $fn_name(self, rhs: Self) -> $enum_name {
                $enum_name(self.as_() $op rhs.as_())
            }
        }

        impl $crate::$trait_name<$enum_name> for $flag_name {
            type Output = $enum_name;

            fn $fn_name(self, rhs: $enum_name) -> $enum_name {
                $enum_name(self.as_() $op rhs.0)
            }
        }

        impl $crate::$trait_name<Self> for $enum_name {
            type Output = Self;

            fn $fn_name(self, rhs: Self) -> Self {
                Self(self.0 $op rhs.0)
            }
        }

        impl $crate::$trait_name<$flag_name> for $enum_name {
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
        impl $crate::$trait_name<Self> for $enum_name {
            fn $fn_name(&mut self, rhs: Self) {
                self.0 $op rhs.0;
            }
        }

        impl $crate::$trait_name<$flag_name> for $enum_name {
            fn $fn_name(&mut self, rhs: $flag_name) {
                self.0 $op rhs.as_();
            }
        }
    };
}
