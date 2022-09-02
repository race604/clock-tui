use clap::{IntoApp, ArgEnum};
use clap_complete::{Shell, generate_to};
use clap_mangen::Man;
use clock_tui::app::App;
use std::fs::File;
use std::io::Result;
use std::path::{PathBuf, Path};
use std::{env, fs};

const BIN_NAME: &str = "tclock";

fn build_shell_completion(outdir: &Path) -> Result<()> {
    let mut app = App::into_app();
    let shells = Shell::value_variants();

    for shell in shells {
        generate_to(*shell, &mut app, BIN_NAME, &outdir)?;
    }

    Ok(())
}

fn build_manpages(outdir: &Path) -> Result<()> {
    let app = App::into_app();

    let file = Path::new(&outdir).join(format!("{}.1", BIN_NAME));
    let mut file = File::create(&file)?;

    Man::new(app).render(&mut file)?;

    Ok(())
}

fn main() -> Result<()> {
    let out_dir = env!("CARGO_MANIFEST_DIR");

    let out_path = PathBuf::from(out_dir).join("../assets/gen");
    fs::create_dir_all(&out_path).unwrap();

    build_shell_completion(&out_path)?;
    build_manpages(&out_path)?;

    Ok(())
}
