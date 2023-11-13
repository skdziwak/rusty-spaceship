use web_sys::HtmlElement;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct MarkdownRendererProps {
    pub markdown: String,
}

pub struct MarkdownRenderer {
    pub props: MarkdownRendererProps,
    pub node_ref: NodeRef,
}

impl Component for MarkdownRenderer {
    type Message = ();
    type Properties = MarkdownRendererProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            props: ctx.props().clone(),
            node_ref: NodeRef::default(),
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div ref={self.node_ref.clone()}/>
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if first_render {
            let node = self.node_ref.cast::<HtmlElement>().unwrap();
            let markdown = &self.props.markdown;
            let html = markdown::to_html(markdown);
            node.set_inner_html(&html);
        }
    }
}


