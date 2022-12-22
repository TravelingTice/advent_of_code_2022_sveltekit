use wasm_bindgen::prelude::wasm_bindgen;
use regex::Regex;

struct File {
    name: String,
    size: i32,
}

struct Folder<'a> {
    name: String,
    parent_folder: Option<&'a Folder<'a>>,
    folders: Vec<Folder<'a>>,
    files: Vec<File>,
}

impl Folder<'_> {
    pub fn new<'a>(name: &'a str, parent_folder: Option<&'a Folder<'a>>) -> Folder<'a> {
        Folder {
            name: String::from(name),
            parent_folder: parent_folder,
            folders: Vec::new(),
            files: Vec::new()
        }
    }

    fn add_contents(&mut self, content_str: &str) {
        let files_folders_vector = content_str.split("\n").collect::<Vec<&str>>();
        
        for file_or_folder_info_str in files_folders_vector {
            let file_or_folder_info_vec = file_or_folder_info_str.split(" ").collect::<Vec<&str>>();
            
            let size = file_or_folder_info_vec[0];

            if size == "dir" {
                // Time to create a new directory
                let folder = Folder::new(file_or_folder_info_vec[1], Some(self));

                self.folders.push(folder);
            } else {
                // Time to add files to the current folder
                self.files.push(File::new(file_or_folder_info_vec[1], file_or_folder_info_vec[0]));
            }
        }
    }
    
    fn find_child_by_name(&mut self, name: &str) -> &mut Folder {
        return self
    }
}

impl File {
    pub fn new(name: &str, size: &str) -> File {
        File {
            name: String::from(name),
            size: size.parse().unwrap()
        }
    }
}

#[wasm_bindgen]
pub fn solve() {
    let example_input = String::from(
        "$ cd /
$ ls
233998 glh.fcb
184686 jzn
dir qcznqph
dir qtbprrq
299692 rbssdzm.ccn
dir vtb
$ cd qcznqph
$ ls
32148 lhsrj.fnr
dir lnj
dir mtr
dir mznnlph
dir pdtpt
24836 rsjcg.lrh
dir vrj
dir wrqcfl",
    );

    let mut root_folder: Folder = Folder::new("root", None);

    let mut current_folder = &mut root_folder;

    let action_strings_arr: Vec<&str> = example_input.trim().split("$").collect::<Vec<&str>>();

    for action_string in action_strings_arr {
        let re = Regex::new(r"^(cd|ls)[\s|\n]([\s\S]*)").unwrap();
        
        if let Some(command_parts) = re.captures(action_string.trim()) {
            let action_name = &command_parts[1];
            let input = &command_parts[2];
            
            if action_name == "ls" {
                current_folder.add_contents(input)
            } else {
                // Action is cd so we're changing current folder
                current_folder = root_folder.find_child_by_name(input)
            }
        }
    }
}
