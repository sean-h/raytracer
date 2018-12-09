extern crate cmdpro;

use std::path::{Path, PathBuf};
use cmdpro::{CommandLineProcessor, ParameterValue};

pub struct Settings {
    width: u32,
    height: u32,
    samples: u32,
    export_path: PathBuf,
    scene_path: PathBuf,
}

impl Settings {
    pub fn from_commandline(commandline: &CommandLineProcessor) -> Settings {
        let width = match commandline.get_parameter_value("width") {
            ParameterValue::UInteger(width) => *width,
            _ => 200,
        };

        let height = match commandline.get_parameter_value("height") {
            ParameterValue::UInteger(height) => *height,
            _ => 100,
        };

        let samples = match commandline.get_parameter_value("samples") {
            ParameterValue::UInteger(samples) => *samples,
            _ => 100,
        };

        let output = match commandline.get_parameter_value("output") {
            ParameterValue::Path(path) => PathBuf::from(path),
            _ => PathBuf::from("output.png"),
        };

        let scene = match commandline.get_parameter_value("scene") {
            ParameterValue::Path(path) => PathBuf::from(path),
            _ => panic!("No scene specified"),
        };

        Settings {
            width,
            height,
            samples,
            export_path: output,
            scene_path: scene,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn samples(&self) -> u32 {
        self.samples
    }

    pub fn export_path(&self) -> &PathBuf {
        &self.export_path
    }

    pub fn scene_path(&self) -> &PathBuf {
        &self.scene_path
    }
}