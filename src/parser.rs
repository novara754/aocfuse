use std::ffi::OsString;

#[derive(Debug, PartialEq, Eq)]
pub enum EntryKind {
    File { size: u64 },
    Directory { children: Vec<(OsString, u64)> },
}

#[derive(Debug, PartialEq, Eq)]
pub struct Entry {
    pub id: u64,
    pub parent_id: u64,
    pub name: OsString,
    pub kind: EntryKind,
}

#[derive(Debug)]
pub struct Parser {
    current_dir: u64,
    entries: Vec<Entry>,
    next_id: u64,
}

impl Parser {
    fn new() -> Self {
        Self {
            current_dir: 1,
            entries: vec![Entry {
                id: 1,
                parent_id: 1,
                name: "".to_owned().into(),
                kind: EntryKind::Directory { children: vec![] },
            }],
            next_id: 2,
        }
    }

    fn get_by_id(&mut self, id: u64) -> &mut Entry {
        self.entries
            .iter_mut()
            .find(|entry| entry.id == id)
            .unwrap()
    }

    fn change_directory(&mut self, dirname: &str) {
        self.current_dir = if dirname == "/" {
            1
        } else {
            let dir = self.get_by_id(self.current_dir);
            let EntryKind::Directory { ref mut children } = dir.kind else {
            panic!("");
        };
            children
                .iter()
                .find(|(name, _id)| name == dirname)
                .unwrap()
                .1
        };
    }

    fn add_file(&mut self, filename: OsString, filesize: u64) {
        let id = self.next_id;
        self.next_id += 1;

        let dir = self.get_by_id(self.current_dir);
        let EntryKind::Directory { ref mut children } = dir.kind else {
            panic!("");
        };
        children.push((filename.clone(), id));

        let entry = Entry {
            id,
            parent_id: self.current_dir,
            name: filename,
            kind: EntryKind::File { size: filesize },
        };

        self.entries.push(entry);
    }

    fn add_directory(&mut self, dirname: OsString) {
        let id = self.next_id;
        self.next_id += 1;

        let dir = self.get_by_id(self.current_dir);
        let EntryKind::Directory { ref mut children } = dir.kind else {
            panic!("");
        };
        children.push((dirname.clone(), id));

        let entry = Entry {
            id,
            parent_id: self.current_dir,
            name: dirname,
            kind: EntryKind::Directory {
                children: vec![("..".to_owned().into(), self.current_dir)],
            },
        };

        self.entries.push(entry);
    }
}

pub fn parse_aoc(input: &str) -> Vec<Entry> {
    let mut parser = Parser::new();

    let mut ls_output = false;
    for line in input.lines() {
        let mut parts = line.split(' ');
        let first = parts.next().unwrap();

        if first == "$" {
            ls_output = false;
            match parts.next().unwrap() {
                "cd" => {
                    let dirname = parts.next().unwrap();
                    parser.change_directory(dirname);
                }
                "ls" => {
                    ls_output = true;
                }
                _ => unreachable!(),
            }
        } else if ls_output {
            if first == "dir" {
                let dirname = parts.next().unwrap();
                parser.add_directory(dirname.into());
            } else {
                let filesize: u64 = first.parse().unwrap();
                let filename = parts.next().unwrap();
                parser.add_file(filename.into(), filesize);
            }
        }
    }

    parser.entries
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_file_in_root() {
        let input = r#"$ cd /
$ ls
1234 a.txt"#;
        let entries = parse_aoc(input);
        assert_eq!(
            &entries,
            &[
                Entry {
                    id: 1,
                    parent_id: 1,
                    name: "".to_owned().into(),
                    kind: EntryKind::Directory {
                        children: vec![("a.txt".to_owned().into(), 2)]
                    },
                },
                Entry {
                    id: 2,
                    parent_id: 1,
                    name: "a.txt".to_owned().into(),
                    kind: EntryKind::File { size: 1234 }
                }
            ]
        );
    }

    #[test]
    fn many_files_in_root() {
        let input = r#"$ cd /
$ ls
1234 a.txt
5555 b.txt
1 c.txt
13512362346 d.txt"#;
        let entries = parse_aoc(input);
        assert_eq!(
            &entries,
            &[
                Entry {
                    id: 1,
                    parent_id: 1,
                    name: "".to_owned().into(),
                    kind: EntryKind::Directory {
                        children: vec![
                            ("a.txt".to_owned().into(), 2),
                            ("b.txt".to_owned().into(), 3),
                            ("c.txt".to_owned().into(), 4),
                            ("d.txt".to_owned().into(), 5)
                        ]
                    },
                },
                Entry {
                    id: 2,
                    parent_id: 1,
                    name: "a.txt".to_owned().into(),
                    kind: EntryKind::File { size: 1234 }
                },
                Entry {
                    id: 3,
                    parent_id: 1,
                    name: "b.txt".to_owned().into(),
                    kind: EntryKind::File { size: 5555 }
                },
                Entry {
                    id: 4,
                    parent_id: 1,
                    name: "c.txt".to_owned().into(),
                    kind: EntryKind::File { size: 1 }
                },
                Entry {
                    id: 5,
                    parent_id: 1,
                    name: "d.txt".to_owned().into(),
                    kind: EntryKind::File { size: 13512362346 }
                }
            ]
        );
    }

    #[test]
    fn single_dir_in_root() {
        let input = r#"$ cd /
$ ls
dir foo"#;
        let entries = parse_aoc(input);
        assert_eq!(
            &entries,
            &[
                Entry {
                    id: 1,
                    parent_id: 1,
                    name: "".to_owned().into(),
                    kind: EntryKind::Directory {
                        children: vec![("foo".to_owned().into(), 2)]
                    },
                },
                Entry {
                    id: 2,
                    parent_id: 1,
                    name: "foo".to_owned().into(),
                    kind: EntryKind::Directory {
                        children: vec![("..".to_owned().into(), 1)]
                    }
                }
            ]
        );
    }

    #[test]
    fn single_file_in_single_dir_in_root() {
        let input = r#"$ cd /
$ ls
dir foo
$ cd foo
$ ls
1234 a.txt"#;
        let entries = parse_aoc(input);
        assert_eq!(
            &entries,
            &[
                Entry {
                    id: 1,
                    parent_id: 1,
                    name: "".to_owned().into(),
                    kind: EntryKind::Directory {
                        children: vec![("foo".to_owned().into(), 2)]
                    },
                },
                Entry {
                    id: 2,
                    parent_id: 1,
                    name: "foo".to_owned().into(),
                    kind: EntryKind::Directory {
                        children: vec![("..".to_owned().into(), 1), ("a.txt".to_owned().into(), 3)]
                    }
                },
                Entry {
                    id: 3,
                    parent_id: 2,
                    name: "a.txt".to_owned().into(),
                    kind: EntryKind::File { size: 1234 }
                }
            ]
        );
    }

    #[test]
    fn aoc_example() {
        let input = r#"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"#;
        let entries = parse_aoc(input);
        assert_eq!(
            &entries,
            &[
                Entry {
                    id: 1,
                    parent_id: 1,
                    name: "".to_owned().into(),
                    kind: EntryKind::Directory {
                        children: vec![
                            ("a".to_owned().into(), 2),
                            ("b.txt".to_owned().into(), 3),
                            ("c.dat".to_owned().into(), 4),
                            ("d".to_owned().into(), 5),
                        ],
                    },
                },
                Entry {
                    id: 2,
                    parent_id: 1,
                    name: "a".to_owned().into(),
                    kind: EntryKind::Directory {
                        children: vec![
                            ("..".to_owned().into(), 1),
                            ("e".to_owned().into(), 6),
                            ("f".to_owned().into(), 7),
                            ("g".to_owned().into(), 8),
                            ("h.lst".to_owned().into(), 9),
                        ],
                    },
                },
                Entry {
                    id: 3,
                    parent_id: 1,
                    name: "b.txt".to_owned().into(),
                    kind: EntryKind::File { size: 14848514 },
                },
                Entry {
                    id: 4,
                    parent_id: 1,
                    name: "c.dat".to_owned().into(),
                    kind: EntryKind::File { size: 8504156 },
                },
                Entry {
                    id: 5,
                    parent_id: 1,
                    name: "d".to_owned().into(),
                    kind: EntryKind::Directory {
                        children: vec![
                            ("..".to_owned().into(), 1),
                            ("j".to_owned().into(), 11),
                            ("d.log".to_owned().into(), 12),
                            ("d.ext".to_owned().into(), 13),
                            ("k".to_owned().into(), 14),
                        ],
                    },
                },
                Entry {
                    id: 6,
                    parent_id: 2,
                    name: "e".to_owned().into(),
                    kind: EntryKind::Directory {
                        children: vec![("..".to_owned().into(), 2), ("i".to_owned().into(), 10),],
                    },
                },
                Entry {
                    id: 7,
                    parent_id: 2,
                    name: "f".to_owned().into(),
                    kind: EntryKind::File { size: 29116 },
                },
                Entry {
                    id: 8,
                    parent_id: 2,
                    name: "g".to_owned().into(),
                    kind: EntryKind::File { size: 2557 },
                },
                Entry {
                    id: 9,
                    parent_id: 2,
                    name: "h.lst".to_owned().into(),
                    kind: EntryKind::File { size: 62596 },
                },
                Entry {
                    id: 10,
                    parent_id: 6,
                    name: "i".to_owned().into(),
                    kind: EntryKind::File { size: 584 },
                },
                Entry {
                    id: 11,
                    parent_id: 5,
                    name: "j".to_owned().into(),
                    kind: EntryKind::File { size: 4060174 },
                },
                Entry {
                    id: 12,
                    parent_id: 5,
                    name: "d.log".to_owned().into(),
                    kind: EntryKind::File { size: 8033020 },
                },
                Entry {
                    id: 13,
                    parent_id: 5,
                    name: "d.ext".to_owned().into(),
                    kind: EntryKind::File { size: 5626152 },
                },
                Entry {
                    id: 14,
                    parent_id: 5,
                    name: "k".to_owned().into(),
                    kind: EntryKind::File { size: 7214296 },
                },
            ]
        );
    }
}
