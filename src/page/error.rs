#[derive(Debug, Clone)]
pub struct PageError {
    pub index: usize,
    pub error: ErrorType,
}

impl PageError {
    pub fn japanese(index: usize) -> Self {
        PageError {
            index,
            error: ErrorType::Japanese,
        }
    }

    pub fn size(index: usize, size: usize) -> Self {
        PageError {
            index,
            error: ErrorType::Size(size),
        }
    }

    pub fn repeat(index: usize) -> Self {
        PageError {
            index,
            error: ErrorType::Repeat,
        }
    }

    pub fn copy(index: usize) -> Self {
        PageError {
            index,
            error: ErrorType::Copy,
        }
    }

    pub fn image(index: usize) -> Self {
        PageError {
            index,
            error: ErrorType::Image,
        }
    }

    pub fn header(index: usize) -> Self {
        PageError {
            index,
            error: ErrorType::Header,
        }
    }

    pub fn link(index: usize) -> Self {
        PageError {
            index,
            error: ErrorType::Link,
        }
    }

    pub fn end(index: usize) -> Self {
        PageError {
            index,
            error: ErrorType::End,
        }
    }

    /// Whether the check that found this error read section `part`'s
    /// translation, so that translating `part` again can change it. The Copy
    /// check compares a section with every section before it; the others
    /// read only their own section.
    pub fn depends_on(&self, part: usize) -> bool {
        self.index == part || (self.index > part && matches!(self.error, ErrorType::Copy))
    }
}

#[derive(Debug, Clone)]
pub enum ErrorType {
    Japanese,
    Size(usize),
    Repeat,
    Copy,
    Image,
    Header,
    Link,
    End,
}

#[cfg(test)]
mod test {
    use super::PageError;

    #[test]
    fn only_own_errors_and_later_copies_depend_on_part() {
        let part = 3;
        assert!(PageError::size(part, 10).depends_on(part));
        assert!(PageError::copy(part).depends_on(part));
        assert!(PageError::copy(part + 1).depends_on(part));

        assert!(!PageError::copy(part - 1).depends_on(part));
        assert!(!PageError::japanese(part + 1).depends_on(part));
        assert!(!PageError::size(part - 1, 10).depends_on(part));
    }
}
