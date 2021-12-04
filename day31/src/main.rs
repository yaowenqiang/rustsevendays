extern crate git2;
extern crate time;
use git2::{Commit, ObjectType, Repository};

fn find_last_commit(repo: &Repository) -> Result<Commit, git2::Error> {
    let obj = repo.head()?.resolve()?.peel(ObjectType::Commit)?;
    obj.into_commit().map_err(|_| git2::Error::from_str("Couldn't find commit"))
}

fn display_commit(commit: &Commit) {
    let timestamp = commit.time().seconds();
    let tm = time::at(time::Timespec::new(timestamp, 0));
    println!("commit: {}\nAuthor: {}  \nDate  : {}\n  {}",
             commit.id(),
             commit.author(),
             tm.rfc822(),
             commit.message().unwrap_or("no commit message"));
}

fn main() {
    let repo_root = std::env::args().nth(1).unwrap_or(".".to_string());
    let repo = Repository::open(repo_root.as_str()).expect("Couldn't open repository");
    println!("{} stat={:?}", repo.path().display(), repo.state());

    let commit = find_last_commit(&repo).expect("Coulnd't find last commit");
    display_commit(&commit);
}
