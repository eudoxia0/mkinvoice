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

use chrono::NaiveDate;
use serde::Deserialize;

/// An invoice.
#[derive(Debug, Deserialize)]
pub struct Invoice {
    pub metadata: Metadata,
    pub issuer: Issuer,
    pub recipient: Recipient,
    pub labour: Vec<Labour>,
    pub expenses: Vec<Expense>,
    pub payment: Payment,
}

#[derive(Debug, Deserialize)]
pub struct Metadata {
    pub invoice_id: String,
    pub issue_date: NaiveDate,
    pub payment_terms: String,
    pub tax_rate: f64,
    pub currency: String,
}

#[derive(Debug, Deserialize)]
pub struct Issuer {
    pub name: String,
    pub email: String,
}

#[derive(Debug, Deserialize)]
pub struct Recipient {
    pub name: String,
    pub company: String,
    pub email: String,
}

#[derive(Debug, Deserialize)]
pub struct Labour {
    pub date: NaiveDate,
    pub description: String,
    pub unit_price: f64,
    pub quantity: u32,
}

#[derive(Debug, Deserialize)]
pub struct Expense {
    pub date: NaiveDate,
    pub description: String,
    pub unit_price: f64,
    pub quantity: u32,
}

#[derive(Debug, Deserialize)]
pub struct Payment {
    pub name: String,
    pub bsb: String,
    pub acct: String,
    pub bank: String,
    pub swift: String,
}

impl Labour {
    /// Calculate the total for this item.
    pub fn total(&self) -> f64 {
        let quantity: f64 = self.quantity as f64;
        self.unit_price * quantity
    }
}

impl Expense {
    /// Calculate the total for this item.
    pub fn total(&self) -> f64 {
        self.unit_price * self.quantity as f64
    }
}

impl Invoice {
    /// Calculate the subtotal: the total cost of all invoice items.
    pub fn subtotal(&self) -> f64 {
        let labour_total: f64 = self.labour.iter().map(|l| l.total()).sum();
        let expenses_total: f64 = self.expenses.iter().map(|e| e.total()).sum();
        labour_total + expenses_total
    }

    /// Calculate the amount owed in tax.
    pub fn tax_amount(&self) -> f64 {
        self.subtotal() * (self.metadata.tax_rate / 100.0)
    }

    /// The total amount due: the subtotal plus the tax amount.
    pub fn total(&self) -> f64 {
        self.subtotal() + self.tax_amount()
    }
}
