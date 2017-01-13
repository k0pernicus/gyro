use ansi_term::Style;
use git2::Repository;
use std::fmt;

pub struct Repo(pub Repository);

impl fmt::Display for Repo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let &Repo(ref repository) = self;
        write!(f,
               "----> {}\n|\t`{}`: {}\n|\t`{}`: {:?}\n----",
               Style::new().italic().paint(repository.path().to_str().unwrap()),
               Style::new().bold().paint("is bare?"),
               repository.is_bare(),
               Style::new().bold().paint("state?"),
               repository.state())
    }
}
