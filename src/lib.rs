mod helpers;

use num_traits::{AsPrimitive, PrimInt};

pub use {
    core::ops::{
        BitAnd as __OpsBitAnd, BitAndAssign as __OpsBitAndAssign, BitOr as __OpsBitOr,
        BitOrAssign as __OpsBitOrAssign, BitXor as __OpsBitXor, BitXorAssign as __OpsBitXorAssign,
    },
    num_traits::AsPrimitive as __NumTraitsAsPrimitive,
    paste::paste as __Paste,
    serde::{Deserialize as __SerdeDeserialize, Serialize as __SerdeSerialize},
};

pub trait BitFlags<Repr: 'static + PrimInt> {
    type Flag: AsPrimitive<Repr>;

    fn contains(&self, flag: Self::Flag) -> bool;
}

// TODO: Once inherent associated types are stable, use that instead of <name>Flag

#[macro_export]
macro_rules! bitflags {
    {
        $( #[$outer_meta:meta] )*
        $( pub $( ($vis:vis) )? )? enum $enum_name:ident: $repr_name:ident {
            $(
                $( #[$inner_meta:meta] )*
                $variant:ident = $value:expr,
            )+
        }
    } => {
        use $crate::__NumTraitsAsPrimitive;

        $crate::__Paste! {
            $crate::__bitflags! {
                #[repr($repr_name)]
                $( #[$outer_meta] )*
                $( pub $( ($vis) )? )? enum $enum_name: $repr_name {
                    $(
                        $( #[$inner_meta] )*
                        $variant = $value,
                    )+
                } ([< $enum_name Flag >])
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::bitflags;

    bitflags! {
        pub enum PrimitiveType: u8 {
            Null = 1,
            Boolean = 1 << 1,
            Object = 1 << 2,
            Array = 1 << 3,
            Number = 1 << 4,
            String = 1 << 5,
            Integer = 1 << 6,
        }
    }

    fn ty(v: u8) -> PrimitiveType {
        PrimitiveType(v)
    }

    type Ty = PrimitiveType;

    #[test]
    fn sanity() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn bitand_bitflags_with_flag() {
        let lhs = ty(0b0010111);
        let rhs = Ty::Null;

        assert_eq!(lhs & rhs, ty(0b0000001));
    }

    #[test]
    fn bitand_bitflags_with_bitflags() {
        let lhs = ty(0b0010111);
        let rhs = ty(0b1010010);

        assert_eq!(lhs & rhs, ty(0b0010010));
    }

    #[test]
    fn bitand_flag_with_flag() {
        let lhs = Ty::String;
        let rhs = Ty::Integer;

        assert_eq!(lhs & rhs, ty(0b0000000));

        assert_eq!(Ty::Boolean & Ty::Boolean, ty(0b0000010));
    }

    #[test]
    fn bitand_flag_with_bitflags() {
        let lhs = Ty::Object;
        let rhs = ty(0b0111110);

        assert_eq!(lhs & rhs, ty(0b0000100));
    }

    #[test]
    fn bitor_bitflags_with_flag() {
        let lhs = ty(0b0010110);
        let rhs = Ty::Null;

        assert_eq!(lhs | rhs, ty(0b0010111));
    }

    #[test]
    fn bitor_bitflags_with_bitflags() {
        let lhs = ty(0b0010111);
        let rhs = ty(0b1010010);

        assert_eq!(lhs | rhs, ty(0b1010111));
    }

    #[test]
    fn bitor_flag_with_flag() {
        let lhs = Ty::Number;
        let rhs = Ty::Boolean;

        assert_eq!(lhs | rhs, ty(0b0010010));
    }

    #[test]
    fn bitor_flag_with_bitflags() {
        let lhs = Ty::Object;
        let rhs = ty(0b0111110);

        assert_eq!(lhs | rhs, ty(0b0111110));
    }

    #[test]
    fn bitxor_bitflags_with_flag() {
        let lhs = ty(0b0010110);
        let rhs = Ty::Null;

        assert_eq!(lhs ^ rhs, ty(0b0010111));
    }

    #[test]
    fn bitxor_bitflags_with_bitflags() {
        let lhs = ty(0b0010111);
        let rhs = ty(0b1010010);

        assert_eq!(lhs ^ rhs, ty(0b1000101));
    }

    #[test]
    fn bitxor_flag_with_flag() {
        let lhs = Ty::Array;
        let rhs = Ty::Integer;

        assert_eq!(lhs ^ rhs, ty(0b1001000));
    }

    #[test]
    fn bitxor_flag_with_bitflags() {
        let lhs = Ty::Object;
        let rhs = ty(0b0111110);

        assert_eq!(lhs ^ rhs, ty(0b0111010));
    }

    #[test]
    fn bitand_assign_bitflags_with_flag() {
        let mut lhs = ty(0b0010111);
        let rhs = Ty::Null;

        lhs &= rhs;

        assert_eq!(lhs, ty(0b0000001));
    }

    #[test]
    fn bitand_assign_bitflags_with_bitflags() {
        let mut lhs = ty(0b0010111);
        let rhs = ty(0b1010010);

        lhs &= rhs;

        assert_eq!(lhs, ty(0b0010010));
    }

    #[test]
    fn bitor_assign_bitflags_with_flag() {
        let mut lhs = ty(0b0010110);
        let rhs = Ty::Null;

        lhs |= rhs;

        assert_eq!(lhs, ty(0b0010111));
    }

    #[test]
    fn bitor_assign_bitflags_with_bitflags() {
        let mut lhs = ty(0b0010111);
        let rhs = ty(0b1010010);

        lhs |= rhs;

        assert_eq!(lhs, ty(0b1010111));
    }

    #[test]
    fn bitxor_assign_bitflags_with_flag() {
        let mut lhs = ty(0b0010110);
        let rhs = Ty::Null;

        lhs ^= rhs;

        assert_eq!(lhs, ty(0b0010111));
    }

    #[test]
    fn bitxor_assign_bitflags_with_bitflags() {
        let mut lhs = ty(0b0010111);
        let rhs = ty(0b1010010);

        lhs ^= rhs;

        assert_eq!(lhs, ty(0b1000101));
    }
}
