use crate::constants::{ASK_HOW_MANY_DAYS, DAYS, DAYS_ERROR_MESSAGE, SEARCHING_FORECAST_DAYS};
use crate::helpers::get_number_from_user;

pub fn ask_for_day() -> Option<i32> {
    println!("{}", ASK_HOW_MANY_DAYS);
    let dias = get_number_from_user(1, 3, DAYS_ERROR_MESSAGE);
    if let Some(dias_number) = dias {
        println!();
        println!("{}{}{}", SEARCHING_FORECAST_DAYS, dias_number, DAYS);
        println!();
        Some(dias_number)
    } else {
        None
    }
}
