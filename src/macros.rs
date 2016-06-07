macro_rules! fake_enum_nonrec {
    ($( #[ $attrs:meta ] )* pub enum $name:ident { $($variant:ident = $value:expr),* $(,)* }) => {
        #[cfg(target_env = "msvc")]
        $( #[ $attrs ] )*
        pub type $name = i32;
        #[cfg(not(target_env = "msvc"))]
        $( #[ $attrs ] )*
        pub type $name = u32;
        $( pub const $variant: $name = $value; )*
    };
}

macro_rules! fake_enum {
    ($( #[ $attrs:meta ] )* pub enum $name:ident { $( $variants:tt )* }) => {
        #[cfg(target_env = "msvc")]
        $( #[ $attrs ] )*
        pub type $name = i32;
        #[cfg(not(target_env = "msvc"))]
        $( #[ $attrs ] )*
        pub type $name = u32;
        fake_enum!(@items $name, @ 0, /* no attrs */, $( $variants )*);
    };
    ($( #[$attrs:meta] )* pub enum $name:ident: $t:ty { $( $variants:tt )* }) => {
        $( #[ $attrs ] )*
        pub type $name = $t;
        fake_enum!(@items $name, @ 0, /* no attrs */, $( $variants )*);
    };
// Read an attribute
    (@items $name:ident, $base:tt $shift:expr, $( #[ $attrs:meta ] )*, #[ $attr:meta ] $( $tail:tt )*) => {
        fake_enum!(@items $name, $base $shift, $( #[ $attrs ] )* #[ $attr ], $( $tail )*);
    };
// Empty enum
    (@items $name:ident, $base:tt $shift:expr, $( #[ $attrs:meta ] )*,,) => {
        $( #[ $attrs ] )*
    };
    (@items $name:ident, $base:tt $shift:expr, $( #[ $attrs:meta ] )*,) => {
        $( #[ $attrs ] )*
    };
// Read a variant with set value, non-empty buffer
    (@items $name:ident, $base:tt $shift:expr, $( #[ $attrs:meta ] )*, $variant:ident = $vvalue:expr, $( $tail:tt )* ) => {
        fake_enum!(@item $name, $( #[ $attrs ] )*, $variant, @ $vvalue);
        fake_enum!(@items $name, $variant 1, /*no attrs*/, $( $tail )*);
    };
// Read a last variant with set value
    ( @items $name:ident, $base:tt $shift:expr, $( #[ $attrs:meta ] )*, $variant:ident = $vvalue:expr) => {
        fake_enum!(@item $name, $( #[ $attrs ] )*, $variant, @ $vvalue);
    };
// No value, non-empty buffer
    (@items $name:ident, $base:tt $shift:expr, $( #[ $attrs:meta ] )*, $variant:ident, $( $tail:tt )*) => {
        fake_enum!(@item $name, $( #[ $attrs ] )*, $variant, $base $shift);
        fake_enum!(@items $name, $base $shift+1, /*no attrs*/, $( $tail )*);
    };
// No value, empty buffer
    (@items $name:ident, $base:tt $shift:expr, $( #[ $attrs:meta ] )*, $variant:ident) => {
        fake_enum!(@item $name, $( #[ $attrs ] )*, $variant, $base $shift);
    };
// Implement the item
    (@item $name:ident, $( #[ $attrs:meta ] )*, $variant:ident, @ $shift:expr) => {
        $( #[ $attrs ] )*
        pub const $variant: $name = $shift;
    };
    (@item $name:ident, $( #[ $attrs:meta ] )*, $variant:ident, $base:ident $shift:expr) => {
        $( #[ $attrs ] )*
        pub const $variant: $name = $base + $shift;
    };
}
