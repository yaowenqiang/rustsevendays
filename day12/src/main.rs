use uuid::Uuid;

fn main() -> Result<(), uuid::Error> {
    let my_uuid = Uuid::new_v4();
    println!("{}", my_uuid);

    let my_uuid2 = Uuid::parse_str("63da2896-8e16-4944-aae4-0bddf471fb4f")?;
    println!("{}", my_uuid2.to_urn());
    Ok(())
}
