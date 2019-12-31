use flame::{self, Span};
use iced::{button, scrollable, Background, Button, Container, Element, Length, Scrollable, Text};
use id_arena::{Arena, Id};
use rand::Rng;

use crate::scope::RegionTree;
use crate::tree_profile::{Scope, TreeProfile};
use crate::tree_view::{SubProfile, TreeView};
use crate::{Message, MAX_UNITS, SCOPE_HEIGHT};
const ROOT_COLOR: [f32; 3] = [0.5, 0.5, 0.5];

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
    regions: &[RegionTree<f64>],
    parent: Option<Id<TreeProfile>>,
    nodes: &mut Arena<TreeProfile>,
) -> Vec<Id<TreeProfile>> {
    let mut rng = rand::thread_rng();
    regions
        .iter()
        .map(|region| {
            let width = (MAX_UNITS as f64 * (region.end - region.start)).round() as u16;
            let temperature = if let Some(parent) = parent {
                width as f32 / nodes[parent].data.width as f32
            } else {
                1.0
            };
            let current_node = nodes.alloc(TreeProfile {
                data: Scope {
                    width,
                    desc: region.desc.clone(),
                    color: [
                        temperature, // rng.gen_range(0., 1.),
                        0.7 * rng.gen_range(0., 1.),
                        1. - temperature,
                    ],
                },
                children: vec![],
            });
            let children = from_regions(&region.regions, Some(current_node), nodes);
            nodes[current_node].children = children;
            current_node
        })
        .collect()
}

impl Profile {
    pub fn new(spans: &[Span]) -> Self {
        let regions = RegionTree::from_flame(spans);
        let mut nodes = Arena::new();
        let tree = from_regions(&regions, None, &mut nodes);
        let root = nodes.alloc(TreeProfile {
            data: Scope {
                width: MAX_UNITS,
                desc: "root".to_string(),
                color: ROOT_COLOR,
            },
            children: tree,
        });
        let profile_view = TreeView::from_tree_profile(root, & nodes);
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
            .background(Background::Color(ROOT_COLOR.into()))
            .height(Length::Units(SCOPE_HEIGHT))
            .width(Length::Units(MAX_UNITS))
            .on_press(Message::SubProfile(SubProfile(self.root)));
        // TODO update lazely
        // self.profile_view = TreeView::from_tree_profile(self.selected, &mut self.nodes);
        let content = self
            .profile_view
            .view()
            .map(Message::SubProfile);
        let mut res = Scrollable::new(&mut self.scroll)
            .max_height(500)
            .push(Container::new(content).width(Length::Fill).center_x());
        if self.selected != self.root {
            res = res.push(button)
        };
        res.into()
    }
}
