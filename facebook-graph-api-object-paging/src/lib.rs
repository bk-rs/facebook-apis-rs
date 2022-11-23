//! https://developers.facebook.com/docs/graph-api/results

// https://developers.facebook.com/docs/graph-api/results#cursors
pub mod cursor_based_pagination {
    use serde::{Deserialize, Serialize};

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Paging {
        pub cursors: PagingCursors,
        pub previous: Option<String>,
        pub next: Option<String>,
    }

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct PagingCursors {
        pub before: Option<Box<str>>,
        pub after: Option<Box<str>>,
    }

    impl Paging {
        pub fn next_cursor(&self) -> Option<String> {
            if self.next.is_some() {
                self.cursors.after.as_ref().map(|after| after.to_string())
            } else {
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_de_paging_for_cursor_based_pagination() {
        let paging: cursor_based_pagination::Paging = serde_json::from_str(
            r#"{
                "cursors": {
                    "before": "QVFIUnk4X3VfWlhjc1ZA6R2FPNWJhMDNFWm90Qm1mT2hKUkF5NW1aa25hbWJiY0lnT1o3SWtKcW9UckxhcGR1YnBTd2x1aTktSWlrME4tUFljWU5tRTIwNldR",
                    "after": "QVFIUktSN2czMUNMNUxHWmJGYWkzUFBKTjNObWlaVHEzYTBua1ZA2X1BDaEM0bXhGdkhGZAkxkeDlrX2tkRkpQclRUWjlSaEtLZAWV3TlRPeUoyQVJ1VFpYTjR3"
                }
            }"#,
        ).unwrap();
        assert_eq!(paging.next_cursor(), None);
    }
}
