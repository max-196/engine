use engine::run;

fn main() {
    use pollster::{FutureExt};
    match run().block_on() {
        Ok(()) => log::info!("Program exited with no fatal errors"),
        Err(e) => log::error!("Fatal error: {e}"),
    }
}
