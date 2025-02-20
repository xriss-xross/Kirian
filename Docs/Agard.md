# Check out me history (of errors)
## Hello World!
Every good graphics engine needs a window.
To start this project I will be using
[winit]("https://crates.io/crates/winit"), the crate for
cross-platform window creation. First thing's first,
create the `EventLoop` which acts as a 'context'. It
initialises everything needed to start creating windows
such as opening a connection to say a Wayland server.
These functionalities are accessed through `create_window`.
This window, once created can generate various events.
The `Window` object can for example generate a window
event when a key is pressed while the window is in focus.
Events are then accessed through `EventLoop::run_app()`
which will run until `exit()` is called. 