use rand::RngExt;

pub struct Grid {
    pub width: usize,
    pub height: usize,
    current: Vec<bool>,
    next: Vec<bool>,
}

impl Grid {
    pub fn random(width: usize, height: usize) -> Self {
        let mut rnd = rand::rng();
        let current = (0..width * height).map(|_| rnd.random_bool(0.35)).collect();

        Self {
            width,
            height,
            current,
            next: vec![false; width * height],
        }
    }

    #[inline]
    fn idx(&self, y: usize, x: usize) -> usize {
        y * self.width + x
    }

    #[inline]
    pub fn alive(&self, x: usize, y: usize) -> bool {
        self.current[self.idx(y, x)]
    }

    fn neighbors(&self, x: usize, y: usize) -> u8 {
        let mut count = 0;

        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }

                let nx = ((x as isize + dx + self.width as isize) % self.width as isize) as usize;

                let ny = ((y as isize + dy + self.height as isize) % self.height as isize) as usize;

                if self.alive(nx, ny) {
                    count += 1;
                }
            }
        }

        count
    }

    pub fn step(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let n = self.neighbors(x, y);
                let alive = self.alive(x, y);

                self.next[y * self.width + x] = matches!((alive, n), (true, 2 | 3) | (false, 3));
            }
        }

        std::mem::swap(&mut self.current, &mut self.next);
    }
}
