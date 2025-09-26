# Data Race

A data race is when 2 instructions try to access memory at the same time. As we are asking the GPU
to perform tasks in parrellel this can be achieved quite easily. And doing so can lead to an
undefined result.