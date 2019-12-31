use crate::tree_profile::Scope;
use crate::tree_profile::TreeProfile;
use crate::{MAX_UNITS, SCOPE_HEIGHT, SPACING};

use crate::BUTTON_BORDER_RADIUS;
use iced::{button, Background, Button, Column, Container, Element, Length, Row, Text};
use id_arena::{Arena, Id};

#[derive(Debug, Clone, Copy)]
pub struct SubProfile(pub Id<TreeProfile>);

#[derive(Clone)]
pub struct TreeView {
    pub profile_tree: Id<TreeProfile>,
    pub state: button::State,
    pub data: Scope,
    pub children: Vec<TreeView>,
}

impl TreeView {
    pub fn view(&mut self) -> Element<SubProfile> {
        let button = Button::new(&mut self.state, Text::new(self.data.desc.clone()))
            .background(Background::Color(self.data.color.into()))
            .height(Length::Units(SCOPE_HEIGHT))
            .width(Length::Units(self.data.width))
            .border_radius(BUTTON_BORDER_RADIUS)
            .on_press(SubProfile(self.profile_tree));
        let subtree = self
            .children
            .iter_mut()
            .fold(Row::new().spacing(SPACING), |row, scope| {
                row.push(scope.view())
            });
        let subtree_container = Container::new(subtree)
            .width(Length::Units(self.data.width))
            .center_x()
            .center_y();
        let column = Column::new()
            .spacing(SPACING)
            .push(subtree_container)
            .push(button);
        column.into()
    }

    pub fn renormalized_tree(
        tree: Id<TreeProfile>,
        nodes: &Arena<TreeProfile>,
        scale: f32,
    ) -> TreeView {
        let mut data = nodes[tree].data.clone();
        data.width = (data.width as f32 * scale) as u16;
        TreeView {
            profile_tree: tree,
            state: Default::default(),
            data,
            children: nodes[tree]
                .children
                .iter()
                .map(|child| TreeView::renormalized_tree(*child, nodes, scale))
                .collect(),
        }
    }

    pub fn from_tree_profile(tree: Id<TreeProfile>, nodes: &Arena<TreeProfile>) -> TreeView {
        let scale = MAX_UNITS as f32 / nodes[tree].data.width as f32;
        TreeView::renormalized_tree(tree, nodes, scale)
    }
}
