use std::cmp::Ordering;

/// Type of vector matching score
pub type ScoreType = f32;
/// Type of point index inside a segment
pub type PointOffsetType = u32;

#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub struct ScoredPointOffset {
    pub idx: PointOffsetType,
    pub score: ScoreType,
}

impl Eq for ScoredPointOffset {}

impl Ord for ScoredPointOffset {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.score.total_cmp(&other.score)
    }
}

impl PartialOrd for ScoredPointOffset {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Copy, Clone, Debug)]
pub struct TelemetryDetail {
    pub level: DetailsLevel,
    pub histograms: bool,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum DetailsLevel {
    Level0,
    Level1,
    Level2,
}

impl Default for TelemetryDetail {
    fn default() -> Self {
        TelemetryDetail {
            level: DetailsLevel::Level0,
            histograms: false,
        }
    }
}

impl From<usize> for DetailsLevel {
    fn from(value: usize) -> Self {
        match value {
            0 => DetailsLevel::Level0,
            1 => DetailsLevel::Level1,
            _ => DetailsLevel::Level2,
        }
    }
}
