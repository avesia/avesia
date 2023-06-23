mod server;

static APP_DEFAULT_URL: &str = "avs://localhost/index.html";

pub struct AvesiaDesktopOptions {
    pub url: String,
}

impl Default for AvesiaDesktopOptions {
    fn default() -> Self {
        Self {
            url: APP_DEFAULT_URL.to_owned()
        }
    }
}

#[derive(Default)]
pub struct AvesiaDesktop {
    options: AvesiaDesktopOptions,
}

impl AvesiaDesktop {
    pub fn new() -> Self {
        AvesiaDesktop::default()
    }

    pub fn with_options(&mut self, options: AvesiaDesktopOptions) -> &mut Self {
        self.options = options;

        self
    }

    pub fn start(&mut self) -> wry::Result<()> {
        use wry::{
            application::{
                event::{Event, StartCause, WindowEvent},
                event_loop::{ControlFlow, EventLoop},
                window::WindowBuilder,
            },
            webview::WebViewBuilder,
        };
    
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_title("avesia")
            .build(&event_loop)
            .unwrap();

        let _webview = WebViewBuilder::new(window)
            .unwrap()
            .with_custom_protocol("avs".into(), server::server)
            // tell the webview to load the custom protocol
            .with_url(self.options.url.as_str())?
            .with_devtools(true)
            .build()?;
    
        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;
    
            match event {
                Event::NewEvents(StartCause::Init) => println!("Wry application started!"),
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => *control_flow = ControlFlow::Exit,
                _ => (),
            }
        });
    }    
}
