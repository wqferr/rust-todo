pub struct Item {
    title: String,
    description: Option<String>,
    done: bool
}

impl Item {
    pub fn new(title: &str, desc: Option<&str>) -> Item {
        Item {
            title: String::from(title),
            description: desc.map(|x| String::from(x)),
            done: false
        }
    }

    pub fn set_done(&mut self, done_now: bool) {
        self.done = done_now;
    }
}

impl ToString for Item {
    fn to_string(&self) -> String {
        let mut result = String::new();
        if self.done {
            result.push_str("[x] ");
        } else {
            result.push_str("[ ] ");
        }

        result.push_str(&self.title);

        match &self.description {
            Some(description) => {
                result.push_str(": ");
                result.push_str(&description);
            },
            None => {}
        }
        result
    }
}
