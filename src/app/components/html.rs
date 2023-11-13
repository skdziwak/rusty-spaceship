use web_sys::HtmlElement;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct HtmlRendererProps {
    pub html: String,
}

pub struct HtmlRenderer {
    pub props: HtmlRendererProps,
    pub node_ref: NodeRef,
}

impl Component for HtmlRenderer {
    type Message = ();
    type Properties = HtmlRendererProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            props: ctx.props().clone(),
            node_ref: NodeRef::default(),
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div ref={self.node_ref.clone()} />
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if first_render {
            let node = self.node_ref.cast::<HtmlElement>().unwrap();
            node.set_inner_html(&self.props.html);
        }
    }
}


