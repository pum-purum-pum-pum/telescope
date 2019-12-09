use iced::{button, Background, Button, Column, Element, Length, Row, Sandbox, Settings, Text};
use rand::{Rng};
use std::{thread, time};
// use serde::{Deserialize, Serialize};
const MAX_DEPTH: usize = 30;
const MAX_UNITS: u16 = 1000; // max width of span (width of 0.0..1.0 span)
const BACKGROUND_COLOR: [f32; 3] = [1., 1., 1.];
const SPACING: u16 = 1;
use flame::{self, Span};
use scope::Region;

mod scope;

#[derive(Debug)]
struct FlattenRegions {
    pub scopes: Vec<Vec<Region<f64>>>,
}

impl Default for FlattenRegions {
    fn default() -> Self {
        FlattenRegions {
            scopes: (0..MAX_DEPTH).map(|_| Vec::new()).collect(),
        }
    }
}

impl FlattenRegions {
    fn from_flame_spans(spans: Vec<Span>) -> Self {
        // first traverse depth for normalization parameters
        let mut start = std::u64::MAX;
        let mut end = 0u64;
        for span in spans.iter() {
            start = start.min(span.start_ns);
            end = end.max(span.end_ns);
        }
        dbg!(start, end);
        let normalization = (end - start) as f64;
        let mut last_spans = spans;
        let mut result_scopes: Vec<Vec<Region<f64>>> = (0..MAX_DEPTH).map(|_| Vec::new()).collect();
        for depth in 0..MAX_DEPTH {
            let mut new_level_spans = vec![];
            for span in last_spans.iter() {
                let start = (span.start_ns - start) as f64 / normalization;
                let end = start + (span.end_ns - span.start_ns) as f64 / normalization;
                result_scopes[depth].push(Region { start, end });
                // inefficient clone here. possible to store just reference
                new_level_spans.extend(span.children.iter().map(|span| span.clone()));
            }
            last_spans = new_level_spans;
        }
        FlattenRegions {
            scopes: result_scopes,
        }
    }
}

pub fn main() {
    test_spans();
    // generate_spans(0);
    dbg!(flame::spans());
    dbg!(FlattenRegions::from_flame_spans(flame::spans()));
    Profile::run(Settings::default())
}

pub fn dummy_sleep(millis: u64) {
    let ten_millis = time::Duration::from_millis(millis);
    thread::sleep(ten_millis);
}

pub fn generate_spans(depth: usize) {
    let mut rng = rand::thread_rng();
    if depth == 15 {
        let p = 0.7;
        if rng.gen_range(0.0, 1.0) > 1. - p {
            dummy_sleep(10);
        }
        return
    }
    if depth > 10 {
        let p = 0.8;
        if rng.gen_range(0.0, 1.0) > 1. - p {
            return
        }
    }
    for i in 0..rng.gen_range(1, 3) {
        let name = format!("span_{}_{}", depth, i);
        flame::start(name.clone());
        generate_spans(depth + 1);
        flame::end(name.clone());
    }
}

pub fn test_spans() {
    flame::start("all");
    dummy_sleep(10);
    {
        flame::start("inside1");
        dummy_sleep(20);
        flame::end("inside1");
        flame::start("inside2");
        dummy_sleep(40);
        {
            flame::start("deep_inside1");
            dummy_sleep(20);
            flame::end("deep_inside1");
            flame::start("deep_inside2");
            dummy_sleep(50);
            flame::end("deep_inside2");
        }
        flame::end("inside2");
    }
    flame::end("all");
}

#[derive(Debug, Clone, Copy)]
pub enum ScopeMessage {
    Pressed,
    Hovered,
    Free,
}

#[derive(Debug, Default)]
struct Scope {
    pub width: u16,
    pub desc: String,
    pub color: [f32; 3],
    pub state: button::State,
}

impl Scope {
    pub fn view(&mut self) -> Element<ScopeMessage> {
        Button::new(&mut self.state, Text::new(self.desc.clone()))
            .background(Background::Color(self.color.into()))
            // .padding(0)
            .height(Length::Units(15))
            .width(Length::Units(self.width))
            .on_press(ScopeMessage::Pressed)
            .into()
    }
}

#[derive(Debug, Default)]
struct LevelScope {
    pub scopes: Vec<Scope>
}

impl LevelScope {
    pub fn view(&mut self) -> Element<Message> {
        let scopes: Element<_> = self
            .scopes
            .iter_mut()
            .enumerate()
            .fold(Row::new().spacing(SPACING), |column, (_, scope)| {
                column.push(
                    scope
                        .view()
                        .map(move |message| Message::ScopeMessage(message)),
                )
            })
            .into();
        scopes
    }
}

#[derive(Default)]
struct Profile {
    value: i32,
    test_buttons: [Scope; 20],
    scopes: Vec<LevelScope>, // regions: FlattenRegions,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    ScopeMessage(ScopeMessage),
}

impl Sandbox for Profile {
    type Message = Message;

    fn new() -> Self {
        let mut scopes: Vec<LevelScope> = (0..MAX_DEPTH).map(|_| Default::default()).collect();
        let regions = FlattenRegions::from_flame_spans(flame::spans()).scopes;
        for i in 0..MAX_DEPTH {
            let mut s = vec![];
            let mut start = 0.;
            for region in regions[i].iter() {
                if ((MAX_UNITS as f64 * (region.start - start)).round() as u16) > SPACING {
                    // spacing
                    s.push(Scope {
                        width: (MAX_UNITS as f64 * (region.start - start)).round() as u16 - SPACING,// TODO SPACING min size
                        color: BACKGROUND_COLOR,
                        ..Default::default()
                    })
                }
                // dbg!( MAX_UNITS as f64 * (region.end - region.start));
                start = region.end;
                if (MAX_UNITS as f64 * (region.end - region.start)).round() as u16 > SPACING {
                    s.push(Scope {
                        width: (MAX_UNITS as f64 * (region.end - region.start)).round() as u16 - SPACING,// TODO SPACING min size
                        color: [0.11, 0.42, 0.87],
                        ..Default::default()
                    });
                }
            }
            scopes[i] = LevelScope{scopes: s};
            // scopes[i] = LevelScope {
            // 	scopes: regions[i]
	           //      .iter()
	           //      .map(|region| Scope {
	           //          width: (MAX_UNITS as f64 * (region.end - region.start)) as u16,
            //             color: [0.11, 0.42, 0.87],
	           //          ..Default::default()
	           //      })
	           //      .collect()
	           //  }
        }
        Self {
        	scopes,
            ..Default::default()
        }
    }

    fn title(&self) -> String {
        String::from("A simple counter")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::ScopeMessage(_) => {}
        }
    }

    fn view(&mut self) -> Element<Message> {
        // let mut profile = Column::new();
        // for i in 0..MAX_DEPTH {
        // 	// let level = 
	       //  // let scopes: Element<_> = self
	       //  //     .scopes[i]
	       //  //     .iter_mut()
	       //  //     .enumerate()
	       //  //     .fold(Row::new().spacing(2), |column, (i, scope)| {
	       //  //         column.push(
	       //  //             scope
	       //  //                 .view()
	       //  //                 .map(move |message| Message::ScopeMessage(message)),
	       //  //         )
	       //  //     })
	       //  //     .into();
	       //  profile = profile.push(self.scopes[i].view());
        // };
        self.scopes
        	.iter_mut()
        	.enumerate()
            .fold(Column::new().spacing(SPACING), |column, (_, level)| {
                column.push(
                    level
                        .view()
                        // .map(move |message| Message::ScopeMessage(message)),
                )
            }).into()
    }
}
