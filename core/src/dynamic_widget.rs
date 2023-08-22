use smallvec::SmallVec;

use crate::{
  builtin_widgets::key::AnyKey,
  impl_proxy_query, impl_query_self_only,
  prelude::*,
  widget::{
    widget_id::{empty_node, split_arena},
    *,
  },
  window::DelayEvent,
};
use std::{
  cell::RefCell,
  collections::{HashMap, HashSet},
  rc::Rc,
};

#[derive(Clone, Copy)]
/// the information of a widget generated by `DynWidget`.
pub(crate) enum DynWidgetGenInfo {
  /// DynWidget generate single result, and have static children. The depth
  /// describe the distance from first dynamic widget (self) to the static
  /// child.
  DynDepth(usize),
  /// `DynWidget` without static children, and the whole subtree of generated
  /// widget are dynamic widgets. The value record how many dynamic siblings
  /// have.
  WholeSubtree(usize),
}

// todo: we can remove `DynWidget` after syntax refactor.
//  - 1. Stateful Compose/ComposeChild as a parent needn't keep
//    Stateful<DynWidget<W>>.
//  - 2. Stateful Render can be directly replace the widget in the tree.
//  - 3. Stateful Multi Widget in Stateful<DynWidget<W>> that should be include
//    in `Multi` widget.

/// Widget that as a container of dynamic widgets

#[derive(Declare)]
pub struct DynWidget<D> {
  #[declare(convert=custom)]
  pub(crate) dyns: Option<D>,
}

impl<D> DynWidgetDeclarer<D> {
  pub fn dyns(mut self, d: D) -> Self {
    self.dyns = Some(Some(d));
    self
  }
}

impl<D> DynWidget<D> {
  pub fn set_declare_dyns(&mut self, dyns: D) { self.dyns = Some(dyns); }

  pub fn into_inner(mut self) -> D {
    self
      .dyns
      .take()
      .unwrap_or_else(|| unreachable!("stateless `DynWidget` must be initialized."))
  }
}

/// Widget help to limit which `DynWidget` can be a parent widget and which can
/// be a child.
pub(crate) struct DynRender<D> {
  dyn_widgets: Stateful<DynWidget<D>>,
  self_render: RefCell<Box<dyn Render>>,
  gen_info: RefCell<Option<DynWidgetGenInfo>>,
  dyns_to_widgets: fn(D) -> Box<dyn Iterator<Item = Widget>>,
  drop_until_widgets: WidgetsHost,
}

// A dynamic widget must be stateful, depends others.
impl<D: 'static> Render for DynRender<D> {
  fn perform_layout(&self, clamp: BoxClamp, ctx: &mut LayoutCtx) -> Size {
    if !self.regen_if_need(ctx) {
      self.self_render.borrow().perform_layout(clamp, ctx)
    } else {
      ctx.new_layouter(ctx.id).perform_widget_layout(clamp)
    }
  }

  fn paint(&self, ctx: &mut PaintingCtx) {
    if !self.drop_until_widgets.is_empty() {
      ctx.painter.save();
      // set the position back to parent.
      let rc = ctx.box_rect().unwrap();
      ctx.painter.translate(-rc.min_x(), -rc.min_y());
      self.drop_until_widgets.paint(ctx);
      ctx.painter.restore();
    }

    self.self_render.borrow().paint(ctx);
  }

  fn only_sized_by_parent(&self) -> bool {
    // Dyn widget effect the children of its parent. Even if its self render is only
    // sized by parent, but itself effect siblings, sibling effect parent, means
    // itself not only sized by parent but also its sibling.
    false
  }

  fn hit_test(&self, ctx: &HitTestCtx, pos: Point) -> HitTest {
    self.self_render.borrow().hit_test(ctx, pos)
  }

  fn get_transform(&self) -> Option<Transform> { self.self_render.borrow().get_transform() }
}

#[derive(Default, Clone)]
struct WidgetsHost {
  wids: Rc<RefCell<HashSet<WidgetId>>>,
}

impl WidgetsHost {
  fn add(&self, wid: WidgetId) { self.wids.borrow_mut().insert(wid); }

  fn is_empty(&self) -> bool { self.wids.borrow().is_empty() }

  fn paint(&self, ctx: &mut PaintingCtx) {
    self.wids.borrow().iter().for_each(|wid| {
      wid.paint_subtree(ctx);
    });
  }
}

impl_query_self_only!(WidgetsHost);

impl<D: 'static> DynRender<D> {
  pub(crate) fn single(dyns: Stateful<DynWidget<D>>) -> Self
  where
    Widget: From<D>,
  {
    DynRender {
      dyn_widgets: dyns,
      self_render: RefCell::new(Box::new(Void)),
      gen_info: <_>::default(),
      dyns_to_widgets: move |w| Box::new(std::iter::once(w.into())),
      drop_until_widgets: <_>::default(),
    }
  }

  fn regen_if_need(&self, ctx: &mut LayoutCtx) -> bool {
    let Some(new_widgets) = self.dyn_widgets.silent_ref().dyns.take() else {
      return false;
    };

    let mut new_widgets = (self.dyns_to_widgets)(new_widgets)
      .map(|w| {
        let build_ctx = BuildCtx::new(ctx.parent(), ctx.tree);
        w.build(&build_ctx)
      })
      .collect::<Vec<_>>();

    if new_widgets.is_empty() {
      new_widgets.push(empty_node(&mut ctx.tree.arena));
    }

    let gen_info = *self.gen_info.borrow_mut().get_or_insert_with(|| {
      if ctx.has_child() {
        DynWidgetGenInfo::DynDepth(1)
      } else {
        DynWidgetGenInfo::WholeSubtree(1)
      }
    });

    self.update_key_state(ctx.id, &new_widgets, &ctx.tree.arena);

    let mut tmp_render = std::mem::replace(&mut *self.self_render.borrow_mut(), Box::new(Void {}));
    // Place the real old render in node.
    std::mem::swap(&mut tmp_render, ctx.id.assert_get_mut(&mut ctx.tree.arena));

    self.dyn_widgets.modify_notifier.reset();

    let wrap_render = |gen_info, tree: &mut WidgetTree| {
      let new_render = DynRender {
        dyn_widgets: self.dyn_widgets.clone(),
        self_render: RefCell::new(Box::new(Void {})),
        gen_info: RefCell::new(Some(gen_info)),
        dyns_to_widgets: self.dyns_to_widgets,
        drop_until_widgets: self.drop_until_widgets.clone(),
      };

      // Place the first new render in `DynRender`.
      std::mem::swap(
        &mut *new_render.self_render.borrow_mut(),
        new_widgets[0].assert_get_mut(&mut tree.arena),
      );
      // use the dyn render as the first new widget.
      *new_widgets[0].assert_get_mut(&mut tree.arena) = Box::new(new_render);
    };

    let tree = &mut ctx.tree;

    match gen_info {
      DynWidgetGenInfo::DynDepth(depth) => {
        assert_eq!(new_widgets.len(), 1);

        let declare_child_parent = single_down(ctx.id, &tree.arena, depth as isize - 1);
        let (new_leaf, down_level) = down_to_leaf(new_widgets[0], &tree.arena);
        wrap_render(DynWidgetGenInfo::DynDepth(down_level + 1), tree);

        if let Some(declare_child_parent) = declare_child_parent {
          // Safety: control two subtree not intersect.
          let (arena1, arena2) = unsafe { split_arena(&mut tree.arena) };
          declare_child_parent
            .children(arena1)
            .for_each(|c| new_leaf.append(c, arena2));
        }

        ctx.id.insert_after(new_widgets[0], &mut tree.arena);
        self.remove_old_subtree(ctx.id, self.drop_until_widgets.clone(), tree);

        let mut w = new_widgets[0];
        loop {
          w.on_mounted(tree);
          if w == new_leaf {
            break;
          }
          w = w.single_child(&tree.arena).unwrap();
        }
      }

      DynWidgetGenInfo::WholeSubtree(siblings) => {
        wrap_render(DynWidgetGenInfo::WholeSubtree(new_widgets.len()), tree);
        let mut cursor = Some(ctx.id);
        new_widgets
          .iter()
          .for_each(|w| ctx.id.insert_before(*w, &mut tree.arena));

        (0..siblings).for_each(|_| {
          let o = cursor.unwrap();
          cursor = o.next_sibling(&tree.arena);
          self.remove_old_subtree(o, self.drop_until_widgets.clone(), tree);
        });

        new_widgets.iter().for_each(|w| w.on_mounted_subtree(tree));
      }
    };

    if ctx.id == tree.root() {
      tree.root = new_widgets.first().copied();
    }
    ctx.id = new_widgets[0];

    true
  }

  fn remove_old_subtree(&self, wid: WidgetId, host: WidgetsHost, tree: &mut WidgetTree) {
    fn detach(
      host: WidgetsHost,
      wid: WidgetId,
      drop_until: Stateful<DelayDropWidget>,
      tree: &mut WidgetTree,
    ) {
      let mut handlers = SmallVec::<[_; 1]>::new();
      tree.detach(wid);

      let arena = &mut tree.arena;
      wid.assert_get(arena).query_all_type(
        |notifier: &StateChangeNotifier| {
          let state_changed = tree.dirty_set.clone();
          // abandon the old subscribe
          notifier.reset();
          let h = notifier
            .raw_modifies()
            .filter(|b| b.contains(ModifyScope::FRAMEWORK))
            .subscribe(move |_| {
              state_changed.borrow_mut().insert(wid);
            })
            .unsubscribe_when_dropped();
          handlers.push(h);
          true
        },
        QueryOrder::OutsideFirst,
      );

      let wnd_id = tree.window().id();
      let tmp = drop_until.clone();
      drop_until
        .raw_modifies()
        .filter(move |b| b.contains(ModifyScope::FRAMEWORK) && tmp.state_ref().delay_drop_until)
        .take(1)
        .delay(std::time::Duration::ZERO, tree.window().frame_scheduler())
        .subscribe(move |_| {
          if let Some(wnd) = AppCtx::get_window(wnd_id) {
            wnd.widget_tree.borrow_mut().remove_subtree(wid);
          }
          host.wids.borrow_mut().remove(&wid);
          handlers.clear();
        });
    }

    let wnd = tree.window();

    let drop_until = wid
      .assert_get(&tree.arena)
      .query_on_first_type(QueryOrder::OutsideFirst, |w: &Stateful<DelayDropWidget>| {
        w.clone_stateful()
      });

    let is_drop = drop_until
      .as_ref()
      .map_or(true, |w| w.state_ref().delay_drop_until);
    wnd.add_delay_event(DelayEvent::Disposed { id: wid, delay_drop: !is_drop });
    if !is_drop {
      detach(host, wid, drop_until.unwrap(), tree);
      self.drop_until_widgets.add(wid);
      tree.dirty_set.borrow_mut().insert(wid);
    } else {
      tree.detach(wid);

      let (arena1, arena2) = unsafe { split_arena(&mut tree.arena) };
      wid
        .descendants(arena1)
        .for_each(|wid| wid.mark_drop(arena2))
    }
  }

  fn update_key_state(&self, sign_id: WidgetId, new_widgets: &[WidgetId], arena: &TreeArena) {
    let mut old_key_list = HashMap::new();

    let mut gen_info = self.gen_info.borrow_mut();
    let Some(gen_info) = &mut *gen_info else { return };

    let siblings = match gen_info {
      DynWidgetGenInfo::DynDepth(_) => 1,
      DynWidgetGenInfo::WholeSubtree(width) => *width,
    };
    let mut remove = Some(sign_id);
    (0..siblings).for_each(|_| {
      let o = remove.unwrap();
      inspect_key(&o, arena, |old_key_widget: &dyn AnyKey| {
        let key = old_key_widget.key();
        old_key_list.insert(key, o);
      });

      remove = o.next_sibling(arena);
    });

    new_widgets.iter().for_each(|n| {
      inspect_key(n, arena, |new_key_widget: &dyn AnyKey| {
        let key = &new_key_widget.key();
        if let Some(wid) = old_key_list.get(key) {
          inspect_key(wid, arena, |old_key_widget: &dyn AnyKey| {
            new_key_widget.record_prev_key_widget(old_key_widget);
            old_key_widget.record_next_key_widget(new_key_widget);
          });
        }
      });
    });
  }
}

impl<D> DynRender<Multi<D>> {
  pub(crate) fn multi(dyns: Stateful<DynWidget<Multi<D>>>) -> Self
  where
    D: IntoIterator + 'static,
    Widget: From<D::Item>,
  {
    Self {
      dyn_widgets: dyns,
      self_render: RefCell::new(Box::new(Void)),
      gen_info: <_>::default(),
      dyns_to_widgets: move |d| Box::new(d.into_inner().into_iter().map(|w| w.into())),
      drop_until_widgets: <_>::default(),
    }
  }
}

impl<D: 'static> DynRender<Option<D>>
where
  Widget: From<D>,
{
  pub(crate) fn option(dyns: Stateful<DynWidget<Option<D>>>) -> Self {
    DynRender {
      dyn_widgets: dyns,
      self_render: RefCell::new(Box::new(Void)),
      gen_info: <_>::default(),
      dyns_to_widgets: move |w| Box::new(w.into_iter().map(From::from)),
      drop_until_widgets: <_>::default(),
    }
  }
}

impl_proxy_query!(paths [self_render.borrow(), dyn_widgets], DynRender<D>, <D>, where D: 'static );
impl_query_self_only!(DynWidget<D>, <D>, where D: 'static);

fn inspect_key(id: &WidgetId, tree: &TreeArena, mut cb: impl FnMut(&dyn AnyKey)) {
  #[allow(clippy::borrowed_box)]
  id.assert_get(tree)
    .query_on_first_type(QueryOrder::OutsideFirst, |key_widget: &Box<dyn AnyKey>| {
      cb(&**key_widget)
    });
}

fn single_down(id: WidgetId, arena: &TreeArena, mut down_level: isize) -> Option<WidgetId> {
  let mut res = Some(id);
  while down_level > 0 {
    down_level -= 1;
    res = res.unwrap().single_child(arena);
  }
  res
}

fn down_to_leaf(id: WidgetId, arena: &TreeArena) -> (WidgetId, usize) {
  let mut leaf = id;
  let mut depth = 0;
  while let Some(c) = leaf.single_child(arena) {
    leaf = c;
    depth += 1;
  }
  (leaf, depth)
}

// impl IntoWidget

// only `DynWidget` gen single widget can as a parent widget
impl<D: 'static> WidgetBuilder for Stateful<DynWidget<D>>
where
  Widget: From<D>,
{
  fn build(self, ctx: &BuildCtx) -> WidgetId { DynRender::single(self).build(ctx) }
}

impl<D: 'static> WidgetBuilder for Stateful<DynWidget<Option<D>>>
where
  Widget: From<D>,
{
  fn build(self, ctx: &BuildCtx) -> WidgetId {
    DynRender {
      dyn_widgets: self,
      self_render: RefCell::new(Box::new(Void)),
      gen_info: <_>::default(),
      dyns_to_widgets: move |w| Box::new(w.into_iter().map(From::from)),
      drop_until_widgets: <_>::default(),
    }
    .build(ctx)
  }
}

impl<W: SingleChild> SingleChild for DynWidget<W> {}

#[cfg(test)]
mod tests {
  use std::{
    cell::{Ref, RefCell},
    rc::Rc,
  };

  use crate::{
    builtin_widgets::key::KeyChange, impl_query_self_only, prelude::*, test_helper::*,
    widget::TreeArena,
  };

  #[test]
  fn expr_widget_as_root() {
    let _guard = unsafe { AppCtx::new_lock_scope() };

    let size = Stateful::new(Size::zero());
    let w = widget! {
      states { size: size.clone() }
      DynWidget {
        dyns: MockBox { size: *size },
        Void {}
      }
    };
    let wnd = TestWindow::new(w);
    let mut tree = wnd.widget_tree.borrow_mut();
    tree.layout(Size::zero());
    let ids = tree.root().descendants(&tree.arena).collect::<Vec<_>>();
    assert_eq!(ids.len(), 2);
    {
      *size.state_ref() = Size::new(1., 1.);
    }
    tree.layout(Size::zero());
    let new_ids = tree.root().descendants(&tree.arena).collect::<Vec<_>>();
    assert_eq!(new_ids.len(), 2);

    assert_eq!(ids[1], new_ids[1]);
  }

  #[test]
  fn expr_widget_with_declare_child() {
    let _guard = unsafe { AppCtx::new_lock_scope() };

    let size = Stateful::new(Size::zero());
    let w = widget! {
      states { size: size.clone() }
      MockBox {
        size: Size::zero(),
        DynWidget {
          dyns: MockBox { size: *size },
          Void {}
        }
      }
    };
    let wnd = TestWindow::new(w);
    let mut tree = wnd.widget_tree.borrow_mut();
    tree.layout(Size::zero());
    let ids = tree.root().descendants(&tree.arena).collect::<Vec<_>>();
    assert_eq!(ids.len(), 3);
    {
      *size.state_ref() = Size::new(1., 1.);
    }
    tree.layout(Size::zero());
    let new_ids = tree.root().descendants(&tree.arena).collect::<Vec<_>>();
    assert_eq!(new_ids.len(), 3);

    assert_eq!(ids[0], new_ids[0]);
    assert_eq!(ids[2], new_ids[2]);
  }

  #[test]
  fn expr_widget_mounted_new() {
    let _guard = unsafe { AppCtx::new_lock_scope() };

    let v = Stateful::new(vec![1, 2, 3]);

    let new_cnt = Stateful::new(0);
    let drop_cnt = Stateful::new(0);
    let w = widget! {
      states {
        v: v.clone(),
        new_cnt: new_cnt.clone(),
        drop_cnt: drop_cnt.clone(),
      }

      MockMulti {
        Multi::new(v.clone().into_iter().map(move |_| {
          widget! {
            MockBox{
              size: Size::zero(),
              on_mounted: move |_| *new_cnt += 1,
              on_disposed: move |_| *drop_cnt += 1
            }
          }
        }))
      }
    };

    let mut wnd = TestWindow::new(w);
    wnd.on_wnd_resize_event(Size::zero());
    wnd.draw_frame();
    assert_eq!(*new_cnt.state_ref(), 3);
    assert_eq!(*drop_cnt.state_ref(), 0);

    v.state_ref().push(4);
    wnd.draw_frame();
    assert_eq!(*new_cnt.state_ref(), 7);
    assert_eq!(*drop_cnt.state_ref(), 3);

    v.state_ref().pop();
    wnd.draw_frame();
    assert_eq!(*new_cnt.state_ref(), 10);
    assert_eq!(*drop_cnt.state_ref(), 7);
  }

  #[test]
  fn dyn_widgets_with_key() {
    let _guard = unsafe { AppCtx::new_lock_scope() };

    let v = Stateful::new(vec![(1, '1'), (2, '2'), (3, '3')]);
    let enter_list: Stateful<Vec<char>> = Stateful::new(vec![]);
    let update_list: Stateful<Vec<char>> = Stateful::new(vec![]);
    let leave_list: Stateful<Vec<char>> = Stateful::new(vec![]);
    let key_change: Stateful<KeyChange<char>> = Stateful::new(KeyChange::default());
    let w = widget! {
      states {
        v: v.clone(),
        enter_list: enter_list.clone(),
        update_list: update_list.clone(),
        leave_list: leave_list.clone(),
        key_change: key_change.clone(),
      }

      MockMulti {
        Multi::new(v.clone().into_iter().map(move |(i, c)| {
          widget! {
            KeyWidget {
              id: key,
              key: Key::from(i),
              value: c,

              MockBox {
                size: Size::zero(),
                on_mounted: move |_| {
                  if key.is_enter() {
                    (*enter_list).push(key.value);
                  }

                  if key.is_changed() {
                    (*update_list).push(key.value);
                    *key_change = key.get_change();
                  }
                },
                on_disposed: move |_| {
                  if key.is_leave() {
                    (*leave_list).push(key.value);
                  }
                }
              }
            }
          }
        }))
      }
    };

    // 1. 3 item enter
    let mut wnd = TestWindow::new(w);
    wnd.draw_frame();
    let expect_vec = vec!['1', '2', '3'];
    assert_eq!((*enter_list.state_ref()).len(), 3);
    assert!(
      (*enter_list.state_ref())
        .iter()
        .all(|item| expect_vec.contains(item))
    );
    // clear enter list vec
    (*enter_list.state_ref()).clear();

    // 2. add 1 item
    v.state_ref().push((4, '4'));
    wnd.on_wnd_resize_event(ZERO_SIZE);
    wnd.draw_frame();

    let expect_vec = vec!['4'];
    assert_eq!((*enter_list.state_ref()).len(), 1);
    assert!(
      (*enter_list.state_ref())
        .iter()
        .all(|item| expect_vec.contains(item))
    );
    // clear enter list vec
    (*enter_list.state_ref()).clear();

    // 3. update the second item
    v.state_ref()[1].1 = 'b';
    wnd.draw_frame();

    let expect_vec = vec![];
    assert_eq!((*enter_list.state_ref()).len(), 0);
    assert!(
      (*enter_list.state_ref())
        .iter()
        .all(|item| expect_vec.contains(item))
    );

    let expect_vec = vec!['b'];
    assert_eq!((*update_list.state_ref()).len(), 1);
    assert!(
      (*update_list.state_ref())
        .iter()
        .all(|item| expect_vec.contains(item))
    );
    assert_eq!(*key_change.state_ref(), KeyChange(Some('2'), 'b'));
    (*update_list.state_ref()).clear();

    // 4. remove the second item
    v.state_ref().remove(1);
    wnd.draw_frame();
    let expect_vec = vec!['b'];
    assert_eq!((*leave_list.state_ref()), expect_vec);
    assert_eq!((*leave_list.state_ref()).len(), 1);
    assert!(
      (*leave_list.state_ref())
        .iter()
        .all(|item| expect_vec.contains(item))
    );
    (*leave_list.state_ref()).clear();

    // 5. update the first item
    v.state_ref()[0].1 = 'a';
    wnd.draw_frame();

    assert_eq!((*enter_list.state_ref()).len(), 0);

    let expect_vec = vec!['a'];
    assert_eq!((*update_list.state_ref()).len(), 1);
    assert!(
      (*update_list.state_ref())
        .iter()
        .all(|item| expect_vec.contains(item))
    );
    assert_eq!(*key_change.state_ref(), KeyChange(Some('1'), 'a'));
    (*update_list.state_ref()).clear();
  }

  #[test]
  fn delay_drop_widgets() {
    let _guard = unsafe { AppCtx::new_lock_scope() };

    #[derive(Default, Clone)]
    struct Task {
      mounted: u32,
      pin: bool,
      paint_cnt: Rc<RefCell<u32>>,
      layout_cnt: Rc<RefCell<u32>>,
      trigger: u32,
      wid: Option<WidgetId>,
    }

    fn build(item: Stateful<Task>) -> Widget {
      widget! {
        states { task: item.clone() }
        TaskWidget {
          delay_drop_until: !task.pin,
          layout_cnt: task.layout_cnt.clone(),
          paint_cnt: task.paint_cnt.clone(),
          trigger: task.trigger,
          on_mounted: move |ctx| {
            task.mounted += 1;
            task.wid = Some(ctx.id);
          },
          on_disposed: move |ctx| {
            let wid = task.wid.take();
            assert_eq!(wid, Some(ctx.id));
          }
        }
      }
      .into()
    }

    #[derive(Declare)]
    struct TaskWidget {
      trigger: u32,
      paint_cnt: Rc<RefCell<u32>>,
      layout_cnt: Rc<RefCell<u32>>,
    }

    impl Render for TaskWidget {
      fn perform_layout(&self, _: BoxClamp, _: &mut LayoutCtx) -> Size {
        *self.layout_cnt.borrow_mut() += 1;
        Size::new(1., 1.)
      }

      fn paint(&self, _: &mut PaintingCtx) { *self.paint_cnt.borrow_mut() += 1; }
    }

    impl_query_self_only!(TaskWidget);

    fn child_count(wnd: &Window) -> usize {
      let tree = wnd.widget_tree.borrow();
      let root = tree.root();
      root.children(&tree.arena).count()
    }

    let tasks = (0..3)
      .map(|_| Stateful::new(Task::default()))
      .collect::<Vec<_>>();
    let tasks = Stateful::new(tasks);
    let w = widget! {
      states {tasks: tasks.clone()}
      MockMulti {
        Multi::new(tasks.clone().into_iter().map(build))
      }
    };

    let mut wnd = TestWindow::new(w);
    let mut removed = vec![];

    wnd.draw_frame();
    assert_eq!(child_count(&wnd), 3);

    // the first pined widget will still paint it
    tasks.state_ref()[0].state_ref().pin = true;
    removed.push(tasks.state_ref().remove(0));
    wnd.draw_frame();
    assert_eq!(child_count(&wnd), 2);
    assert_eq!(*removed[0].state_ref().paint_cnt.borrow(), 2);

    // the remove pined widget will paint and no layout when no changed
    let first_layout_cnt = *removed[0].state_ref().layout_cnt.borrow();
    tasks.state_ref().get(0).unwrap().state_ref().pin = true;
    removed.push(tasks.state_ref().remove(0));
    wnd.draw_frame();
    assert_eq!(child_count(&wnd), 1);
    assert_eq!(*removed[0].state_ref().paint_cnt.borrow(), 3);
    assert_eq!(*removed[1].state_ref().paint_cnt.borrow(), 3);
    assert_eq!(
      *removed[0].state_ref().layout_cnt.borrow(),
      first_layout_cnt
    );

    // the remove pined widget only mark self dirty
    let first_layout_cnt = *removed[0].state_ref().layout_cnt.borrow();
    let secord_layout_cnt = *removed[1].state_ref().layout_cnt.borrow();
    let host_layout_cnt = *tasks.state_ref()[0].state_ref().layout_cnt.borrow();
    removed[0].state_ref().trigger += 1;
    wnd.draw_frame();
    assert_eq!(
      *removed[0].state_ref().layout_cnt.borrow(),
      first_layout_cnt + 1
    );
    assert_eq!(*removed[0].state_ref().paint_cnt.borrow(), 4);
    assert_eq!(
      *removed[1].state_ref().layout_cnt.borrow(),
      secord_layout_cnt
    );
    assert_eq!(
      *tasks.state_ref()[0].state_ref().layout_cnt.borrow(),
      host_layout_cnt
    );

    // when unpined, it will no paint anymore
    removed[0].state_ref().pin = false;
    wnd.draw_frame();
    assert_eq!(*removed[0].state_ref().paint_cnt.borrow(), 4);
    assert_eq!(*removed[1].state_ref().paint_cnt.borrow(), 5);

    // after removed, it will no paint and layout anymore
    let first_layout_cnt = *removed[0].state_ref().layout_cnt.borrow();
    removed[0].state_ref().trigger += 1;
    wnd.draw_frame();
    assert_eq!(*removed[0].state_ref().paint_cnt.borrow(), 4);
    assert_eq!(*removed[1].state_ref().paint_cnt.borrow(), 5);
    assert_eq!(
      *removed[0].state_ref().layout_cnt.borrow(),
      first_layout_cnt
    );

    // other pined widget is work fine.
    let first_layout_cnt = *removed[0].state_ref().layout_cnt.borrow();
    let second_layout_cnt = *removed[1].state_ref().layout_cnt.borrow();
    removed[1].state_ref().trigger += 1;
    wnd.draw_frame();
    assert_eq!(*removed[0].state_ref().paint_cnt.borrow(), 4);
    assert_eq!(*removed[1].state_ref().paint_cnt.borrow(), 6);
    assert_eq!(
      *removed[0].state_ref().layout_cnt.borrow(),
      first_layout_cnt
    );
    assert_eq!(
      *removed[1].state_ref().layout_cnt.borrow(),
      second_layout_cnt + 1,
    );
  }

  #[test]
  fn remove_delay_drop_widgets() {
    let _guard = unsafe { AppCtx::new_lock_scope() };

    let child = Stateful::new(Some(()));
    let child_destroy_until = Stateful::new(false);
    let grandson = Stateful::new(Some(()));
    let grandson_destroy_until = Stateful::new(false);
    let w = widget! {
    states {
      child: child.clone(),
      child_destroy_until: child_destroy_until.clone(),
      grandson: grandson.clone(),
      grandson_destroy_until: grandson_destroy_until.clone(),
    }
    MockMulti {
      Option::map(child.as_ref(), move|_| widget! {
        MockMulti {
          delay_drop_until: *child_destroy_until,
          Option::map(grandson.as_ref(), move|_| widget! {
            MockBox {
              delay_drop_until: *grandson_destroy_until,
              size: Size::zero(),
            }
          })
        }
      })
      }
    };
    let mut wnd = TestWindow::new(w);
    wnd.draw_frame();

    fn tree_arena(wnd: &TestWindow) -> Ref<TreeArena> {
      let tree = wnd.widget_tree.borrow();
      Ref::map(tree, |t| &t.arena)
    }

    let grandson_id = {
      let arena = tree_arena(&wnd);
      let root = wnd.widget_tree.borrow().root();
      root
        .first_child(&arena)
        .unwrap()
        .first_child(&arena)
        .unwrap()
    };

    wnd.draw_frame();
    assert!(!grandson_id.is_dropped(&tree_arena(&wnd)));

    child.state_ref().take();
    wnd.draw_frame();
    assert!(!grandson_id.is_dropped(&tree_arena(&wnd)));

    *child_destroy_until.state_ref() = true;
    wnd.draw_frame();
    assert!(grandson_id.is_dropped(&tree_arena(&wnd)));
  }
}
