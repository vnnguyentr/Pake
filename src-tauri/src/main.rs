// at the top of main.rs - that will prevent the console from showing
#![windows_subsystem = "windows"]
extern crate image;
use tauri_utils::config::{Config, WindowConfig};

#[cfg(target_os="macos")]
use wry::{
    application::{
        accelerator::{Accelerator, SysMods},
        event::{Event, StartCause, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        keyboard::KeyCode,
        menu::{MenuBar as Menu, MenuItem, MenuItemAttributes, MenuType},
        window::{Fullscreen, Window, WindowBuilder},
    },
    webview::WebViewBuilder,
};


#[cfg(target_os="windows")]
use wry::{
    application::{
        event::{Event, StartCause, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        menu::{MenuType},
        window::{Fullscreen, Window, WindowBuilder, Icon},
    },
    webview::WebViewBuilder,
};


#[cfg(target_os="linux")]
use wry::{
    application::{
        event::{Event, StartCause, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        menu::{MenuType},
        window::{Fullscreen, Window, WindowBuilder},
    },
    webview::WebViewBuilder,
};


fn main() -> wry::Result<()> {
    
    #[cfg(target_os = "macos")]
    let mut menu_bar_menu = Menu::new();
    #[cfg(target_os = "macos")]
    let mut first_menu = Menu::new()
        .add_native_item(MenuItem::Hide)
        .add_native_item(MenuItem::EnterFullScreen)
        .add_native_item(MenuItem::Minimize)
        .add_native_item(MenuItem::Separator)
        .add_native_item(MenuItem::Copy)
        .add_native_item(MenuItem::Cut)
        .add_native_item(MenuItem::Paste)
        .add_native_item(MenuItem::Undo)
        .add_native_item(MenuItem::Redo)
        .add_native_item(MenuItem::SelectAll)
        .add_native_item(MenuItem::Separator);
    #[cfg(target_os = "macos")]
    let close_item = first_menu.add_item(
        MenuItemAttributes::new("CloseWindow")
            .with_accelerators(&Accelerator::new(SysMods::Cmd, KeyCode::KeyW)),
    );

    #[cfg(target_os = "macos")]
    first_menu.add_native_item(MenuItem::Quit);

    #[cfg(target_os = "macos")]
    menu_bar_menu.add_submenu("App", true, first_menu);

    let WindowConfig {
        url,
        width,
        height,
        resizable,
        fullscreen,
        ..
    } = get_windows_config().unwrap_or(WindowConfig::default());
    let event_loop = EventLoop::new();

    let common_window = WindowBuilder::new()
        .with_resizable(resizable)
        .with_fullscreen(if fullscreen {
            Some(Fullscreen::Borderless(None))
        } else {
            None
        })
        .with_inner_size(wry::application::dpi::LogicalSize::new(width, height));
    #[cfg(target_os = "windows")]
    let icon_path = concat!(env!("CARGO_MANIFEST_DIR"), "/icons/icon.ico");
    #[cfg(target_os = "windows")]
    let icon = load_icon(std::path::Path::new(icon_path));


    #[cfg(target_os = "windows")]
    let window = common_window
        .with_decorations(true)
        .with_title("")
        .with_window_icon(Some(icon))
        .build(&event_loop)
        .unwrap();

    #[cfg(target_os = "linux")]
    let window = common_window
        .with_title("")
        .build(&event_loop)
        .unwrap(); 

    #[cfg(target_os = "macos")]
    let window = common_window
        .with_fullsize_content_view(true)
        .with_titlebar_buttons_hidden(false)
        .with_titlebar_transparent(transparent)
        .with_title_hidden(true)
        .with_menu(menu_bar_menu)
        .build(&event_loop)
        .unwrap();

    let handler = move |window: &Window, req: String| {
        if req == "drag_window" {
            let _ = window.drag_window();
        } else if req == "fullscreen" {
            if window.fullscreen().is_some() {
                window.set_fullscreen(None);
            } else {
                window.set_fullscreen(Some(Fullscreen::Borderless(None)));
            }
        } else if req.starts_with("open_browser"){
          let href = req.replace("open_browser:", "");
          webbrowser::open(&href).expect("no browser");
        }
    };
    #[cfg(target_os = "windows")]
    let  webview = WebViewBuilder::new(window)?
        .with_url(&url.to_string())?
        .with_devtools(cfg!(feature = "devtools"))
        .with_initialization_script(include_str!("pake.js"))
        .with_ipc_handler(handler)
        .build()?;
    
    
    #[cfg(target_os = "linux")]
    let webview = WebViewBuilder::new(window)?
        .with_url(&url.to_string())?
        .with_devtools(cfg!(feature = "devtools"))
        .with_initialization_script(include_str!("pake.js"))
        .with_ipc_handler(handler)
        .build()?;

    #[cfg(target_os = "macos")]
    let webview = WebViewBuilder::new(window)?
        .with_url(&url.to_string())?
        .with_devtools(cfg!(feature = "devtools"))
        .with_initialization_script(include_str!("pake-mac.js"))
        .with_ipc_handler(handler)
        .build()?;

    #[cfg(feature = "devtools")] {
        webview.open_devtools();
    }

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::NewEvents(StartCause::Init) => println!("Wry has started!"),
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            Event::MenuEvent {
                menu_id,
                origin: MenuType::MenuBar,
                ..
            } => {
                #[cfg(target_os = "macos")]
                if menu_id == close_item.clone().id() {
                    webview.window().set_minimized(true);
                }
                println!("Clicked on {:?}", menu_id);
                println!("Clicked on {:?}", webview.window().is_visible());
            }
            _ => (),
        }
    });
}

fn get_windows_config() -> Option<WindowConfig> {
    let config_file = include_str!("../tauri.conf.json");
    let config: Config = serde_json::from_str(config_file).expect("failed to parse windows config");

    config.tauri.windows.iter().next().cloned()
}

#[cfg(target_os = "windows")]
fn load_icon(path: &std::path::Path) -> Icon {
    let (icon_rgba, icon_width, icon_height) = {
      // alternatively, you can embed the icon in the binary through `include_bytes!` macro and use `image::load_from_memory`
      let image = image::open(path)
        .expect("Failed to open icon path")
        .into_rgba8();
      let (width, height) = image.dimensions();
      let rgba = image.into_raw();
      (rgba, width, height)
    };
    Icon::from_rgba(icon_rgba, icon_width, icon_height).expect("Failed to open icon")
  }
