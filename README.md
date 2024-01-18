![GitHub License](https://img.shields.io/github/license/multirious/helix-win-runner?style=for-the-badge)
![GitHub Release](https://img.shields.io/github/v/release/multirious/helix-win-runner?sort=semver&style=for-the-badge)

# helix-win-runner

Windows CLI tool to enhance Helix as external editor.
This is made mostly for Godot but could work with others.

# This uses macros

Currently Helix do not support external connection which gives quality IDE [features](#features).
This tool is currently made as a workaround for it.

> [!NOTE]
> Try to avoid interacting with your input devices while the script is processing.

# Features

- File switching
- Focuses Helix window that already exists or create one if not.
- Jump to errors and warnings
- Supports using Helix in WSL.
 
![demo](https://github-production-user-asset-6210df.s3.amazonaws.com/77918086/295451165-24059abe-350c-41a1-a617-7a1e1391e806.gif?X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Credential=AKIAVCODYLSA53PQK4ZA%2F20240110%2Fus-east-1%2Fs3%2Faws4_request&X-Amz-Date=20240110T044835Z&X-Amz-Expires=300&X-Amz-Signature=24455f8f2ac93a467edd19051b3acfb74fc59bb490ede68028343062e2fa0f52&X-Amz-SignedHeaders=host&actor_id=77918086&key_id=0&repo_id=612031113)

You can issue a feature request!

# Usages

Run `-h`, `--help` to see availiable flags to use.
Basic usages are explained in the help command.

See [Quick Start section](#quick-start).

> [!NOTE]
> This tool do not launch Helix on its own.

Use `--run` flag to let the CLI run a command when window is not found.
The CLI will run a macro after running a command by default, to prevent this use `--no-init-macro`.
The CLI also works without a launch script but then you need to launch Helix yourself (only need to be done one time per session).

Use `--list` flag to display windows currently opened. Use the information given for `-n`, `--window-process-name` or `-t`, `--window-title`.

Recommended to use `-r`, `--relative` and/or `--clipboard` to speed up the process!

# Quick Start

Launch Helix in Windows Terminal

Godot settings:
| Property   | Value            |
|----------  |------------------|
|`exec_path` |`<path to helix-win-runner>`|
|`exec_flags`|`--clipboard -r --run "wt nt --title 'Helix' hx -w {project} {file}:{line}:{col}" --no-init-macro -t "Helix" -n "WindowsTerminal.exe" -p {project} -f {file} -l {line} -c {col}`|



