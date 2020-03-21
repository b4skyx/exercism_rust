#[derive(Debug)]
pub struct HighScores<'a>{
    score : &'a [u32],
}
impl<'a> HighScores<'a>{
    pub fn new(scores: &'a [u32]) -> Self {
        HighScores{ score : scores}
    }

    pub fn scores(&self) -> &[u32] {
        self.score
    }

    pub fn latest(&self) -> Option<u32> {
        self.score.last().cloned()
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.score.iter().max().map(|x| *x)
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut score_vec = self.score.to_vec();
        score_vec.sort_by(|a, b| a.cmp(b).reverse());
        score_vec.truncate(3);
        score_vec
    }
}
