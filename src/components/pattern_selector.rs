use crate::lexicon::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlSelectElement;
use yew::prelude::*;

pub struct PatternSelector {
  lexicon: Lexicon,
  selected: Option<usize>,
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
      selected: None,
    }
  }

  fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
    match msg {
      Msg::PatternChanged(selected) => {
        self.selected = Some(selected);
        true
      }
      Msg::Apply => {
        if let Some(selected) = self.selected {
          let on_apply_pattern = ctx.props().on_apply_pattern.clone();
          let selected_term = self.lexicon.terms[selected].clone();
          on_apply_pattern.emit(selected_term);
        }
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
          <option disabled={true} selected={self.selected.is_none()}>{"Select a pattern…"}</option>
          {for {
            self.lexicon.terms.iter().enumerate()
              .filter(|(_, term)| !term.cells.is_empty())
              .map(|(i, term)| html! {
                <option
                  value={i.to_string()}
                  selected={self.selected == Some(i)}
                >{format_term_option(&term)}</option>
              })
          }}
        </select>
        <button disabled={self.selected.is_none()} onclick={ctx.link().callback(move |_| Msg::Apply)}>{"Apply"}</button>
      </div>
    }
  }
}

fn format_term_option(term: &Term) -> String {
  format!(
    "{}{}",
    &term.name,
    if (&term.tags).len() > 0 {
      format!(" ({})", &term.tags.join(", "))
    } else {
      "".to_string()
    }
  )
}
