use collab_ui::collab_panel;
use gpui::{Menu, MenuItem, OsAction};
use terminal_view::terminal_panel;

pub fn app_menus() -> Vec<Menu> {
    use zed_actions::Quit;

    vec![
        Menu {
            name: "Zed".into(),
            items: vec![
                MenuItem::action("关于 Zed...", zed_actions::About),
                MenuItem::action("检查更新", auto_update::Check),
                MenuItem::separator(),
                MenuItem::submenu(Menu {
                    name: "首选项".into(),
                    items: vec![
                        MenuItem::action("打开设置", super::OpenSettings),
                        MenuItem::action("打开按键绑定", zed_actions::OpenKeymap),
                        MenuItem::action("打开默认设置", super::OpenDefaultSettings),
                        MenuItem::action("打开默认键绑定", super::OpenDefaultKeymap),
                        MenuItem::action("打开本地设置", super::OpenLocalSettings),
                        MenuItem::action("选择主题...", theme_selector::Toggle::default()),
                    ],
                }),
                MenuItem::action("扩展", extensions_ui::Extensions),
                MenuItem::action("安装 CLI", install_cli::Install),
                MenuItem::separator(),
                MenuItem::action("隐藏 Zed", super::Hide),
                MenuItem::action("隐藏其他", super::HideOthers),
                MenuItem::action("显示全部", super::ShowAll),
                MenuItem::action("退出", Quit),
            ],
        },
        Menu {
            name: "文件".into(),
            items: vec![
                MenuItem::action("新建", workspace::NewFile),
                MenuItem::action("新建窗口", workspace::NewWindow),
                MenuItem::separator(),
                MenuItem::action("打开...", workspace::Open),
                MenuItem::action(
                    "打开最近...",
                    recent_projects::OpenRecent {
                        create_new_window: true,
                    },
                ),
                MenuItem::separator(),
                MenuItem::action("将文件夹添加到项目...", workspace::AddFolderToProject),
                MenuItem::action("保存", workspace::Save { save_intent: None }),
                MenuItem::action("另存为...", workspace::SaveAs),
                MenuItem::action("保存所有", workspace::SaveAll { save_intent: None }),
                MenuItem::action(
                    "关闭编辑器",
                    workspace::CloseActiveItem { save_intent: None },
                ),
                MenuItem::action("关闭窗口", workspace::CloseWindow),
            ],
        },
        Menu {
            name: "编辑".into(),
            items: vec![
                MenuItem::os_action("撤消", editor::actions::Undo, OsAction::Undo),
                MenuItem::os_action("重做", editor::actions::Redo, OsAction::Redo),
                MenuItem::separator(),
                MenuItem::os_action("剪切", editor::actions::Cut, OsAction::Cut),
                MenuItem::os_action("复制", editor::actions::Copy, OsAction::Copy),
                MenuItem::os_action("粘贴", editor::actions::Paste, OsAction::Paste),
                MenuItem::separator(),
                MenuItem::action("查找", search::buffer_search::Deploy::find()),
                MenuItem::action("在项目中查找", workspace::DeploySearch::find()),
                MenuItem::separator(),
                MenuItem::action(
                    "切换行注释",
                    editor::actions::ToggleComments::default(),
                ),
            ],
        },
        Menu {
            name: "选择".into(),
            items: vec![
                MenuItem::os_action(
                    "全部选择",
                    editor::actions::SelectAll,
                    OsAction::SelectAll,
                ),
                MenuItem::action("扩大选择", editor::actions::SelectLargerSyntaxNode),
                MenuItem::action("缩小选择", editor::actions::SelectSmallerSyntaxNode),
                MenuItem::separator(),
                MenuItem::action("在上方添加光标", editor::actions::AddSelectionAbove),
                MenuItem::action("在下方添加光标", editor::actions::AddSelectionBelow),
                MenuItem::action(
                    "选择下一个",
                    editor::actions::SelectNext {
                        replace_newest: false,
                    },
                ),
                MenuItem::separator(),
                MenuItem::action("向上移动列", editor::actions::MoveLineUp),
                MenuItem::action("向下移动列", editor::actions::MoveLineDown),
                MenuItem::action("重复选择", editor::actions::DuplicateLineDown),
            ],
        },
        Menu {
            name: "查看".into(),
            items: vec![
                MenuItem::action("放大", zed_actions::IncreaseBufferFontSize),
                MenuItem::action("缩小", zed_actions::DecreaseBufferFontSize),
                MenuItem::action("重置缩放", zed_actions::ResetBufferFontSize),
                MenuItem::separator(),
                MenuItem::action("切换左侧停靠栏", workspace::ToggleLeftDock),
                MenuItem::action("切换右侧停靠栏", workspace::ToggleRightDock),
                MenuItem::action("切换底部停靠栏", workspace::ToggleBottomDock),
                MenuItem::action("关闭所有停靠栏", workspace::CloseAllDocks),
                MenuItem::submenu(Menu {
                    name: "编辑器布局".into(),
                    items: vec![
                        MenuItem::action("向上分割", workspace::SplitUp),
                        MenuItem::action("向下分割", workspace::SplitDown),
                        MenuItem::action("左侧分割", workspace::SplitLeft),
                        MenuItem::action("右侧分割", workspace::SplitRight),
                    ],
                }),
                MenuItem::separator(),
                MenuItem::action("项目面板", project_panel::ToggleFocus),
                MenuItem::action("大纲面板", outline_panel::ToggleFocus),
                MenuItem::action("协作面板", collab_panel::ToggleFocus),
                MenuItem::action("终端面板", terminal_panel::ToggleFocus),
                MenuItem::separator(),
                MenuItem::action("诊断", diagnostics::Deploy),
                MenuItem::separator(),
            ],
        },
        Menu {
            name: "转到".into(),
            items: vec![
                MenuItem::action("返回", workspace::GoBack),
                MenuItem::action("向前", workspace::GoForward),
                MenuItem::separator(),
                MenuItem::action("命令面板...", command_palette::Toggle),
                MenuItem::separator(),
                MenuItem::action("转到文件...", workspace::ToggleFileFinder::default()),
                // MenuItem::action("Go to Symbol in Project", project_symbols::Toggle),
                MenuItem::action("转到编辑器中的符号...", editor::actions::ToggleOutline),
                MenuItem::action("转到行/列...", editor::actions::ToggleGoToLine),
                MenuItem::separator(),
                MenuItem::action("转到定义", editor::actions::GoToDefinition),
                MenuItem::action("转到类型定义", editor::actions::GoToTypeDefinition),
                MenuItem::action("查找所有参考资料", editor::actions::FindAllReferences),
                MenuItem::separator(),
                MenuItem::action("下一个问题", editor::actions::GoToDiagnostic),
                MenuItem::action("上一个问题", editor::actions::GoToPrevDiagnostic),
            ],
        },
        Menu {
            name: "窗口".into(),
            items: vec![
                MenuItem::action("最小化", super::Minimize),
                MenuItem::action("缩放", super::Zoom),
                MenuItem::separator(),
            ],
        },
        Menu {
            name: "帮助".into(),
            items: vec![
                MenuItem::action("查看遥测数据", zed_actions::OpenTelemetryLog),
                MenuItem::action("查看依赖项许可证", zed_actions::OpenLicenses),
                MenuItem::action("显示欢迎", workspace::Welcome),
                MenuItem::action("提供反馈...", feedback::GiveFeedback),
                MenuItem::separator(),
                MenuItem::action(
                    "文档",
                    super::OpenBrowser {
                        url: "https://zed.dev/docs".into(),
                    },
                ),
                MenuItem::action(
                    "Zed Twitter",
                    super::OpenBrowser {
                        url: "https://twitter.com/zeddotdev".into(),
                    },
                ),
                MenuItem::action(
                    "加入团队",
                    super::OpenBrowser {
                        url: "https://zed.dev/jobs".into(),
                    },
                ),
            ],
        },
    ]
}
