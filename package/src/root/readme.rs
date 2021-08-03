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
#![allow(clippy::upper_case_acronyms)]

//! The `README.md` file.

use leo_errors::{LeoError, PackageError};

use backtrace::Backtrace;
use eyre::eyre;
use serde::Deserialize;
use std::{borrow::Cow, fs::File, io::Write, path::Path};

pub static README_FILENAME: &str = "README.md";

#[derive(Deserialize)]
pub struct README {
    pub package_name: String,
}

impl README {
    pub fn new(package_name: &str) -> Self {
        Self {
            package_name: package_name.to_string(),
        }
    }

    pub fn package_name(&self) -> String {
        self.package_name.clone()
    }

    pub fn exists_at(path: &Path) -> bool {
        let mut path = Cow::from(path);
        if path.is_dir() {
            path.to_mut().push(README_FILENAME);
        }
        path.exists()
    }

    pub fn write_to(self, path: &Path) -> Result<(), LeoError> {
        let mut path = Cow::from(path);
        if path.is_dir() {
            path.to_mut().push(README_FILENAME);
        }

        // Have to handle error mapping this way because of rust error: https://github.com/rust-lang/rust/issues/42424.
        let mut file = match File::create(&path) {
            Ok(file) => file,
            Err(e) => return Err(PackageError::io_error_readme_file(eyre!(e), Backtrace::new()).into()),
        };

        // Have to handle error mapping this way because of rust error: https://github.com/rust-lang/rust/issues/42424.
        match file.write_all(self.template().as_bytes()) {
            Ok(v) => Ok(v),
            Err(e) => Err(PackageError::io_error_readme_file(eyre!(e), Backtrace::new()).into()),
        }
    }

    fn template(&self) -> String {
        format!(
            r"# {}

## Build Guide

To compile this Leo program, run:
```bash
leo build
```

To test this Leo program, run:
```bash
leo test
```

## Development

To output the number of constraints, run:
```bash
leo build -d
```
",
            self.package_name
        )
    }
}
