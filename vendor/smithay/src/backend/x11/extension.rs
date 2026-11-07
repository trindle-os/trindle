use tracing::{error, info};

use super::{MissingExtensionError, X11Error};

macro_rules! extensions {
    (
        $(
            $extension:ident {
                $extension_fn:ident,
                required: $required:expr,
                minimum: ($min_major:expr, $min_minor:expr),
                request: ($req_major:expr, $req_minor:expr),
            },
        )*
    ) => {
        #[allow(dead_code)]
        #[derive(Debug, Copy, Clone)]
        pub struct Extensions {
            $(
                #[doc = concat!(" The version of the `", stringify!($extension), "` extension.")]
                pub $extension: Option<(u32, u32)>,
            )*
        }

        impl Extensions {
            pub fn check_extensions<C: x11rb::connection::Connection>(connection: &C) -> Result<Extensions, X11Error> {
                $(
                    let $extension = {
                        use x11rb::protocol::$extension::{ConnectionExt as _, X11_EXTENSION_NAME};

                        if connection.extension_information(X11_EXTENSION_NAME)?.is_some() {
                            let version = connection.$extension_fn($req_major, $req_minor)?.reply()?;
                            let major_version = u32::from(version.major_version);
                            let minor_version = u32::from(version.minor_version);

                            #[allow(unused_comparisons)]
                            if major_version >= $req_major
                                || (major_version == $req_major && minor_version >= $req_minor)
                            {
                                info!(
                                    "Loaded extension {} version {}.{}",
                                    X11_EXTENSION_NAME,
                                    major_version,
                                    minor_version,
                                );

                                Some((major_version, minor_version))
                            } else {
                                if $required {
                                    error!(
                                        "required extension {} version is too low (have {}.{}, expected {}.{})",
                                        X11_EXTENSION_NAME,
                                        major_version,
                                        minor_version,
                                        $req_major,
                                        $req_minor,
                                    );

                                    return Err(MissingExtensionError::WrongVersion {
                                        name: X11_EXTENSION_NAME,
                                        required_major: $req_major,
                                        required_minor: $req_minor,
                                        available_major: major_version,
                                        available_minor: minor_version,
                                    }.into());
                                } else {
                                    None
                                }
                            }
                        } else {
                            if $required {
                                error!("required extension {} not found", X11_EXTENSION_NAME);

                                return Err(MissingExtensionError::NotFound {
                                    name: X11_EXTENSION_NAME,
                                    major: $min_major,
                                    minor: $min_minor,
                                }
                                .into());
                            } else {
                                None
                            }
                        }
                    };
                )*

                Ok(Extensions {
                    $(
                        $extension,
                    )*
                })
            }
        }
    };
}

extensions! {
    present {
        present_query_version,
        required: true,
        minimum: (1, 0),
        request: (1, 0),
    },

    xfixes {
        xfixes_query_version,
        required: true,
        minimum: (4, 0),
        request: (4, 0),
    },

    dri3 {
        dri3_query_version,
        required: true,
        minimum: (1, 0),
        request: (1, 2),
    },

    xinput {
        xinput_xi_query_version,
        required: true,
        minimum: (2, 0),
        request: (2, 4),
    },
}
