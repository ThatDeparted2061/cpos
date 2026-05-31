use std::collections::{HashMap, HashSet};

use crate::data::models::*;
use crate::engine::weakness::compute_tag_stats;

pub struct Recommendation {
    pub problem: Problem,
    pub reason: String,
}

/// Recommend unsolved problems to practice next.
///
/// Two regimes:
/// - **Cold start** (no accepted submissions): there's nothing personal to go
///   on, so we surface popular, well-known problems around a sensible default
///   level, spread across topics and ratings.
/// - **Warm** (history present): we target the user's weak topics at a rating
///   just above their current level, while keeping the list diverse so it isn't
///   twenty near-identical problems.
pub fn recommend_problems(
    submissions: &[Submission],
    all_problems: &[Problem],
    user_rating: Option<u32>,
    count: usize,
) -> Vec<Recommendation> {
    let solved_ids: HashSet<String> = submissions
        .iter()
        .filter(|s| s.verdict == Verdict::Accepted)
        .map(|s| format!("{:?}:{}", s.platform, s.problem_id))
        .collect();

    let attempted_ids: HashSet<String> = submissions
        .iter()
        .map(|s| format!("{:?}:{}", s.platform, s.problem_id))
        .collect();

    let has_history = !solved_ids.is_empty();

    let tag_stats = compute_tag_stats(submissions, all_problems);
    let weak_tags: Vec<String> = tag_stats
        .iter()
        .filter(|t| t.solved + t.attempted >= 2)
        .take(6)
        .map(|t| t.tag.to_lowercase())
        .collect();

    let center = user_rating.unwrap_or(1200);
    // Practice a little above your current level to actually improve.
    let target = center + 100;
    let lo = center.saturating_sub(200);
    let hi = center + 300;

    // Score every eligible candidate.
    let mut scored: Vec<Scored> = Vec::new();
    for problem in all_problems {
        let key = format!("{:?}:{}", problem.platform, problem.id);
        if solved_ids.contains(&key) {
            continue;
        }
        let Some(rating) = problem.rating else {
            continue;
        };
        if rating < lo || rating > hi {
            continue;
        }

        let tags_lower: Vec<String> = problem.tags.iter().map(|t| t.to_lowercase()).collect();
        let weak_matches: Vec<&String> =
            tags_lower.iter().filter(|t| weak_tags.contains(t)).collect();

        let mut score = 0.0_f64;

        // Popularity: well-known problems are better practice. Log-scaled so a
        // few mega-popular problems don't dominate everything.
        let pop = (problem.solved_count.unwrap_or(0) as f64 + 1.0).ln();
        score += pop;

        // Closeness to the practice target.
        let dist = (rating as f64 - target as f64).abs();
        score += (4.0 - dist / 150.0).max(0.0);

        // Weak-topic emphasis (only meaningful once there's history).
        score += weak_matches.len() as f64 * 3.0;

        // A small nudge toward problems you've tried but not solved.
        if attempted_ids.contains(&key) {
            score += 2.5;
        }

        let reason = if !weak_matches.is_empty() {
            let tags = weak_matches
                .iter()
                .take(2)
                .map(|s| s.as_str())
                .collect::<Vec<_>>()
                .join(", ");
            format!("Weak topic: {tags}")
        } else if attempted_ids.contains(&key) {
            "Unfinished — give it another go".to_string()
        } else if has_history {
            format!("Just above your level · {rating}")
        } else {
            format!("Popular {rating} problem")
        };

        let primary_tag = tags_lower
            .first()
            .cloned()
            .unwrap_or_else(|| "misc".to_string());

        scored.push(Scored {
            problem: problem.clone(),
            reason,
            score,
            primary_tag,
            rating,
        });
    }

    scored.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));

    diversify(scored, count)
}

struct Scored {
    problem: Problem,
    reason: String,
    score: f64,
    primary_tag: String,
    rating: u32,
}

/// Greedily pick the highest-scoring problems while capping how many share the
/// same primary tag or exact rating, so the list stays varied. If the caps are
/// too strict to fill `count`, the remainder is topped up from what's left.
fn diversify(scored: Vec<Scored>, count: usize) -> Vec<Recommendation> {
    if count == 0 {
        return Vec::new();
    }
    let tag_cap = (count / 3).max(3);
    let rating_cap = (count / 4).max(3);

    let mut tag_count: HashMap<String, usize> = HashMap::new();
    let mut rating_count: HashMap<u32, usize> = HashMap::new();
    let mut chosen: Vec<bool> = vec![false; scored.len()];
    let mut out: Vec<Recommendation> = Vec::new();

    for (i, s) in scored.iter().enumerate() {
        if out.len() >= count {
            break;
        }
        let t = tag_count.get(&s.primary_tag).copied().unwrap_or(0);
        let r = rating_count.get(&s.rating).copied().unwrap_or(0);
        if t >= tag_cap || r >= rating_cap {
            continue;
        }
        *tag_count.entry(s.primary_tag.clone()).or_insert(0) += 1;
        *rating_count.entry(s.rating).or_insert(0) += 1;
        chosen[i] = true;
        out.push(Recommendation {
            problem: s.problem.clone(),
            reason: s.reason.clone(),
        });
    }

    // Top up if the diversity caps left us short.
    if out.len() < count {
        for (i, s) in scored.iter().enumerate() {
            if out.len() >= count {
                break;
            }
            if !chosen[i] {
                out.push(Recommendation {
                    problem: s.problem.clone(),
                    reason: s.reason.clone(),
                });
            }
        }
    }

    out
}
