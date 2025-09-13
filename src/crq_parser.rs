use regex::Regex;
use std::collections::HashMap;

pub struct CRQ {
    pub id: String,
    pub title: String,
    pub objective: String,
    pub description: String,
    pub expected_outcome: String,
    pub justification_benefit: String,
    pub dependencies: Option<String>,
    pub partial_progress_learnings: Option<String>,
}

impl CRQ {
    pub fn new(
        id: String,
        title: String,
        objective: String,
        description: String,
        expected_outcome: String,
        justification_benefit: String,
        dependencies: Option<String>,
        partial_progress_learnings: Option<String>,
    ) -> Self {
        CRQ {
            id,
            title,
            objective,
            description,
            expected_outcome,
            justification_benefit,
            dependencies,
            partial_progress_learnings,
        }
    }

    pub fn parse_crq_markdown(crq_content: &str, crq_id: &str) -> Result<Self, String> {
        let mut sections: HashMap<String, String> = HashMap::new();
        let mut current_heading: Option<String> = None;
        let mut current_content = String::new();

        let heading_re = Regex::new(r"^###\s*(.*)").unwrap();

        for line in crq_content.lines() {
            if let Some(caps) = heading_re.captures(line) {
                // New heading found
                if let Some(heading) = current_heading.take() {
                    sections.insert(heading, current_content.trim().to_string());
                }
                current_heading = Some(caps[1].trim().to_string());
                current_content.clear();
            } else if current_heading.is_some() {
                // Accumulate content for the current heading
                current_content.push_str(line);
                current_content.push('\n');
            }
        }

        // Insert the last section
        if let Some(heading) = current_heading.take() {
            sections.insert(heading, current_content.trim().to_string());
        }

        let title_re = Regex::new(r"## Change Request: ([^\n]+)").unwrap();
        let title = title_re.captures(crq_content)
            .map(|caps| caps[1].trim().to_string())
            .ok_or_else(|| " Title not found".to_string())?;

        let get_section = |key: &str| {
            sections.get(key).cloned()
        };

        let objective = get_section("Objective")
            .ok_or_else(|| " Objective section not found".to_string())?;
        let description = get_section("Description")
            .ok_or_else(|| " Description section not found".to_string())?;
        let expected_outcome = get_section("Expected Outcome")
            .ok_or_else(|| " Expected Outcome section not found".to_string())?;
        let justification_benefit = get_section("Justification/Benefit")
            .ok_or_else(|| " Justification/Benefit section not found".to_string())?;

        let dependencies = get_section("Dependencies");
        let partial_progress_learnings = get_section("Partial Progress/Learnings");

        Ok(CRQ::new(
            crq_id.to_string(),
            title,
            objective,
            description,
            expected_outcome,
            justification_benefit,
            dependencies,
            partial_progress_learnings,
        ))
    }
}