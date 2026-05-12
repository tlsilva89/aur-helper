use std::process::Command;

use crate::core::shared::AppError;

pub fn check_binary(bin: &str) -> bool {
    Command::new("which")
        .arg(bin)
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

pub fn check_base_devel() -> bool {
    Command::new("pacman")
        .args(["-Qi", "base-devel"])
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

pub async fn run_setup_in_terminal() -> Result<(), AppError> {
    let script = r#"#!/bin/bash
printf '\033[1;36m'
echo '╔══════════════════════════════════════════╗'
echo '║     AUR Helper — Configuração Inicial    ║'
echo '╚══════════════════════════════════════════╝'
printf '\033[0m'
echo ''

echo -e '\033[1;33m[1/3]\033[0m Instalando git e base-devel...'
sudo pacman -S --needed --noconfirm git base-devel
if [ $? -ne 0 ]; then
    echo -e '\033[1;31mFalha ao instalar dependências base.\033[0m'
    read -rp 'Pressione Enter para fechar...'
    exit 1
fi

echo -e '\033[1;33m[2/3]\033[0m Clonando paru do AUR...'
rm -rf /tmp/aur-helper-paru-build
git clone https://aur.archlinux.org/paru.git /tmp/aur-helper-paru-build
if [ $? -ne 0 ]; then
    echo -e '\033[1;31mFalha ao clonar o repositório do paru.\033[0m'
    read -rp 'Pressione Enter para fechar...'
    exit 1
fi

echo -e '\033[1;33m[3/3]\033[0m Compilando e instalando paru...'
cd /tmp/aur-helper-paru-build
makepkg -si --noconfirm
if [ $? -ne 0 ]; then
    echo -e '\033[1;31mFalha ao compilar o paru.\033[0m'
    read -rp 'Pressione Enter para fechar...'
    exit 1
fi

echo ''
printf '\033[1;32m'
echo '✔  paru instalado com sucesso!'
printf '\033[0m'
read -rp 'Pressione Enter para continuar...'
"#;

    let script_path = "/tmp/aur-helper-setup.sh";
    tokio::fs::write(script_path, script)
        .await
        .map_err(|e| AppError::IoError(e.to_string()))?;

    let _ = tokio::process::Command::new("chmod")
        .args(["+x", script_path])
        .output()
        .await;

    let terminals: &[(&str, &[&str])] = &[
        ("alacritty", &["--", "bash", script_path]),
        ("kitty", &["bash", script_path]),
        ("konsole", &["--", "bash", script_path]),
        ("gnome-terminal", &["--", "bash", script_path]),
        ("xfce4-terminal", &["-x", "bash", script_path]),
        ("foot", &["bash", script_path]),
        ("xterm", &["-e", "bash", script_path]),
    ];

    for (term, args) in terminals {
        if check_binary(term) {
            let status = tokio::process::Command::new(term)
                .args(*args)
                .status()
                .await
                .map_err(|e| AppError::CommandFailed(e.to_string()))?;

            return if status.success() {
                Ok(())
            } else {
                Err(AppError::CommandFailed("Setup script exited with error".into()))
            };
        }
    }

    Err(AppError::CommandFailed(
        "Nenhum emulador de terminal encontrado (alacritty, kitty, konsole, gnome-terminal, xfce4-terminal, foot, xterm)".into(),
    ))
}
