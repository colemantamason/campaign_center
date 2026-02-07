#[cfg(feature = "cms")]
pub mod cms;
#[cfg(feature = "events")]
pub mod events;
#[cfg(feature = "mobile_app")]
pub mod mobile_app;
pub mod shared;
#[cfg(feature = "support")]
pub mod support;
#[cfg(feature = "surveys")]
pub mod surveys;
#[cfg(feature = "web_app")]
pub mod web_app;

// #[cfg(feature = "cms")]
// pub use cms::*;
// #[cfg(feature = "events")]
// pub use events::*;
// #[cfg(feature = "mobile_app")]
// pub use mobile_app::*;
pub use shared::*;
// #[cfg(feature = "support")]
// pub use support::*;
// #[cfg(feature = "surveys")]
// pub use surveys::*;
#[cfg(feature = "web_app")]
pub use web_app::*;

#[macro_export]
macro_rules! define_enum {
    (
        $(#[$meta:meta])*
        $vis:vis enum $name:ident {
            $( $variant:ident => ($db_value:literal, $display:literal) ),+ $(,)?
        }
    ) => {
        #[derive(Clone, Copy, serde::Deserialize, PartialEq, serde::Serialize)]
        $(#[$meta])*
        $vis enum $name {
            $( $variant, )+
        }

        impl $name {
            pub fn as_str(&self) -> &'static str {
                match self {
                    $( $name::$variant => $db_value, )+
                }
            }

            pub fn from_str(string: &str) -> Option<Self> {
                match string {
                    $( $db_value => Some($name::$variant), )+
                    _ => None,
                }
            }

            pub fn display_name(&self) -> &'static str {
                match self {
                    $( $name::$variant => $display, )+
                }
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "{}", self.display_name())
            }
        }
    };
}
