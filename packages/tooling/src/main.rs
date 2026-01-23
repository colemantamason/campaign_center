use lightningcss::{
    stylesheet::{MinifyOptions, ParserOptions, PrinterOptions, StyleSheet},
    targets::{Browsers, Targets},
};
use std::{env, fs, path::Path};
use walkdir::WalkDir;

pub enum CssSource<'a> {
    Path(&'a Path),
    String(&'a str),
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!(
            "Usage: {} <input_path_or_css_string> [output_path]",
            args[0]
        );
        std::process::exit(1);
    }

    let input = &args[1];
    let output = if args.len() > 2 {
        Some(Path::new(&args[2]))
    } else {
        None
    };

    let path = Path::new(input);
    let source = if path.exists() {
        CssSource::Path(path)
    } else {
        CssSource::String(input)
    };

    process_css(source, output)?;
    println!("CSS processing complete.");

    Ok(())
}

pub fn process_css(
    source: CssSource,
    output_path: Option<&Path>,
) -> Result<(), Box<dyn std::error::Error>> {
    match source {
        CssSource::Path(path) => {
            if path.is_dir() {
                process_directory(path)
            } else {
                process_file(path)
            }
        }
        CssSource::String(content) => {
            let out = output_path.ok_or("Output path required for string source")?;
            let mut final_out = out.to_path_buf();

            if final_out.extension().map_or(true, |ext| ext != "css") {
                let mut new_name = final_out.file_name().unwrap_or_default().to_os_string();
                new_name.push(".css");
                final_out.set_file_name(new_name);
            }

            process_content(content, &final_out)
        }
    }
}

fn process_directory(dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.extension().map_or(false, |ext| ext == "css") {
            process_file(path)?;
        }
    }
    Ok(())
}

fn process_file(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    process_content(&content, path)
}

fn process_content(content: &str, output_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let mut stylesheet = StyleSheet::parse(content, ParserOptions::default())
        .map_err(|e| format!("Failed to parse CSS: {}", e))?;

    let targets = Targets {
        browsers: Browsers::from_browserslist([">= 0.01%", "not dead", "not op_mini all"])?,
        ..Targets::default()
    };

    stylesheet.minify(MinifyOptions {
        targets,
        ..MinifyOptions::default()
    })?;

    let css = stylesheet.to_css(PrinterOptions {
        minify: true,
        targets,
        ..PrinterOptions::default()
    })?;

    fs::write(output_path, css.code)?;
    Ok(())
}
