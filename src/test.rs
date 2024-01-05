use super::window_path_to_wsl;

#[test]
fn test_window_path_to_wsl() {
    let path = "C:\\Users\\USER\\OneDrive\\Desktop";
    let linux_path = window_path_to_wsl(path);
    assert_eq!(linux_path, "/mnt/c/Users/USER/OneDrive/Desktop");

    let path = "Users\\USER\\OneDrive\\Desktop";
    let linux_path = window_path_to_wsl(path);
    assert_eq!(linux_path, "Users/USER/OneDrive/Desktop");
}
