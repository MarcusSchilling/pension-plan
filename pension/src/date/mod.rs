use std::cmp::Ordering;

// create a date struct with year, month and day fields
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct Date {
    year: u32,
    month: u8,
    day: u8,
}

impl Date {
    // create a new date instance
    pub fn new(year: u32, month: u8, day: u8) -> Self {
        Date { year, month, day }
    }

    pub fn total_months(&self, current_date: Date) -> u16 {
        if self.month > current_date.month {
            return ((12 - self.month as u16) + current_date.month as u16) + ((current_date.year - self.year - 1) as u16* 12u16);  // if the month of the date is greater than the current month, then we need to add the months from the end of the year and the beginning of the next year. 
        }
        (current_date.month as u16 - self.month as u16) + ((current_date.year - self.year) as u16* 12u16)
    }
}

impl PartialOrd for Date {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering>{
        match self.year.cmp(&other.year) {
            Ordering::Equal => match self.month.cmp(&other.month){
                Ordering::Equal => Some(self.day.cmp(&other.day)),
                other => Some(other),
            },
            other => Some(other)
        } 
    }
}

impl Ord for Date {
    fn cmp(&self, other: &Self) -> Ordering{
        match self.year.cmp(&other.year) {
            Ordering::Equal => match self.month.cmp(&other.month){
                Ordering::Equal => self.day.cmp(&other.day),
                other => other,
            },
            other => other
        } 
    }
}

mod test{
    use super::*;
    #[test]
    fn test_date_cmp() {
        let date1 = Date::new(2023, 10, 5);
        let date2 = Date::new(2023, 10, 6);
        assert!(date1 < date2);
        assert!(date2 > date1);
        assert!(date2 >= date1);
        assert_eq!(date1.cmp(&date2), Ordering::Less);
    }
    #[test]
    fn test_date_cmp_2() {
        let date1 = Date::new(2023, 10, 5);
        let date2 = Date::new(2024, 10, 6);
        assert!(date1 < date2);
        assert!(date2 > date1);
        assert!(date2 >= date1);
        assert_eq!(date1.cmp(&date2), Ordering::Less);
    }

    #[test]
    fn test_date_cmp_3() {
        let date1 = Date::new(2024, 10, 5);
        let date2 = Date::new(2024, 9, 6);
        assert!(date1 > date2);
        assert!(date2 < date1);
        assert!(date1 >= date2);
        assert_eq!(date1.cmp(&date2), Ordering::Greater);
    }

    #[test]
    fn test_total_months() {
        let date1 = Date::new(2023, 10, 5);
        let date2 = Date::new(2024, 9, 6);
        assert_eq!(date1.total_months(date2), 11);
    }

    #[test]
    fn total_month() {
        let date1 = Date::new(2023, 10, 5);
        let date2 = Date::new(2024, 12, 6);
        assert_eq!(date1.total_months(date2), 14);
    }

    #[test]
    fn total_month_multiple_years() {
        let date1 = Date::new(2023, 10, 5);
        let date2 = Date::new(2026, 12, 6);
        assert_eq!(date1.total_months(date2), 38);
    }
}
