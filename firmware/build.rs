use std::{
    env,
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

use anyhow::{anyhow, Context};
use ghakuf::{
    messages::{MetaEvent, MidiEvent, SysExEvent},
    reader::{Handler, HandlerStatus, Reader},
};
use indoc::{indoc, writedoc};

fn main() -> anyhow::Result<()> {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR")?;

    println!("cargo:rerun-if-changed=music");
    println!("cargo:rerun-if-changed=build.rs");

    // Read in all of the music from the music directory
    let music_in_dir = [&manifest_dir, "music"].iter().collect::<PathBuf>();
    let music = fs::read_dir(&music_in_dir)
        .with_context(|| format!("Failed to open folder {:?}", music_in_dir))?;

    // Prepare the rust file output directory
    let music_out_dir = [&manifest_dir, "src", "music"].iter().collect::<PathBuf>();

    if music_out_dir.exists() {
        fs::remove_dir_all(&music_out_dir)?;
    }

    fs::create_dir_all(&music_out_dir)?;

    // Prepare the module file
    let mut mod_file =
        File::create(music_out_dir.join("mod.rs")).context("failed to create module file")?;

    writedoc!(
        mod_file,
        "
            //! Songs and music ready to be played on an embedded system
            //! 
            //! Auto generated by build.rs script... DO NOT MODIFY
            
        "
    )?;

    // Loop through each song in the input
    for midi_file in music {
        // Catch errors
        let midi_file = midi_file?;

        // Get the paths for the input and output files respectively
        let midi_file_path = midi_file.path();
        let midi_file_name = midi_file.file_name();

        if midi_file_path.is_dir() {
            println!(
                "cargo:warning=encountered directory in music directory: {:?}",
                midi_file_name
            );
            continue;
        }

        // Do some file things to get the right file name for the rust module
        let (rust_file_path, module_name) = {
            let mut rust_file_path = music_out_dir.join(&midi_file_name);

            let module_name = rust_file_path
                .file_stem()
                .context("midi file has no filename")?
                .to_string_lossy()
                .to_lowercase()
                .replace(" ", "_");

            rust_file_path.set_file_name(&module_name);
            rust_file_path.set_extension("rs");

            (rust_file_path, module_name)
        };

        let static_name = module_name.to_uppercase();

        writedoc!(
            mod_file,
            "
                mod {mod};
                pub use {mod}::{static};
            ",
            mod = module_name,
            static = static_name
        )?;

        let mut output_file = File::create(&rust_file_path)
            .with_context(|| format!("failed to create {:?}", rust_file_path))?;

        writedoc!(
            output_file,
            "
                use crate::note::Music;

                /// Song generated from {midi:?}... DO NOT MODIFY
                #[allow(dead_code)]
                pub static {static}: Music = Music {{
                    title: {midi:?},
            ",
            midi = midi_file_name,
            static = static_name
        )?;

        // Setup midi parser
        let mut handler = MidiHandler { output_file };

        let mut reader = Reader::new(&mut handler, &midi_file_path)
            .map_err(|e| anyhow!("{}", e))
            .with_context(|| format!("Failed to start reading {:?}", midi_file_path))?;

        reader
            .read()
            .map_err(|e| anyhow!("{}", e))
            .with_context(|| format!("Failed to read {:?}", midi_file_path))?;

        writeln!(handler.output_file, "}};")?;
    }

    Ok(())
}

struct MidiHandler {
    pub output_file: File,
}

impl Handler for MidiHandler {}