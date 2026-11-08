//! The order a list keeps while it is open: no list moves just because
//! something in it is chosen. A list sorted by state, such as the connected
//! device first, would move the row just pressed to another place. So a
//! list is sorted once, when it is opened or read again because the person
//! asked, and after that every row keeps its place however its state
//! changes: a row that goes leaves the rest as they were, and a new one is
//! added at the end. Settings' lists of Bluetooth devices and Wi-Fi
//! networks and the ring's quick lists all keep their order this one way.


#[derive(Default, Debug)]
pub struct KeptOrder {
    rows: Vec<String>,
}

impl KeptOrder {

    pub fn forget(&mut self) {
        self.rows.clear();
    }

    pub fn keep<T>(&mut self, items: &mut [T], id: impl Fn(&T) -> String) {
        let mut ids: Vec<String> = items.iter().map(&id).collect();
        self.rows.retain(|r| ids.contains(r));
        for i in ids.drain(..) {
            if !self.rows.contains(&i) {
                self.rows.push(i);
            }
        }
        let rows = &self.rows;
        items.sort_by_key(|t| {
            let i = id(t);
            rows.iter().position(|r| *r == i).unwrap_or(usize::MAX)
        });
    }
}

#[cfg(test)]
mod tests {
    use super::KeptOrder;

    fn sorted(mut rows: Vec<(&'static str, bool)>) -> Vec<(&'static str, bool)> {
        rows.sort_by_key(|r| (!r.1, r.0));
        rows
    }

    fn names(rows: &[(&'static str, bool)]) -> Vec<&'static str> {
        rows.iter().map(|r| r.0).collect()
    }

    #[test]
    fn a_row_keeps_its_place_when_its_state_changes() {
        let mut order = KeptOrder::default();

        let mut rows = sorted(vec![("mouse", false), ("headphones", true), ("keyboard", false)]);
        order.keep(&mut rows, |r| r.0.to_string());
        assert_eq!(names(&rows), ["headphones", "keyboard", "mouse"]);

        let mut rows = sorted(vec![("mouse", true), ("headphones", false), ("keyboard", false)]);
        assert_eq!(names(&rows), ["mouse", "headphones", "keyboard"]);
        order.keep(&mut rows, |r| r.0.to_string());
        assert_eq!(names(&rows), ["headphones", "keyboard", "mouse"], "nothing moved");
        assert!(rows[2].1, "and the row shows its new state");
    }

    #[test]
    fn new_rows_go_to_the_end_and_gone_ones_leave_the_rest() {
        let mut order = KeptOrder::default();
        let mut rows = sorted(vec![("b", false), ("a", false), ("c", false)]);
        order.keep(&mut rows, |r| r.0.to_string());
        let mut rows = sorted(vec![("c", false), ("aa", true), ("a", false)]);
        order.keep(&mut rows, |r| r.0.to_string());
        assert_eq!(names(&rows), ["a", "c", "aa"]);

        order.forget();
        let mut rows = sorted(vec![("c", false), ("aa", true), ("a", false)]);
        order.keep(&mut rows, |r| r.0.to_string());
        assert_eq!(names(&rows), ["aa", "a", "c"]);
    }
}
