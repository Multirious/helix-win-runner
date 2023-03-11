# helix-win-runner

CLI too to run Helix in Window as an editor for mostly Godot but might work with others.

Note that this works through keyboard macros, so try not to move around while the script is processing.

# Features
 
![preview](https://user-images.githubusercontent.com/77918086/224227674-7baa9fc7-a26d-422e-8cd7-8919bd6c8e5c.gif)
 
 
# How to use

Run `-h`, `--help` to see availiable flags to use.
Mose features are explained in the help command.

See [Examples](#examples) section.

## Addtional notes
When using `-e`, `--execute-path` flag and your editor requires additional initialization, you might need to provide a path to some kind of script to intialize the editor properly.

Use `--list` flag to display windows currently opened. Use the information in here for `-n`, `--window-process-name` or `-t`, `--window-title`.

# Examples

Godot config:
| Property   | Value            |
|----------  |------------------|
|`exec_path` |`helix-win-runner`|
|`exec_flags`|`-r -e "C:\Users\Windows10\Desktop\helix_cmder.bat" -w 3 -t "cmd - hx" -n "ConEmu.exe" -p {project} -f {file}  -l {line} -c {col}`|
