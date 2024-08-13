use std::collections::VecDeque;
use cached::proc_macro::cached;
use crate::models::bucket::{BucketStep, BucketResponse, State};

#[cached]
pub fn solve_bucket_problem(x_capacity: i32, y_capacity: i32, z_goal: i32) -> Option<BucketResponse> {
    let mut visited = vec![vec![false; y_capacity as usize + 1]; x_capacity as usize + 1];
    let mut queue = VecDeque::new();

    queue.push_back(State::new(0, 0, Vec::new()));
    visited[0][0] = true;

    while let Some(current) = queue.pop_front() {
        if current.x == z_goal || current.y == z_goal {
            let solution = current.steps.clone();
            return Some(BucketResponse {
                solution,
                status: "solved".to_string(),
            });
        }

        let new_states = vec![
            State::new(x_capacity, current.y, append_step(&current.steps, current.x, current.y, "fill_bucket_x")),
            State::new(current.x, y_capacity, append_step(&current.steps, current.x, current.y, "fill_bucket_y")),
            State::new(0, current.y, append_step(&current.steps, current.x, current.y, "empty_bucket_x")),
            State::new(current.x, 0, append_step(&current.steps, current.x, current.y, "empty_bucket_y")),
            State::new(
                (current.x - (y_capacity - current.y).min(current.x)).max(0),
                (current.y + current.x).min(y_capacity),
                append_step(&current.steps, current.x, current.y, "transfer_from_bucket_x_to_y"),
            ),
            State::new(
                (current.x + current.y).min(x_capacity),
                (current.y - (x_capacity - current.x).min(current.y)).max(0),
                append_step(&current.steps, current.x, current.y, "transfer_from_bucket_y_to_x"),
            ),
        ];

        for new_state in new_states {
            if !visited[new_state.x as usize][new_state.y as usize] {
                visited[new_state.x as usize][new_state.y as usize] = true;
                queue.push_back(new_state);
            }
        }
    }

    None
}

fn append_step(steps: &Vec<BucketStep>, x: i32, y: i32, action: &str) -> Vec<BucketStep> {
    let mut new_steps = steps.clone();
    new_steps.push(BucketStep {
        step: new_steps.len() + 1,
        bucket_x: x,
        bucket_y: y,
        action: action.to_string(),
    });
    new_steps
}
