// TODO: Write a macro which can turn any variant into its corresponding type
macro_rules! variant {
    ($name:ident) => {
        name
    };
    ($name:ident(count)) => {
        // TODO: Finish this.
    };
    ($name:ident(count, args)) => {
        // TODO: Finish this.
    };
    ($name:ident(args)) => {
        // TODO: Finish this.
    };
}

/// A macro which allows you to create your own custom command.
///
/// Given something like this:
///
/// ```rust,ignore
/// custom_command!(enum MyCommand {
///   Play,
///   ShowBoard,
/// })
/// ```
///
/// The macro will expand to something like this:
///
/// ```
/// #[derive(Clone, PartialEq, Hash, Debug)]
/// #[allow(missing_docs)]
/// pub enum MyCommand {
///   Play,
///   ShowBoard,
///   UnrecognisedCommand(Option<u32>, String, Vec<String>),
/// }
/// ```
///
/// Note the `UnrecognisedCommand` variant, this acts as a catch all if you
/// are sent an unknown command.
///
/// The macro also provides a `std::convert::From` impl so you can convert
/// from a `RawCommand` into your command. This allows the parser to
/// transparently convert a line from the `Go Text Protocol` into your custom
/// type.
///
///
/// # Note
///
/// At the moment the macro isn't complete. In the future I'd like to be able
/// to do something like the following and have the command automatically
/// attach the count and/or args accordingly.
///
/// ```rust,ignore
/// custom_command!( enum MyCommand {
///   Play(count, args),
///   ShowBoard,
///   BoardSize(args),
///   Quit,
/// })
/// ```
#[macro_export]
macro_rules! custom_command {
    ( $(#[$attr:meta])* enum $name:ident { $($command:tt,)* } ) => {
        $(
            #[$attr]
            )*
        #[derive(Clone, PartialEq, Hash, Debug)]
        #[allow(missing_docs)]
        pub enum $name {
            $(
                $command,
                )*

            /// A command which doesn't currently have a variant.
            UnrecognisedCommand(Option<u32>, String, Vec<String>),
        }


        impl ::std::convert::From<$crate::RawCommand> for $name {
            fn from(raw: $crate::RawCommand) -> Self {
                let name_as_lower = raw.name.to_lowercase();

                $(
                    if name_as_lower == stringify!($command).to_lowercase() {
                        return $name::$command;
                    }
                    )*

                // If we got this far then there were no matches
                $name::UnrecognisedCommand(raw.count, raw.name, raw.args)
            }
        }
    }
}
