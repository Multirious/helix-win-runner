# helix-win-runner

CLI tool to enhance Helix in your prefered terminal as external editor in Windows for mostly Godot but might work with others.

> [!NOTE]
> This works through keyboard macros, so try to avoid interacting with your input devices while the script is processing.

# Why macros?

Currently Helix do not support external connection which give many quality IDE features.
This is a workaround for that to be as close as possible for what it could be through automation.

# Features

- Switching files
- Focuses Helix window that already exists or create one if not.
- Support for using Helix in WSl and with Windows app using the `--wsl` flag.
 
![preview](https://user-images.githubusercontent.com/77918086/224227674-7baa9fc7-a26d-422e-8cd7-8919bd6c8e5c.gif)
 
 
# How to use
Run `-h`, `--help` to see availiable flags to use.
Basic usages are explained in the help command.

See [Examples](#examples) section.

> [!NOTE]
> This tool actually don't contains any script for initializing Helix on a terminal so you can use any terminal and configuration you want but you have to create your own launch script and provide them to the `-e`, `--execute-path` flag. In the [Examples](#examples) I've used a batch script for this purpose. If you don't want to create your own script then it's fine too as well! The tool works without launch script but then you need to launch the Helix your self (only need to be done one time per session).

Use `--list` flag to display windows currently opened. Use the information in here for `-n`, `--window-process-name` or `-t`, `--window-title`.

Recommended to use `-r`, `--relative` and/or `--clipboard` to speed up the process!

# Examples

Godot config:
| Property   | Value            |
|----------  |------------------|
|`exec_path` |`helix-win-runner`|
|`exec_flags`|`-r -e "C:\Users\Windows10\Desktop\helix_cmder.bat" -w 3 -t "cmd - hx" -n "ConEmu.exe" -p {project} -f {file}  -l {line} -c {col}`|
