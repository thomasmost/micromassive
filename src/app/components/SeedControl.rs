use yew::prelude::*;

pub struct SeedControl {
    state: State
}

pub enum Msg {
  EnableLoadSeed(bool),
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
          onsignal: None
      }
    }
}

pub struct State {
   pub showLoadSeedControl: bool
}

impl Component for SeedControl {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        SeedControl {
            state: State { showLoadSeedControl: false }
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
      match msg {
          Msg::EnableLoadSeed(fEnable) => {
             self.state.showLoadSeedControl = fEnable;
          }
      }
      false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
      true
    }
}

impl Renderable<SeedControl> for SeedControl {
  
    fn view(&self) -> Html<Self> {

      html! {
         <div>
            <h3>{ "Seed" }</h3>
            <input type="radio" name="SeedSrc" value="random"      
               checked=!self.state.showLoadSeedControl
               onclick=|_| Msg::EnableLoadSeed(false) />
               { "Use random seed." }
            <input type="radio" name="SeedSrc" value="load" checked=self.state.showLoadSeedControl
               onclick=|_| Msg::EnableLoadSeed(true) /> 
               { "Load seed from disk." }
         </div>
        }
    }
}