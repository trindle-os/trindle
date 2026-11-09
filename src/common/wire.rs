//! The program's side of Trindle's own Wayland protocols (the XML is in
//! src/compose/protocols). The compositor builds its side from the same
//! files, so the two cannot drift apart.

macro_rules! client_of {
    ($module:ident, $xml:literal) => {
        #[allow(dead_code, non_camel_case_types, unused_unsafe, unused_variables, non_upper_case_globals,
                non_snake_case, unused_imports, missing_docs, clippy::all)]
        pub mod $module {
            use wayland_client;
            use wayland_client::protocol::*;

            pub mod __interfaces {
                use wayland_client::backend as wayland_backend;
                use wayland_client::protocol::__interfaces::*;
                wayland_scanner::generate_interfaces!($xml);
            }
            use self::__interfaces::*;

            wayland_scanner::generate_client_code!($xml);
        }
    };
}

client_of!(windows, "src/compose/protocols/windows.xml");
client_of!(workspaces, "src/compose/protocols/workspaces.xml");
client_of!(night, "src/compose/protocols/night.xml");
client_of!(screens, "src/compose/protocols/screens.xml");
