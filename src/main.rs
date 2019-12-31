use rand::Rng;
use std::collections::VecDeque;

use flame;
use iced::{
    button, Background, Button, Column, Container, Element, Length, Row, Sandbox, Settings, Text,
};

use misc::generate_spans;
use profile::Profile;
use tree_view::{SubProfile, TreeView};

pub const MAX_UNITS: u16 = 1000; // max width of span (width of 0.0..1.0 span)
pub const SPACING: u16 = 1;
const SCOPE_HEIGHT: u16 = 30;
const MAX_GENERATED_DEPTH: usize = 15;
pub const BUTTON_BORDER_RADIUS: u16 = 5;

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
pub enum Message {
    SubProfile(SubProfile),
    UpdateProfile(usize),
}

struct Pick {
    pub state: button::State,
    pub color: [f32; 3],
    pub desc: String,
    pub height: f32,
    // pub profile: Profile,
}

impl Pick {
    fn view(&mut self, id: usize) -> Element<Message> {
        Button::new(&mut self.state, Text::new(self.desc.clone()).size(10))
            .background(Background::Color(self.color.into()))
            .height(Length::Units((100f32 * self.height) as u16))
            .width(Length::Units(20))
            .on_press(Message::UpdateProfile(id))
            .border_radius(BUTTON_BORDER_RADIUS)
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
        let mut rng = rand::thread_rng();
        for i in 0..40 {
            flame::clear();
            generate_spans(0, MAX_GENERATED_DEPTH);
            let height = rng.gen_range(0.5, 1.);
            picks.push_back(Pick {
                state: Default::default(),
                color: [height, 0.5, 0.5],
                desc: i.to_string(),
                height,
            });
            profiles.push_back(Profile::new(&flame::spans()));
        }
        Profiler {
            picks,
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
                self.profiles[self.profile_id].profile_view =
                    TreeView::from_tree_profile(node_id, &self.profiles[self.profile_id].nodes);
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
