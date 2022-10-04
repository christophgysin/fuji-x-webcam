use std::{thread, time};
use std::io::{self, Write};
use log::{debug};
use gphoto2::{Context, Result};

fn main() -> Result<()> {
    env_logger::init();

    let image = include_bytes!("../technical-difficulties-1024.jpg");
    let context = Context::new()?;

    loop {
        match context.autodetect_camera() {
            Ok(camera) => {
                loop {
                    match camera.capture_preview()
                        .and_then(|preview| preview.get_data())
                        .and_then(|data| {
                            Ok(io::stdout().write_all(&(*data)))
                        }) {
                            Ok(_) => {},
                            Err(err) => {
                                debug!("Error capturing preview: {}", err);
                                io::stdout().write_all(image)?;
                                break;
                            },
                        };
                }
            },
            Err(err) => {
                debug!("Error detecting camera: {}", err);
                io::stdout().write_all(image)?;
                thread::sleep(time::Duration::from_secs(1));
            },
        };
    };
}
