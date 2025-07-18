#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod payroll {
    use ink::storage::Mapping;

    #[ink(storage)]
    pub struct Payroll {
        owner: AccountId,
        salaries: Mapping<AccountId, Balance>,
        budget: Balance,
    }

    #[ink(event)]
    pub struct EmployeeAdded {
        employee: AccountId,
        salary: Balance,
    }

    #[ink(event)]
    pub struct SalaryPaid {
        employee: AccountId,
        amount: Balance,
    }

    impl Default for Payroll {
        fn default() -> Self {
            Self::new()
        }
    }

    impl Payroll {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                owner: Self::env().caller(),
                salaries: Mapping::default(),
                budget: 0,
            }
        }

        #[ink(message)]
        pub fn add_employee(&mut self, employee: AccountId, salary: Balance) {
            assert_eq!(self.owner, self.env().caller(), "Only owner");
            assert!(salary > 0, "Invalid salary");
            
            self.salaries.insert(employee, &salary);
            
            Self::env().emit_event(EmployeeAdded { employee, salary });
        }

        #[ink(message, payable)]
        pub fn deposit(&mut self) {
            let amount = self.env().transferred_value();
            self.budget = self.budget.saturating_add(amount);
        }

        #[ink(message)]
        pub fn pay_salary(&mut self, employee: AccountId) {
            assert_eq!(self.owner, self.env().caller(), "Only owner");
            
            let salary = self.salaries.get(employee).unwrap_or(0);
            assert!(salary > 0, "Employee not found");
            assert!(self.budget >= salary, "Insufficient budget");

            self.env().transfer(employee, salary).unwrap();
            self.budget = self.budget.saturating_sub(salary);
            
            Self::env().emit_event(SalaryPaid {
                employee,
                amount: salary,
            });
        }

        #[ink(message)]
        pub fn get_salary(&self, employee: AccountId) -> Balance {
            self.salaries.get(employee).unwrap_or(0)
        }

        #[ink(message)]
        pub fn get_budget(&self) -> Balance {
            self.budget
        }

        #[ink(message)]
        pub fn get_owner(&self) -> AccountId {
            self.owner
        }

        #[ink(message)]
        pub fn remove_employee(&mut self, employee: AccountId) {
            assert_eq!(self.owner, self.env().caller(), "Only owner");
            self.salaries.remove(employee);
        }

        #[ink(message)]
        pub fn withdraw(&mut self, amount: Balance) {
            assert_eq!(self.owner, self.env().caller(), "Only owner");
            assert!(amount <= self.budget, "Insufficient budget");
            
            self.env().transfer(self.owner, amount).unwrap();
            self.budget = self.budget.saturating_sub(amount);
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn test_basic() {
            let mut contract = Payroll::new();
            let employee = AccountId::from([0x1; 32]);
            
            contract.add_employee(employee, 1000);
            assert_eq!(contract.get_salary(employee), 1000);
            
            assert_eq!(contract.get_budget(), 0);
        }
    }
}