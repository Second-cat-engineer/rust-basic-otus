// Создайте структуру Account с полем balance имеющим тип i64.
// - Реализуйте типаж Drop для Account в котором если баланс не равен нулю печатается предупреждение,
// если он равен нулю, то сообщается об успешном уничтожении аккаунта

// Создайте функцию для печати баланса аккаунта print_balance.
// Создайте функцию transfer_funds для перемещения заданного кол-ва средств с одного аккаунта на другой.
// Создайте функцию destroy_account, которая принимает «уничтожаемый» аккаунт и аккаунт на который будут переведены остатки дебета/кредита с «уничтоженного» аккаунта

#[derive(Debug)]
struct Account {
    balance: i64,
}
impl Drop for Account {
    fn drop(&mut self) {
        if self.balance != 0 {
            println!("Баланс аккаунта не равен 0!");
        } else {
            println!("Аккаунт удачно уничтожен!");
        }
    }
}

fn print_balance(account: Account) {
    println!("Баланс: {}", account.balance);
}

fn transfer_funds(account1: &mut Account, account2: &mut Account, amount: i64) {
    account1.balance += amount;
    account2.balance -= amount;
}

fn destroy_account(destroyed: &mut Account, lucky: &mut Account) {
    lucky.balance += destroyed.balance;
    destroyed.balance = 0;

}

struct Bank {
    accounts: Vec<Account>,
    credit_rate: u32,
    debit_rate: u32,
}

fn accrue_interest(bank: &mut Bank) {
    for account in bank.accounts.iter_mut() {
        if account.balance > 0 {
            account.balance += account.balance * bank.debit_rate as i64 / 10_000;
        } else {
            account.balance += account.balance * bank.credit_rate as i64 / 10_000;
        }
    }
}

fn bank_balance(bank: &Bank) {
    let mut debit = 0_i64;
    let mut credit = 0_i64;

    for account in bank.accounts.iter() {
        if account.balance > 0 {
            debit += account.balance;
        } else {
            credit += account.balance;
        }
    }

    println!("debit: {}", debit);
    println!("credit: {}", credit);
}

fn main() {
    let akk1 = Account { balance: 0 };
    let akk2 = Account { balance: -100 };
    let akk3 = Account { balance: 100 };
    println!("1: {:?}, 2: {:?}, 3: {:?}", akk1, akk2, akk3);
    let mut bank = Bank {
        accounts: vec![akk1, akk2, akk3],
        credit_rate: 1000,
        debit_rate: 100,
    };
    accrue_interest(&mut bank);
    println!("Accrue interest: {:?}", bank.accounts);
    bank_balance(&bank);


    let akk4 = Account { balance: 20 };
    print_balance(akk4);

    let mut akk5 = Account { balance: -10 };
    let mut akk6 = Account { balance: 20 };
    transfer_funds(&mut akk5, &mut akk6, 5);
    print_balance(akk5);

    let mut akk7 = Account { balance: -10 };
    let mut akk8 = Account { balance: 20 };
    destroy_account(&mut akk8, &mut akk7);
    print_balance(akk7);
}
