use iced::{
    button, Background, Button, Column, Container, Element, Length, Row, Sandbox, Settings, Text, Scrollable, scrollable
};
use rand::Rng;

const MAX_DEPTH: usize = 100;
const MAX_UNITS: u16 = 700; // max width of span (width of 0.0..1.0 span)
const BACKGROUND_COLOR: [f32; 3] = [1., 1., 1.];
const SPACING: u16 = 1;
use flame::{self, Span};
// use scope::Region;
use misc::{generate_spans, test_spans};
use scope::RegionTree;
use std::fs::File;

mod misc;
mod scope;

pub fn main() {
    // test_spans();
    generate_spans(0);
    // dbg!(flame::spans());
    // dbg!(FlattenRegions::from_flame_spans(flame::spans()));
    // flame::dump_html(&mut File::create("flame-graph.html").unwrap()).unwrap();
    Profile::run(Settings::default())
}

#[derive(Debug, Clone, Copy)]
pub enum ScopeMessage {
    Pressed,
    Hovered,
    Free,
}

struct ScopeTree {
    pub width: u16,
    pub desc: String,
    pub color: [f32; 3],
    pub state: button::State,
    children: Vec<ScopeTree>,
}

impl ScopeTree {
    pub fn view(&mut self) -> Element<ScopeMessage> {
        let button = Button::new(&mut self.state, Text::new(self.desc.clone()))
            .background(Background::Color(self.color.into()))
            .height(Length::Units(40))
            .width(Length::Units(self.width))
            .on_press(ScopeMessage::Pressed);
        let subtree = self
            .children
            .iter_mut()
            .fold(Row::new().spacing(SPACING), |row, scope| {
                row.push(scope.view())
            });
        let subtree_container = Container::new(subtree).width(Length::Units(self.width)). center_x().center_y();
        let column = Column::new()
            .spacing(SPACING)
            .push(button)
            .push(subtree_container);
        column.into()
    }
}


#[derive(Debug, Clone, Copy)]
enum Message {
    ScopeMessage(ScopeMessage),
}

fn from_regions(regions: &Vec<RegionTree<f64>>) -> Vec<ScopeTree> {
    let mut rng = rand::thread_rng();
    regions
        .iter()
        .map(|region| ScopeTree {
            width: (MAX_UNITS as f64 * (region.end - region.start)).round() as u16,
            desc: "123".to_string(),
            color: [rng.gen_range(0., 1.), rng.gen_range(0., 1.), rng.gen_range(0., 1.)],
            children: from_regions(&region.regions),
            state: Default::default(),
        })
        .collect()
}

struct Profile {
    scroll: scrollable::State,
    root: ScopeTree
}

impl Sandbox for Profile {
    type Message = Message;

    fn new() -> Self {
        let regions = RegionTree::from_flame(&flame::spans());
        Profile {
            scroll: Default::default(),
            root: ScopeTree {
                width: MAX_UNITS,
                desc: "root".to_string(),
                color: [0., 0., 0.],
                state: Default::default(),
                children: from_regions(&regions),
            }
        }
    }

    fn title(&self) -> String {
        String::from("Profiler")
    }

    fn update(&mut self, message: Message) {
        // flame::clear();
        // generate_spans(0);
        // let regions = RegionTree::from_flame(&flame::spans());
        // self.root.children = from_regions(&regions);
        match message {
            Message::ScopeMessage(_) => {}
        }
    }

    fn view(&mut self) -> Element<Message> {
        let content = self.root
            .view()
            .map(move |message| Message::ScopeMessage(message));
        Scrollable::new(&mut self.scroll)
            .push(
                Container::new(content).width(Length::Fill).center_x(),
            )
            .into()
    }
}
