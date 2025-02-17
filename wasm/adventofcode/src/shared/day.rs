#[macro_export]
macro_rules! define_day_mod {
    () => {
        pub mod solution1;
        pub mod solution2;

        pub(super) fn retrieve_problem(problem: u8) -> String {
            match problem {
                1 => include_str!("problem1.txt").to_string(),
                2 => include_str!("problem2.txt").to_string(),
                _ => panic!("Problem not found"),
            }
        }

        #[cfg(not(target_arch = "wasm32"))]
        pub(super) fn retrieve_code(problem: u8) -> String {
            match problem {
                1 => include_str!("solution1.rs").to_string(),
                2 => include_str!("solution2.rs").to_string(),
                _ => panic!("Solution not found"),
            }
        }

        #[cfg(target_arch = "wasm32")]
        pub(super) fn retrieve_html(problem: u8, darkmode: bool) -> String {
            match darkmode {
                true => retrieve_dark_html(problem),
                false => retrieve_light_html(problem),
            }
        }

        #[cfg(target_arch = "wasm32")]
        fn retrieve_dark_html(problem: u8) -> String {
            match problem {
                1 => include_str!("solution1-dark.html").to_string(),
                2 => include_str!("solution2-dark.html").to_string(),
                _ => panic!("Html not found"),
            }
        }

        #[cfg(target_arch = "wasm32")]
        fn retrieve_light_html(problem: u8) -> String {
            match problem {
                1 => include_str!("solution1-light.html").to_string(),
                2 => include_str!("solution2-light.html").to_string(),
                _ => panic!("Html not found"),
            }
        }

        pub(super) fn solve(input: &str, problem: u8) -> String {
            match problem {
                1 => solution1::solve(input),
                2 => solution2::solve(input),
                _ => panic!("Solution not found"),
            }
        }

        #[cfg(not(target_arch = "wasm32"))]
        pub(super) fn input() -> String {
            include_str!("input.txt").to_string()
        }
    };
}
