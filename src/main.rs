use iced::keyboard::key;
use iced::{
    clipboard, event, keyboard, mouse,
    widget::{column, container, mouse_area, responsive, row, scrollable, stack, text},
    Element, Event, Fill, Font, Point, Shrink, Subscription, Task, Theme,
};
use std::env;

const DIVIDER_WIDTH: f32 = 6.0;

pub fn main() -> iced::Result {
    iced::application(EnvView::default, EnvView::update, EnvView::view)
        .title("Environment Variables")
        .subscription(EnvView::subscription)
        .run()
}

struct EnvView {
    vars: Vec<(String, String)>,
    col_ratio: f32, // left column as a fraction of total width
    dragging: bool,
    drag_start_x: Option<f32>, // set on first CursorMoved after press, not at press time
    drag_start_ratio: f32,
    container_width: f32,
    context_menu: Option<ContextMenu>,
    cursor_pos: Point, // last known window-absolute cursor position
}

// Tracks which row was right-clicked and where to show the menu
struct ContextMenu {
    position: Point,
    row: usize,
    hovered_item: Option<usize>, // 0=Copy name, 1=Copy value, 2=Copy name=value
}

impl Default for EnvView {
    fn default() -> Self {
        let mut vars: Vec<(String, String)> = env::vars().collect();
        vars.sort_by(|a, b| a.0.cmp(&b.0));
        Self {
            vars,
            col_ratio: 0.4,
            dragging: false,
            drag_start_x: None,
            drag_start_ratio: 0.4,
            container_width: 800.0,
            context_menu: None,
            cursor_pos: Point::ORIGIN,
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    DividerPressed(f32), // carries total container width from responsive
    CursorMoved(Point),
    MouseReleased,
    RowRightClicked(usize),
    MenuItemHovered(usize),
    CopyText(String),
    DismissMenu,
}

impl EnvView {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::DividerPressed(total_width) => {
                self.context_menu = None;
                self.dragging = true;
                self.drag_start_x = None;
                self.drag_start_ratio = self.col_ratio;
                if total_width > 0.0 {
                    self.container_width = total_width;
                }
                Task::none()
            }
            Message::CursorMoved(pos) => {
                self.cursor_pos = pos;
                if self.dragging {
                    // Defer anchor to first move so the divider doesn't jump on click
                    let start_x = *self.drag_start_x.get_or_insert(pos.x);
                    let dx = pos.x - start_x;
                    self.col_ratio =
                        (self.drag_start_ratio + dx / self.container_width).clamp(0.1, 0.9);
                }
                Task::none()
            }
            Message::MouseReleased => {
                self.dragging = false;
                self.drag_start_x = None;
                Task::none()
            }
            Message::RowRightClicked(row) => {
                self.context_menu = Some(ContextMenu {
                    position: self.cursor_pos,
                    row,
                    hovered_item: None,
                });
                Task::none()
            }
            Message::MenuItemHovered(i) => {
                if let Some(menu) = &mut self.context_menu {
                    menu.hovered_item = Some(i);
                }
                Task::none()
            }
            Message::CopyText(s) => {
                self.context_menu = None;
                clipboard::write::<Message>(s)
            }
            Message::DismissMenu => {
                self.context_menu = None;
                Task::none()
            }
        }
    }

    // Always-on subscription: tracks cursor for drag, context-menu positioning, and Escape
    fn subscription(&self) -> Subscription<Message> {
        event::listen_with(|ev, _status, _window| match ev {
            Event::Mouse(mouse::Event::CursorMoved { position }) => {
                Some(Message::CursorMoved(position))
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
                Some(Message::MouseReleased)
            }
            Event::Keyboard(keyboard::Event::KeyPressed {
                key: keyboard::Key::Named(key::Named::Escape),
                ..
            }) => Some(Message::DismissMenu),
            _ => None,
        })
    }

    fn view(&self) -> Element<'_, Message> {
        let col_ratio = self.col_ratio;
        let vars = &self.vars;
        let context_menu = &self.context_menu;

        // responsive gives us the actual pixel width so column sizes are exact
        let content = responsive(move |size| {
            let left_w = size.width * col_ratio - DIVIDER_WIDTH / 2.0;
            let right_w = size.width * (1.0 - col_ratio) - DIVIDER_WIDTH / 2.0;
            let total_w = size.width;

            let header = row![
                text("Name").font(Font::MONOSPACE).size(14).width(left_w),
                text("Value").font(Font::MONOSPACE).size(14).width(right_w),
            ]
            .padding([4, 8]);

            let data_rows: Vec<Element<'_, Message>> = vars
                .iter()
                .enumerate()
                .map(|(i, (key, val))| {
                    let row_content = row![
                        text(key.as_str())
                            .font(Font::MONOSPACE)
                            .size(13)
                            .width(left_w),
                        text(val.as_str())
                            .font(Font::MONOSPACE)
                            .size(13)
                            .width(right_w),
                    ]
                    .padding([3, 8]);

                    // Alternate row shading for readability
                    let styled: Element<'_, Message> = if i % 2 == 0 {
                        container(row_content).style(container::rounded_box).into()
                    } else {
                        container(row_content).into()
                    };

                    mouse_area(styled)
                        .on_right_press(Message::RowRightClicked(i))
                        .into()
                })
                .collect();

            let table = column(data_rows).spacing(2).width(Fill);

            // Single divider line overlaid via stack — avoids gaps between rows
            let divider_line = mouse_area(container("").width(DIVIDER_WIDTH).height(Fill).style(
                |theme: &Theme| container::Style {
                    background: Some(theme.extended_palette().background.strong.color.into()),
                    ..Default::default()
                },
            ))
            .on_press(Message::DividerPressed(total_w))
            .interaction(mouse::Interaction::ResizingColumn);

            // Spacer pushes the divider to the correct horizontal position
            let divider_overlay = row![container("").width(left_w), divider_line]
                .width(Fill)
                .height(Fill);

            let mut layers: Vec<Element<'_, Message>> = vec![
                column![header, scrollable(table).height(Fill).width(Fill)]
                    .spacing(4)
                    .width(Fill)
                    .height(Fill)
                    .into(),
                divider_overlay.into(),
            ];

            if let Some(menu) = context_menu {
                if let Some((key, val)) = vars.get(menu.row) {
                    let key = key.clone();
                    let val = val.clone();
                    let pair = format!("{}={}", key, val);

                    let menu_widget = container({
                        let mk_item = |i: usize, label: &'static str, msg: Message| {
                            let highlighted = menu.hovered_item == Some(i);
                            mouse_area(
                                container(text(label).size(13))
                                    .padding([4, 12])
                                    .width(Fill)
                                    .style(move |theme: &Theme| {
                                        if highlighted {
                                            container::Style {
                                                background: Some(
                                                    theme
                                                        .extended_palette()
                                                        .primary
                                                        .weak
                                                        .color
                                                        .into(),
                                                ),
                                                ..Default::default()
                                            }
                                        } else {
                                            container::Style::default()
                                        }
                                    }),
                            )
                            .on_move(move |_| Message::MenuItemHovered(i))
                            .on_press(msg)
                        };
                        column![
                            mk_item(0, "Copy name", Message::CopyText(key.clone())),
                            mk_item(1, "Copy value", Message::CopyText(val.clone())),
                            mk_item(2, "Copy name=value", Message::CopyText(pair)),
                        ]
                        .spacing(0)
                        .padding(4)
                        .width(180)
                    })
                    .style(container::rounded_box);

                    // Subtract outer container padding to get position relative to responsive area
                    const PADDING: f32 = 12.0;
                    let x = (menu.position.x - PADDING).min(total_w - 220.0).max(0.0);
                    let y = (menu.position.y - PADDING).max(0.0);
                    let positioned = row![
                        container("").width(x),
                        column![container("").height(y), menu_widget].width(Shrink),
                    ]
                    .width(Fill)
                    .height(Fill);

                    // Background dismiss area sits behind the menu; the menu items handle their own presses
                    let overlay = mouse_area(positioned)
                        .on_press(Message::DismissMenu)
                        .on_right_press(Message::DismissMenu);

                    layers.push(overlay.into());
                }
            }

            stack(layers).width(Fill).height(Fill).into()
        });

        container(content)
            .padding(12)
            .width(Fill)
            .height(Fill)
            .into()
    }
}
