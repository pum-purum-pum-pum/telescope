use std::collections::VecDeque;
use std::rc::Rc;
use std::cell::{RefCell, Cell};
use rand::Rng;

use iced::{
    button, scrollable, Background, Button, Column, Container, Element, Length, Row, Sandbox,
    Scrollable, Settings, Text,
};
use id_arena::{Arena, Id};
use flame::{self, Span};

use misc::{generate_spans, test_spans};
use scope::RegionTree;

type SharedArena = Rc<RefCell<Arena<ScopeTree>>>;

const MAX_UNITS: u16 = 700; // max width of span (width of 0.0..1.0 span)
const SPACING: u16 = 1;

mod misc;
mod scope;

pub fn main() {
    // test_spans();
    generate_spans(0);
    // dbg!(flame::spans());
    // dbg!(FlattenRegions::from_flame_spans(flame::spans()));
    // flame::dump_html(&mut File::create("flame-graph.html").unwrap()).unwrap();
    Profiler::run(Settings::default())
}

#[derive(Debug, Clone, Copy)]
pub enum ScopeMessage {
    Pressed,
    Hovered,
    Free,
}

#[derive(Clone)]
struct ScopeTree {
    pub width: u16,
    pub desc: String,
    pub color: [f32; 3],
    pub state: button::State,
    children: Vec<Rc<RefCell<ScopeTree>>>,
    // children: Vec<Id<ScopeTree>>,
}

impl ScopeTree {
    pub fn view(&mut self) -> Element<ScopeMessage> {
        let button = Button::new(&mut self.state, Text::new(self.desc.clone()))
            .background(Background::Color(self.color.into()))
            .height(Length::Units(30))
            .width(Length::Units(self.width))
            .on_press(ScopeMessage::Pressed);
        let subtree = self
            .children
            .iter_mut()
            .fold(Row::new().spacing(SPACING), |row, scope| {
                row.push(scope.borrow_mut().view())
            });
        let subtree_container = Container::new(subtree)
            .width(Length::Units(self.width))
            .center_x()
            .center_y();
        let column = Column::new()
            .spacing(SPACING)
            .push(button)
            .push(subtree_container);
        column.into()
        // button.into()
    }
}

#[derive(Debug, Clone, Copy)]
enum Message {
    ScopeMessage(ScopeMessage),
    UpdateProfile(usize),
}

fn from_regions(regions: &Vec<RegionTree<f64>>) -> Vec<Rc<RefCell<ScopeTree>>> {
    let mut rng = rand::thread_rng();
    regions
        .iter()
        .map(|region| {
            let children = from_regions(&region.regions);
            Rc::new(RefCell::new(ScopeTree {
                width: (MAX_UNITS as f64 * (region.end - region.start)).round() as u16,
                desc: region.desc.clone(),
                color: [
                    rng.gen_range(0., 1.),
                    rng.gen_range(0., 1.),
                    rng.gen_range(0., 1.),
                ],
                children: children,
                state: Default::default(),
            }))
        })
        .collect()
}

struct Pick {
    pub state: button::State,
    pub color: [f32; 3],
    pub desc: String,
    // pub profile: Profile,
}

impl Pick {
    fn view(&mut self, id: usize) -> Element<Message> {
        Button::new(&mut self.state, Text::new(self.desc.clone()))
            .background(Background::Color(self.color.into()))
            .height(Length::Units(100))
            .width(Length::Units(20))
            .on_press(Message::UpdateProfile(id))
            .into()
    }
}

struct Profiler {
    picks: VecDeque<Pick>,
    profiles: VecDeque<Profile>,
    profile_id: usize,
}

#[derive(Clone)]
struct Profile {
    scroll: scrollable::State,
    root: ScopeTree,
    nodes: Arena<ScopeTree>,
}

impl Profile {
    pub fn new(spans: &Vec<Span>) -> Self {
        let regions = RegionTree::from_flame(spans);
        let mut nodes = Arena::new();
        // let nodes = Rc::new(RefCell::new(nodes));
        let tree = from_regions(&regions);
        Profile {
            scroll: Default::default(),
            root: ScopeTree {
                width: MAX_UNITS,
                desc: "root".to_string(),
                color: [0., 0., 0.],
                state: Default::default(),
                children: tree,
            },
            nodes: nodes
        }
    }

    // fn view_node<'a>(&'a self, node: Id<ScopeTree>) -> Element<'a, Message> {
    //     let desc = self.nodes.borrow()[node].desc.clone();
    //     let width = self.nodes.borrow()[node].width;
    //     // let color = self.nodes[node].color.into();
    //     let mut nodes = self.nodes.borrow_mut();
    //     let button = nodes[node].view()
    //         .map(move |message| Message::ScopeMessage(message));
    //     let subtree = self.nodes.borrow_mut()[node]
    //         .children
    //         .iter_mut()
    //         .fold(Row::new().spacing(SPACING), |row, scope| {
    //             row.push(self.view_node(*scope))//nodes.borrow_mut()[*scope].view())
    //         });
    //     let subtree_container = Container::new(subtree)
    //         .width(Length::Units(width))
    //         .center_x()
    //         .center_y();
    //     let column = Column::new()
    //         .spacing(SPACING)
    //         .push(button)
    //         .push(subtree_container);
    //     column.into()
    // }

    fn view(&mut self) -> Element<Message> {
        let content = self
            .root
            .view()
            .map(move |message| Message::ScopeMessage(message));
        Scrollable::new(&mut self.scroll)
            .push(Container::new(content).width(Length::Fill).center_x())
            .into()
    }
}

impl Sandbox for Profiler {
    type Message = Message;

    fn new() -> Self {
        let mut picks = VecDeque::new();
        let mut profiles = VecDeque::new();
        for i in 0..10 {
            flame::clear();
            generate_spans(0);
            picks.push_back(Pick {
                state: Default::default(),
                color: [0.5, 0.5, 0.5],
                desc: i.to_string(),
            });
            profiles.push_back(Profile::new(&flame::spans()));
        }
        Profiler {
            picks: picks,
            profiles: profiles,
            profile_id: 0,
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
            Message::UpdateProfile(id) => {
                self.profile_id = id
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        let picks = self
            .picks
            .iter_mut()
            .enumerate()
            .fold(Row::new().spacing(SPACING), |row, pick| {
                row.push(pick.1.view(pick.0))
            });
        Column::new()
            .push(
                Container::new(picks)
                    .width(Length::Units(MAX_UNITS))
                    .center_x()
                    .center_y(),
            )
            .push(self.profiles[self.profile_id].view())
            .into()
    }
}
