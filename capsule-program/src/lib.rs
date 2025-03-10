use sp1_sdk::SP1Stdin;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct CapsuleData {
    unlock_date: u64,
    message: String,
    photo_data: Vec<u8>,
}

#[no_mangle]
pub extern "C" fn main() {
    let mut stdin = SP1Stdin::new();
    let data: CapsuleData = stdin.read();
    // Логика обработки данных (просто читаем для примера)
    let _ = data.unlock_date;
    let _ = data.message;
    let _ = data.photo_data;
}