//! Maps raw CLI arguments into domain objects.

use crate::domain::policies::exclusion_rules::ExclusionRules;

/// Builds an `ExclusionRules` from the raw CLI argument list.
pub fn exclusion_rules_from_args(args: &[String]) -> ExclusionRules {
    let exclude_hidden = args.iter().any(|a| a == "--exclude-hidden");

    let mut dir_starts_with_patterns = parse_list(args, "--exclude-dir-pattern");
    let mut file_starts_with_patterns = parse_list(args, "--exclude-file-pattern");

    if exclude_hidden {
        if !dir_starts_with_patterns.contains(&".".to_string()) {
            dir_starts_with_patterns.push(".".to_string());
        }
        if !file_starts_with_patterns.contains(&".".to_string()) {
            file_starts_with_patterns.push(".".to_string());
        }
    }
    
    ExclusionRules {
        dirs: parse_list(args, "--exclude-dirs"),
        files: parse_list(args, "--exclude-files"),
        dir_starts_with_patterns: parse_list(args, "--exclude-dir-pattern"),
        file_starts_with_patterns: parse_list(args, "--exclude-file-pattern"),
    }
}

const KNOWN_FLAGS: &[&str] = &[
    "--input-dir",
    "--output-dir",
    "--exclude-dirs",
    "--exclude-files",
    "--exclude-dir-pattern",
    "--exclude-file-pattern",
    "--exclude-hidden",
    "--help",
    "-h",
];

/// Returns an error if any argument is an unrecognised flag.
pub fn validate_args(args: &[String]) -> Result<(), String> {
    for arg in args {
        let name = arg.splitn(2, '=').next().unwrap_or(arg);
        if name.starts_with('-') && !KNOWN_FLAGS.contains(&name) {
            return Err(format!("unknown flag: {name}"));
        }
    }
    Ok(())
}

/// Returns the value of `--flag <value>` or `--flag=<value>` from the args list.
pub fn flag_value(args: &[String], flag: &str) -> Option<String> {
    let prefix = format!("{flag}=");
    for (i, arg) in args.iter().enumerate() {
        if let Some(value) = arg.strip_prefix(&prefix) {
            return Some(value.to_string());
        }
        if arg == flag {
            return args.get(i + 1).cloned();
        }
    }
    None
}

fn parse_list(args: &[String], flag: &str) -> Vec<String> {
    flag_value(args, flag)
        .map(|v| v.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect())
        .unwrap_or_default()
}
