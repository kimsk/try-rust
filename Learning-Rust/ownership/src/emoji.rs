pub fn emoji() {
    let sparkle_heart = vec![240, 159, 146, 150];
    let y = String::from_utf8(sparkle_heart).unwrap();
    println!("y {} length {}, capacity {}", y, y.len(), y.capacity());
    println!("🔥 length {}", "🔥".len());
}
