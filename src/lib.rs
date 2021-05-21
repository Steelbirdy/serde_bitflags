mod helpers;

use num_traits::{AsPrimitive, PrimInt};

pub use {
    core::ops::{
        BitAnd as __OpsBitAnd, BitAndAssign as __OpsBitAndAssign, BitOr as __OpsBitOr,
        BitOrAssign as __OpsBitOrAssign, BitXor as __OpsBitXor, BitXorAssign as __OpsBitXorAssign,
        Sub as __OpsSub, SubAssign as __OpsSubAssign,
    },
    num_traits::AsPrimitive as __NumTraitsAsPrimitive,
    paste::paste as __Paste,
    serde::{Deserialize as __SerdeDeserialize, Serialize as __SerdeSerialize},
    From as __CoreFrom, Into as __CoreInto, Vec as __CoreVec,
};

pub trait BitFlags<Repr: 'static + PrimInt>: From<Vec<Self::Flag>> + Into<Vec<Self::Flag>> {
    type Flag: AsPrimitive<Repr>;

    fn contains(&self, flag: Self) -> bool;
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
        use $crate::{__CoreVec, __NumTraitsAsPrimitive, BitFlags as __BitFlags};

        $crate::__Paste! {
            $crate::__bitflags! {
                #[repr($repr_name)]
                $( #[$outer_meta] )*
                $( pub $( ($vis) )? )? enum $enum_name {
                    $(
                        $( #[$inner_meta] )*
                        $variant = $value,
                    )+
                } ([< __ $enum_name Flag >])
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::bitflags;

    bitflags! {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[serde(rename_all = "lowercase")]
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
    type Flag = __PrimitiveTypeFlag;

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

    #[test]
    fn sub_assign_bitflags_with_flag() {
        let mut lhs = ty(0b0010111);
        let rhs = Ty::Null;

        lhs -= rhs;

        assert_eq!(lhs, ty(0b0010110));
    }

    #[test]
    fn sub_assign_bitflags_with_bitflags() {
        let mut lhs = ty(0b0010111);
        let rhs = ty(0b1010010);

        lhs -= rhs;

        assert_eq!(lhs, ty(0b0000101));
    }

    #[test]
    fn bitflags_from_flag_vec() {
        let flag_vec = vec![Flag::Null, Flag::Boolean, Flag::String];

        assert_eq!(Ty::from(flag_vec), ty(0b0100011));
    }

    #[test]
    fn bitflags_into_flag_vec() {
        let flags = ty(0b0100011);

        let flag_vec = Vec::from(flags);

        assert_eq!(flag_vec, vec![Flag::Null, Flag::Boolean, Flag::String]);
    }

    #[test]
    fn serialize_bitflags_as_json_array() {
        let input = ty(0b0101010);

        let expected = r#"[
  "boolean",
  "array",
  "string"
]"#
        .to_owned();

        assert_eq!(serde_json::to_string_pretty(&input).unwrap(), expected);
    }

    #[test]
    fn deserialize_json_array_as_bitflags() {
        let input = r#"["boolean", "array", "string"]"#;

        let expected = ty(0b0101010);

        assert_eq!(
            serde_json::from_str::<PrimitiveType>(input).unwrap(),
            expected,
        );
    }
}
