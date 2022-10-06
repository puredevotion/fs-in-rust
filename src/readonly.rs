use libc::ENOENT;
use std::ffi::OsStr;
use std::time::{Duration, UNIX_EPOCH};

use fuser::{
    FileAttr, FileType, Filesystem, MountOption, ReplyAttr, ReplyData, ReplyDirectory, ReplyEntry,
    Request,
};

// the Struct that contains the FS details
struct ReadonlyFS;

const TTL: Duration = Duration::from_secs(1); // 1 second

// the file attributes for our directory
const HELLO_DIR_ATTR: FileAttr = FileAttr {
    ino: 1,
    size: 0,
    blocks: 0,
    atime: UNIX_EPOCH, // 1970-01-01 00:00:00
    mtime: UNIX_EPOCH,
    ctime: UNIX_EPOCH,
    crtime: UNIX_EPOCH,
    kind: FileType::Directory,
    perm: 0o755,
    nlink: 2,
    uid: 1000,
    gid: 1000,
    rdev: 0,
    flags: 0,
    blksize: 512,
};

// the file attributes for our plain-text file
const HELLO_TXT_ATTR: FileAttr = FileAttr {
    ino: 2, // second inode, after the directory
    size: 13, // 12 characters "in hello world", plus a newline
    blocks: 1,
    atime: UNIX_EPOCH, // 1970-01-01 00:00:00
    mtime: UNIX_EPOCH,
    ctime: UNIX_EPOCH,
    crtime: UNIX_EPOCH,
    kind: FileType::RegularFile,
    perm: 0o644,
    nlink: 1,
    uid: 1000,
    gid: 1000,
    rdev: 0,
    flags: 0,
    blksize: 512,
};

const HELLO_TXT_CONTENT: &str = "Hello World!\n"; //contents of the text file

impl Filesystem for ReadonlyFS {
    fn lookup(&mut self, _req: &Request, parent: u64, name: &OsStr, reply: ReplyEntry) {
        if parent == 1 && name.to_str() == Some("hello.txt") {
            reply.entry(&TTL, &HELLO_TXT_ATTR, 0);
        } else {
            reply.error(ENOENT);
        }
    }

    /// Get attributes of files and directories
    fn getattr(&mut self, _req: &Request, ino: u64, reply: ReplyAttr) {
        // based on the inode number we are returning file attributes.
        // If wrong inode is provided err with "file or dir does not exist"
        match ino {
            1 => reply.attr(&TTL, &HELLO_DIR_ATTR),
            2 => reply.attr(&TTL, &HELLO_TXT_ATTR),
            _ => reply.error(ENOENT),
        }
    }

    /// Read file contents
    fn read(
        &mut self,
        _req: &Request,
        ino: u64,
        _fh: u64,
        offset: i64,
        _size: u32,
        _flags: i32,
        _lock: Option<u64>,
        reply: ReplyData,
    ) {
        // there's only ino #2 to read, the text file. return its contents or err if other inode requested.
        if ino == 2 {
            reply.data(&HELLO_TXT_CONTENT.as_bytes()[offset as usize..]);
        } else {
            reply.error(ENOENT);
        }
    }

    /// Read directory contents
    fn readdir(
        &mut self,
        _req: &Request,
        ino: u64,
        _fh: u64,
        offset: i64,
        mut reply: ReplyDirectory,
    ) {
        if ino != 1 {
            reply.error(ENOENT);
            return;
        }

        // provide a static list of all files
        let entries = vec![
            (1, FileType::Directory, "."),
            (1, FileType::Directory, ".."),
            (2, FileType::RegularFile, "hello.txt"),
        ];

        for (i, entry) in entries.into_iter().enumerate().skip(offset as usize) {
            // i + 1 means the index of the next entry
            if reply.add(entry.0, (i + 1) as i64, entry.1, entry.2) {
                break;
            }
        }
        reply.ok();
    }
}

pub struct MountOptions {pub auto_unmount: bool, pub root: bool}

pub fn main(mountpoint: &str, mountoptions: MountOptions ) {
    env_logger::init();

    let mut options = vec![MountOption::RO, MountOption::FSName("readonly-fs".to_string())];
    if mountoptions.auto_unmount {
        options.push(MountOption::AutoUnmount);
    }
    if mountoptions.root {
        options.push(MountOption::AllowRoot);
    }
    // mount the FS with specified options
    fuser::mount2(ReadonlyFS, mountpoint, &options).unwrap();
}
