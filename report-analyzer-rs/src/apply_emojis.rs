use crate::types::Ontology; // Added import

pub fn apply_emoji_ontology(text: &str, ontology: &Option<Ontology>) -> String {
    if let Some(ont) = ontology {
        let mut result = text.to_string();
        let mut sorted_keys: Vec<&String> = ont.0.keys().collect();
        sorted_keys.sort_by(|a, b| b.len().cmp(&a.len()));

        for key in sorted_keys {
            if let Some(emoji) = ont.0.get(key) {
                result = result.replace(key, emoji);
            }
        }
        result
    } else {
        text.to_string()
    }
}