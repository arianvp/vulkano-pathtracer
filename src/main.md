# Using multiple queues in Vulkan - A case study

Many introduction Vulkan tutorials float around the web, that either focus on
one of the two subjects:
* Do graphics with a single graphics queue
* Do compute with a single compute queue, and write back the results to disk

However, a non-trivial Vulkan application ought to use as much of the hardware
as possible.  This means, using compute and graphics queues in parallel, and
sharing results between them.  This is  never dealt with in existing tutorials,
and I missed a nice introductory tutorial on how you select proper queues, and
how you can combine multiple queues to for example use compute shaders within
your standard graphics pipeline.

In this blog series, we will be working on an interactive path tracer. Path
tracing is an interesting subject, because it has many interesting properties
in which Vulkan can be of help.  First of all, for a path tracer to be
performant, an acceleration structure (like A BVH) needs to be maintained, and
in an interactive tracer, needs to be asynchronously updated in the background.
This likely happens on the CPU, and we will want to communicate the results of
these recalculations without waiting on the GPU or CPU.

Second of all, for maximum occupancy, a path tracer is usually split up in
multiple phases [0]
* Primary ray casting
* Scene traversal
* Shadow ray casting
* Shading

This is a pipeline that does not fit in the classical graphics pipeline model,
so we'll have to create a 'custom' pipeline using multiple Compute Shaders.
This will teach us how to have interdependencies between shaders.

Finally, devices may expose multiple compute queues. We would like to balance
work between these queues to make optimal use of the hardware at hand. This
will force us to learn about how to synchronise queues.



[0](http://research.nvidia.com/publication/megakernels-considered-harmful-wavefront-path-tracing-gpus)

```
