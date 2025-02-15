use std::fs;
use serde::Deserialize;
mod date;
mod interests;
use date::Date;
use interests::{MeanReversionIterator, MeanReversion};
// create a struct that is a saving item containing amount year and month
#[derive(Clone, Copy, Debug)]
struct SavingItem {
    amount: f64,
    date: Date,
}

impl SavingItem {

    
    //current interests
    fn current_interest(&self, until: &Date, interest_rate: f64) -> f64{
        self.amount * ((1.+interest_rate / 12.0f64).powi(self.date.total_months(*until) as i32) -1.)
    }

    fn current_saving(&self, until: &Date, interest_rate: f64) -> f64{
        self.amount * (1.0 + interest_rate / 12.0f64).powi(self.date.total_months(*until) as i32)
    }
    
    fn current_savings_after_tax(&self, until: &Date, interest_rate: f64) -> f64 {
        let current_saving = self.current_saving(until, interest_rate);
        let capital_tax = self.current_interest(until, interest_rate) * 0.25;
        let church_tax = capital_tax * 0.08;
        let soli = capital_tax * 0.055;
        current_saving - capital_tax - church_tax - soli
    }


    fn sell(&mut self, netto_target: f64, interest_rate: f64, until: &Date) {
        let percentage_interest = self.current_interest(until, interest_rate) / self.current_saving(until, interest_rate); 
        let netto_to_brutto = 1. + percentage_interest * 0.28375;
        let brutto_required = netto_target * netto_to_brutto;
        assert!(brutto_required <= self.current_saving(until, interest_rate), "Cannot spend more than we have");
        self.amount *= 1. - (brutto_required / self.current_saving(until, interest_rate));
    }

}

enum WithdrawlProcedure {
    FIFO, 
    LIFO
}
struct RetirementSavings {
    monthly_saving: f64,
    interest_rate_working_phase: f64,
    cash_buffer_ratio: f64,
    working_years: u32,
    pension_years: u32,
    savings: Vec<SavingItem>, 
}

impl RetirementSavings {
    fn new(monthly_saving: f64, interest_rate_working: f64, cash_buffer_ratio: f64, working_years: u32, pension_years: u32) -> Self {
        RetirementSavings {
            monthly_saving,
            interest_rate_working_phase: interest_rate_working,
            cash_buffer_ratio: cash_buffer_ratio,
            working_years,
            pension_years,
            savings: Vec::new(),
        }
    }

    fn simulate_saving_phase(&mut self) {
        for year in 0..self.working_years {
            for month in 0..12 {
                self.savings.push(SavingItem { amount: self.monthly_saving,  date: Date::new(year, month, 0)});
            }
        }
    }

    fn total_savings(&self, until: &Date) -> f64 {
        self.savings.iter()
        .filter(|saving_item| saving_item.date <= *until)
        .map(|saving_item| saving_item.current_saving(until, self.interest_rate_working_phase))
        .sum()
    }

    fn simulate_withdrawal_phase(&mut self, monthly_withdrawal: f64, withdrawl: WithdrawlProcedure) -> (u32, f64){
        match withdrawl {
            WithdrawlProcedure::FIFO => self.fifo_withdrawl(monthly_withdrawal),
            WithdrawlProcedure::LIFO => self.lifo_withdrawl(monthly_withdrawal),
        }
    }

    fn lifo_withdrawl(&self, monthly_withdrawal: f64) -> (u32, f64)  {// returns month of withdrawl and amount
        let mut saving_copy = self.savings.clone();
        saving_copy.sort_by(|a: &SavingItem, b: &SavingItem| b.date.cmp(&a.date));//sort decreasing for years as the newest year should be first touched
        self.total_withdrawl(monthly_withdrawal, saving_copy)        
    }

    fn fifo_withdrawl(&self, monthly_withdrawal: f64) -> (u32, f64){
        let mut saving_copy = self.savings.clone();
        saving_copy.sort_by(|a, b| a.date.cmp(&b.date));
        self.total_withdrawl(monthly_withdrawal, saving_copy)        
    }

    fn total_withdrawl(&self, monthly_withdrawal: f64, mut saving_copy: Vec<SavingItem>) -> (u32, f64) {// returns month of withdrawl and amount
        let mut total_withdrawl: f64 = 0.;
        let total_cash_buffer = self.cash_buffer_ratio * self.total_savings(&Date::new(self.working_years, 0,0));
        if let Some((_, _)) = self.withdraw_amount(total_cash_buffer, &mut saving_copy, &mut total_withdrawl, self.working_years, 0u8){
            panic!("Should not happen the cash buffer ratio should be less than 100% and therefore be able to be sold in the beginning. Could also be that the ratio is so high that the taxes lead to this error.");
        } else {
            let mut pension_year_counter = 0;
            loop { 
                let current_year = pension_year_counter + self.working_years;
                for month in 0..12 {//entnahme am jahresanfang
                    if let Some((_, _)) = self.withdraw_amount(monthly_withdrawal, &mut saving_copy, &mut total_withdrawl, current_year, month) {
                        let cash_buffer_months = total_cash_buffer / monthly_withdrawal;
                        total_withdrawl += total_cash_buffer;
                        return (pension_year_counter * 12 + month as u32 + cash_buffer_months as u32, total_withdrawl);
                    }
                }
                pension_year_counter += 1;
            }
        }
    }

    fn withdraw_amount(&self, amount: f64, ordered_savings: &mut Vec<SavingItem>, total_withdrawl: &mut f64, current_year: u32, month: u8) -> Option<(u32, f64)> {
        let current_date = Date::new(current_year, month, 0);
        let mut remaining_withdrawl_this_month = amount;
        while remaining_withdrawl_this_month > 0. {
            if ordered_savings.is_empty() {
                return Some(((current_year-self.pension_years) * 12 + month as u32, *total_withdrawl));
            }
            let current_saving_after_tax: f64 = ordered_savings.first().unwrap().current_savings_after_tax(&current_date, self.interest_rate_working_phase);
            if  current_saving_after_tax > remaining_withdrawl_this_month {
                *total_withdrawl += remaining_withdrawl_this_month;
                ordered_savings.first_mut().unwrap().sell(remaining_withdrawl_this_month, self.interest_rate_working_phase, &current_date);
                remaining_withdrawl_this_month = 0.0;
            } else {
                let saving_item_after_tax = ordered_savings.first().unwrap().current_savings_after_tax(&current_date, self.interest_rate_working_phase);
                *total_withdrawl += saving_item_after_tax;
                remaining_withdrawl_this_month -= saving_item_after_tax;
                ordered_savings.remove(0);
            }
        }
        None
    }
}

#[derive(Deserialize, Debug)]
struct Config {
    cash_buffer_ratio: f64,
    netto_monthly_withdrawal: f64,
    monthly_saving: f64,
    interest_rate_working: f64,
    initial_savings: f64,
    working_years: u32,
    pension_years: u32,
}

impl Config {
    fn from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }
}

fn main() {
    let config = Config::from_file("../config.toml").expect("Failed to read config");

    let cash_buffer_ratio = config.cash_buffer_ratio; // To not be forced to sell etf shares in crisis should hold for 3-4 years
    let netto_monthly_withdrawal = config.netto_monthly_withdrawal;
    let monthly_saving = config.monthly_saving;
    let interest_rate_working = config.interest_rate_working; //interest rate is at 5% because 7% suspected in an World ETF and 2% inflation
    let initial_savings = config.initial_savings;
    let working_years = config.working_years;
    let pension_years = config.pension_years;
 
    let mut savings: RetirementSavings = RetirementSavings::new(monthly_saving, interest_rate_working, cash_buffer_ratio, working_years, pension_years);
    savings.savings.push(SavingItem{amount: initial_savings, date: Date::new(0,0,0)});
    savings.simulate_saving_phase();
    // print total_savings result
    let until = Date::new(40, 0, 0);
    println!("Total Savings at start of pension: {:?}", savings.total_savings(&until));
    
    println!("First-In-First-Out Withdrawal Procedure:");
    let (total_months,  total_withdrawl) = savings.simulate_withdrawal_phase(netto_monthly_withdrawal, WithdrawlProcedure::FIFO);
    if total_months >= pension_years * 12 {
        println!("It LASTED");
    } else {
        println!("IT DIDN'T LAST");
    }
    println!("Total months until pension is depleted: {}", total_months);
    println!("Total years until pension is depleted: {}", total_months as f64 / 12.0);
    println!("Total amount withdrawn from pension: {:}", total_withdrawl);
    
    println!("Last-In-First-Out Withdrawl Procedure: ");
    let (total_months_lifo,  total_withdrawl_lifo) = savings.simulate_withdrawal_phase(netto_monthly_withdrawal, WithdrawlProcedure::LIFO);
    if total_months_lifo >= pension_years * 12 {
        println!("It LASTED");
    } else {
        println!("IT DIDN'T LAST");
    }
    println!("Total months until pension is depleted: {}", total_months_lifo);
    println!("Total years until pension is depleted: {}", total_months_lifo as f64 / 12.0);
    println!("Total amount withdrawn from pension: {:?}", total_withdrawl_lifo);
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn test_current_interest() {
        let date = Date::new(0, 0, 0);
        let item = SavingItem { amount: 1000.0, date };
        assert_eq!(item.current_interest(&Date::new(1, 0, 0), 0.05), 51.161897881733424);
    }

    #[test]
    fn sell() {
        let date = Date::new(0, 0, 0);
        let mut item = SavingItem { amount: 1000.0, date };
        item.sell(1000.0, 0.05, &Date::new(1, 0, 0));
        assert_ne!(item.amount, 0.0);
    }
    #[test]
    fn test() {
        match Config::from_file("../config.toml") {
            Ok(config) => println!("{:?}", config),
            Err(e) => eprintln!("Failed to read config: {}", e),
        }
    }

}