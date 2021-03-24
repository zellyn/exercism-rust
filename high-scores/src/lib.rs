#[derive(Debug)]
pub struct HighScores<'a>(&'a[u32]);

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        HighScores(scores)
    }

    pub fn scores(&self) -> &[u32] {
        self.0
    }

    pub fn latest(&self) -> Option<u32> {
        self.0.last().copied()
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.0.iter().max().copied()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut v: Vec<u32> = self.0.to_vec();
        v.sort_unstable_by(|a, b| b.cmp(a));
        v.truncate(3);
        v
    }
}
