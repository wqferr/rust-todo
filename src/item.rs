#[derive(Debug)]
pub struct Item {
    title: String,
    description: Option<String>,
    done: bool,

    str_repr: String
}

impl Item {
    pub fn new(title: &str, desc: Option<&str>) -> Item {
        let mut new_item = Item {
            title: String::from(title),
            description: desc.map(|x| String::from(x)),
            done: false,
            str_repr: String::new()
        };
        new_item.update_str_repr();
        new_item
    }

    pub fn set_done(&mut self, done_now: bool) {
        self.done = done_now;
        self.update_str_repr();
    }

    pub fn done(&self) -> bool {
        self.done
    }

    fn update_str_repr(&mut self) {
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
        self.str_repr = result;
    }
}

impl ToString for Item {
    fn to_string(&self) -> String {
        String::clone(&self.str_repr)
    }
}
