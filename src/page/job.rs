use crate::page::Page;

/// One step of a run over the pages of a book, taken from the front of a queue.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Job {
    Page(usize),
    Part {
        page: usize,
        part: usize,
    },
    /// Shows the completion dialog. Only queued at the end of a full run.
    Finish,
}

impl Job {
    /// Every page from `first` to the end, then [`Job::Finish`].
    pub fn run_from(first: usize, len: usize) -> impl Iterator<Item = Job> {
        (first..len).map(Job::Page).chain([Job::Finish])
    }

    pub fn error_run_page(index: usize, page: &Page) -> Vec<Job> {
        page.error_indexes()
            .into_iter()
            .map(move |part| Job::Part { page: index, part })
            .collect()
    }

    pub fn error_run_from(pages: &[Page]) -> Vec<Job> {
        pages
            .iter()
            .enumerate()
            .map(|(page, p)| {
                p.error_indexes()
                    .into_iter()
                    .map(move |part| Job::Part { page, part })
            })
            .flatten()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_run_covers_the_rest_of_the_book_then_finishes() {
        let jobs: Vec<_> = Job::run_from(1, 3).collect();
        assert_eq!(jobs, [Job::Page(1), Job::Page(2), Job::Finish]);
    }

    #[test]
    fn a_run_past_the_end_only_finishes() {
        let jobs: Vec<_> = Job::run_from(5, 3).collect();
        assert_eq!(jobs, [Job::Finish]);
    }
}
