// Copyright (C) 2019-2021 Aleo Systems Inc.
// This file is part of the Leo library.

// The Leo library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Leo library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Leo library. If not, see <https://www.gnu.org/licenses/>.

use leo_errors::{LeoError, PackageError};

use std::{
    borrow::Cow,
    fs,
    path::{Path, PathBuf},
};

use backtrace::Backtrace;
use eyre::eyre;

pub static SOURCE_DIRECTORY_NAME: &str = "src/";

pub static SOURCE_FILE_EXTENSION: &str = ".leo";

pub struct SourceDirectory;

impl SourceDirectory {
    /// Creates a directory at the provided path with the default directory name.
    pub fn create(path: &Path) -> Result<(), LeoError> {
        let mut path = Cow::from(path);
        if path.is_dir() && !path.ends_with(SOURCE_DIRECTORY_NAME) {
            path.to_mut().push(SOURCE_DIRECTORY_NAME);
        }

        fs::create_dir_all(&path)
            .map_err(|e| PackageError::failed_to_create_source_directory(eyre!(e), Backtrace::new()).into())
    }

    /// Returns a list of files in the source directory.
    pub fn files(path: &Path) -> Result<Vec<PathBuf>, LeoError> {
        let mut path = Cow::from(path);
        path.to_mut().push(SOURCE_DIRECTORY_NAME);

        // Have to handle error mapping this way because of rust error: https://github.com/rust-lang/rust/issues/42424.
        let directory = match fs::read_dir(&path) {
            Ok(read_dir) => read_dir,
            Err(e) => return Err(PackageError::failed_to_read_inputs_directory(eyre!(e), Backtrace::new()).into()),
        };

        let mut file_paths = Vec::new();
        for file_entry in directory.into_iter() {
            let file_entry =
                file_entry.map_err(|e| PackageError::failed_to_get_source_file_entry(eyre!(e), Backtrace::new()))?;
            let file_path = file_entry.path();

            // Verify that the entry is structured as a valid file
            let file_type = file_entry.file_type().map_err(|e| {
                PackageError::failed_to_get_source_file_type(
                    file_path.as_os_str().to_owned(),
                    eyre!(e),
                    Backtrace::new(),
                )
            })?;
            if !file_type.is_file() {
                return Err(PackageError::invalid_source_file_type(
                    file_path.as_os_str().to_owned(),
                    file_type,
                    Backtrace::new(),
                )
                .into());
            }

            // Verify that the file has the default file extension
            let file_extension = file_path.extension().ok_or_else(|| {
                PackageError::failed_to_get_source_file_extension(file_path.as_os_str().to_owned(), Backtrace::new())
            })?;
            if file_extension != SOURCE_FILE_EXTENSION.trim_start_matches('.') {
                return Err(PackageError::invalid_source_file_extension(
                    file_path.as_os_str().to_owned(),
                    file_extension.to_owned(),
                    Backtrace::new(),
                )
                .into());
            }

            file_paths.push(file_path);
        }

        Ok(file_paths)
    }
}
