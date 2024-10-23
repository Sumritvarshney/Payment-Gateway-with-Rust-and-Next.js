use crate::payment::Payment;
use dotenv::dotenv;

pub struct AppState{
    pub payment: Payment
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            payment: Payment::new()
        }
    }
}