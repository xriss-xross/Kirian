# Images

Not all content has to be placed into a buffer. One exception to this rule are **images**. Vulkan
describes images as multidemnsional arrays which can store our common perception of images but also
arbitrary data. Our perception of images is usually that they are two-dimensional, but in Vulkan one
can create *one* or even *three* dimensional images which are declared when creating such an image.
When creating an image the format pixels are stored with information are RGBA and so can have up to
four values. Similar to buffers, images are created by providing information about the image and
allocation, but they differ in that they can start in a *uninitialised* state.
