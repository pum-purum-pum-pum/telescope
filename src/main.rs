use rand::Rng;
use std::cell::{Cell, RefCell};
use std::collections::VecDeque;
use std::rc::Rc;
use std::sync::Arc;

use flame::{self, Span};
use iced::{
    button, scrollable, Background, Button, Column, Container, Element, Length, Row, Sandbox,
    Scrollable, Settings, Text,
};
use id_arena::{Arena, Id};

use misc::{generate_spans, test_spans};
use profile::Profile;
use scope::RegionTree;
use tree_view::{SubProfile, TreeView};

pub const MAX_UNITS: u16 = 1000; // max width of span (width of 0.0..1.0 span)
pub const SPACING: u16 = 1;
const SCOPE_HEIGHT: u16 = 40;
const MAX_GENERATED_DEPTH: usize = 20;

mod misc;
mod profile;
mod scope;
mod tree_profile;
mod tree_view;

pub fn main() {
    // test_spans();
    generate_spans(0, MAX_GENERATED_DEPTH);
    // dbg!(flame::spans());
    // dbg!(FlattenRegions::from_flame_spans(flame::spans()));
    // flame::dump_html(&mut File::create("flame-graph.html").unwrap()).unwrap();
    Profiler::run(Settings::default())
}

#[derive(Debug, Clone, Copy)]
pub enum ScopeMessage {
    Pressed((usize)), // (profile_id, link)
    Hovered,
    Free,
}

#[derive(Clone)]
struct ScopeTree {
    pub width: u16,
    pub desc: String,
    pub color: [f32; 3],
    pub state: button::State,
    children: Vec<ScopeTree>,
    // children: Vec<Id<ScopeTree>>,
}

impl ScopeTree {
    pub fn view(&mut self, id: usize) -> Element<ScopeMessage> {
        let cur_ptr = self as *mut ScopeTree;
        let button = Button::new(&mut self.state, Text::new(self.desc.clone()))
            .background(Background::Color(self.color.into()))
            .height(Length::Units(30))
            .width(Length::Units(self.width))
            .on_press(ScopeMessage::Pressed(id));
        let subtree = self
            .children
            .iter_mut()
            .fold(Row::new().spacing(SPACING), |row, scope| {
                row.push(scope.view(id))
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
pub enum Message {
    SubProfile(SubProfile),
    UpdateProfile(usize),
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

impl Sandbox for Profiler {
    type Message = Message;

    fn new() -> Self {
        let mut picks = VecDeque::new();
        let mut profiles = VecDeque::new();
        for i in 0..10 {
            flame::clear();
            generate_spans(0, MAX_GENERATED_DEPTH);
            picks.push_back(Pick {
                state: Default::default(),
                color: [0.5, 0.5, 0.5],
                desc: i.to_string(),
            });
            profiles.push_back(Profile::new(&flame::spans()));
        }
        Profiler {
            picks: picks,
            profiles,
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
        dbg!(message);
        match message {
            Message::SubProfile(SubProfile(node_id)) => {
                dbg!("update");
                self.profiles[self.profile_id].selected = node_id;
                self.profiles[self.profile_id].profile_view = TreeView::from_tree_profile(node_id, &mut self.profiles[self.profile_id].nodes);

            }
            Message::UpdateProfile(id) => self.profile_id = id,
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
