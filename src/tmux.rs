use std::process::Command;

pub fn change_window(window_name: &str, path: &str) {
    let mut find_window = Command::new("tmux");
    find_window.arg("list-windows");

    if String::from_utf8(find_window.output().unwrap().stdout)
        .unwrap()
        .contains(window_name)
    {
        let mut swap_window = Command::new("tmux");
        swap_window
            .arg("select-window")
            .arg("-t")
            .arg(window_name);
        swap_window.output().unwrap();
    } else {
        let mut create_new_window = Command::new("tmux");
        create_new_window
            .arg("neww")
            .arg("-n")
            .arg(window_name)
            .arg("-c")
            .arg(path);
        create_new_window.output().unwrap();
    }
}

pub fn remove_window(window_name: &str) {
    let mut remove_tmux_window = Command::new("tmux");
    remove_tmux_window
        .arg("kill-window")
        .arg("-t")
        .arg(window_name);
    remove_tmux_window.output().unwrap();
}
