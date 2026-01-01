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

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_labour(unit_price: f64, quantity: u32) -> Labour {
        Labour {
            date: NaiveDate::from_ymd_opt(2026, 1, 1).unwrap(),
            description: "Test labour".to_string(),
            unit_price,
            quantity,
        }
    }

    fn create_test_expense(unit_price: f64, quantity: u32) -> Expense {
        Expense {
            date: NaiveDate::from_ymd_opt(2026, 1, 1).unwrap(),
            description: "Test expense".to_string(),
            unit_price,
            quantity,
        }
    }

    fn create_test_invoice(labour: Vec<Labour>, expenses: Vec<Expense>, tax_rate: f64) -> Invoice {
        Invoice {
            metadata: Metadata {
                invoice_id: "TEST-001".to_string(),
                issue_date: NaiveDate::from_ymd_opt(2026, 1, 1).unwrap(),
                payment_terms: "Net 30".to_string(),
                tax_rate,
                currency: "USD".to_string(),
            },
            issuer: Issuer {
                name: "Test Issuer".to_string(),
                email: "issuer@test.com".to_string(),
            },
            recipient: Recipient {
                name: "Test Recipient".to_string(),
                company: "Test Company".to_string(),
                email: "recipient@test.com".to_string(),
            },
            labour,
            expenses,
            payment: Payment {
                name: "Test Account".to_string(),
                bsb: "123-456".to_string(),
                acct: "12345678".to_string(),
                bank: "Test Bank".to_string(),
                swift: "TESTSWIFT".to_string(),
            },
        }
    }

    /// Test Labour::total() with simple integer values (100 * 5 = 500)
    #[test]
    fn test_labour_total_simple() {
        let labour = create_test_labour(100.0, 5);
        assert_eq!(labour.total(), 500.0);
    }

    /// Test Labour::total() with decimal prices (75.50 * 3 = 226.5)
    #[test]
    fn test_labour_total_with_decimals() {
        let labour = create_test_labour(75.50, 3);
        assert_eq!(labour.total(), 226.5);
    }

    /// Test Labour::total() with zero quantity edge case.
    #[test]
    fn test_labour_total_zero_quantity() {
        let labour = create_test_labour(100.0, 0);
        assert_eq!(labour.total(), 0.0);
    }

    /// Test Labour::total() with single quantity.
    #[test]
    fn test_labour_total_single_quantity() {
        let labour = create_test_labour(123.45, 1);
        assert_eq!(labour.total(), 123.45);
    }

    /// Test Labour::total() with large quantity (50 * 10 = 500)
    #[test]
    fn test_labour_total_large_quantity() {
        let labour = create_test_labour(50.0, 10);
        assert_eq!(labour.total(), 500.0);
    }

    /// Test Expense::total() with simple calculation (25 * 4 = 100)
    #[test]
    fn test_expense_total_simple() {
        let expense = create_test_expense(25.0, 4);
        assert_eq!(expense.total(), 100.0);
    }

    /// Test Expense::total() with decimal prices (12.99 * 7 = 90.93)
    #[test]
    fn test_expense_total_with_decimals() {
        let expense = create_test_expense(12.99, 7);
        assert_eq!(expense.total(), 90.93);
    }

    /// Test Expense::total() with zero quantity edge case
    #[test]
    fn test_expense_total_zero_quantity() {
        let expense = create_test_expense(50.0, 0);
        assert_eq!(expense.total(), 0.0);
    }

    /// Test Invoice::subtotal() with labour items only (500 + 150 = 650)
    #[test]
    fn test_invoice_subtotal_labour_only() {
        let labour = vec![create_test_labour(100.0, 5), create_test_labour(75.0, 2)];
        let invoice = create_test_invoice(labour, vec![], 10.0);
        assert_eq!(invoice.subtotal(), 650.0); // 500 + 150
    }

    /// Test Invoice::subtotal() with expense items only (100 + 100 = 200)
    #[test]
    fn test_invoice_subtotal_expenses_only() {
        let expenses = vec![create_test_expense(25.0, 4), create_test_expense(50.0, 2)];
        let invoice = create_test_invoice(vec![], expenses, 10.0);
        assert_eq!(invoice.subtotal(), 200.0); // 100 + 100
    }

    /// Test Invoice::subtotal() with combined labour and expenses (500 + 100 = 600)
    #[test]
    fn test_invoice_subtotal_labour_and_expenses() {
        let labour = vec![create_test_labour(100.0, 5)];
        let expenses = vec![create_test_expense(25.0, 4)];
        let invoice = create_test_invoice(labour, expenses, 10.0);
        assert_eq!(invoice.subtotal(), 600.0); // 500 + 100
    }

    /// Test Invoice::subtotal() with empty invoice
    #[test]
    fn test_invoice_subtotal_empty() {
        let invoice = create_test_invoice(vec![], vec![], 10.0);
        assert_eq!(invoice.subtotal(), 0.0);
    }

    /// Test Invoice::subtotal() with multiple items of both types
    #[test]
    fn test_invoice_subtotal_multiple_items() {
        let labour = vec![
            create_test_labour(100.0, 5),
            create_test_labour(75.0, 2),
            create_test_labour(50.0, 10),
        ];
        let expenses = vec![
            create_test_expense(25.0, 4),
            create_test_expense(30.0, 3),
            create_test_expense(15.0, 2),
        ];
        let invoice = create_test_invoice(labour, expenses, 10.0);
        assert_eq!(invoice.subtotal(), 1370.0); // (500 + 150 + 500) + (100 + 90 + 30)
    }

    /// Test Invoice::tax_amount() with standard 10% tax rate (10% of 1000 = 100)
    #[test]
    fn test_invoice_tax_amount_ten_percent() {
        let labour = vec![create_test_labour(100.0, 10)];
        let invoice = create_test_invoice(labour, vec![], 10.0);
        assert_eq!(invoice.tax_amount(), 100.0); // 10% of 1000
    }

    /// Test Invoice::tax_amount() with 20% tax rate (20% of 1000 = 200)
    #[test]
    fn test_invoice_tax_amount_twenty_percent() {
        let labour = vec![create_test_labour(100.0, 10)];
        let invoice = create_test_invoice(labour, vec![], 20.0);
        assert_eq!(invoice.tax_amount(), 200.0); // 20% of 1000
    }

    /// Test Invoice::tax_amount() with zero tax rate
    #[test]
    fn test_invoice_tax_amount_zero_rate() {
        let labour = vec![create_test_labour(100.0, 10)];
        let invoice = create_test_invoice(labour, vec![], 0.0);
        assert_eq!(invoice.tax_amount(), 0.0);
    }

    /// Test Invoice::tax_amount() with fractional tax rate (7.5% of 1000 = 75)
    #[test]
    fn test_invoice_tax_amount_fractional_rate() {
        let labour = vec![create_test_labour(100.0, 10)];
        let invoice = create_test_invoice(labour, vec![], 7.5);
        assert_eq!(invoice.tax_amount(), 75.0); // 7.5% of 1000
    }

    /// Test Invoice::tax_amount() with decimal subtotal using floating-point tolerance
    #[test]
    fn test_invoice_tax_amount_with_decimal_subtotal() {
        let labour = vec![create_test_labour(33.33, 3)];
        let invoice = create_test_invoice(labour, vec![], 10.0);
        let expected_tax = 99.99 * 0.10;
        assert!((invoice.tax_amount() - expected_tax).abs() < 0.001);
    }

    /// Test Invoice::total() with simple total and 10% tax (1000 + 100 = 1100)
    #[test]
    fn test_invoice_total_simple() {
        let labour = vec![create_test_labour(100.0, 10)];
        let invoice = create_test_invoice(labour, vec![], 10.0);
        assert_eq!(invoice.total(), 1100.0); // 1000 subtotal + 100 tax
    }

    /// Test Invoice::total() with zero tax rate
    #[test]
    fn test_invoice_total_zero_tax() {
        let labour = vec![create_test_labour(100.0, 10)];
        let invoice = create_test_invoice(labour, vec![], 0.0);
        assert_eq!(invoice.total(), 1000.0);
    }

    /// Test Invoice::total() with complex invoice containing multiple items and 15% tax
    #[test]
    fn test_invoice_total_complex() {
        let labour = vec![create_test_labour(100.0, 5), create_test_labour(75.50, 4)];
        let expenses = vec![create_test_expense(25.0, 6), create_test_expense(50.25, 2)];
        let invoice = create_test_invoice(labour, expenses, 15.0);
        // Subtotal: (500 + 302) + (150 + 100.5) = 1052.5
        // Tax: 1052.5 * 0.15 = 157.875
        // Total: 1052.5 + 157.875 = 1210.375
        assert_eq!(invoice.subtotal(), 1052.5);
        assert_eq!(invoice.tax_amount(), 157.875);
        assert_eq!(invoice.total(), 1210.375);
    }

    /// Test Invoice::total() with empty invoice
    #[test]
    fn test_invoice_total_empty_invoice() {
        let invoice = create_test_invoice(vec![], vec![], 10.0);
        assert_eq!(invoice.total(), 0.0);
    }

    /// Test Invoice::total() with high tax rate (25% of 1000 = 250, total = 1250)
    #[test]
    fn test_invoice_total_high_tax_rate() {
        let labour = vec![create_test_labour(100.0, 10)];
        let invoice = create_test_invoice(labour, vec![], 25.0);
        assert_eq!(invoice.total(), 1250.0); // 1000 + 250
    }
}
