#[derive(Debug)]
struct User {
    name: String,
    credit_line: u64,
    balance: i64,
}

impl User {
    fn update_balance(&mut self, amount: i64) {
        self.balance += amount;
    }
}

struct Bank {
    users: Vec<User>,
    name: String,
    credit_interest: u64,
    debit_interest: u64,
}

impl Bank {
    fn new(name: String, credit_interest: u64, debit_interest: u64) -> Self {
        Bank {
            users: Vec::new(),
            name,
            credit_interest,
            debit_interest,
        }
    }

    fn add_user(&mut self, name: String, credit_line: u64) {
        let user = User {
            name,
            credit_line,
            balance: 0,
        };
        self.users.push(user);
    }

    fn get_user(&self, name: &str) -> Option<&User> {
        self.users.iter().find(|user| user.name == name)
    }

    fn calc_balance(&self) -> (i64, u64) {
        let total_liabilities: i64 = self
            .users
            .iter()
            .filter(|user| user.balance < 0)
            .map(|user| user.balance)
            .sum();

        let total_assets: u64 = self
            .users
            .iter()
            .filter(|user| user.credit_line > 0)
            .map(|user| user.credit_line)
            .sum();

        (total_liabilities, total_assets)
    }

    fn transfer_funds(
        &mut self,
        from_user: &str,
        to_user: &str,
        amount: u64,
    ) -> Result<(), String> {
        let from_index = self.users.iter().position(|user| user.name == from_user);
        let to_index = self.users.iter().position(|user| user.name == to_user);

        if let (Some(from_index), Some(to_index)) = (from_index, to_index) {
            let (from_user, to_user) = if from_index < to_index {
                let (left, right) = self.users.split_at_mut(to_index);
                (&mut left[from_index], &mut right[0])
            } else {
                let (left, right) = self.users.split_at_mut(from_index);
                (&mut right[0], &mut left[to_index])
            };
            if from_user.balance + (amount as i64) < 0 {
                return Err(format!(
                    "Transfer failed: {} does not have enough balance.",
                    from_user.name
                ));
            }

            // Update balances
            from_user.balance -= amount as i64;
            to_user.balance += amount as i64;
            Ok(())
        } else {
            Err("Transfer failed: One or both users not found.".to_string())
        }
    }

    fn accrue_interest(&mut self) {
        for user in &mut self.users {
            if user.balance < 0 {
                // Debit interest
                user.balance -= (user.balance.abs() as u64 * self.debit_interest / 100) as i64;
            } else if user.credit_line > 0 {
                // Credit interest
                user.balance += (user.credit_line * self.credit_interest / 100) as i64;
            }
        }
    }

    fn merge_bank(&mut self, other: Bank) {
        for user in other.users {
            if let Some(existing_user) = self.users.iter_mut().find(|u| u.name == user.name) {
                // Update existing user's balance
                existing_user.update_balance(user.balance);
            } else {
                // Add new user
                self.users.push(user);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bank() {
        let mut bank = Bank::new("Test Bank".to_string(), 5, 10);
        bank.add_user("Alice".to_string(), 1000);
        bank.add_user("Bob".to_string(), 500);

        assert_eq!(bank.get_user("Alice").unwrap().name, "Alice");
        assert!(bank.get_user("Charlie").is_none());
        assert_eq!(bank.calc_balance(), (0, 1500));
        assert_eq!(bank.users.len(), 2);
        assert_eq!(bank.users[0].balance, 0);
        assert_eq!(bank.users[1].balance, 0);
        assert_eq!(bank.users[0].credit_line, 1000);
        assert_eq!(bank.users[1].credit_line, 500);
    }
}
