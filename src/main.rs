use std::collections::HashMap;

#[derive(Debug)]
struct Worker {
    name: String,
    skill: HashMap<String, usize>,
}

#[derive(Debug)]
struct Project {
    name: String,
    duration: usize,
    score: usize,
    before: usize,
    roles: HashMap<String, usize>,
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

fn main() {
    let (workers, projects) = parse_input("data/a.txt");
    println!("Workers: {workers:#?}");
    println!("Projects: {projects:#?}");
}
