/* A Marco Polo Game
   If the name Marco is given, the response should be Polo.
   If the name is not Marco, the response should be What?.
*/

pub fn marco_polo(name: &str) -> String {
    if name == "Marco" {
        "Polo".to_string()
    } else {
        "What?".to_string()
    }
}
