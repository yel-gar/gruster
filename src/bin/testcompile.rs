const DATA: [(u16, &str); 3] = [(120, "bobux"), (23, "gaming"), (800, "ilikefeet")];

fn main() {
    for (num, s) in DATA {
        println!("{num}: {s}");
    }
}
