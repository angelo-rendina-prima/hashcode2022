use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::env;

#[derive(Debug, Clone)]
struct Worker {
    name: String,
    skill: HashMap<String, usize>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Project {
    name: String,
    duration: usize,
    score: usize,
    before: usize,
    roles: Vec<(String, usize)>,
}


#[derive(Debug, PartialEq, Eq, Clone)]
struct ProjectWrapper {
    score: usize,
    project: Project,
}

impl ProjectWrapper {
    fn new(project: Project, current_day: usize) -> Self {
        // if you finish on time you get more points
        let score = project.before.saturating_sub(current_day) + project.score;

        Self {
            score,
            project
        }
    }
}

impl PartialOrd for ProjectWrapper {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.score.partial_cmp(&other.score)
    }
}

impl Ord for ProjectWrapper {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn parse_input(filename: &str) -> (Vec<Worker>, Vec<Project>) {
    let file = std::fs::File::open(filename).expect("Cannot open file");
    let mut lines = std::io::BufRead::lines(std::io::BufReader::new(file));
    // line 1
    let line = lines.next().expect("No more lines").expect("Invalid line");
    let mut tokens = line.split_whitespace();
    let worker_count = tokens
        .next()
        .expect("No more tokens")
        .parse::<usize>()
        .expect("Not a number");
    let project_count = tokens
        .next()
        .expect("No more tokens")
        .parse::<usize>()
        .expect("Not a number");
    let mut workers = Vec::<Worker>::with_capacity(worker_count);
    let mut projects = Vec::<Project>::with_capacity(project_count);

    // read workers
    for _ in 0..worker_count {
        // name + skills
        let line = lines.next().expect("No more lines").expect("Invalid line");
        let mut tokens = line.split_whitespace();
        let mut worker = Worker {
            name: tokens.next().expect("No name given").to_string(),
            skill: HashMap::new(),
        };
        let skill_count = tokens
            .next()
            .expect("No skill count")
            .parse::<usize>()
            .expect("Not a number");
        // skill per worker
        for _ in 0..skill_count {
            let line = lines.next().expect("No more lines").expect("Invalid line");
            let mut tokens = line.split_whitespace();
            let skill_name = tokens.next().expect("No skill name").to_string();
            let skill_level = tokens
                .next()
                .expect("No skill level")
                .parse::<usize>()
                .expect("Not a number");
            worker.skill.insert(skill_name, skill_level);
        }
        workers.push(worker);
    }

    // read projects
    for _ in 0..project_count {
        // name + skills
        let line = lines.next().expect("No more lines").expect("Invalid line");
        let mut tokens = line.split_whitespace();
        let name = tokens.next().expect("No name given").to_string();
        let duration = tokens
            .next()
            .expect("No duration given")
            .parse::<usize>()
            .expect("Not a number");
        let score = tokens
            .next()
            .expect("No score given")
            .parse::<usize>()
            .expect("Not a number");
        let before = tokens
            .next()
            .expect("No before given")
            .parse::<usize>()
            .expect("Not a number");
        let skill_count = tokens
            .next()
            .expect("No skill count")
            .parse::<usize>()
            .expect("Not a number");
        let mut project = Project {
            name,
            duration,
            score,
            before,
            roles: Vec::with_capacity(skill_count),
        };
        // skill per project
        for _ in 0..skill_count {
            let line = lines.next().expect("No more lines").expect("Invalid line");
            let mut tokens = line.split_whitespace();
            let skill_name = tokens.next().expect("No skill name").to_string();
            let skill_level = tokens
                .next()
                .expect("No skill level")
                .parse::<usize>()
                .expect("Not a number");
            project.roles.push((skill_name, skill_level));
        }
        projects.push(project);
    }

    (workers, projects)
}

#[derive(Clone)]
struct WipProject {
    project: Project,
    started_at: usize,
    workers: Vec<String>,
}

fn main() {
    let arg = env::args().nth(1).unwrap();
    let input = format!("data/{}.txt", arg);

    let (mut workers, projects) = parse_input(&input);

    let projects: Vec<ProjectWrapper> = projects.into_iter().map(|p| ProjectWrapper::new(p, 0)).collect();

    let mut heap = BinaryHeap::from(projects);
    let mut postponed = Vec::new();

    let mut assigned_projects = Vec::new();
    let mut finished_project: Vec<WipProject> = Vec::new();
    let mut assigned_names: HashSet<String> = HashSet::new();

    let mut day = 0;

    loop {
        while let Some(ProjectWrapper {project, ..}) = heap.pop() {
            let mut candidates = Vec::<String>::new();

            for (skill, level) in &project.roles {
                let found_worker = workers
                    .iter()
                    .filter(|worker| {
                        !assigned_names.contains(&worker.name) && !candidates.contains(&worker.name)
                    })
                    .find(|worker| worker.skill.get(skill).map(|s| s >= level).unwrap_or(false));

                if found_worker.is_none() {
                    // postpone it for the next day
                    postponed.push(ProjectWrapper::new(project.clone(), day+1));
                    break;
                }

                let found_worker = found_worker.unwrap();
                // println!("Found worker {found_worker:?}");

                candidates.push(found_worker.name.to_string());
            }
            // println!("Candidates {candidates:?}");
            // println!("All workers: {workers:?}");
            // println!();
            if project.roles.len() == candidates.len() {
                assigned_names.extend(candidates.iter().cloned());
                assigned_projects.push(WipProject {
                    project: project.clone(),
                    started_at: day,
                    workers: candidates,
                });
            }
        }

        if assigned_projects.is_empty() {
            break;
        }

        let (finished, temp_assigned_projects): (Vec<WipProject>, Vec<WipProject>) =
            assigned_projects
                .to_vec()
                .into_iter()
                .partition(|proj| proj.started_at + proj.project.duration + 1 >= day);

        for proj in finished.iter() {
            for (role_id, worker_name) in proj.workers.iter().enumerate() {
                if let Some(w) = workers.iter_mut().find(|w| &w.name == worker_name) {
                    let (skill_name, skill_lvl) = &proj.project.roles[role_id];
                    let w_skill = w.skill.entry(skill_name.to_string()).or_insert(0);
                    if *w_skill <= *skill_lvl {
                        *w_skill += 1;
                    }
                    assigned_names.remove(worker_name);
                }
            }
        }

        assigned_projects = temp_assigned_projects;

        finished_project.extend(finished);

        heap.extend(postponed.drain(..));
        day += 1;

        // all jobs completed
        if heap.is_empty() {
            break;
        }
    }

    println!("{}", finished_project.len());
    for proj in finished_project {
        println!("{}", proj.project.name);
        println!("{}", proj.workers.join(" "));
    }
}
