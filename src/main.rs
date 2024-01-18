use std::borrow::Cow;

use clap::Parser;
use helix_win_runner::{
    error::{Error, Result},
    keyboard_macro::{self, sleep},
    window::get_windows,
};

#[cfg(test)]
mod test;

/// Program to run Helix from external source.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// [DEPRECATED] Command to execute when no window found.
    /// If not provided, only search will be done.
    ///
    /// DEPRECATED: use `run` flag.
    #[arg(short = 'e', long, value_name = "PATH")]
    execute_path: Option<String>,

    /// [DEPRECATED] Time to wait after executing `execute_path`.
    /// Does nothing if not execute.
    ///
    /// DEPRECATED: use `run_wait` flag.
    #[arg(short = 'w', long, value_name = "SECS")]
    execute_wait: Option<f64>,

    /// Search for window containing this string in the title.
    /// Atleast one of `window_title` or `window_process_name` must provided.
    #[arg(short = 't', long, value_name = "STRING")]
    window_title: Option<String>,
    /// Search for window containing this string in the process name.
    /// Atleast one of `window_title` or `window_process_name` must provided.
    #[arg(short = 'n', long, value_name = "STRING")]
    window_process_name: Option<String>,

    /// Project path for helix to change directory to.
    /// Will only be done when executed `execute_path`.
    #[arg(short = 'p', long, value_name = "PATH")]
    project_path: Option<String>,
    /// File path for helix to open.
    #[arg(short = 'f', long, value_name = "PATH")]
    file_path: Option<String>,
    /// Line number in the file for helix to open.
    #[arg(short = 'l', long, value_name = "NUM")]
    line: Option<u32>,
    /// Column number in the file for helix to open.
    #[arg(short = 'c', long, value_name = "NUM")]
    column: Option<u32>,

    /// Option to reduce amount of time when writing full file path.
    /// Only availiable when project path is provided.
    #[arg(short = 'r', long)]
    relative: bool,

    /// List availiable windows.
    /// Format like so: [<process name>] <window title>
    #[arg(long = "list")]
    list_windows: bool,

    /// Search through all windows.
    /// Normally, process's name that contain "Default IME" or "MSCTFIME UI" are ignored by default;
    /// add this flag to include them.
    #[arg(long = "all")]
    all: bool,

    /// Support for using Helix in WSL with application outside WSL
    /// All it do currently is convert Windows path to Linux path
    #[arg(long)]
    wsl: bool,

    /// Use clipboard instead of keyboard macros to speed up the process.
    /// Assuming Ctrl+V works.
    /// This will not replace your current clipboard!
    #[arg(long)]
    clipboard: bool,

    /// Use no macro when a command is ran
    #[arg(long)]
    no_init_macro: bool,

    /// Command to run when no window is found.
    /// If not provided, only search will be done.
    #[arg(long = "run", value_name = "CMD")]
    run_command: Option<Vec<String>>,

    /// After running a command, wait for a bit before running a macro.
    ///
    /// `no-launch-macro` flag disable this entirely
    #[arg(long = "run-wait", value_name = "SECS")]
    run_command_and_wait: Option<f64>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let is_incomplete_arg = matches!(
        (&args.window_title, &args.window_process_name),
        (None, None)
    );

    if args.list_windows {
        let res = list_windows(args.all);
        if is_incomplete_arg {
            return res;
        }
    }

    if is_incomplete_arg {
        return Err(Error::IncompleteSearchArgument);
    }

    let window_title = args.window_title.unwrap_or_else(|| "".to_owned());
    let window_process_name = args.window_process_name.unwrap_or_else(|| "".to_owned());

    let res = focus_window(args.all, &window_title, &window_process_name);
    let mut is_change_directory = false;
    let run_command_args = args
        .run_command
        .or_else(|| args.execute_path.map(|p| vec![p]));
    match (res, run_command_args) {
        (Err(Error::WindowNotFound), Some(run_command_args)) => {
            run_command(run_command_args)?;
            if let Some(wait) = args.run_command_and_wait.or(args.execute_wait) {
                sleep(wait);
            }
            focus_window(args.all, &window_title, &window_process_name)?;
            if args.no_init_macro {
                return Ok(());
            }
            if let Some(project_path) = &args.project_path {
                let project_path = if args.wsl {
                    Cow::Owned(window_path_to_wsl(project_path))
                } else {
                    Cow::Borrowed(project_path)
                };
                keyboard_macro::helix_change_directory(&project_path, args.clipboard);
                is_change_directory = true;
            }
        }
        (Err(e), _) => return Err(e),
        _ => {}
    }

    sleep(0.1);

    if let Some(file_path) = args.file_path {
        if is_change_directory {
            sleep(0.1);
        }
        let file_path = match (args.relative, args.project_path) {
            (true, Some(project_path)) if file_path[..project_path.len()] == project_path => {
                // add one to remove the `/`.
                // It means absolute path with that on
                &file_path[project_path.len() + 1..]
            }
            _ => &file_path[..],
        };
        let file_path = if args.wsl {
            Cow::Owned(window_path_to_wsl(file_path))
        } else {
            Cow::Borrowed(file_path)
        };
        let line = args.line.unwrap_or(0) + 1;
        let column = args.column.unwrap_or(0) + 1;
        keyboard_macro::helix_open_file(&file_path, line, column, args.clipboard);
    }

    Ok(())
}

fn run_command(args: Vec<String>) -> Result<()> {
    use std::process::Command;
    Command::new("cmd").arg("/C").args(args).spawn()?;
    Ok(())
}

fn list_windows(all: bool) -> Result<()> {
    get_windows(all)?
        .into_iter()
        .for_each(|(name, title, _window)| println!("[{name}] {title}"));
    Ok(())
}

fn focus_window(
    all: bool,
    search_window_title: &str,
    search_window_process_name: &str,
) -> Result<()> {
    use helix_win_runner::window::{attach_thread_input, get_current_thread_id};
    let mut windows = get_windows(all)?;
    windows.sort_by(|a, b| a.0.cmp(&b.0));
    let window = windows.into_iter().find(|(name, title, _window)| {
        name.contains(search_window_process_name) && title.contains(search_window_title)
    });
    let Some((name, title, window)) = window else {
        return Err(Error::WindowNotFound);
    };

    println!("Focusing [{name}] {title}");

    let current_thread_id = get_current_thread_id();
    attach_thread_input(window.thread_id(), current_thread_id, true)?;

    window.pop_focus()?;
    window.pop_focus()?;
    window.pop_focus()?;

    attach_thread_input(window.thread_id(), current_thread_id, false)?;
    Ok(())
}

fn window_path_to_wsl(path: &str) -> String {
    let path = path.replace('\\', "/");
    let is_absolute = &path[1..=2] == ":/";
    if is_absolute {
        let drive = path.chars().next().unwrap().to_lowercase();
        let path = &path[3..];
        format!("/mnt/{}/{}", drive, path)
    } else {
        path
    }
}
