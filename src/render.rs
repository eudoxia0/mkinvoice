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

use maud::Markup;
use maud::PreEscaped;
use maud::html;
use tempfile::tempdir;

use crate::types::Expense;
use crate::types::Invoice;
use crate::types::Labour;

const STYLESHEET: &str = include_str!("style.css");

/// Render an invoice to HTML.
pub fn render_html(invoice: &Invoice) -> Markup {
    html! {
        (PreEscaped("<!doctype html>"))
        html lang="en" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                title {}
                style {
                    (PreEscaped(STYLESHEET))
                }
            }
            body {
                div class="page" {
                    // Invoice metadata section
                    div class="section" {
                        div class="big-title" { "invoice" }
                        div class="content" {
                            table class="kv-table" {
                                tr {
                                    td class="key" { "invoice #" }
                                    td class="val" { (invoice.metadata.invoice_id) }
                                }
                                tr {
                                    td class="key" { "issue date" }
                                    td class="val" { (invoice.metadata.issue_date) }
                                }
                                tr {
                                    td class="key" { "payment terms" }
                                    td class="val" { (invoice.metadata.payment_terms) }
                                }
                            }
                        }
                    }

                    // Issuer section
                    div class="section" {
                        div class="title" { "issuer" }
                        div class="content" {
                            div class="contact" {
                                div class="line" { (invoice.issuer.name) }
                                div class="line" {
                                    a href=(format!("mailto:{}", invoice.issuer.email)) {
                                        (invoice.issuer.email)
                                    }
                                }
                            }
                        }
                    }

                    // Bill to section
                    div class="section" {
                        div class="title" { "bill to" }
                        div class="content" {
                            div class="contact" {
                                div class="line" { (invoice.recipient.name) }
                                div class="line" { (invoice.recipient.company) }
                                div class="line" {
                                    a href=(format!("mailto:{}", invoice.recipient.email)) {
                                        (invoice.recipient.email)
                                    }
                                }
                            }
                        }
                    }

                    // Items section
                    div class="section" {
                        div class="title" { "items" }
                        div class="content" {
                            table class="items" {
                                thead {
                                    tr {
                                        th { "Date" }
                                        th { "Description" }
                                        th { "Quantity" }
                                        th { "Price" }
                                        th { "Total" }
                                    }
                                }
                                tbody {
                                    // Labour items
                                    @if !invoice.labour.is_empty() {
                                        tr class="table-heading" {
                                            th colspan="5" { "Labour" }
                                        }
                                        @for item in &invoice.labour {
                                            (render_labour_row(item, &invoice.metadata.currency))
                                        }
                                    }

                                    // Expense items
                                    @if !invoice.expenses.is_empty() {
                                        tr class="table-heading" {
                                            th colspan="5" { "Expenses" }
                                        }
                                        @for item in &invoice.expenses {
                                            (render_expense_row(item, &invoice.metadata.currency))
                                        }
                                    }

                                    // Totals
                                    tr class="table-heading" {
                                        th colspan="5" { "Totals" }
                                    }
                                    tr {
                                        td class="total" colspan="4" { "Subtotal" }
                                        td class="numeric-cell" {
                                            (format_currency(&invoice.metadata.currency, invoice.subtotal()))
                                        }
                                    }
                                    tr {
                                        td class="total" colspan="4" { "Tax Rate" }
                                        td class="numeric-cell" {
                                            (format!("{}%", invoice.metadata.tax_rate))
                                        }
                                    }
                                    tr {
                                        td class="total" colspan="4" { "Total Tax" }
                                        td class="numeric-cell" {
                                            (format_currency(&invoice.metadata.currency, invoice.tax_amount()))
                                        }
                                    }
                                    tr {
                                        td class="total" colspan="4" { "Balance Due" }
                                        td class="numeric-cell" {
                                            (format_currency(&invoice.metadata.currency, invoice.total()))
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // Payment section
                    div class="section" {
                        div class="title" { "payment" }
                        div class="content" {
                            table class="kv-table" {
                                tr {
                                    td class="key" { "name" }
                                    td class="val" { (invoice.payment.name) }
                                }
                                tr {
                                    td class="key" { "bsb" }
                                    td class="val" { (invoice.payment.bsb) }
                                }
                                tr {
                                    td class="key" { "acct" }
                                    td class="val" { (invoice.payment.acct) }
                                }
                                tr {
                                    td class="key" { "bank" }
                                    td class="val" { (invoice.payment.bank) }
                                }
                                tr {
                                    td class="key" { "bic/swift" }
                                    td class="val" { (invoice.payment.swift) }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn render_labour_row(item: &Labour, currency: &str) -> Markup {
    html! {
        tr {
            td class="date-cell" { (item.date) }
            td { (item.description) }
            td class="numeric-cell" { (item.quantity) }
            td class="numeric-cell" { (format_currency(currency, item.unit_price)) }
            td class="numeric-cell" { (format_currency(currency, item.total())) }
        }
    }
}

fn render_expense_row(item: &Expense, currency: &str) -> Markup {
    html! {
        tr {
            td class="date-cell" { (item.date) }
            td { (item.description) }
            td class="numeric-cell" { (item.quantity) }
            td class="numeric-cell" { (format_currency(currency, item.unit_price)) }
            td class="numeric-cell" { (format_currency(currency, item.total())) }
        }
    }
}

fn format_currency(currency: &str, amount: f64) -> String {
    format!("{:.2} {}", amount, currency)
}

/// Generate a PDF from an invoice.
pub fn generate_pdf(
    invoice: &Invoice,
    output_path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
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
        return Err(format!("Chromium failed: {}", stderr).into());
    }

    Ok(())
}
