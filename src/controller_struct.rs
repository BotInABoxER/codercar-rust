use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
struct Controller {
  id: String,
  axes: Vec<String>,
  buttons: Vec<bool>
}

fn main() -> Result<()> {
  // Some JSON input data as a &str. Maybe this comes from the user.
  let data = r#"
    {
      "id": "usb game controller",
      "axes": [
        "0.05",
        "-0.94",
        "1.00"
      ],
      "buttons": [
        false,
        true,
        true,
        false,
        false,
        true
      ]
    }"#;

  let p: Controller = serde_json::from_str(data)?;
  
  println!("Button 0 on {} is currently {}", p.id, p.buttons[0]);
  Ok(())
}
