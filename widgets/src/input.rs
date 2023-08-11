mod caret;
mod caret_state;
mod editarea;
mod glyphs_helper;
mod handle;
mod selected_text;
mod text_selectable;
use std::time::Duration;

pub use caret_state::CaretState;

pub use self::editarea::PlaceholderStyle;
use self::editarea::TextEditorArea;
pub use self::selected_text::SelectedTextStyle;
use crate::{declare_writer, layout::ConstrainedBox};

use ribir_core::prelude::*;
use std::ops::{Deref, DerefMut};
pub use text_selectable::TextSelectable;

pub struct Placeholder(CowArc<str>);

impl Placeholder {
  #[inline]
  pub fn new(str: impl Into<CowArc<str>>) -> Self { Self(str.into()) }
}
#[derive(Clone, PartialEq)]
pub struct InputStyle {
  pub size: Option<f32>,
}

impl CustomStyle for InputStyle {
  fn default_style(_: &BuildCtx) -> Self { InputStyle { size: Some(20.) } }
}

#[derive(Declare)]
pub struct Input {
  #[declare(default = TypographyTheme::of(ctx).body_large.text.clone())]
  pub style: CowArc<TextStyle>,
  #[declare(skip)]
  text: CowArc<str>,
  #[declare(skip)]
  caret: CaretState,
  #[declare(default = InputStyle::of(ctx).size)]
  size: Option<f32>,
}

#[derive(Declare)]
pub struct TextArea {
  #[declare(default = TypographyTheme::of(ctx).body_large.text.clone())]
  pub style: CowArc<TextStyle>,
  #[declare(default = true)]
  pub auto_wrap: bool,
  #[declare(skip)]
  text: CowArc<str>,
  #[declare(skip)]
  caret: CaretState,
  #[declare(default = TextAreaStyle::of(ctx).rows)]
  rows: Option<f32>,
  #[declare(default = TextAreaStyle::of(ctx).cols)]
  cols: Option<f32>,
}

impl Input {
  pub fn text(&self) -> CowArc<str> { self.text.clone() }

  pub fn caret(&self) -> &CaretState { &self.caret }

  pub fn set_text(&mut self, text: impl Into<CowArc<str>>) {
    self.text = text.into();
    self.caret = self.caret.valid(self.text.len());
  }

  pub fn set_caret(&mut self, caret: CaretState) { self.caret = caret.valid(self.text.len()); }

  pub fn writer(&mut self) -> impl DerefMut<Target = TextWriter> + '_ { InputWriter::new(self) }
}
declare_writer!(InputWriter, Input);

impl TextArea {
  pub fn text(&self) -> CowArc<str> { self.text.clone() }

  pub fn caret(&self) -> &CaretState { &self.caret }

  pub fn set_text(&mut self, text: impl Into<CowArc<str>>) {
    self.text = text.into();
    self.caret = self.caret.valid(self.text.len());
  }

  pub fn set_caret(&mut self, caret: CaretState) { self.caret = caret.valid(self.text.len()); }

  pub fn writer(&mut self) -> impl DerefMut<Target = TextWriter> + '_ { TextAreaWriter::new(self) }
}
declare_writer!(TextAreaWriter, TextArea);

impl ComposeChild for Input {
  type Child = Option<State<Placeholder>>;
  fn compose_child(this: State<Self>, placeholder: Self::Child) -> Widget {
    widget! {
      init ctx => {
        let frame_scheduler = ctx.wnd_ctx().frame_scheduler();
      }
      states {
        this: this.into_writable(),
      }
      FocusScope {
        ConstrainedBox {
          clamp: size_clamp(&this.style, Some(1.), this.size),
          TextEditorArea {
            id: area,
            text: this.text.clone(),
            style: this.style.clone(),
            caret: this.caret().clone(),
            multi_line: false,
            auto_wrap: false,

            widget::from(placeholder)
          }
        }
      }
      finally {
        let_watch!(area.clone_stateful())
          .delay(Duration::ZERO, frame_scheduler)
          .subscribe(move |area| {
            let area = area.state_ref();
            if area.caret != this.caret {
              this.silent().caret = area.caret.clone();
            }
            if area.text != this.text {
              this.silent().text = area.text.clone();
            }
          });
      }
    }
    .into_widget()
  }
}

#[derive(Clone, PartialEq)]
pub struct TextAreaStyle {
  pub rows: Option<f32>,
  pub cols: Option<f32>,
}
impl CustomStyle for TextAreaStyle {
  fn default_style(_: &BuildCtx) -> Self { TextAreaStyle { rows: Some(2.), cols: Some(20.) } }
}

impl ComposeChild for TextArea {
  type Child = Option<State<Placeholder>>;
  fn compose_child(this: State<Self>, placeholder: Self::Child) -> Widget {
    widget! {
      init ctx => {
        let frame_scheduler = ctx.wnd_ctx().frame_scheduler();
      }
      states {
        this: this.into_writable(),
      }
      FocusScope {
        ConstrainedBox {
          clamp: size_clamp(&this.style, this.rows, this.cols),
          TextEditorArea {
            id: area,
            text: this.text.clone(),
            style: this.style.clone(),
            caret: this.caret.clone(),
            multi_line: true,
            auto_wrap: no_watch!(this.auto_wrap),

            widget::from(placeholder)
          }
        }
      }
      finally {
        let_watch!(area.clone_stateful())
          .delay(Duration::ZERO, frame_scheduler)
          .subscribe(move |area| {
            let area = area.state_ref();
            if area.caret != this.caret {
              this.silent().caret = area.caret.clone();
            }
            if area.text != this.text {
              this.silent().text = area.text.clone();
            }
          });
      }
    }
    .into_widget()
  }
}

fn size_clamp(style: &TextStyle, rows: Option<f32>, cols: Option<f32>) -> BoxClamp {
  let mut clamp: BoxClamp = BoxClamp {
    min: Size::new(0., 0.),
    max: Size::new(f32::INFINITY, f32::INFINITY),
  };
  if let Some(cols) = cols {
    let width = cols * glyph_width(style.font_size);
    clamp = clamp.with_fixed_width(width);
  }
  if let Some(rows) = rows {
    let height = rows * line_height(style.line_height.unwrap_or(style.font_size.into_em()));
    clamp = clamp.with_fixed_height(height);
  }
  clamp
}

fn glyph_width(font_size: FontSize) -> f32 {
  FontSize::Em(font_size.relative_em(1.)).into_pixel().value()
}

fn line_height(line_height: Em) -> f32 { FontSize::Em(line_height).into_pixel().value() }
