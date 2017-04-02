macro_rules! typed_command {
    ( $(#[$attr:meta])* enum $name:ident { $($variant:tt),* } ) => {
        $(
            #[$attr]
            )*
        #[derive(Clone, PartialEq, Hash, Debug)]
        pub enum $name {
            $(
                $variant,
                )*

            /// A command which doesn't currently have a variant.
            UnrecognisedCommand(String),
        }


        impl ::std::convert::From<$crate::RawCommand> for $name {
            fn from(_other: $crate::RawCommand) -> Self {
                unimplemented!()
            }
        }
    }
}
