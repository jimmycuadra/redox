#![allow(dead_code)]

struct ExampleGroup {
    name: &'static str,
    examples: Vec<Example>,
}

struct Example {
    name: &'static str,
    body: Box<Fn() -> bool>,
}

impl ExampleGroup {
    pub fn it<B: Fn() -> bool + 'static>(&mut self, name: &'static str, body: B) {
        self.examples.push(Example {
            name: name,
            body: Box::new(body),
        });
    }

    pub fn run(&self) {
        for example in self.examples.iter() {
            let result = example.run();

            println!("{}: {}", self.format_result(result), self.example_name(example));
        }
    }

    fn example_name(&self, example: &Example) -> String {
        format!("{} {}", self.name, example.name)
    }

    fn format_result(&self, result: bool) -> &'static str {
        if result {
            "PASSED"
        } else {
            "FAILED"
        }
    }
}

impl Example {
    pub fn run(&self) -> bool {
        (self.body)()
    }
}

pub fn describe<B: Fn(&mut ExampleGroup)>(name: &'static str, body: B) {
    let example_group = &mut ExampleGroup {
        name: name,
        examples: vec![],
    };

    body(example_group);

    example_group.run();
}
