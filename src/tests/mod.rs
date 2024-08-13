#[cfg(test)]
mod tests {
    use super::super::services::bucket_service::solve_bucket_problem;

    #[test]
    fn test_solve_bucket_problem_basic() {
        let result = solve_bucket_problem(2, 10, 4);
        assert!(result.is_some());
        let solution = result.unwrap();
        assert_eq!(solution.solution.len(), 4);
        assert_eq!(solution.solution[0].action, "fill_bucket_x");
        assert_eq!(solution.solution[3].action, "transfer_from_bucket_x_to_y");
    }

    #[test]
    fn test_no_solution_possible() {
        let result = solve_bucket_problem(2, 6, 5);
        assert!(result.is_none());
    }

    #[test]
    fn test_larger_buckets() {
        let result = solve_bucket_problem(2, 100, 96);
        assert!(result.is_some());
        let solution = result.unwrap();
        assert!(solution.solution.len() < 10);
    }

    #[test]
    fn test_identical_buckets() {
        let result = solve_bucket_problem(5, 5, 5);
        assert!(result.is_some());
        let solution = result.unwrap();
        assert_eq!(solution.solution.len(), 1);
    }
}
