use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};

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
    roles: HashMap<String, usize>,
}

impl PartialOrd for Project {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.score.partial_cmp(&other.score)
    }
}

impl Ord for Project {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score.cmp(&other.score)
    }
}


// impl Project {
//     fn skill_match(&self, worker: &Worker) -> HashMap<String, isize> {
//         let mut match_scores = HashMap::new();
//
//         for (skill, req_level) in &self.roles {
//             if let Some(level) = worker.skill.get(skill) {
//                 let qualification_score = (level as isize) - (req_level as isize);
//                 match_scores.insert(skill.to_string(), qualification_score);
//             }
//         }
//
//         match_scores
//     }
// }

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
            roles: HashMap::new(),
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
            project.roles.insert(skill_name, skill_level);
        }
        projects.push(project);
    }

    (workers, projects)
}

#[derive(Clone)]
struct WipProject {
    project: Project,
    started_at: usize,
    workers: HashMap<String, String>,
}

fn main() {
    let (mut workers, projects) = parse_input("data/f.txt");

    let mut heap = BinaryHeap::from(projects);
    let mut postponed = BinaryHeap::new();

    let mut assigned_projects = Vec::new();
    let mut finished_project: Vec<WipProject> = Vec::new();
    let mut assigned_names: HashSet<String> = HashSet::new();

    let mut day = 0;

    loop {
        while let Some(project) = heap.pop() {
            let mut candidates = HashMap::new();

            for (skill, level) in &project.roles {
                let found_worker = workers
                    .iter()
                    .filter(|worker| {
                        !assigned_names.contains(&worker.name)
                            && !candidates.contains_key(&worker.name)
                    })
                    .find(|worker| worker.skill.get(skill).map(|s| s >= level).unwrap_or(false));

                if found_worker.is_none() {
                    postponed.push(project.clone());
                    break;
                }

                let found_worker = found_worker.unwrap();
                candidates.insert(found_worker.name.to_string(), skill.to_string());
            }

            if project.roles.len() == candidates.len() {
                let names = candidates.iter().map(|(name, _)| name.to_string()).collect::<Vec<String>>();
                assigned_names.extend(names);
                assigned_projects.push(WipProject {
                    project: project.clone(),
                    started_at: day,
                    workers: candidates.drain().collect(),
                });
            }
        }

        if assigned_projects.is_empty() {
            break;
        }

        let (finished, ap): (Vec<WipProject>, Vec<WipProject>) = assigned_projects
            .to_vec()
            .into_iter()
            .partition(|proj| proj.started_at + proj.project.duration + 1 >= day);

        for proj in finished.iter() {
            for (name, skill) in &proj.workers {
                if let Some(w) = workers.iter_mut().find(|w| &w.name == name) {
                    w.skill.entry(skill.to_string()).and_modify(|e| *e += 1);
                }
            }
        }

        assigned_projects = ap;

        finished_project.extend(finished);

        heap.extend(postponed.drain());
        day += 1;

        if heap.is_empty() {
            break;
        }
    }

    println!("{}", finished_project.len());
    for proj in finished_project {
        println!("{}", proj.project.name);
        let names = proj.workers.iter().map(|(name, _)| name.as_str()).collect::<Vec<_>>();
        println!("{}", names.join(" "));
    }
}
