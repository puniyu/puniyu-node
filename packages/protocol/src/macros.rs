#[macro_export]
macro_rules! impl_from_trait {
    ($type_a:ty, $type_b:ty, {$($field_a:ident => $field_b:ident),* $(,)?}) => {
        impl From<$type_a> for $type_b {
            fn from(value: $type_a) -> Self {
                Self {
                    $(
                        $field_b: value.$field_a.into(),
                    )*
                }
            }
        }

        impl From<$type_b> for $type_a {
            fn from(value: $type_b) -> Self {
                Self {
                    $(
                        $field_a: value.$field_b.into(),
                    )*
                }
            }
        }
    };
}

#[macro_export]
macro_rules! impl_enum_from_trait {
    ($type_a:ty, $type_b:ty, {$($variant_a:ident => $variant_b:ident $( ( $($field:tt)* ) )?),* $(,)?}) => {
        impl From<$type_a> for $type_b {
            fn from(value: $type_a) -> Self {
                match value {
                    $(
                        <$type_a>::$variant_a $( ( $($field)* ) )? => <$type_b>::$variant_b $( ( $($field)* ) )?,
                    )*
                }
            }
        }

        impl From<$type_b> for $type_a {
            fn from(value: $type_b) -> Self {
                match value {
                    $(
                        <$type_b>::$variant_b $( ( $($field)* ) )? => <$type_a>::$variant_a $( ( $($field)* ) )?,
                    )*
                }
            }
        }
    };
}
