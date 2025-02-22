# Check out me history (of errors)

## Hello World!

Vulkan is a modern graphics API that I want to use to learn about low level programming and what
goes into putting something on my screen. To start I need to initialise a Rust wrapper for Vulkan
called Vulkano which allows for Rust idiomatic programming an envirionment that tends to be very
unsafe. `UNUMERATE_PORTABILITY` will allow for support in devices that do not fully conform to the
Vulkan specification. Creating an instance returns a `Result`. In the case of an error, the program
terminates and a message is printed to the console. In a production ready build, the error would be
handled a little more eloquently in the form of perhaps a dialogue box.

Next, we must chose a device on which to perform operations on. SLI is a thing of the past and the
best option to pick a device to use is to enumerate over them and pick one. There is again some
error handling but it is unlikely to fail in the average case. In some rare cases Vulkan may not be
supported on any installed physical device in which case no device can be selected.

To then communicate with this **physical** device we must perform device creation - an object
being the open channel of communication with said physical device. To do this we must also tell the
Vulkan implementation which type of queues we want to use. Queues are grouped into their queue
families which describe their capabilities.

---

### What are queues?

Just like how multithreading allows for multiple tasks to run in parrallel on a CPU, multiple
operations can also be run on your GPU. The Vulkan version of parallel processing is a **queue**.
Whenever we want the device to do something, we submit an operation to a queue. Some queues
specialise in graphics and other may be compute operations.

---



## Windows

Every good graphics engine needs a window. To start this project I will be using
[winit]("https://crates.io/crates/winit"), the crate for **cross-platform window creation**. First
thing's first, create the `EventLoop` which acts as a 'context'. It initialises everything needed to
start creating windows such as opening a connection to say a Wayland server. These functionalities
are accessed through `create_window`. This window, once created can generate various events. The
`Window` object can for example generate a window event when a key is pressed while the window is in
focus. Events are then accessed through `EventLoop::run_app()` which will run until `exit()` is
called. After setting some window attributes for for our... well window, we can check for several
different `window_event`s such as `CloseRequested` and `RedrawRequested`. The event loop itself
is set to `ControlFlow::Poll` which continuously runs the event loop even if the OS hasn't
dispatched any events which is ideal for games and similar applications. `WindowId` is also a
member of `WindowEvent` but this is only relevant for multi-window applications which is not
something I will be persuing.
