use yew::prelude::*;

enum Msg {
    AddOne,
}

struct App {
    count: i64,
}

// Component implementation for the APp
impl Component for App {
    type Message = Msg;
    type Properties = ();

    // Create the app and set the counter 0, the App context we do not use now
    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            count: 0,
        }
    }

    // update the app if the message match with the Msg::AddOne, the App context we do not use now
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                self.count += 1;
                true // re-render component
            }
        }
    }

    // Create the view, this retirn the html with the mouse click from the context
    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html! {
            <div class="container">
                <p>{ self.count }</p>
                <button onclick={link.callback(|_| Msg::AddOne)}>{ "+1" }</button>
            </div>
        }
    }
}
fn main() {
    // start the app, by default the app use the index.html and the style.css from the root
    // start the app call: trunk serve [This start the server and render the aplication on localhost: 8000]
    yew::Renderer::<App>::new().render();
}