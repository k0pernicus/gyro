use ConfigurationContent;
use ansi_term::Style;
use git2::Repository;
use std::fmt;
use std::path::Path;

pub struct Repo(pub Repository);

pub fn get_statuses_from(vector_of_repositories: &Vec<String>) {
    let valid_path_repositories = vector_of_repositories.iter()
        .filter(|path| Repository::init(path).is_ok())
        .collect::<Vec<&String>>();
    let git_objects = valid_path_repositories.iter()
        .map(|path| Repo(Repository::init(path).unwrap()));
    for object in git_objects {
        println!("{}", object);
    }
}

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
