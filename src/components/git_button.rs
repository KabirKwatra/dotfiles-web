use yew::prelude::*;
// Model
#[derive(Properties, Clone, PartialEq)]
pub struct GitButton {
    pub children: Children,
    pub button_href: &'static str,
}
// View
impl Component for GitButton {
    type Message = ();
    type Properties = GitButton;
    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        props
    }
    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }
    fn view(&self) -> Html {
        html! {
            <a href={self.button_href}>
                <button class="flex mx-auto text-white bg-indigo-500 mar py-2 px-2 focus:outline-none hover:bg-indigo-600 rounded-full text-lg">
                    {self.children.clone()}
                </button>
            </a>
        }
    }
}
