use yew::prelude::*;

pub struct Controls {
    title: String,
    exits: [u8; 3],
    current_click: u64,
    onsignal: Option<Callback<crate::app::Msg>>,
}

pub enum Msg {
  ButtonPressed(crate::app::Msg)
}

#[derive(PartialEq, Clone)]
pub struct Props {
    pub exits: [u8; 3],
    pub current_click: u64,
    pub onsignal: Option<Callback<crate::app::Msg>>,
}

impl Default for Props {
    fn default() -> Self {
      Self {
          exits: [0, 0, 0],
          current_click: 0,
          onsignal: None,
      }
    }
}

impl Component for Controls {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Controls {
            title: "Controls".into(),
            exits: props.exits,
            current_click: props.current_click,
            onsignal: props.onsignal,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
      match msg {
          Msg::ButtonPressed(msg) => {
              if let Some(ref mut callback) = self.onsignal {
                  callback.emit(msg);
              }
          }
      }
      false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
      self.current_click = props.current_click;
      self.onsignal = props.onsignal;
      true
    }
}

impl Renderable<Controls> for Controls {
  
    fn view(&self) -> Html<Self> {

        use crate::app::Msg::*;

        html! {
            <div class=("container", "container-controls"),>
                <div class="title",>{&self.title}</div>
                <div class="exits",>
                  {format!("exits: {}, {}, {}", self.exits[0], self.exits[1], self.exits[2])}
                </div>
                <div class="current_click",>
                  {format!("Click: {}", self.current_click)}
                </div>
                <div class="step-forward",>
                  <button onclick=|_| Msg::ButtonPressed(StepForward()),>
                    { "Step Forward" }
                  </button>
                </div>
            </div>
        }
    }
}