# Graphics pipeline introduction
While I actually sort of already went through the graphics pipeline when I dabbled in OpenGL, I
never went to implementing any OpenGL and it's been a while so why not go again? When looking at
the official Vulkan documentation, the intial impression can be very daunting... and I don't think
that the feeling will ever go away.
## Abstracted and simplfied overview
*As provided by [Vulkan Tutorial](https://vulkan-tutorial.com/Drawing_a_triangle/Graphics_pipeline_basics/Introduction)*
1. **Input assembler**

    Collects the raw vertex data from the buffers.

    <img src="../images/graphics pipeline/input assembler.png" alt="input assembler" width="25%"/>
    
2. **Vertex shader**

    Run for every vertex and generally applies transformations to turn vertex positions from model
    space to screen space.

    <img src="../images/graphics pipeline/vertex shader.png" alt="input assembler" width="25%"/>
    
3. **Tessellation**

    Allows for subdivision of geometry based on certain rules to increase the mesh quality. Often
    used to make surfaces like brick walls and staircases look less flat when they are nearby.

    <img src="../images/graphics pipeline/tessellation.png" alt="input assembler" width="25%"/>
    
4. **Geometry shader**

    Run on every primitive (triangle, line, point) and can discard it or output more primitives than
    came in. This is similar to the tessellation shader, but much more flexible. Not used much in
    today's applications because the performance is subpar on most graphics cards bar Intel's
    integrated GPUs.

    <img src="../images/graphics pipeline/geometry shader.png" alt="input assembler" width="25%"/>

5. **Rasterisation**

    Discretizes the primitives into fragments. These are the pixel elements that they fill on the
    framebuffer. Fragments falling outside the screen are discarded and attributes outputted by the
    vertex shader are interpolated across the fragments. Usually the fragments that are behind other
    primitive fragments are also discarded through depth testing.

    <img src="../images/graphics pipeline/rasterisation.png" alt="input assembler" width="25%"/>

6. **Fragment shader**

    Invoked for every fragment that survives and determines which framebuffer(s) the fragments are
    written to and with which color and depth values through the interpolated data from the vertex
    shader. Can include things like texture coordinates and normals for lighting.

    <img src="../images/graphics pipeline/fragment shader.png" alt="input assembler" width="25%"/>

7. **Colour blending**

    Applies operations to mix different fragments that map to the same pixel in the framebuffer.
    Fragments can simply overwrite each other, add up or be mixed based upon transparency.

    <img src="../images/graphics pipeline/colour blending.png" alt="input assembler" width="25%"/>

---

The command buffers used up until making the sine wave performed 2 kinds of operations:
1. Memory transfers (copying data between buffers and clearing an image)
2. Compute operations (dispatching a compute shader)

There is however a third kind of operation:
3. Graphical operations

To be used for graphics GPUs come with specialised well-optimsed steps called
**the graphics pipeline**. The graphics pipeline is more restrictive than compute operations but 
much faster. To use the graphics pipeline as described above to create anything you will need:
- A **graphics pipeline object** that describes the way a GPU should behave like how a compute
pipeline object describes a compute operation 
- One of multiple **buffers** containing the shape(s) we want to draw
- A **framebuffer** object containing a collection of images to write to
- **Descriptor sets** and **push constants**