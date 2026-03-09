#[derive(Debug)]
pub struct BankAccount {
    balance: f64,
}

impl BankAccount {
    pub fn new(initial_balance: f64) -> BankAccount {
        let balance = if initial_balance < 0.0 { 0.0 } else { initial_balance };
        BankAccount { balance }
    }

    pub fn deposit(&mut self, amount: f64) {
        if amount > 0.0 {
            self.balance += amount;
        }
    }

    pub fn withdraw(&mut self, amount: f64) {
        if amount > 0.0 && amount <= self.balance {
            self.balance -= amount;
        }
    }

    pub fn balance(&self) -> f64 {
        self.balance
    }

    pub fn apply_interest(&mut self, rate: f64) {
        if rate > 0.0 {
            self.balance += self.balance * rate;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f64 = 1e-10;

    fn approx_eq(a: f64, b: f64) -> bool {
        (a - b).abs() < EPSILON
    }

    #[test]
    fn test_new_account() {
        let account = BankAccount::new(100.0);
        assert!(approx_eq(account.balance(), 100.0));
    }

    #[test]
    fn test_new_account_zero() {
        let account = BankAccount::new(0.0);
        assert!(approx_eq(account.balance(), 0.0));
    }

    #[test]
    fn test_new_account_negative_initial_balance() {
        // Negative initial balance should default to 0.0
        let account = BankAccount::new(-50.0);
        assert!(approx_eq(account.balance(), 0.0));
    }

    #[test]
    fn test_deposit() {
        let mut account = BankAccount::new(100.0);
        account.deposit(50.0);
        assert!(approx_eq(account.balance(), 150.0));
    }

    #[test]
    fn test_deposit_negative_amount() {
        // Negative deposit should be ignored
        let mut account = BankAccount::new(100.0);
        account.deposit(-50.0);
        assert!(approx_eq(account.balance(), 100.0));
    }

    #[test]
    fn test_deposit_zero() {
        let mut account = BankAccount::new(100.0);
        account.deposit(0.0);
        assert!(approx_eq(account.balance(), 100.0));
    }

    #[test]
    fn test_multiple_deposits() {
        let mut account = BankAccount::new(0.0);
        account.deposit(25.0);
        account.deposit(75.0);
        assert!(approx_eq(account.balance(), 100.0));
    }

    #[test]
    fn test_withdraw() {
        let mut account = BankAccount::new(100.0);
        account.withdraw(40.0);
        assert!(approx_eq(account.balance(), 60.0));
    }

    #[test]
    fn test_withdraw_entire_balance() {
        let mut account = BankAccount::new(100.0);
        account.withdraw(100.0);
        assert!(approx_eq(account.balance(), 0.0));
    }

    #[test]
    fn test_withdraw_more_than_balance() {
        let mut account = BankAccount::new(100.0);
        account.withdraw(200.0);
        assert!(approx_eq(account.balance(), 100.0));
    }

    #[test]
    fn test_withdraw_negative_amount() {
        let mut account = BankAccount::new(100.0);
        account.withdraw(-50.0);
        assert!(approx_eq(account.balance(), 100.0));
    }

    #[test]
    fn test_withdraw_zero() {
        let mut account = BankAccount::new(100.0);
        account.withdraw(0.0);
        assert!(approx_eq(account.balance(), 100.0));
    }

    #[test]
    fn test_balance_after_operations() {
        let mut account = BankAccount::new(200.0);
        account.deposit(50.0);
        account.withdraw(100.0);
        assert!(approx_eq(account.balance(), 150.0));
    }

    #[test]
    fn test_apply_interest() {
        let mut account = BankAccount::new(100.0);
        account.apply_interest(0.10); // 10% interest
        assert!(approx_eq(account.balance(), 110.0));
    }

    #[test]
    fn test_apply_negative_interest() {
        let mut account = BankAccount::new(100.0);
        account.apply_interest(-0.10);
        assert!(approx_eq(account.balance(), 100.0));
    }
}