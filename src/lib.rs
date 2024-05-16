use floem::{
    prop, prop_extractor,
    views::{svg, Decorators},
    View, ViewId,
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
    id: ViewId,
    lucide_props: LucideProps,
    svg_id: ViewId,
    original_svg: String,
}
impl View for Lucide {
    fn id(&self) -> floem::ViewId {
        self.id
    }
    fn style(&mut self, cx: &mut floem::context::StyleCx<'_>) {
        if self.lucide_props.read(cx) {
            let replaced = RE.replace_all(
                &self.original_svg,
                format!(r#"stroke-width="{}""#, self.lucide_props.stroke_width()).as_str(),
            );

            self.svg_id.update_state(replaced.to_string());
        }
        cx.style_view(self.svg_id);
    }
}

pub fn lucide(original_svg: String) -> Lucide {
    let cloned = original_svg.clone();
    let child = svg(move || cloned.clone()).style(|s| s.size_full());
    let svg_id = child.id();
    let id = ViewId::new();
    id.set_children(vec![child]);

    Lucide {
        id,
        lucide_props: Default::default(),
        svg_id,
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
