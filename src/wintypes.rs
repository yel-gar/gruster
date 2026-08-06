use crate::freakbob::ENTRIES;
use crate::util::secret_hash;

type WinLabel = String;
type BtnLabel = String;

#[derive(Clone)]
pub enum WindowType {
    Root,
    Normal,
    Message(String),
    Slowdown(WinLabel, BtnLabel),
    Accelerate(WinLabel, BtnLabel),
}

const ENTRY_NUMBERS: &[u32] = &[
    6, 8, 14, 17, 30, 31, 44, 46, 67, 120,
    162, 165, 222, 234, 270, 300, 309, 367,
    381, 477, 478, 482, 510, 520, 567, 599,
    650, 652, 658, 660
];

const ACCEL_NUMBERS: &[u32] = &[
    37, 665
];

const SLOW_NUMBERS: &[u32] = &[
    480, 664
];

fn get_entry(id: u64) -> String {
    ENTRIES.get(
        &secret_hash(id)
    ).unwrap_or(&"help me smth broke").to_string()
}

impl WindowType {
    pub fn from_id(id: u32) -> Self {
        if id == 0 {
            return WindowType::Root;  // should not happen
        }
        else if ENTRY_NUMBERS.contains(&id) {
            return WindowType::Message(get_entry(id as u64));
        }
        else if ACCEL_NUMBERS.contains(&id) {
            let lbl = get_entry(id as u64);
            let btn = match id {
                37 => "",
                665 => "  ",
                _ => "???"
            }.to_string();
            return WindowType::Accelerate(lbl, btn);
        }
        else if SLOW_NUMBERS.contains(&id) {
            let lbl = get_entry(id as u64);
            let btn = match id {
                480 => "",
                664 => "  ",
                _ => "???"
            }.to_string();
            return WindowType::Slowdown(lbl, btn);
        }
        WindowType::Normal
    }
}
