#[derive(Clone, Copy)]
pub struct TournamentPair(pub usize, pub usize);

pub struct Schedule {
    players: usize,
    rounds: usize,
    game: usize,
    pair: usize,
    top: Vec<usize>,
    bottom: Vec<usize>,
    current: Option<TournamentPair>,
}

impl Schedule {
    pub fn from(players: usize, rounds: usize) -> Self {
        let count = players + (players % 2);

        Schedule {
            players,
            rounds,
            game: 0,
            pair: 0,
            top: (0..count / 2).collect(),
            bottom: (count / 2..count).collect(),
            current: None,
        }
    }

    pub fn games(&self) -> usize {
        (self.players * (self.players - 1)) * self.rounds
    }
}

impl Iterator for Schedule {
    type Item = TournamentPair;

    fn next(&mut self) -> Option<Self::Item> {
        if self.game >= self.games() {
            return None;
        }

        // Every secod game has switched sides
        if self.game % 2 != 0 {
            self.game += 1;

            let current = self.current.as_ref().unwrap();
            return Some(TournamentPair(current.1, current.0));
        }

        if self.pair >= self.top.len() {
            self.pair = 0;

            // The first column is fixed and the others are rotated clockwise
            self.top.insert(1, self.bottom.remove(0));
            self.bottom.push(self.top.pop().unwrap());
        }

        let red = self.top[self.pair];
        let blue = self.bottom[self.pair];

        self.pair += 1;

        // If number of players is odd one player does not play this round
        self.current = if red < self.players && blue < self.players {
            self.game += 1;

            Some(TournamentPair(red, blue))
        } else {
            self.next()
        };

        self.current
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.games(), Some(self.games()))
    }
}
