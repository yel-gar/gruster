use crate::secret::ENTRIES;
use std::collections::HashMap;
use std::sync::LazyLock;

type WinLabel = String;
type BtnLabel = String;

#[derive(Clone, PartialEq)]
pub enum WindowType {
    Root,
    Normal,
    Message(String),
    Slowdown(WinLabel, BtnLabel),
    Accelerate(WinLabel, BtnLabel),
    Prompt,
    Flag,
}

const ENTRY_NUMBERS: &[u32] = &[
    6, 8, 14, 17, 30, 31, 44, 46, 67, 120, 162, 165, 222, 234, 270, 300, 309, 367, 381, 477, 478,
    482, 510, 520, 567, 599, 650, 652, 658, 660,
];

const ACCEL_NUMBERS: &[u32] = &[37, 665];

const SLOW_NUMBERS: &[u32] = &[480, 664];

static ENTRY_MAP: LazyLock<HashMap<u16, &str>> =
    LazyLock::new(|| HashMap::from_iter(ENTRIES.iter().cloned()));

fn get_entry(id: u16) -> String {
    ENTRY_MAP
        .get(&id)
        .unwrap_or(&"help me smth broke")
        .to_string()
}

impl WindowType {
    pub fn from_id(id: u32) -> Self {
        match id {
            0 => return WindowType::Root,
            666 => return WindowType::Prompt,
            670 => return WindowType::Flag,
            _ => {}
        }
        if ENTRY_NUMBERS.contains(&id) {
            return WindowType::Message(get_entry(id as u16));
        } else if ACCEL_NUMBERS.contains(&id) {
            let lbl = get_entry(id as u16);
            let btn = match id {
                37 => "",
                665 => "  ",
                _ => "???",
            }
            .to_string();
            return WindowType::Accelerate(lbl, btn);
        } else if SLOW_NUMBERS.contains(&id) {
            let lbl = get_entry(id as u16);
            let btn = match id {
                480 => "",
                664 => "  ",
                _ => "???",
            }
            .to_string();
            return WindowType::Slowdown(lbl, btn);
        }
        WindowType::Normal
    }
}
