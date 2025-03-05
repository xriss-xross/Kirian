## Compute Pipelines

Code/logic that runs on the GPU is called a **shader**. To create a shader source code has to be
written in GLSL. This code is then compiled by Vulkan into SPIR-V which is used by the GPU driver
to perform operations at runtime. GLSL is similar to C but is a whole other language that I will
need to learn serperately. The Vulkano book recomends as practice to mutliply various values by a 
single constant. So that's what I'm going to do.