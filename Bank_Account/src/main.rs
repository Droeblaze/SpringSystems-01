mod bank_account;
use bank_account::BankAccount;

fn main() {
    let mut account = BankAccount::new(500.0);
    println!("Initial balance:  ${:.2}", account.balance());

    account.deposit(200.0);
    println!("After deposit:    ${:.2}", account.balance());

    account.withdraw(100.0);
    println!("After withdrawal: ${:.2}", account.balance());

    account.withdraw(1000.0); // Should be ignored
    println!("After overdraft attempt: ${:.2}", account.balance());

    account.apply_interest(0.05); // 5% interest
    println!("After 5% interest: ${:.2}", account.balance());
}