extern crate fuse;
extern crate libc;
extern crate rustc_serialize;
extern crate time;
//use rustc_serialize::json;
use serde_json::{from_str, json, Error, Value};

use fuse::{
    FileAttr, FileType, Filesystem, ReplyAttr, ReplyData, ReplyDirectory, ReplyEntry, Request,
};
use libc::{ENOENT, ENOSYS};
use std::env;
use std::path::Path;
use time::Timespec;

struct JsonFilesystem {
    tree: Vec<Value>,
}

impl Filesystem for JsonFilesystem {
    fn getattr(&mut self, _req: &Request, ino: u64, reply: ReplyAttr) {
        println!("getatrr(ino={})", ino);
        let ts = Timespec::new(0, 0);
        let attr = FileAttr {
            ino: 1,
            size: 0,
            blocks: 0,
            atime: ts,
            mtime: ts,
            ctime: ts,
            crtime: ts,
            kind: FileType::Directory,
            perm: 0o755,
            nlink: 0,
            uid: 0,
            gid: 0,
            rdev: 0,
            flags: 0,
        };
        let ttl = Timespec::new(1, 0);
        if ino == 1 {
            reply.attr(&ttl, &attr);
        } else {
            reply.error(ENOSYS);
        }
    }

    fn readdir(
        &mut self,
        _req: &Request,
        ino: u64,
        fh: u64,
        offset: i64,
        mut reply: ReplyDirectory,
    ) {
        println!("readdir(ino={}, fh={}, offset={}", ino, fh, offset);
        for item in self.tree.iter() {
            let inode: u64 = 2 + i as u64;
            let offset: i64 = 2 + i as i64;
            reply.add(inode, offset, FileType::RegularFile, &Path::new(key));
        }
        if ino == 1 {
            if offset == 0 {
                reply.add(1, 0, FileType::Directory, &Path::new("."));
                reply.add(1, 0, FileType::Directory, &Path::new(".."));
            }
            //reply.error(ENOSYS);
            reply.ok();
        } else {
            reply.error(ENOSYS);
        }
    }
}

fn main() {
    let input = r#"
    {
        "foo": "bar",
        "answer": "42",
    }"#;
    let tree: Vec<Value> = from_str(input).expect("wrong !");
    let fs = JsonFilesystem { tree: tree.clone() };

    let mountpoint = match env::args().nth(1) {
        Some(path) => path,
        None => {
            println!("Usage: {}, <MOUNTPOINT>", env::args().nth(0).unwrap());
            return;
        }
    };
    fuse::mount(JsonFilesystem, &mountpoint, &[]);
}
