/// WorldLint: Static analysis for .world files.
///
/// Checks a parsed WorldFile for common mistakes and structural issues,
/// producing lint issues with suggested fixes.

use std::collections::HashSet;
use crate::scripting::parser::{WorldFile, Value};

/// Severity of a lint issue.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LintLevel {
    Error,
    Warning,
    Hint,
}

impl LintLevel {
    /// String representation of the lint level (useful for JSON output).
    #[allow(dead_code)]
    fn as_str(&self) -> &'static str {
        match self {
            LintLevel::Error => "error",
            LintLevel::Warning => "warning",
            LintLevel::Hint => "hint",
        }
    }
}

/// A single lint issue found in a .world file.
#[derive(Clone, Debug)]
pub struct LintIssue {
    pub level: LintLevel,
    pub code: String,
    pub message: String,
    pub suggestion: Option<String>,
}

impl LintIssue {
    fn error(code: &str, message: String) -> Self {
        Self {
            level: LintLevel::Error,
            code: code.to_string(),
            message,
            suggestion: None,
        }
    }

    fn warning(code: &str, message: String) -> Self {
        Self {
            level: LintLevel::Warning,
            code: code.to_string(),
            message,
            suggestion: None,
        }
    }

    fn hint(code: &str, message: String) -> Self {
        Self {
            level: LintLevel::Hint,
            code: code.to_string(),
            message,
            suggestion: None,
        }
    }

    fn with_suggestion(mut self, suggestion: String) -> Self {
        self.suggestion = Some(suggestion);
        self
    }
}

/// Compute the Levenshtein edit distance between two strings.
fn levenshtein_distance(a: &str, b: &str) -> usize {
    let a_len = a.len();
    let b_len = b.len();

    if a_len == 0 {
        return b_len;
    }
    if b_len == 0 {
        return a_len;
    }

    let mut prev_row: Vec<usize> = (0..=b_len).collect();
    let mut curr_row: Vec<usize> = vec![0; b_len + 1];

    for (i, ca) in a.chars().enumerate() {
        curr_row[0] = i + 1;
        for (j, cb) in b.chars().enumerate() {
            let cost = if ca == cb { 0 } else { 1 };
            curr_row[j + 1] = (prev_row[j + 1] + 1)
                .min(curr_row[j] + 1)
                .min(prev_row[j] + cost);
        }
        std::mem::swap(&mut prev_row, &mut curr_row);
    }

    prev_row[b_len]
}

/// Find the closest match to `name` among `candidates`.
/// Returns None if no candidate is within a reasonable edit distance.
pub fn find_closest_match<'a>(name: &str, candidates: &[&'a str]) -> Option<&'a str> {
    if candidates.is_empty() {
        return None;
    }

    let max_distance = match name.len() {
        0..=2 => 1,
        3..=5 => 2,
        _ => 3,
    };

    let mut best: Option<(&str, usize)> = None;
    for &candidate in candidates {
        let dist = levenshtein_distance(name, candidate);
        if dist <= max_distance {
            if let Some((_, best_dist)) = best {
                if dist < best_dist {
                    best = Some((candidate, dist));
                }
            } else {
                best = Some((candidate, dist));
            }
        }
    }

    best.map(|(s, _)| s)
}

/// Lint a parsed WorldFile, returning all issues found.
pub fn lint_world(wf: &WorldFile) -> Vec<LintIssue> {
    let mut issues = Vec::new();

    // Collect all known entity tags
    let mut all_entity_tags: HashSet<String> = HashSet::new();
    for entity in &wf.entities {
        for tag in &entity.tags {
            all_entity_tags.insert(tag.clone());
        }
    }
    // Also include tags from templates
    for tmpl in &wf.templates {
        for tag in &tmpl.tags {
            all_entity_tags.insert(tag.clone());
        }
    }

    // Collect all template names
    let template_names: Vec<&str> = wf.templates.iter().map(|t| t.id.as_str()).collect();

    // Collect all timer names
    let timer_names: HashSet<&str> = wf.timers.iter().map(|t| t.name.as_str()).collect();

    // W001: Physics entity without collider
    for entity in &wf.entities {
        if entity.physics.is_some() && entity.collider.is_none() {
            issues.push(LintIssue::warning(
                "W001",
                format!("Entity '{}' has physics but no collider — it will move but never collide", entity.id),
            ));
        }
    }
    for tmpl in &wf.templates {
        if tmpl.physics.is_some() && tmpl.collider.is_none() {
            issues.push(LintIssue::warning(
                "W001",
                format!("Template '{}' has physics but no collider — spawned entities will move but never collide", tmpl.id),
            ));
        }
    }

    // Check rules for template/tag/timer references
    for rule in &wf.rules {
        // E002: Rule references unknown template (in spawn actions)
        for (action_name, action_args) in &rule.actions {
            if action_name == "spawn" || action_name == "spawn_template" {
                if let Some(arg) = action_args.first() {
                    let tmpl_name = match arg {
                        Value::Ident(s) => s.as_str(),
                        Value::Str(s) => s.as_str(),
                        _ => continue,
                    };
                    if !template_names.contains(&tmpl_name) {
                        let mut issue = LintIssue::error(
                            "E002",
                            format!(
                                "Rule '{}' references unknown template '{}'",
                                rule.name, tmpl_name
                            ),
                        );
                        if let Some(closest) = find_closest_match(tmpl_name, &template_names) {
                            issue = issue.with_suggestion(format!("Did you mean '{}'?", closest));
                        }
                        issues.push(issue);
                    }
                }
            }

            // E002: Also check start_timer/cancel_timer for timer references
            // (we use E005 for that below)
        }

        // W003: Rule references tag no entity has
        // Check condition args for tag references
        if rule.condition_name == "collision" || rule.condition_name == "trigger_enter" {
            for arg in &rule.condition_args {
                if let Value::Ident(tag_name) = arg {
                    if !all_entity_tags.contains(tag_name.as_str()) {
                        let tag_vec: Vec<&str> = all_entity_tags.iter().map(|s| s.as_str()).collect();
                        let mut issue = LintIssue::warning(
                            "W003",
                            format!(
                                "Rule '{}' references tag '{}' but no entity or template has that tag",
                                rule.name, tag_name
                            ),
                        );
                        if let Some(closest) = find_closest_match(tag_name, &tag_vec) {
                            issue = issue.with_suggestion(format!("Did you mean '{}'?", closest));
                        }
                        issues.push(issue);
                    }
                }
            }
        }

        // E005: Timer referenced in rule condition but not defined
        if rule.condition_name == "timer" || rule.condition_name == "timer_fired" {
            if let Some(arg) = rule.condition_args.first() {
                let timer_name = match arg {
                    Value::Ident(s) => s.as_str(),
                    Value::Str(s) => s.as_str(),
                    _ => "",
                };
                if !timer_name.is_empty() && !timer_names.contains(timer_name) {
                    let timer_vec: Vec<&str> = timer_names.iter().copied().collect();
                    let mut issue = LintIssue::error(
                        "E005",
                        format!(
                            "Rule '{}' references timer '{}' but no timer with that name is defined",
                            rule.name, timer_name
                        ),
                    );
                    if let Some(closest) = find_closest_match(timer_name, &timer_vec) {
                        issue = issue.with_suggestion(format!("Did you mean '{}'?", closest));
                    }
                    issues.push(issue);
                }
            }
        }
    }

    // H006: No player entity (hint)
    let has_player = wf.entities.iter().any(|e| e.tags.contains(&"player".to_string()));
    if !has_player {
        issues.push(LintIssue::hint(
            "H006",
            "No entity is tagged 'player' — most games need a player entity".to_string(),
        ));
    }

    issues
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scripting::parser::parse_world;

    fn lint(src: &str) -> Vec<LintIssue> {
        let wf = parse_world(src).expect("test world should parse");
        lint_world(&wf)
    }

    fn has_code(issues: &[LintIssue], code: &str) -> bool {
        issues.iter().any(|i| i.code == code)
    }

    #[test]
    fn clean_world_produces_no_errors() {
        let issues = lint(r#"
            world "Test" { bounds: 960 x 540 }
            entity player {
                position: (100, 200)
                physics: { mass: 1.0 }
                collider: { shape: circle, radius: 12 }
                tags: ["player"]
            }
        "#);
        // Should only produce no errors/warnings (player exists, physics has collider)
        let error_or_warn: Vec<_> = issues.iter()
            .filter(|i| i.level == LintLevel::Error || i.level == LintLevel::Warning)
            .collect();
        assert!(error_or_warn.is_empty(), "expected no errors or warnings, got: {:?}", error_or_warn);
    }

    #[test]
    fn w001_physics_without_collider() {
        let issues = lint(r#"
            world "Test" {}
            entity ghost {
                position: (100, 200)
                physics: { mass: 1.0, vx: 50 }
                tags: ["player"]
            }
        "#);
        assert!(has_code(&issues, "W001"), "expected W001 for physics without collider");
        let w001 = issues.iter().find(|i| i.code == "W001").expect("W001");
        assert_eq!(w001.level, LintLevel::Warning);
        assert!(w001.message.contains("ghost"));
    }

    #[test]
    fn w001_template_physics_without_collider() {
        let issues = lint(r#"
            world "Test" {}
            entity player { tags: ["player"] }
            template floater {
                physics: { mass: 0.5 }
                tags: ["floater"]
            }
        "#);
        assert!(has_code(&issues, "W001"), "expected W001 for template physics without collider");
    }

    #[test]
    fn e002_unknown_template_in_spawn_action() {
        let issues = lint(r#"
            world "Test" {}
            entity player { tags: ["player"] }
            template bullet {
                collider: { shape: circle, radius: 3 }
                tags: ["bullet"]
            }
            rule "Fire" {
                when: collision(player, enemy)
                then: [
                    spawn(rockets)
                ]
            }
        "#);
        assert!(has_code(&issues, "E002"), "expected E002 for unknown template 'rockets'");
        let e002 = issues.iter().find(|i| i.code == "E002").expect("E002");
        assert_eq!(e002.level, LintLevel::Error);
        assert!(e002.message.contains("rockets"));
    }

    #[test]
    fn e002_known_template_no_error() {
        let issues = lint(r#"
            world "Test" {}
            entity player { tags: ["player"] }
            template bullet {
                collider: { shape: circle, radius: 3 }
                tags: ["bullet"]
            }
            rule "Fire" {
                when: collision(player, enemy)
                then: [
                    spawn(bullet)
                ]
            }
        "#);
        assert!(!has_code(&issues, "E002"), "should not produce E002 when template exists");
    }

    #[test]
    fn w003_rule_references_unknown_tag() {
        let issues = lint(r#"
            world "Test" {}
            entity player {
                tags: ["player"]
            }
            rule "Hit" {
                when: collision(player, enemey)
                then: [
                    log("hit")
                ]
            }
        "#);
        assert!(has_code(&issues, "W003"), "expected W003 for unknown tag 'enemey'");
        let w003 = issues.iter().find(|i| i.code == "W003").expect("W003");
        assert_eq!(w003.level, LintLevel::Warning);
        assert!(w003.message.contains("enemey"));
    }

    #[test]
    fn e005_timer_not_defined() {
        let issues = lint(r#"
            world "Test" {}
            entity player { tags: ["player"] }
            timer spawn_wave {
                delay: 3.0
            }
            rule "Wave spawn" {
                when: timer(spawn_waves)
                then: [
                    log("wave!")
                ]
            }
        "#);
        assert!(has_code(&issues, "E005"), "expected E005 for undefined timer 'spawn_waves'");
        let e005 = issues.iter().find(|i| i.code == "E005").expect("E005");
        assert_eq!(e005.level, LintLevel::Error);
        assert!(e005.message.contains("spawn_waves"));
        // Should have "did you mean" suggestion since spawn_wave is close
        assert!(e005.suggestion.is_some(), "expected 'did you mean' suggestion");
        let suggestion = e005.suggestion.as_ref().expect("suggestion");
        assert!(suggestion.contains("spawn_wave"), "suggestion should mention 'spawn_wave', got: {}", suggestion);
    }

    #[test]
    fn h006_no_player_entity() {
        let issues = lint(r#"
            world "Test" {}
            entity wall {
                position: (0, 0)
                tags: ["wall"]
            }
        "#);
        assert!(has_code(&issues, "H006"), "expected H006 hint about no player entity");
        let h006 = issues.iter().find(|i| i.code == "H006").expect("H006");
        assert_eq!(h006.level, LintLevel::Hint);
    }

    #[test]
    fn h006_not_triggered_when_player_exists() {
        let issues = lint(r#"
            world "Test" {}
            entity hero {
                tags: ["player"]
            }
        "#);
        assert!(!has_code(&issues, "H006"), "H006 should not fire when player tag exists");
    }

    #[test]
    fn levenshtein_exact_match() {
        assert_eq!(levenshtein_distance("hello", "hello"), 0);
    }

    #[test]
    fn levenshtein_one_edit() {
        assert_eq!(levenshtein_distance("hello", "hallo"), 1);
        assert_eq!(levenshtein_distance("cat", "car"), 1);
    }

    #[test]
    fn levenshtein_different_lengths() {
        assert_eq!(levenshtein_distance("abc", "abcd"), 1);
        assert_eq!(levenshtein_distance("", "abc"), 3);
    }

    #[test]
    fn find_closest_match_returns_best() {
        let candidates = vec!["bullet", "asteroid", "player"];
        assert_eq!(find_closest_match("bullat", &candidates), Some("bullet"));
        assert_eq!(find_closest_match("playr", &candidates), Some("player"));
    }

    #[test]
    fn find_closest_match_returns_none_for_distant() {
        let candidates = vec!["bullet", "asteroid"];
        assert_eq!(find_closest_match("xyzzy", &candidates), None);
    }

    #[test]
    fn find_closest_match_empty_candidates() {
        assert_eq!(find_closest_match("test", &[]), None);
    }

    #[test]
    fn e002_with_did_you_mean_suggestion() {
        let issues = lint(r#"
            world "Test" {}
            entity player { tags: ["player"] }
            template bullet {
                collider: { shape: circle, radius: 3 }
                tags: ["bullet"]
            }
            rule "Fire" {
                when: collision(player, enemy)
                then: [
                    spawn(bullat)
                ]
            }
        "#);
        assert!(has_code(&issues, "E002"));
        let e002 = issues.iter().find(|i| i.code == "E002").expect("E002");
        assert!(e002.suggestion.is_some(), "expected suggestion for typo 'bullat'");
        let suggestion = e002.suggestion.as_ref().expect("suggestion");
        assert!(suggestion.contains("bullet"), "suggestion should recommend 'bullet', got: {}", suggestion);
    }
}
