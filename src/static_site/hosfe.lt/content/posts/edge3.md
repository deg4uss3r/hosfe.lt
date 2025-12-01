---
title: "This Blog is on the Edge Part 3: Optimizing"
date: 2025-11-25T13:51:15-05:00
draft: false
toc: true
images:
tags: 
  - blog
  - Fastly
  - setup
  - tech
  - dns
  - http
  - wasm
  - rust
---

# Part 3: Optimizations

This is part three of how I got my blog on the edge using Fastly's Compute network. In this part I'll go over some small optimizations I have made.

{{< admonition type=note title="Note" >}}
This post will be pretty Fastly specific when it comes to DNS and managing the certificates
{{< /admonition >}}

In case you missed it the [first part is here](https://hosfe.lt/posts/edge/) and the [second part is here](https://hosfe.lt/posts/edge2/).

## Using KV Store

One small change I did a few months ago was move to using KV store to store all of this blogs assets. It makes updating so much easier (but still not automated yet!) and allows me to keep updating the content with needing to do a full deploy which I think is nice.

Finally, it helped decrease load times by 1%, not a huge gain but anything helps 💪!

## Moving to IO on Compute

{{< admonition type=note title="Note" >}}
Quick disclaimer, not only do I work at Fastly but this next section is _my_ team's work and I am a little excited about it!
{{< /admonition >}}

Fastly recently [announced](https://www.fastly.com/blog/meet-image-optimizer-compute-flexible-image-workflows-edge) that Image Optimization is now added to their Compute platform directly from their SDKs. This is a great opportunity to use IO on this blog so I can showcase some photography that I very occasionally do. However before this I'd be uploading massive files and have no control over what gets displayed.

For example, this image below on my drive is a whooping 13.4MB, delivering that size on a website is not the best for user experience nor my bills, so using IO it's the same quality but _much_ smaller!

![large image I took with my DLSR at Mt Hamilton Observatory near San Jose, California](/images/edge-post-3_road.jpeg)

So not only is this now optimized for delivery but it's also _super easy_ to add. Checkout the example below:

```rust
    // my default Image Optimization options
    let mut io_options =
        ImageOptimizerOptions::from_region(fastly::image_optimizer::ImageOptimizerRegion::UsEast);
    io_options.format = Some(fastly::image_optimizer::Format::Auto);

        "/images/edge-post-3_road.jpeg" => {
            Ok(req.with_image_optimizer(io_options).send(IMAGE_ORIGIN_BACKEND)?)
        },
```

Sorry if this seems like too much of an advertizement but I am really proud of what we have built and how easy it is to get started.

## What's Next?

Next up I'll be working on making this site much easier to update by building a release script that does everything I need in CI and automatically uploads all assets. This will hopefully not be too much work but we shall see since I'm using a lot of parts I have customized for me. Thanks for reading!
