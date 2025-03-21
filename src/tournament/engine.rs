use std::{
    io::{BufRead, BufReader, Write},
    process::{Child, ChildStdin, ChildStdout, Command, Stdio},
};

pub struct EngineChild {
    name: String,
    child: Child,
    stdin: ChildStdin,
    reader: BufReader<ChildStdout>,
    cheating: bool,
}

impl EngineChild {
    pub fn new(name: &str, mut child: Child, cheating: bool) -> Self {
        let stdin = child.stdin.take().expect("Failed to open stdin");
        let stdout = child.stdout.take().expect("Failed to open stdout");

        let reader = BufReader::new(stdout);

        Self {
            name: name.to_string(),
            child,
            stdin,
            reader,
            cheating,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn cheating(&self) -> bool {
        self.cheating
    }

    pub fn wait(&mut self) {
        let _ = self.child.wait();
    }

    pub fn quit(&mut self) {
        self.write("quit");
    }

    pub fn set_option(&mut self) {
        todo!()
    }

    pub fn position(&mut self, pos: &str, moves: &[String]) {
        self.write(&["position", pos, "moves", &moves.join(" ")].join(" "));
    }

    pub fn new_game(&mut self) {
        self.write("newgame");
    }

    pub fn is_ready(&mut self) {
        self.send("isready", "readyok");
    }

    pub fn deployment(&mut self) -> String {
        self.send("deployment", "deployment")
            .expect(&format!("Failed to get deployment from {}", self.name))
            .trim()
            .split_whitespace()
            .nth(1)
            .unwrap()
            .to_string()
    }

    pub fn go(&mut self) -> String {
        self.send("go", "bestmove")
            .expect(&format!("Failed to get bestmove from {}", self.name))
            .trim()
            .split_whitespace()
            .nth(1)
            .unwrap()
            .to_string()
    }

    fn write(&mut self, input: &str) {
        writeln!(self.stdin, "{}", input).expect(&format!("Failed to write {}'s stdin", self.name));
        self.stdin
            .flush()
            .expect(&format!("Failed to flush {}'s stdin", self.name));
    }

    pub fn send(&mut self, input: &str, stop: &str) -> Option<String> {
        let _ = self.write(input);

        let mut buffer = String::new();
        while self.reader.read_line(&mut buffer).unwrap() > 0 {
            println!("{}: {}", self.name, buffer.trim());

            if buffer.contains(stop) {
                return Some(buffer);
            }

            buffer.clear();
        }

        None
    }
}

pub struct EngineRunner(Command);

impl EngineRunner {
    pub fn new(path: &str) -> Self {
        EngineRunner(Command::new(path))
    }

    pub fn stdout<T: Into<Stdio>>(&mut self, cfg: T) -> &mut Self {
        self.0.stdout(cfg);

        self
    }

    pub fn stdin<T: Into<Stdio>>(&mut self, cfg: T) -> &mut Self {
        self.0.stdin(cfg);

        self
    }

    pub fn spawn(&mut self, name: &str, cheating: bool) -> EngineChild {
        EngineChild::new(
            name,
            self.0.spawn().expect("Failed to spawn engine runner"),
            cheating,
        )
    }
}
