use crate::lexicon::get_lexicon;
use crate::lexicon::Lexicon;
use crate::lexicon::Term;
use wasm_bindgen::JsCast;
use web_sys::HtmlSelectElement;
use yew::prelude::*;

pub struct PatternSelector {
  lexicon: Lexicon,
  selected: usize,
}

#[derive(Properties, PartialEq)]
pub struct Props {
  pub on_apply_pattern: Callback<Term>,
}

pub enum Msg {
  PatternChanged(usize),
  Apply,
}

impl Component for PatternSelector {
  type Message = Msg;
  type Properties = Props;

  fn create(_: &Context<Self>) -> Self {
    Self {
      lexicon: get_lexicon().unwrap(),
      selected: 0,
    }
  }

  fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
    match msg {
      Msg::PatternChanged(selected) => {
        self.selected = selected;
        true
      }
      Msg::Apply => {
        let on_apply_pattern = ctx.props().on_apply_pattern.clone();
        let selected_term = self.lexicon.terms[self.selected].clone();
        on_apply_pattern.emit(selected_term);
        true
      }
    }
  }

  fn view(&self, ctx: &Context<Self>) -> yew::virtual_dom::VNode {
    let on_change_selected = ctx.link().callback(|event: Event| {
      let input = event
        .target()
        .and_then(|t| t.dyn_into::<HtmlSelectElement>().ok())
        .unwrap();
      let selected: usize = input.value().parse().unwrap();
      Msg::PatternChanged(selected)
    });

    html! {
      <div>
        <select onchange={on_change_selected}>
          {for {
            self.lexicon.terms.iter().enumerate()
              .map(|(i, term)| html! {
                <option
                  value={i.to_string()}
                  selected={self.selected == i}
                  disabled={term.cells.is_empty()}
                >{&term.name}</option>
              })
          }}
        </select>
        <button onclick={ctx.link().callback(move |_| Msg::Apply)}>{"Apply"}</button>
      </div>
    }
  }
}
