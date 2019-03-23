//! Static helpers used by generated code
use std::fmt;

macro_rules! wrapper {
    {$(#[$meta: meta])* $ident:ident($ty:ty)} => {
        $(#[$meta])* #[repr(transparent)]
        pub struct $ident($ty);
        impl $ident {
            pub fn from_raw(x: $ty) -> Self { Self(x) }
            pub fn into_raw(self) -> $ty { self.0 }
        }
    }
}

macro_rules! bitmask {
    ($name:ident) => {
        impl $name {
            pub const EMPTY: Self = Self(0);

            #[inline]
            pub fn from_raw(x: u64) -> Self {
                Self(x)
            }

            #[inline]
            pub fn into_raw(self) -> u64 {
                self.0
            }

            #[inline]
            pub fn is_empty(self) -> bool {
                self == $name::EMPTY
            }

            #[inline]
            pub fn intersects(self, other: $name) -> bool {
                self & other != $name::EMPTY
            }

            /// Returns whether `other` is a subset of `self`
            #[inline]
            pub fn contains(self, other: $name) -> bool {
                self & other == other
            }
        }

        impl Default for $name {
            fn default() -> Self {
                Self::EMPTY
            }
        }

        impl std::ops::BitOr for $name {
            type Output = $name;

            #[inline]
            fn bitor(self, rhs: $name) -> $name {
                $name(self.0 | rhs.0)
            }
        }

        impl std::ops::BitOrAssign for $name {
            #[inline]
            fn bitor_assign(&mut self, rhs: $name) {
                *self = *self | rhs
            }
        }

        impl std::ops::BitAnd for $name {
            type Output = $name;

            #[inline]
            fn bitand(self, rhs: $name) -> $name {
                $name(self.0 & rhs.0)
            }
        }

        impl std::ops::BitAndAssign for $name {
            #[inline]
            fn bitand_assign(&mut self, rhs: $name) {
                *self = *self & rhs
            }
        }

        impl std::ops::BitXor for $name {
            type Output = $name;

            #[inline]
            fn bitxor(self, rhs: $name) -> $name {
                $name(self.0 ^ rhs.0)
            }
        }

        impl std::ops::BitXorAssign for $name {
            #[inline]
            fn bitxor_assign(&mut self, rhs: $name) {
                *self = *self ^ rhs
            }
        }

        impl std::ops::Not for $name {
            type Output = $name;

            #[inline]
            fn not(self) -> $name {
                Self(!self.0)
            }
        }
    };
}

macro_rules! handle {
    ($name:ident) => {
        impl $name {
            pub const NULL: Self = Self(0);
            #[inline]
            pub fn from_raw(x: u64) -> Self {
                Self(x)
            }
            #[inline]
            pub fn into_raw(self) -> u64 {
                self.0
            }
        }
        impl Default for $name {
            fn default() -> Self {
                Self::NULL
            }
        }
    };
}

pub fn fmt_enum(f: &mut fmt::Formatter, value: i32, name: Option<&'static str>) -> fmt::Result {
    match name {
        Some(x) => f.pad(x),
        None => <i32 as fmt::Debug>::fmt(&value, f),
    }
}
