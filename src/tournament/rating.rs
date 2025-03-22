pub struct Ranking {
    index: usize,
    wins: usize,
    draws: usize,
    losses: usize,
}

impl Ranking {
    pub fn new(index: usize) -> Self {
        Self {
            index,
            wins: 0,
            draws: 0,
            losses: 0,
        }
    }

    pub fn update(&mut self, reward: f32) {
        match reward {
            0.0 => self.losses += 1,
            0.5 => self.draws += 1,
            1.0 => self.wins += 1,
            _ => unreachable!(),
        }
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn games(&self) -> f32 {
        (self.wins + self.draws + self.losses) as f32
    }

    pub fn wins(&self) -> usize {
        self.wins
    }

    pub fn draws(&self) -> usize {
        self.draws
    }

    pub fn losses(&self) -> usize {
        self.losses
    }

    pub fn points(&self) -> f32 {
        self.wins as f32 + self.draws as f32 / 2.0
    }

    pub fn diff(&self) -> f32 {
        let n = self.games();
        let w = self.wins as f32 / n;
        let d = self.draws as f32 / n;

        let mu = w + d / 2.0;

        -400.0 * (1.0 / mu - 1.0).log10()
    }
}
