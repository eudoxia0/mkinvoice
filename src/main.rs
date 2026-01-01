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

mod render;
mod types;

use std::path::PathBuf;

use clap::Parser;
use types::Invoice;

/// A script to create PDF invoices from TOML files.
#[derive(Parser, Debug)]
#[command(name = "mkinvoice")]
#[command(about = "Generate PDF invoices from TOML files", long_about = None)]
struct Args {
    /// Path to the input TOML file containing invoice data.
    input: PathBuf,
    /// Path to the output PDF file.
    output: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Read the TOML file
    let toml_content = std::fs::read_to_string(&args.input)?;

    // Parse the invoice
    let invoice: Invoice = toml::from_str(&toml_content)?;

    // Generate the PDF
    render::generate_pdf(&invoice, &args.output)?;

    Ok(())
}
