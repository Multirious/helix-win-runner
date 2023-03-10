# helix-win-runner

CLI too to run Helix in Window as an editor for mostly Godot but might work with others.

Note that this works through keyboard macros, so try not to move around while the script is processing.


# Why not shell scripting?

I've tried, and it sucks.
The longer I use Window the more reason I got to move to Linux.


# Features

 - It can switch tab.
 - Something about making your (or just my) life easier.
 - Just see the gif. I'm too lazy to explain these stuffs
 
![preview](https://user-images.githubusercontent.com/77918086/224227674-7baa9fc7-a26d-422e-8cd7-8919bd6c8e5c.gif)
 
 
# How to use

I'm lazy. I will write this section if this thing actually gets traction.


# Examples

Godot config:
| Property   | Value            |
|----------  |------------------|
|`exec_path` |`helix-win-runner`|
|`exec_flags`|`-e "C:\Users\Windows10\Desktop\helix_cmder.bat" -w 3 -t "cmd - hx" -n "ConEmu.exe" -p {project} -f {file}  -l {line} -c {col}`|
