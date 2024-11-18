//! Lucide Floem
//!
//! Example
//! ```rust
//! lucide_floem::Icon::ChevronDown
//!    .style(|s| s.size(50, 50))
//!

use floem::{
    prop, prop_extractor,
    style::{FontSize, Style},
    style_class,
    views::{svg, Decorators},
    IntoView, View, ViewId,
};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE: Regex = Regex::new(r#"stroke-width="(\d+)""#).unwrap();
}

style_class!(pub LucideClass);

prop!(pub StrokeWidth: f64 {} = 2.);

prop_extractor! {
    pub LucideProps {
        pub stroke_width: StrokeWidth,
        pub font_size: FontSize,
    }
}

pub struct Lucide {
    id: ViewId,
    lucide_props: LucideProps,
    svg_id: ViewId,
    size: Option<f32>,
    original_svg: String,
}
impl View for Lucide {
    fn id(&self) -> floem::ViewId {
        self.id
    }

    fn style_pass(&mut self, cx: &mut floem::context::StyleCx<'_>) {
        if self.lucide_props.read(cx) {
            self.id().request_all();
            self.size = self.lucide_props.font_size();
            let replaced = RE.replace_all(
                &self.original_svg,
                format!(r#"stroke-width="{}""#, self.lucide_props.stroke_width()).as_str(),
            );

            self.svg_id.update_state(replaced.to_string());
        }
        cx.style_view(self.svg_id);
    }

    fn layout(&mut self, cx: &mut floem::context::LayoutCx) -> floem::taffy::prelude::NodeId {
        let ni = floem::recursively_layout_view(self.id, cx);
        if let Some(size) = self.size {
            let style = self.id.get_combined_style();
            let node = self.id.taffy_node();
            let style = Style::new().size(size, size).apply(style).to_taffy_style();
            self.id.set_taffy_style(node, style);
        }
        ni
    }

    fn debug_name(&self) -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Borrowed("Lucide")
    }

    fn paint(&mut self, cx: &mut floem::context::PaintCx) {
        cx.paint_children(self.id);
    }
}

/// You should be using the `Icon` enum to create icons instead of using this function directly.
///
/// This function is used to create a Lucide view from an SVG string.
/// The string must be a lucide SVG.
pub fn lucide(original_svg: String) -> Lucide {
    let cloned = original_svg.clone();
    let child = svg(cloned.clone()).style(|s| s.size_full());
    let svg_id = child.id();
    let id = ViewId::new();
    id.set_children(vec![child]);

    Lucide {
        id,
        lucide_props: Default::default(),
        svg_id,
        size: None,
        original_svg,
    }
    .class(LucideClass)
    .style(|s| s.items_center().justify_center().min_size(1., 1.))
}

include!(concat!(env!("OUT_DIR"), "/icons.rs"));

impl Icon {
    fn view(&self) -> Lucide {
        lucide(self.get_svg().to_string()).debug_name(self.get_debug_name())
    }
}

impl IntoView for Icon {
    type V = Lucide;

    fn into_view(self) -> Self::V {
        self.view()
    }
}
