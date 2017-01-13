use git2::Repository;
use std::fmt;

pub struct Repo(pub Repository);

impl fmt::Display for Repo {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let &Repo(ref repository) = self;
        write!(f, "---- {:?}\n| `is bare?`: {}\n| `state?`: {:?}\n----", repository.path(), repository.is_bare(), repository.state())
    }

}
