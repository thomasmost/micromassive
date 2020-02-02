use log::*;
use yew::prelude::*;

pub struct SeedControl {
    state: State
}

pub enum Msg {
  BrowseFileSystem,
  EnableLoadSeed(bool),
}

#[derive(PartialEq, Clone)]
pub struct Props {
    pub onsignal: Option<Callback<crate::app::Msg>>,
}

impl Default for Props {
    fn default() -> Self {
      Self {
          onsignal: None
      }
    }
}

pub struct State {
   pub show_load_seed_control: bool
}

impl Component for SeedControl {
   type Message = Msg;
   type Properties = Props;

   fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
      SeedControl {
         state: State { show_load_seed_control: false }
      }
   }

   fn update(&mut self, msg: Self::Message) -> ShouldRender {
      match msg
      {
         Msg::BrowseFileSystem => {
            false
         }

         Msg::EnableLoadSeed(f_enable) => {
            self.state.show_load_seed_control = f_enable;
            true
         }
      }
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
            <input type="radio" name="SeedSrc" value="random" checked=!self.state.show_load_seed_control
               onclick=|_| Msg::EnableLoadSeed(false) />
               { "Use random seed." }<br />
            <input type="radio" name="SeedSrc" value="load" checked=self.state.show_load_seed_control
               onclick=|_| Msg::EnableLoadSeed(true) /> 
               { "Load seed from disk." }
            { self.view_browse_control(self.state.show_load_seed_control) }
         </div>
        }
    }
}

impl SeedControl
{
   fn view_browse_control(&self, show_browse_control: bool) -> Html<SeedControl>
   {
      if show_browse_control
         {
         html! 
            {
            <button class="clear-completed" onclick=|_| Msg::BrowseFileSystem>
               { "Browse" }
            </button>
            }
         }
      else
         {
         html! 
            {
            <div></div>
            }
         }
   }
}
