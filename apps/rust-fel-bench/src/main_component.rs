use crate::handle;
use crate::table::{Table, TableProps};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Default, Clone)]
pub struct Main {
    id: String,
    state: i32,
    counter: usize,
    child: handle::Handle<Table>,
}

impl Main {
    pub fn create() -> handle::Handle<Self> {
        let main = Main {
            id: "main".to_owned(),
            counter: 0,
            state: 0,
            child: Table::create(),
        };
        handle::Handle(Rc::new(RefCell::new(main)))
    }
}

impl rust_fel::Component for handle::Handle<Main> {
    type Properties = ();
    type Message = i32;
    type State = ();

    fn add_props(&mut self, _props: Self::Properties) {}

    fn reduce_state(&mut self, message: i32) {
        self.0.borrow_mut().state = message;
        self.0.borrow_mut().counter += 1;
        let mut child = self.0.borrow_mut().child.clone();
        child.add_props(TableProps {
            rows: message,
            counter: self.0.borrow_mut().counter,
        })
    }

    fn render(&self) -> rust_fel::Element {
        let borrow = self.0.borrow_mut();
        let child = borrow.child.clone();
        let heading = rust_fel::html("<h1>rust-fel bench</h1>".to_owned());

        let mut clone = self.clone();
        let create_k_button = rust_fel::Element::new(
            "button".to_owned(),
            rust_fel::Props {
                id: Some("create1000".to_owned()),
                text: Some("Create 1K".to_owned()),
                on_click: Some(Box::new(move || clone.reduce_state(1000))),
                ..Default::default()
            },
        );

        let mut clone = self.clone();
        let create_ten_k_button = rust_fel::Element::new(
            "button".to_owned(),
            rust_fel::Props {
                id: Some("create10000".to_owned()),
                text: Some("Create 10K".to_owned()),
                on_click: Some(Box::new(move || clone.reduce_state(10000))),
                ..Default::default()
            },
        );

        let mut clone = self.clone();
        let clear = rust_fel::Element::new(
            "button".to_owned(),
            rust_fel::Props {
                id: Some("clear".to_owned()),
                text: Some("Clear".to_owned()),
                on_click: Some(Box::new(move || clone.reduce_state(0))),
                ..Default::default()
            },
        );

        let header = rust_fel::Element::new(
            "header".to_owned(),
            rust_fel::Props {
                children: Some(vec![heading, create_k_button, create_ten_k_button, clear]),
                ..Default::default()
            },
        );

        rust_fel::Element::new(
            "div".to_owned(),
            rust_fel::Props {
                id: Some(borrow.id.clone()),
                class_name: Some("main".to_owned()),
                children: Some(vec![header, child.render()]),
                ..Default::default()
            },
        )
    }
}
