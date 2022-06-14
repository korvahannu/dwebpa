use std::process::Command;

/// Takes files as arguments presented as a vector of strings deletes them.
///
/// # Examples
///
/// ```
/// let files : Vec<String> = ["file1.webp", "file2.webp"];
/// delete_files(files);
///
/// ```
pub fn delete_files(files: &Vec<String>) {
    for file in files {
        run_command(&format!("rm ./{}", file));
    }
}

/// Takes a filename (string) as a parameter.
/// Checks if that file exists in the current working.
/// Returns bool. True if it exists, false if not.
///
/// # Examples
///
/// ```
/// let result : bool = if_file_exists("hello.jpg");
///
/// ```
pub fn if_file_exists(file: &String) -> bool {
    let all_files = get_non_webp_files();
    if all_files.contains(file) {
        return true;
    }
    false
}

/// Runs a shell command in the current working directory.
///
/// # Examples
///
/// ```
/// run_command(&String::from("rm -rf ./*"));
///
/// ```
pub fn run_command(command: &String) {
    Command::new("sh")
        .arg("-c")
        .arg(command.as_str())
        .output()
        .expect(format!("failed to execute command: {}", command).as_str());
}

/// Gets a list of .webp files from the current working directory and returns them as Vec<String>
///
/// # Examples
///
/// ```
/// let web_files: Vec<String> = get_webp_files();
///
/// ```
pub fn get_webp_files() -> Vec<String> {
    let mut all_files: Vec<String> = get_all_files();
    let mut i: usize = 0;
    while i < all_files.len() {
        if !all_files[i].ends_with(".webp") {
            all_files.remove(i);
        } else {
            i += 1;
        }
    }
    all_files
}

/// Gets a list of non-.webp files from the current working directory and returns them as Vec<String>
///
/// # Examples
///
/// ```
/// let non_web_files: Vec<String> = get_non_webp_files();
///
/// ```
pub fn get_non_webp_files() -> Vec<String> {
    let mut all_files: Vec<String> = get_all_files();
    let mut i: usize = 0;
    while i < all_files.len() {
        if all_files[i].ends_with(".webp") {
            all_files.remove(i);
        } else {
            i += 1;
        }
    }
    all_files
}

/// Gets a list of files from the current working directory and returns them as Vec<String>
///
/// # Examples
///
/// ```
/// let all_files: Vec<String> = get_all_files();
///
/// ```
pub fn get_all_files() -> Vec<String> {
    String::from_utf8(
        Command::new("sh")
            .arg("-c")
            .arg("ls")
            .output()
            .expect("failed to retrieve current directory listing")
            .stdout,
    )
    .unwrap()
    .split('\n')
    .map(|s| s.to_string())
    .collect()
}
