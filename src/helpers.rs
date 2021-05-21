#[macro_export]
#[doc(hidden)]
macro_rules! __bitflags {
    {
        #[$repr_meta:meta]
        $( #[$outer_meta:meta] )*
        $( pub $( ($vis:vis) )? )? enum $enum_name:ident: $repr_name:ident {
            $(
                $( #[$inner_meta:meta] )*
                $variant:ident = $value:expr,
            )+
        } ($flag_name:ident)
    } => {
        #[$repr_meta]
        $( #[$outer_meta] )*
        #[derive(Debug, Copy, Clone, PartialEq, $crate::__SerdeSerialize, $crate::__SerdeDeserialize)]
        $(pub $( ($vis) )? )? enum $flag_name {
            $(
                $( #[$inner_meta] )*
                $variant = $value,
            )+
        }

        #[derive(Debug, Copy, Clone, PartialEq, $crate::__SerdeSerialize, $crate::__SerdeDeserialize)]
        $(pub $( ($vis) )? )? struct $enum_name($repr_name);

        $crate::__bitflags_impl!($enum_name, $repr_name, $flag_name; $($variant),+);
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! __bitflags_impl {
    ($enum_name:ident, $repr_name:ident, $flag_name:ident; $($variant:ident),+) => {
        impl $crate::__NumTraitsAsPrimitive<$repr_name> for $flag_name {
            fn as_(self) -> $repr_name {
                self as $repr_name
            }
        }

        impl $enum_name {
            #![allow(non_upper_case_globals)]

            $(
                pub const $variant: $flag_name = $flag_name::$variant;
            )+
        }

        impl $crate::BitFlags<$repr_name> for $enum_name {
            type Flag = $flag_name;

            fn contains(&self, flag: Self::Flag) -> bool {
                self.0 & flag.as_() != 0
            }
        }

        impl $crate::__CoreFrom<$crate::__CoreVec<<$enum_name as $crate::BitFlags<$repr_name>>::Flag>> for $enum_name {
            fn from(flags: $crate::__CoreVec<<$enum_name as $crate::BitFlags<$repr_name>>::Flag>) -> Self {
                flags.into_iter()
                    .fold(Self(0), |a, b| a|b)
            }
        }

        impl $crate::__CoreFrom<$enum_name> for $crate::__CoreVec<<$enum_name as $crate::BitFlags<$repr_name>>::Flag> {
            fn from(flags: $enum_name) -> Self {
                vec![$($flag_name::$variant),+]
                    .into_iter()
                    .filter(|&f| flags.contains(f))
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
