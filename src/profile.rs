use flame::{self, Span};
use iced::{
    button, scrollable, Background, Button, Column, Container, Element, Length, Row, Sandbox,
    Scrollable, Settings, Text,
};
use id_arena::{Arena, Id};
use rand::Rng;

use crate::scope::RegionTree;
use crate::tree_profile::{Scope, TreeProfile};
use crate::tree_view::{TreeView, SubProfile};
use crate::{Message, MAX_UNITS, SCOPE_HEIGHT};
const MAX_PICKS: usize = 1000;

#[derive(Clone)]
pub struct Profile {
    scroll: scrollable::State,
    pub root_state: button::State,
    root: Id<TreeProfile>,
    pub nodes: Arena<TreeProfile>,
    pub selected: Id<TreeProfile>,
    pub profile_view: TreeView,
}

fn from_regions(
    regions: &Vec<RegionTree<f64>>,
    nodes: &mut Arena<TreeProfile>,
) -> Vec<Id<TreeProfile>> {
    let mut rng = rand::thread_rng();
    regions
        .iter()
        .map(|region| {
            let children = from_regions(&region.regions, nodes);
            nodes.alloc(TreeProfile {
                data: Scope {
                    width: (MAX_UNITS as f64 * (region.end - region.start)).round() as u16,
                    desc: region.desc.clone(),
                    color: [
                        rng.gen_range(0., 1.),
                        rng.gen_range(0., 1.),
                        rng.gen_range(0., 1.),
                    ],
                },
                children: children,
            })
        })
        .collect()
}

impl Profile {
    pub fn new(spans: &Vec<Span>) -> Self {
        let regions = RegionTree::from_flame(spans);
        let mut nodes = Arena::new();
        let tree = from_regions(&regions, &mut nodes);
        let root = nodes.alloc(TreeProfile {
            data: Scope {
                width: MAX_UNITS,
                desc: "root".to_string(),
                color: [0., 0., 0.],
            },
            children: tree,
        });
        let profile_view = TreeView::from_tree_profile(root, &mut nodes);
        Profile {
            scroll: Default::default(),
            root_state: Default::default(),
            root,
            nodes,
            profile_view,
            selected: root,
        }
    }

    pub fn view(&mut self) -> Element<Message> {
        let button = Button::new(&mut self.root_state, Text::new("Root"))
            .background(Background::Color([0.5, 0.5, 0.5].into()))
            .height(Length::Units(SCOPE_HEIGHT))
            .width(Length::Units(MAX_UNITS))
            .on_press(Message::SubProfile(SubProfile(self.root)));
        // TODO update lazely
        // self.profile_view = TreeView::from_tree_profile(self.selected, &mut self.nodes);
        let content = self
            .profile_view
            .view()
            .map(move |message| Message::SubProfile(message));
        let mut res = Scrollable::new(&mut self.scroll);
        if self.selected != self.root {
            res = res
            .push(button)
        };
        res
            .push(Container::new(content).width(Length::Fill).center_x())
            .into()
    }
}
