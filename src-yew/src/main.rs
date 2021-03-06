use yew::prelude::*;

enum Msg {
    AddOne,
    SubOne,
}

struct Model {
    value: i64,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { value: 0 }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                // the value has changed so we need to
                // re-render for it to appear on the page
                true
            }
            Msg::SubOne => {
                self.value -= 1;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let link = ctx.link();
        html! {
            <div>
                <button class="bg-red-500 hover:bg-violet-500 active:bg-violet-700 focus:outline-none focus:ring focus:ring-violet-300 ..." onclick={link.callback(|_| Msg::AddOne)}>{ "+1" }</button>
                <button class="bg-violet-500 hover:bg-violet-600 active:bg-violet-700 focus:outline-none focus:ring focus:ring-violet-300 ..." onclick={link.callback(|_| Msg::SubOne)}>{ "-1" }</button>
                <p>{ if self.value < 10 {
                    self.value.to_string()
                } else {
                    "Ten!".to_string()
                }}</p>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
