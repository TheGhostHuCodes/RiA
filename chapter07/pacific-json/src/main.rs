use serde_json::json;

fn main() {
    let capitals = json!({
        "Cook Islands": "Avarua",
        "Fiji": "Suva",
        "Kiribati": "South Tarawa",
        "Niue": "Alofi",
        "Tonga": "Nuku'alofa",
        "Tuvalu": "Funafuti"
    });

    println!("The capital of Tonga is: {}", capitals["Tonga"]);
}
