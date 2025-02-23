# Project Start

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

So once we have our valid queue family we use it to create our device interfact to the physical
device. Slightly confusing names but in such a verbose graphics API like Vulkan, something are bound
to clash. It is at this stage that we can now ask (very politely) the GPU to perform some
operations.

## Memory allocation

Before creating buffers in memory (where I got to with OpenGL) I have to allocate some memory for
said memory buffers. From my experience in languages such as C and Zig, I have not found this very
fun even if Zig tried to make it as much of an intutitive experience as possible. For now, defaults
will do nicely. In the `memory_allocator`, `device.clone()` is passed as a paramater. Cloning the
Arc is not as expensive as the actual object itself. Device has been declared as an `Arc<device>`
meaning it is handled by the `Arc` smart pointer allowing for shared ownership (and still no
garbage collector in sight!).

## Buffers

Sending information to the GPU is relatively slow. To perform this task more efficiently, a memory
buffer is created to send information into less frequent, larger chunks. There are sevel **memory
types** to chose from each being suited to their own tasks just like queues. To chose memory, we
provide a **memory type filter** which informs the memory allocator which memory we prefer and which
memory we prefer to avoid. Some examples:
- `MemoryTypeFilter::PREFER_DEVICE | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE`
    for continuously uploading data to the GPU
- `MemoryTypeFilter::PREFER_DEVICE`
    increased performance but more complex data access from the CPU
Either side of the `|` are some filters and in the second example, we even only have 1 filter. To
create a buffer, I will first allocate memory for it in memory accesible by the CPU. In the buffer,
notice that the first parameter is an `Arc` of that memory allocator we created earlier. We then
specify what we are using the buffer for, and then we create the information of the allocation. The
`memory_type_filter` is looking for `HOST_SEQUENTIAL_WRITE` memory which is good for sending a
steady stream of data. In future I may change this to accomodate rapidly changing matricies for
example with `HOST_RANDOM_ACCESS`. The final paramater is what we actualy want to send. In this case
we will send some of the most improtant information ever conceived:
>The Ultimate Question of Life, the Universe, and Everything - Douglas Adams
42.

---

### Plain old data (POD)

Vulkan works with raw memory. This means that it doesn't understand high level Rust structs or
enums. Information needs to be laid out in a way that Vulkan can understand. POD refers to
simplified data that Vulkan can understand with no hidden complexities such as pointers. A `struct`
containing the data I want to pass is now deriving `BufferContents` which is a more convenient way
of using `bytemuck` which in itself tell the compiler that a Rust type is safe to be interpreted as
a sequence of bytes. When working with simple types such as `i32` or `u8` I could get away with
using `bytemuch` but especially when working with more complex types, i need to make sure I'm using
one of these *marker traits* to ensure that data is laid out for Vulkan to use.

---

However, despite these giant leaps, most of the time we want to pass a series of values inside of a
buffer, not just one measly struct. Vulkano provides `from_data` to do this. The problem with this
is that if the amount of data we want to pass into the buffer needs to be known. If it isn't we must
use `from_iter` constructer which takes an iterator as a parameter instead of the data itself.

# Windows

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
