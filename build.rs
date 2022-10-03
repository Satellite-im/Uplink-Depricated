use glob::glob;

use std::{
    error::Error,
    fs::{self, File},
    io::Write,
};

use rsass::{compile_scss, output};

#[cfg(windows)]
fn main() -> Result<(), Box<dyn Error>> {
    let mut res = winres::WindowsResource::new();
    res.set("ProductName", "warp_gui");
    res.set("FileDescription", "warp_gui");
    res.set("LegalCopyright", "OSS");
    res.set_icon("../extra/windows/warp_gui.ico");
    res.compile()
        .expect("Failed to run the Windows resource compiler (rc.exe)");

    let scss_output = "./src/.styles.css";
    let mut scss = File::create(scss_output)?;

    let mut contents =
        String::from("/* This file is automatically generated, edits will be overwritten. */\n");

    for entry in glob("src/**/*.scss").expect("Failed to read glob pattern") {
        let path = entry?;

        println!("Adding SCSS :{}", path.display());
        let data = fs::read_to_string(path)?;
        contents += data.as_ref();
    }

    let format = output::Format {
        style: output::Style::Compressed,
        ..Default::default()
    };

    let css = compile_scss(contents.as_bytes(), format)?;

    scss.write_all(&css)?;
    scss.flush()?;

    Ok(())
}

#[cfg(not(windows))]
fn main() -> Result<(), Box<dyn Error>> {
    let scss_output = "./src/.styles.css";
    let mut scss = File::create(scss_output)?;

    let mut contents =
        String::from("/* This file is automatically generated, edits will be overwritten. */\n");

    for entry in glob("src/**/*.scss").expect("Failed to read glob pattern") {
        let path = entry?;

        println!("Adding SCSS :{}", path.display());
        let data = fs::read_to_string(path)?;
        contents += data.as_ref();
    }

    let format = output::Format {
        style: output::Style::Compressed,
        ..Default::default()
    };

    let css = compile_scss(contents.as_bytes(), format)?;

    scss.write_all(&css)?;
    scss.flush()?;

    Ok(())
}
