pub mod macros;

pub(crate) use macros::{
    impl_error,
    impl_error_conversion,
    impl_error_conversions,
};

use crate::client::renderer::err::RendererError;
use log::SetLoggerError;

pub enum Error {
    Renderer(RendererError),
    Logger(SetLoggerError),
}

impl_error!(Error,
    Renderer(e) => "In renderer: {}", e;
    Logger(e) => "While setting logger: {}", e
);

impl_error_conversions!(Error,
    RendererError => Renderer,
    SetLoggerError => Logger
);