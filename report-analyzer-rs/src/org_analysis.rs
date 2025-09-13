pub fn print_organizations_analysis(org_counts: &HashMap<String, usize>, ontology: &Option<Ontology>) {
    if !org_counts.is_empty() {
        let mut sorted_org_counts: Vec<(&String, &usize)> = org_counts.iter().collect();
        sorted_org_counts.sort_by(|a, b| b.1.cmp(a.1));

        println!("\n--- Most Frequently Mentioned Organizations ---");
        for (org, count) in sorted_org_counts.iter().take(10) {
            println!("{}: {}", apply_emoji_ontology(org, ontology), count);
        }
    } else {
        println!("\n--- No Organizations Found ---");
    }
}
