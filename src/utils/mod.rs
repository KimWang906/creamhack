pub mod file_extractor {
    use std::{
        fs::{self, File},
        io,
        path::PathBuf,
    };

    pub fn extract_file(
        path: PathBuf,
        workdir: PathBuf,
        repository: &str,
    ) -> Result<(), anyhow::Error> {
        #[cfg(debug_assertions)]
        log::info!("Extracting file from path: {}", path.to_str().unwrap());

        match path.extension().unwrap().to_str() {
            Some("zip") => Ok(extract_zip(workdir, repository)?),
            Some(ext) => Err(anyhow::anyhow!(
                "The file extension is not supported. Supported extensions: zip\n
                    Current extension: {}",
                ext
            )),
            None => Err(anyhow::anyhow!(
                "The file extension is not supported. Supported extensions: zip"
            )),
        }
    }

    fn extract_zip(path: PathBuf, repository: &str) -> Result<(), anyhow::Error> {
        // Construct the path to the zip file
        let zip_path = path.clone().join(format!("{}.zip", repository));
        #[cfg(debug_assertions)]
        log::info!("Extracting file from path: {}", zip_path.to_str().unwrap());

        // Open the zip file
        let f = File::open(zip_path)?;
        let mut archive = zip::ZipArchive::new(f)?;

        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;

            // Get the path to extract the file to.
            let outpath = match file.enclosed_name() {
                Some(path_inside_zip) => {
                    // Construct the extraction path by placing the repository directory under the main path.
                    let mut new_outpath = path.clone();
                    new_outpath.push(repository);
                    new_outpath.push(path_inside_zip);
                    new_outpath
                }
                None => continue, // Skip to the next file if the path is None.
            };

            #[cfg(debug_assertions)]
            log::info!(
                "Extracting file {} ({} bytes) to {}",
                file.name(),
                file.size(),
                outpath.display()
            );

            // Check if the file is a directory.
            if file.name().ends_with('/') {
                #[cfg(debug_assertions)]
                log::info!("Creating directory: {}", outpath.display());
                fs::create_dir_all(&outpath)?; // Create the directory.
            } else {
                // Create parent directories if they don't exist.
                if let Some(p) = outpath.parent() {
                    if !p.exists() {
                        fs::create_dir_all(p)?;
                    }
                }

                // Create and copy the file contents to the output path.
                let mut outfile = File::create(&outpath)?;
                io::copy(&mut file, &mut outfile)?;
            }

            // Set file permissions if running on a Unix-like system.
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;

                if let Some(mode) = file.unix_mode() {
                    fs::set_permissions(&outpath, fs::Permissions::from_mode(mode))?;
                }
            }
        }
        Ok(())
    }
}
