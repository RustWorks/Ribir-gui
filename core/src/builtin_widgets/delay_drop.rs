use crate::{impl_query_self_only, prelude::*};

/// A widget that can delay drop its child until the `delay_drop_until` field be
/// set to `true`.
///
/// This widget not effect the widget lifecycle, if the widget is dispose but
/// the `delay_drop_until` is `false`, it's not part of the widget tree anymore
/// but not drop immediately, is disposed in `logic`, but not release resource.
/// It's be isolated from the widget tree and can layout and paint normally.
///
/// Once the `delay_drop_until` field be set to `true`, the widget will be
/// dropped.
///
/// It's useful when you need run a leave animation for a widget.
#[derive(Declare, Declare2)]
pub struct DelayDrop {
  #[declare(builtin)]
  pub delay_drop_until: bool,
}

impl ComposeChild for DelayDrop {
  type Child = Widget;
  #[inline]
  fn compose_child(this: State<Self>, child: Self::Child) -> Widget {
    DataWidget::attach(child, this.into_writable())
  }
}

impl_query_self_only!(DelayDrop);