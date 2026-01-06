// Copyright 2026 Fernando Borretti
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

use tempfile::tempdir;

use crate::error::Fallible;
use crate::error::ScriptError;
use crate::html::render_html;
use crate::invoice::Invoice;

/// Generate a PDF from an invoice.
pub fn generate_pdf(invoice: &Invoice, output_path: &Path) -> Fallible<()> {
    // Create temporary directory
    let dir = tempdir()?;
    let dir_path: PathBuf = dir.path().to_path_buf().canonicalize()?;

    // Write HTML to temporary file
    let html_path = dir_path.join("invoice.html");
    let html = render_html(invoice).into_string();
    std::fs::write(&html_path, html)?;

    // Run headless Chromium to generate PDF
    let output = Command::new("chromium")
        .arg("--headless")
        .arg("--run-all-compositor-stages-before-draw")
        .arg(format!("--print-to-pdf={}", output_path.display()))
        .arg("--no-pdf-header-footer")
        .arg(&html_path)
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(ScriptError::new(format!("Chromium failed: {stderr}")));
    }

    Ok(())
}
