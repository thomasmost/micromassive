extern crate stdweb;
#[macro_use]
extern crate yew;

use yew::prelude::*;

pub struct Simulator {
  arrows: u8,
  current_room: u8,
}

#[derive(Debug, Clone)]
pub enum Msg {}

impl Component for Simulator {
  type Message = Msg;
  type Properties = ();

  fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
    Simulator {
      arrows: 5,
      current_room: 1,
    }
  }

  fn update(&mut self, _msg: Self::Message) -> ShouldRender {
    true
  }
}

impl Renderable<Simulator> for Simulator {
  fn view(&self) -> Html<Self> {
    html! {
        <div class="hunt",>
            <div class="header",>{"Hunt the Wumpus"}</div>
            <div class="body",>
              <span class="arrows",>{&format!("Arrows: {}", self.arrows)}</span>
            </div>
        </div>
    }
  }
}
