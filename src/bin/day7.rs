use std::collections::HashMap;

static DAY: u8 = 7;

fn main() {
    let input = advent::read_lines(DAY);
    println!("{DAY}a: {}", sum_directories(&input, 100000));
    println!("{DAY}b: {}", size_deletable_directory(&input, 70000000, 30000000));
}

enum Command {
    Cd { directory: String },
    List,
}

impl Command {
    fn new(cmdline: &str) -> Command {
        if let Some(directory) = cmdline.strip_prefix("cd ") {
            Command::Cd { directory: directory.to_string() }
        } else if cmdline == "ls" {
            Command::List
        } else {
            unimplemented!()
        }
    }
}

struct CommandResult {
    command: Command,
    output: Vec<String>,
}

fn parse_commands(input: &[String]) -> Vec<CommandResult> {
    let mut commandresults = Vec::new();
    let mut command = String::new();
    let mut output = Vec::new();
    for line in input {
        if line.starts_with('$') {
            if !command.is_empty() {
                /* previous command finished */
                commandresults.push(CommandResult {
                    command: Command::new(&command),
                    output: output.clone()
                });
            }
            command = line.strip_prefix("$ ").expect("Line should begin with '$ '").to_string();
            output.clear();
        } else {
            /* line is part of the output */
            output.push(line.to_string());
        }
    }
    commandresults.push(CommandResult {
        command: Command::new(&command),
        output: output.clone()
    });

    commandresults
}

enum FsEntry {
    File { size: usize },
    Directory { name: String },
}

impl FsEntry {
    fn new(line: &str) -> FsEntry {
        if let Some(name) = line.strip_prefix("dir ") {
            FsEntry::Directory { name: name.to_string() }
        } else if let Some((size, _)) = line.split_once(' ') {
            let size = size.parse::<usize>().expect("should parse as number");
            FsEntry::File { size }
        } else {
            unimplemented!()
        }
    }
}

fn eval_program(commandresults: &[CommandResult]) -> HashMap<String, Vec<FsEntry>> {
    let mut filesystem = HashMap::new();
    let mut cwd = Vec::new();
    for CommandResult{command, output} in commandresults {
        match command {
            Command::Cd{ directory } if directory == "/" => { cwd.clear() },
            Command::Cd{ directory } if directory == ".." => { cwd.pop(); },
            Command::Cd{ directory } => { cwd.push(directory.clone()) },
            Command::List => {
                let entries = output.iter().map(|o| FsEntry::new(o)).collect::<Vec<_>>();
                filesystem.insert(format!("/{}", cwd.join("/")), entries);
            },
        }
    }
    filesystem
}

fn directory_size(path: &str, filesystem: &HashMap<String, Vec<FsEntry>>, sizes: &mut HashMap<String, usize>) -> usize {
    let mut dir_size = 0;
    for entry in &filesystem[path] {
        match entry {
            FsEntry::File { size } => { dir_size += size },
            FsEntry::Directory { name } => {
                let subdir = if path == "/" {
                    format!("/{}", name)
                } else {
                    format!("{}/{}", path, name)
                };
                dir_size += directory_size(&subdir, filesystem, sizes);
            }
        }
    }
    sizes.insert(path.to_string(), dir_size);

    dir_size
}

fn sum_directories(input: &[String], max_size: usize) -> usize {
    let commandresults = parse_commands(input);
    let filesystem = eval_program(&commandresults);

    let mut sizes = HashMap::new();
    directory_size("/", &filesystem, &mut sizes);

    sizes.values()
         .filter(|&size| *size <= max_size)
         .sum()
}

fn size_deletable_directory(input: &[String], total_size: usize, needed_size: usize) -> usize {
    let commandresults = parse_commands(input);
    let filesystem = eval_program(&commandresults);

    let mut sizes = HashMap::new();
    directory_size("/", &filesystem, &mut sizes);

    let max_size = total_size - needed_size;
    let need_to_free = sizes["/"] - max_size;

    *sizes.values()
          .filter(|&size| *size > need_to_free)
          .min()
          .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = [
            "$ cd /",
            "$ ls",
            "dir a",
            "14848514 b.txt",
            "8504156 c.dat",
            "dir d",
            "$ cd a",
            "$ ls",
            "dir e",
            "29116 f",
            "2557 g",
            "62596 h.lst",
            "$ cd e",
            "$ ls",
            "584 i",
            "$ cd ..",
            "$ cd ..",
            "$ cd d",
            "$ ls",
            "4060174 j",
            "8033020 d.log",
            "5626152 d.ext",
            "7214296 k",
        ].iter().map(|&x| String::from(x)).collect::<Vec<_>>();

        assert_eq!(sum_directories(&input, 100000), 95437);
        assert_eq!(size_deletable_directory(&input, 70000000, 30000000), 24933642);
    }
}
