use crate::pod::PodSpec;
use std::collections::HashMap;
use std::fs;

pub fn parse_paseofile(path: &str) -> anyhow::Result<PodSpec> {
    let content = fs::read_to_string(path)?;
    let mut entity = String::new();
    let mut agent_path = String::new();
    let mut memory_path = String::new();
    let mut hooks = HashMap::new();
    let mut capabilities = Vec::new();

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let tokens: Vec<&str> = line.splitn(2, ' ').collect();
        match tokens[0] {
            "ENTITY" => entity = parse_quoted(tokens[1])?,
            "AGENT" => agent_path = parse_quoted(tokens[1])?,
            "MEMORY" => memory_path = parse_quoted(tokens[1])?,
            "HOOK" => {
                let parts: Vec<&str> = tokens[1].splitn(2, ' ').collect();
                hooks.insert(parts[0].to_string(), parse_quoted(parts[1])?);
            }
            "CAPABILITY" => capabilities.push(parse_quoted(tokens[1])?),
            _ => return Err(anyhow::anyhow!("Unknown directive: {}", tokens[0])),
        }
    }

    Ok(PodSpec {
        entity,
        agent_path,
        memory_path,
        hooks,
        capabilities,
    })
}

fn parse_quoted(input: &str) -> anyhow::Result<String> {
    let trimmed = input.trim();
    if trimmed.starts_with('"') && trimmed.ends_with('"') {
        Ok(trimmed[1..trimmed.len() - 1].to_string())
    } else {
        Err(anyhow::anyhow!("Expected quoted string: {}", input))
    }
}
