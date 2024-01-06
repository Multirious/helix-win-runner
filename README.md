![GitHub License](https://img.shields.io/github/license/multirious/helix-win-runner?style=for-the-badge)
![GitHub Release](https://img.shields.io/github/v/release/multirious/helix-win-runner?sort=semver&style=for-the-badge)

# helix-win-runner

CLI tool to enhance Helix in your prefered terminal as external editor in Windows for mostly Godot but might work with others.

> [!NOTE]
> This works through keyboard macros, so try to avoid interacting with your input devices while the script is processing.

# Why macros?

Currently Helix do not support external connection which give many quality IDE features.
This is a workaround for that to be as close as possible for what it could be through automation.

# Features

- File switching
- Focuses Helix window that already exists or create one if not
- Supports using Helix in WSL

![preview](https://github.com/Multirious/helix-win-runner/assets/77918086/819eaaf3-a384-4a66-a10f-50ce431f9a91)
 
 
# How to use
Run `-h`, `--help` to see availiable flags to use.
Basic usages are explained in the help command.

See [Examples](#examples) section.

> [!NOTE]
> This tool do not launch Helix on its own

This tool actually don't contains any script for initializing Helix on a terminal so you can use any terminal and configuration you want but you have to create your own launch script and provide them to the `-e`, `--execute-path` flag. In the [Examples](#examples) I've used a batch script for this purpose. If you don't want to create your own script then it's fine too as well! The tool works without launch script but then you need to launch the Helix your self (only need to be done one time per session).

Use `--list` flag to display windows currently opened. Use the information in here for `-n`, `--window-process-name` or `-t`, `--window-title`.

Recommended to use `-r`, `--relative` and/or `--clipboard` to speed up the process!

# Examples

## Godot
Config for quickly try out the tool.

Batch script for launching Helix in Windows Terminal.
```bat
wt nt -p "Windows PowerShell" --title "Helix" hx
```
| Property   | Value            |
|----------  |------------------|
|`exec_path` |`<helix-win-runner file path>`|
|`exec_flags`|`-e <launch script file path> -w 2 -t "Helix" -n "WindowsTerminal.exe" -r --clipboard -p {project} -f {file}  -l {line} -c {col}`|


This is the config for the above gif which is Helix in Windows Terminal with WSL Debian.
| Property   | Value            |
|----------  |------------------|
|`exec_path` |`helix-win-runner.exe`|
|`exec_flags`|`-e "C:\Users\Windows10\Desktop\debian_helix.bat" -w 2 -t "Debian Helix" -n "WindowsTerminal.exe" -r --clipboard --wsl -p {project} -f {file}  -l {line} -c {col}`|
