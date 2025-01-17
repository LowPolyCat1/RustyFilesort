use auto_launch::{AutoLaunch, AutoLaunchBuilder};
use std::env;

pub fn initialize_auto_start() -> Option<AutoLaunch> {
    let exe_path = match env::current_exe() {
        Ok(path) => match path.to_str() {
            Some(path_str) => Ok(path_str.to_string()),
            None => {
                tracing::error!("Failed to convert executable path to string");
                Err("Failed to convert executable path to string")
            }
        },
        Err(err) => {
            tracing::error!("Failed to get current executable path: {}", err);
            Err("Failed to get current executable path")
        }
    };

    match exe_path {
        Ok(app_path) => {
            let auto_launch_agent = AutoLaunchBuilder::new()
                .set_app_name("RustyFilesort")
                // .set_args(&["--minimized"])
                .set_app_path(&app_path)
                .build();

            match auto_launch_agent {
                Ok(autolaunch) => {
                    return Some(autolaunch);
                }
                Err(error) => {
                    tracing::error!("AutoLaunch could not be instantiated: {}", error);
                    return None;
                }
            }
        }
        Err(error) => {
            tracing::error!("{}", error);
            return None;
        }
    }
}
