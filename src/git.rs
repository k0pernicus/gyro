use ConfigurationContent;
use ansi_term::Style;
use git2::{BranchType, Repository};
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
        object.get_status();
    }
}

impl Repo {
    pub fn get_status(&self) {
        let &Repo(ref repository) = self;
        let mut to_display = String::new();
        to_display += &format!("----> {}\n",
                               Style::new().italic().paint(repository.path().to_str().unwrap()));
        to_display += &format!("|\t`{}`: {}\n",
                               Style::new().bold().paint("is bare?"),
                               repository.is_bare());
        let current_git_index = repository.index();
        match current_git_index {
            Ok(_) => {
                let previous_index =
                    repository.diff_index_to_workdir(Some(&(current_git_index.unwrap())), None)
                        .unwrap();
                let deltas = previous_index.deltas();
                let nb_diff_files = deltas.count();
                if nb_diff_files == 0 {
                    to_display += &format!("|\t`{}`: CLEAN\n", Style::new().bold().paint("diff?"));
                } else {
                    to_display += &format!("|\t`{}`: DIRTY ({} files)\n",
                                           Style::new().bold().paint("diff?"),
                                           nb_diff_files);
                }
            }
            Err(_) => to_display += &format!("|\tNo informations about the index!\n"),
        }
        to_display += &format!("|\t`{}`: {:?}\n",
                               Style::new().bold().paint("state?"),
                               repository.state());
        match repository.remotes() {
            Ok(array_of_remotes) => {
                to_display += &array_of_remotes.iter()
                    .map(|remote| format!("|\tExisting remote: {}\n", remote.unwrap()))
                    .collect::<String>();
            }
            Err(_) => to_display += &format!("|\tNo remotes to display!"),
        }
        match repository.head() {
            Ok(head) => to_display += &format!("|\tHead: {}\n", head.shorthand().unwrap()),
            Err(_) => to_display += &format!("|\tNo head to display!"),
        }

        println!("{}", to_display);
    }
}
