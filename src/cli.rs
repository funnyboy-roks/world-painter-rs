use std::{env, path::PathBuf, str::FromStr};

use anyhow::bail;
use color_art::Color;

pub(crate) struct Cli {
    pub files: Vec<FileArg>,
}

#[derive(Debug, Clone)]
pub(crate) struct FileArg {
    pub path: PathBuf,
    pub worldborder: u32,
    pub color: Color,
}

impl Cli {
    pub fn parse() -> anyhow::Result<Self> {
        let mut out = Vec::new();

        let args: Vec<_> = env::args().skip(1).collect();

        for a in args.chunks(3) {
            let path = PathBuf::from_str(&a[0])?;
            if !path.exists() {
                bail!("Unknown path")
            }
            if !path.is_dir() {
                bail!("Path must be a directory!")
            }
            let worldborder = a[1].parse()?;
            let color = a[2].parse()?;

            out.push(FileArg {
                path,
                worldborder,
                color,
            });
        }
        Ok(Self { files: out })
    }
}
