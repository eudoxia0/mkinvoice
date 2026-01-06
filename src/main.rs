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

mod error;
mod invoice;
mod render;

use std::path::PathBuf;
use std::process::ExitCode;

use clap::Parser;
use invoice::Invoice;

use crate::error::Fallible;
use crate::render::generate_pdf;

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

fn entrypoint() -> Fallible<()> {
    let args = Args::parse();
    let invoice: Invoice = Invoice::parse(&args.input)?;
    generate_pdf(&invoice, &args.output)?;
    Ok(())
}

fn main() -> ExitCode {
    match entrypoint() {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("{e}");
            ExitCode::FAILURE
        }
    }
}
