use ui::{prelude::*, ContextMenu, NumericStepper, PopoverMenu, Tooltip};

#[derive(IntoElement)]
pub struct ApplicationMenu;

impl ApplicationMenu {
    pub fn new() -> Self {
        Self
    }
}

impl RenderOnce for ApplicationMenu {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        PopoverMenu::new("application-menu")
            .menu(move |cx| {
                ContextMenu::build(cx, move |menu, cx| {
                    menu.header("工作区")
                        .action("打开命令面板", Box::new(command_palette::Toggle))
                        .when_some(cx.focused(), |menu, focused| menu.context(focused))
                        .custom_row(move |cx| {
                            h_flex()
                                .gap_2()
                                .w_full()
                                .justify_between()
                                .cursor(gpui::CursorStyle::Arrow)
                                .child(Label::new("缓冲区字体大小"))
                                .child(
                                    NumericStepper::new(
                                        theme::get_buffer_font_size(cx).to_string(),
                                        |_, cx| {
                                            cx.dispatch_action(Box::new(
                                                zed_actions::DecreaseBufferFontSize,
                                            ))
                                        },
                                        |_, cx| {
                                            cx.dispatch_action(Box::new(
                                                zed_actions::IncreaseBufferFontSize,
                                            ))
                                        },
                                    )
                                    .when(
                                        theme::has_adjusted_buffer_font_size(cx),
                                        |stepper| {
                                            stepper.on_reset(|_, cx| {
                                                cx.dispatch_action(Box::new(
                                                    zed_actions::ResetBufferFontSize,
                                                ))
                                            })
                                        },
                                    ),
                                )
                                .into_any_element()
                        })
                        .custom_row(move |cx| {
                            h_flex()
                                .gap_2()
                                .w_full()
                                .justify_between()
                                .cursor(gpui::CursorStyle::Arrow)
                                .child(Label::new("UI 字体大小"))
                                .child(
                                    NumericStepper::new(
                                        theme::get_ui_font_size(cx).to_string(),
                                        |_, cx| {
                                            cx.dispatch_action(Box::new(
                                                zed_actions::DecreaseUiFontSize,
                                            ))
                                        },
                                        |_, cx| {
                                            cx.dispatch_action(Box::new(
                                                zed_actions::IncreaseUiFontSize,
                                            ))
                                        },
                                    )
                                    .when(
                                        theme::has_adjusted_ui_font_size(cx),
                                        |stepper| {
                                            stepper.on_reset(|_, cx| {
                                                cx.dispatch_action(Box::new(
                                                    zed_actions::ResetUiFontSize,
                                                ))
                                            })
                                        },
                                    ),
                                )
                                .into_any_element()
                        })
                        .header("项目")
                        .action(
                            "将文件夹添加到项目...",
                            Box::new(workspace::AddFolderToProject),
                        )
                        .action("打开一个新项目...", Box::new(workspace::Open))
                        .action(
                            "打开最近的项目...",
                            Box::new(recent_projects::OpenRecent {
                                create_new_window: false,
                            }),
                        )
                        .header("帮助")
                        .action("关于 Zed", Box::new(zed_actions::About))
                        .action("欢迎", Box::new(workspace::Welcome))
                        .link(
                            "文档",
                            Box::new(zed_actions::OpenBrowser {
                                url: "https://zed.dev/docs".into(),
                            }),
                        )
                        .action("提供反馈", Box::new(feedback::GiveFeedback))
                        .action("检查更新", Box::new(auto_update::Check))
                        .action("查看遥测数据", Box::new(zed_actions::OpenTelemetryLog))
                        .action(
                            "查看依赖项许可证",
                            Box::new(zed_actions::OpenLicenses),
                        )
                        .separator()
                        .action("退出", Box::new(zed_actions::Quit))
                })
                .into()
            })
            .trigger(
                IconButton::new("application-menu", ui::IconName::Menu)
                    .style(ButtonStyle::Subtle)
                    .tooltip(|cx| Tooltip::text("Open Application Menu", cx))
                    .icon_size(IconSize::Small),
            )
            .into_any_element()
    }
}
