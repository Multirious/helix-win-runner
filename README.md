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
- Supports using Helix in WSL.
 
![preview](https://github.com/Multirious/helix-win-runner/assets/77918086/3a5396ca-5a1d-40f2-900d-9557769f67b9)

You can issue a feature request!

# How to use

Run `-h`, `--help` to see availiable flags to use.
Basic usages are explained in the help command.

See [Quick Start section](#quick-start).

> [!NOTE]
> This tool do not launch Helix on its own.

This tool actually don't contains any script for initializing Helix on a terminal.
You can use any terminal and configuration you want by creating a launch script and provide them to the `-e`, `--execute-path` flag.
In the [Quick Start section](#quick-start) I've used a batch script for this purpose.
The CLI also works without a launch script but then you need to launch Helix yourself (only need to be done one time per session).

Use `--list` flag to display windows currently opened. Use the information given for `-n`, `--window-process-name` or `-t`, `--window-title`.

Recommended to use `-r`, `--relative` and/or `--clipboard` to speed up the process!

# Quick Start

This is a batch script to launch Helix in Windows Terminal:
```bat
wt nt -p "Windows PowerShell" --title "Helix" hx
```

Godot settings:
| Property   | Value            |
|----------  |------------------|
|`exec_path` |`<path to helix-win-runner>`|
|`exec_flags`|`-e <path to launch script> -w 3 -t "Helix" -n "WindowsTerminal.exe" -p {project} -f {file}  -l {line} -c {col} -r --clipboard`|

