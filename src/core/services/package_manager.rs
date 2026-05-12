use std::collections::HashSet;
use std::process::Command;

use crate::core::models::{AurSearchResponse, Package};
use crate::core::services::validator::check_binary;
use crate::core::shared::AppError;

pub async fn search_packages(query: String) -> Result<Vec<Package>, AppError> {
    if query.len() < 2 {
        return Ok(vec![]);
    }

    let encoded: String = query
        .chars()
        .map(|c| {
            if c.is_alphanumeric() || matches!(c, '-' | '_' | '.') {
                c.to_string()
            } else {
                format!("%{:02X}", c as u32)
            }
        })
        .collect();

    let url = format!("https://aur.archlinux.org/rpc/v5/search/{encoded}");

    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .header("User-Agent", "aur-helper/0.1")
        .timeout(std::time::Duration::from_secs(8))
        .send()
        .await
        .map_err(|e| AppError::NetworkError(e.to_string()))?;

    let aur_resp: AurSearchResponse = response
        .json()
        .await
        .map_err(|e| AppError::ParseError(e.to_string()))?;

    let installed = get_installed_names_sync();

    let packages = aur_resp
        .results
        .into_iter()
        .take(25)
        .map(|p| {
            let is_installed = installed.contains(&p.name);
            Package {
                name: p.name,
                version: p.version,
                description: p.description.unwrap_or_default(),
                is_installed,
                repository: "aur".into(),
                votes: p.num_votes,
                popularity: p.popularity,
                out_of_date: p.out_of_date.is_some(),
            }
        })
        .collect();

    Ok(packages)
}

fn get_installed_names_sync() -> HashSet<String> {
    Command::new("paru")
        .args(["-Qm"])
        .output()
        .map(|o| {
            String::from_utf8_lossy(&o.stdout)
                .lines()
                .filter_map(|l| l.split_whitespace().next().map(String::from))
                .collect()
        })
        .unwrap_or_default()
}

pub async fn get_installed_packages() -> Result<Vec<Package>, AppError> {
    let output = tokio::process::Command::new("paru")
        .args(["-Qm"])
        .output()
        .await
        .map_err(|e| AppError::CommandFailed(e.to_string()))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let packages = stdout
        .lines()
        .filter_map(|line| {
            let mut parts = line.splitn(2, ' ');
            let name = parts.next()?.to_string();
            let version = parts.next().unwrap_or("").to_string();
            Some(Package {
                name,
                version,
                description: String::new(),
                is_installed: true,
                repository: "aur".into(),
                votes: None,
                popularity: None,
                out_of_date: false,
            })
        })
        .collect();

    Ok(packages)
}

pub async fn get_updates() -> Result<Vec<Package>, AppError> {
    let output = tokio::process::Command::new("paru")
        .args(["-Qua"])
        .output()
        .await
        .map_err(|e| AppError::CommandFailed(e.to_string()))?;

    let stdout = String::from_utf8_lossy(&output.stdout);

    let packages = stdout
        .lines()
        .filter_map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 4 {
                Some(Package {
                    name: parts[0].to_string(),
                    version: format!("{} → {}", parts[1], parts[3]),
                    description: "Atualização disponível".into(),
                    is_installed: true,
                    repository: "aur".into(),
                    votes: None,
                    popularity: None,
                    out_of_date: false,
                })
            } else {
                None
            }
        })
        .collect();

    Ok(packages)
}

pub async fn install_package(name: String) -> Result<(), AppError> {
    run_paru_in_terminal(&format!("paru -S {name}")).await
}

pub async fn remove_package(name: String) -> Result<(), AppError> {
    run_paru_in_terminal(&format!("paru -Rns {name}")).await
}

pub async fn update_all() -> Result<(), AppError> {
    run_paru_in_terminal("paru -Syu --aur").await
}

async fn run_paru_in_terminal(command: &str) -> Result<(), AppError> {
    let script = format!(
        r#"#!/bin/bash
{command}
RC=$?
echo ""
if [ $RC -eq 0 ]; then
    printf '\033[1;32m✔  Concluído com sucesso!\033[0m\n'
else
    printf '\033[1;31m✖  Falha (código: %d)\033[0m\n' "$RC"
fi
read -rp "Pressione Enter para fechar..."
"#
    );

    let script_path = "/tmp/aur-helper-operation.sh";

    tokio::fs::write(script_path, &script)
        .await
        .map_err(|e| AppError::IoError(e.to_string()))?;

    let _ = tokio::process::Command::new("chmod")
        .args(["+x", script_path])
        .output()
        .await;

    let terminals: &[(&str, &[&str])] = &[
        ("alacritty", &["--", "bash", script_path]),
        ("kitty", &["bash", script_path]),
        ("konsole", &["-e", "bash", script_path]),
        ("gnome-terminal", &["--", "bash", script_path]),
        ("xfce4-terminal", &["-x", "bash", script_path]),
        ("foot", &["bash", script_path]),
        ("xterm", &["-e", "bash", script_path]),
    ];

    for (term, args) in terminals {
        if check_binary(term) {
            tokio::process::Command::new(term)
                .args(*args)
                .status()
                .await
                .map_err(|e| AppError::CommandFailed(e.to_string()))?;
            return Ok(());
        }
    }

    Err(AppError::CommandFailed(
        "Nenhum emulador de terminal encontrado (instale alacritty, kitty, konsole, gnome-terminal ou xterm)".into(),
    ))
}
