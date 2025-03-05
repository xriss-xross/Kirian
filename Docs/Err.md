## Compute Pipleline #1

```
error[E0433]: failed to resolve: could not find `ShaderExecution` in `shader`
   --> src\main.rs:84:13
    |
84  | /             vulkano_shaders::shader!{
85  | |                 ty: "compute",
86  | |                 src: r"
87  | |                     #version 450
...   |
99  | |                 ",
100 | |             }
    | |_____________^ could not find `ShaderExecution` in `shader`
    |
    = note: this error originates in the macro `vulkano_shaders::shader` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0412]: cannot find type `ShaderCreationError` in module `vulkano::shader`
   --> src\main.rs:84:13
    |
84  | /             vulkano_shaders::shader!{
85  | |                 ty: "compute",
86  | |                 src: r"
87  | |                     #version 450
...   |
99  | |                 ",
100 | |             }
    | |_____________^ not found in `vulkano::shader`
    |
    = note: this error originates in the macro `vulkano_shaders::shader` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0405]: cannot find trait `SpecializationConstants` in module `vulkano::shader`
   --> src\main.rs:84:13
    |
84  | /             vulkano_shaders::shader!{
85  | |                 ty: "compute",
86  | |                 src: r"
87  | |                     #version 450
...   |
99  | |                 ",
100 | |             }
    | |_____________^ not found in `vulkano::shader`
    |
    = note: this error originates in the macro `vulkano_shaders::shader` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0412]: cannot find type `SpecializationMapEntry` in module `vulkano::shader`
   --> src\main.rs:84:13
    |
84  | /             vulkano_shaders::shader!{
85  | |                 ty: "compute",
86  | |                 src: r"
87  | |                     #version 450
...   |
99  | |                 ",
100 | |             }
    | |_____________^ not found in `vulkano::shader`
    |
    = note: this error originates in the macro `vulkano_shaders::shader` (in Nightly builds, run with -Z macro-backtrace for more info)

Some errors have detailed explanations: E0405, E0412, E0433.
For more information about an error, try `rustc --explain E0405`.
error: could not compile `Kirian` (bin "Kirian") due to 4 previous errors
```