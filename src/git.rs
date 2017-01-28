use ansi_term::Style;
use git2::Repository;

pub struct Repo(pub Repository);

pub fn get_statuses_from(vector_of_repositories: &Vec<String>,
                         get_only_clean: bool,
                         get_only_dirty: bool) {
    let valid_path_repositories = vector_of_repositories.iter()
        .filter(|path| Repository::init(path).is_ok())
        .collect::<Vec<&String>>();
    let git_objects = valid_path_repositories.iter()
        .map(|path| Repo(Repository::init(path).unwrap()));
    let filtered_git_objects = git_objects
        .filter(|repo| repo.is_clean() == get_only_clean || !repo.is_clean() == get_only_dirty)
        .collect::<Vec<_>>();
    for object in filtered_git_objects {
        object.get_status();
    }
}

impl Repo {
    pub fn is_clean(&self) -> bool {
        let &Repo(ref repository) = self;
        let current_git_index = repository.index();
        match current_git_index {
            Ok(_) => {
                let previous_index =
                    repository.diff_index_to_workdir(Some(&(current_git_index.unwrap())), None)
                        .unwrap();
                let deltas = previous_index.deltas();
                deltas.count() == 0
            }
            Err(_) => false,
        }
    }

    pub fn get_label(&self) -> String {
        if self.is_clean() {
            return String::from("CLEAN");
        }
        String::from("DIRTY")
    }

    pub fn get_status(&self) {
        let &Repo(ref repository) = self;
        let mut to_display = String::new();
        to_display += &format!("----> {}\n",
                               Style::new().italic().paint(repository.path().to_str().unwrap()));
        to_display += &format!("|\t`{}`: {}\n",
                               Style::new().bold().paint("is bare?"),
                               repository.is_bare());
        to_display += &format!("|\t`{}`: {}\n",
                               Style::new().bold().paint("label?"),
                               self.get_label());
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
