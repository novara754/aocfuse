use std::{
    ffi::OsStr,
    time::{Duration, UNIX_EPOCH},
};

use fuser::{FileAttr, FileType, Filesystem};
use libc::ENOENT;

use crate::parser::{Entry, EntryKind};

pub struct AoCFS {
    entries: Vec<crate::parser::Entry>,
}

impl AoCFS {
    pub fn new(aoc_input: &str) -> Self {
        Self {
            entries: crate::parser::parse_aoc(aoc_input),
        }
    }

    fn get_entry(&self, id: u64) -> Option<&Entry> {
        self.entries.iter().find(|entry| entry.id == id)
    }
}

fn get_entry_attrs(entry: &Entry) -> FileAttr {
    FileAttr {
        ino: entry.id,
        size: match entry.kind {
            EntryKind::Directory { .. } => 0,
            EntryKind::File { size } => size,
        },
        blocks: 0,
        atime: UNIX_EPOCH,
        mtime: UNIX_EPOCH,
        ctime: UNIX_EPOCH,
        crtime: UNIX_EPOCH,
        kind: match entry.kind {
            EntryKind::Directory { .. } => FileType::Directory,
            EntryKind::File { .. } => FileType::RegularFile,
        },
        perm: 0o755,
        nlink: 2,
        uid: unsafe { libc::getuid() },
        gid: unsafe { libc::getgid() },
        rdev: 0,
        flags: 0,
        blksize: 512,
    }
}

impl Filesystem for AoCFS {
    fn lookup(
        &mut self,
        _req: &fuser::Request<'_>,
        parent: u64,
        name: &OsStr,
        reply: fuser::ReplyEntry,
    ) {
        if let Some(entry) = self
            .entries
            .iter()
            .find(|entry| entry.name == name && entry.parent_id == parent)
        {
            reply.entry(&Duration::from_secs(1), &get_entry_attrs(entry), 0);
        } else {
            reply.error(ENOENT);
        }
    }

    fn getattr(&mut self, _req: &fuser::Request<'_>, ino: u64, reply: fuser::ReplyAttr) {
        if let Some(entry) = self.get_entry(ino) {
            reply.attr(&Duration::from_secs(1), &get_entry_attrs(entry))
        } else {
            reply.error(ENOENT);
        }
    }

    fn readdir(
        &mut self,
        _req: &fuser::Request<'_>,
        ino: u64,
        _fh: u64,
        offset: i64,
        mut reply: fuser::ReplyDirectory,
    ) {
        if let Some(Entry {
            kind: EntryKind::Directory { ref children },
            ..
        }) = self.get_entry(ino)
        {
            for entry in children.iter().skip(offset.try_into().unwrap()) {
                let filetype = match self.get_entry(entry.1).unwrap().kind {
                    EntryKind::Directory { .. } => FileType::Directory,
                    EntryKind::File { .. } => FileType::RegularFile,
                };

                if reply.add(
                    entry.1,
                    (entry.1 + 1).try_into().unwrap(),
                    filetype,
                    &entry.0,
                ) {
                    break;
                }
            }
            reply.ok();
        } else {
            reply.error(ENOENT);
        }
    }
}
