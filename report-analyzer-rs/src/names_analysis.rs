pub fn print_names_analysis(name_counts: &HashMap<String, usize>, ontology: &Option<Ontology>) {
    if !name_counts.is_empty() {
        let mut sorted_name_counts: Vec<(&String, &usize)> = name_counts.iter().collect();
        sorted_name_counts.sort_by(|a, b| b.1.cmp(a.1));

        println!("\n--- Most Frequently Mentioned Repository/Submodule Names ---");
        for (name, count) in sorted_name_counts.iter().take(10) {
            println!("{}: {}", apply_emoji_ontology(name, ontology), count);
        }
    } else {
        println!("\n--- No Repository/Submodule Names Found ---");
    }
}

