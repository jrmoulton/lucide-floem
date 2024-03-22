use floem::{
    id::Id,
    prop, prop_extractor,
    taffy::Size,
    view::{default_compute_layout, View, ViewData, Widget},
    views::{svg, Decorators, Svg},
};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE: Regex = Regex::new(r#"stroke-width="(\d+)""#).unwrap();
}

prop!(pub StrokeWidth: f64 {} = 2.);

prop_extractor! {
    pub LucideProps {
        pub stroke_width: StrokeWidth,
    }
}

pub struct Lucide {
    view_data: ViewData,
    lucide_props: LucideProps,
    size: floem::taffy::prelude::Size<f32>,
    svg: Svg,
    original_svg: String,
}
impl View for Lucide {
    fn view_data(&self) -> &ViewData {
        &self.view_data
    }

    fn view_data_mut(&mut self) -> &mut ViewData {
        &mut self.view_data
    }

    fn build(self) -> floem::view::AnyWidget {
        Box::new(self)
    }
}

impl Widget for Lucide {
    fn view_data(&self) -> &ViewData {
        &self.view_data
    }

    fn view_data_mut(&mut self) -> &mut ViewData {
        &mut self.view_data
    }

    fn for_each_child<'a>(&'a self, for_each: &mut dyn FnMut(&'a dyn Widget) -> bool) {
        for_each(&self.svg);
    }

    fn for_each_child_mut<'a>(&'a mut self, for_each: &mut dyn FnMut(&'a mut dyn Widget) -> bool) {
        for_each(&mut self.svg);
    }

    fn for_each_child_rev_mut<'a>(
        &'a mut self,
        for_each: &mut dyn FnMut(&'a mut dyn Widget) -> bool,
    ) {
        for_each(&mut self.svg);
    }
    fn compute_layout(
        &mut self,
        cx: &mut floem::context::ComputeLayoutCx,
    ) -> Option<floem::kurbo::Rect> {
        let layout = cx.get_layout(self.id()).unwrap();

        self.size = layout.size;

        default_compute_layout(self, cx)
    }

    fn style(&mut self, cx: &mut floem::context::StyleCx<'_>) {
        if self.lucide_props.read(cx) {
            let replaced = RE.replace_all(
                &self.original_svg,
                format!(r#"stroke-width="{}""#, self.lucide_props.stroke_width()).as_str(),
            );

            self.svg.id().update_state(replaced.to_string());
        }
        self.for_each_child_mut(&mut |child| {
            cx.style_view(child);
            false
        });
    }
}

pub fn lucide(original_svg: String) -> Lucide {
    let cloned = original_svg.clone();
    Lucide {
        view_data: ViewData::new(Id::next()),
        lucide_props: Default::default(),
        size: Size::zero(),
        svg: svg(move || cloned.clone()).style(|s| s.size_full()),
        original_svg,
    }
    .style(|s| s.items_center().justify_center())
}

include!(concat!(env!("OUT_DIR"), "/icons.rs"));

impl Icon {
    pub fn view(&self) -> Lucide {
        lucide(self.get_svg().to_string())
    }
}


// impl Lucide {
//     pub fn lucide_style(
//         mut self,
//         style: impl Fn(LucideCustomStyle) -> LucideCustomStyle + 'static,
//     ) -> Self {
//         let id = self.id();
//         let offset = Widget::view_data_mut(&mut self).style.next_offset();
//         let style = create_updater(
//             move || style(ToggleButtonCustomStyle(Style::new())),
//             move |style| id.update_style(style.0, offset),
//         );
//         Widget::view_data_mut(&mut self).style.push(style.0);
//         self
//     }
// }

// pub struct LucideCustomStyle(Style);

// impl LucideCustomStyle {
//     pub fn stroke_width(mut self, stroke_width: f64) -> Self {
//         self = Self(self.0.set(StrokeWidth, stroke_width));
//         self
//     }
//     /// Apply regular style properties
//     pub fn style(mut self, style: impl Fn(Style) -> Style + 'static) -> Self {
//         self = Self(self.0.apply(style(Style::new())));
//         self
//     }
// }
