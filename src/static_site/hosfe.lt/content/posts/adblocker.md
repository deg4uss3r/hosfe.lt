---
title: "Adblocker on an Edge Network"
date: 2024-05-15
images:
tags: 
  - rust
  - fastly
  - compute
  - personal project
  - development
  - WASM 
  - DNS
  - Adblocker
---

## Creating a Personal Adblocker over DNS over HTTPS

Like most people I know, I really dislike advertisements. Especially the really creepy ones that are far too relevant to what you _talked_ about not even searched for. Also, like most of my friends I own a [Raspberry Pi](https://www.raspberrypi.org/) and have it running the [Pi-hole](https://pi-hole.net/) software (among other things).

Well, I _did_ have one setup. Ever since my move I have been a little lazy and do not have much room in my apartment for it let alone do not have drops for a ethernet interface. I also wanted a setup that worked well on the go without too much hassle to tunnel into my home network.

So I took a look at what my options were and decided to try to the [Fastly's Compute](https://www.fastly.com/products/compute) network, DNS over HTTPS, and WASM to get the job done. So what I came up with was a DNS over HTTPS provider with built in Pi-hole-style adblocking. You can take a look at the code in the [repo](https://github.com/deg4uss3r/wasm-dns-https), I'll go over a lot of the stuff in the `README` as well as some tradeoffs I had to make with the original idea for now.

## Why DoH

Aside from being a Simpsons fan, DNS queries over HTTPS ([RFC 8484](https://www.rfc-editor.org/rfc/rfc8484)) is a relatively simple spec and implementation and when done properly can be secure. That plus it works nicely using an edge network that can be distributed around the world. Making this a really attractive, fast, and cheap solution! That coupled with being able to block Ads at the DNS level leads to a great solution.

## Original Design and Tradeoffs

Originally, I wanted to send raw DNS packets directly to the edge DNS resolver for authoritative answers; however, WASI-Sockets just [released their specification](https://github.com/WebAssembly/wasi-sockets) into the preview when I was beginning to look into this personal project so that made it not quite ready for prime time, but I look forward to seeing that in stable in the future!

Due to that limitation, I had to go to just pass through requests using another service. Although I am mostly Google-free (I still have my Gmail address I got in the beta back in highschool 👉😎👉) I opted for the [Google API](https://developers.google.com/speed/public-dns/docs/doh/json) since it is clean, well done, and has a lot of options I can use in the future.

Therefore the flow of a request looks like this:

```text
┌───────────────────────────┐              ┌───────────────────────────┐                 ┌───────────────────────────┐
│                           │              │                           │                 │                           │
│                           │              │                           │                 │                           │
│ RECEIVE REQUEST FROM USER ├──────────────►  CHECK IF URL SHOULD BE   ├────────────────X│     BLOCK URL AND SEND    │
│                           │              │           BLOCKED         │                 │       RESPONSE (418)      │
│                           │              │                           │                 │                           │
└───────────────────────────┘              └──────────────┬────────────┘                 └───────────────────────────┘
                                                          │                                                           
                                                          │                                                           
                                                          │                                                           
                                                          │                                                           
                                                          │                                                           
                                                          │                                                           
                                                          │                                                           
                                                          │                                                           
                                            ┌─────────────▼─────────────┐                                             
                                            │                           │                                             
                                            │                           │                                             
                                            │  CHECK CACHE FOR RESPONSE │                                             
                                            │                           │                                             
                                            │                           │                                             
                                            └─────────────┬─────────────┘                                             
                                                          │                                                           
                                                          │                                                           
                                                          │                                                           
                                                          │                                                           
                                                          │                                                           
                                            ┌─────────────┴──────────────┐                                            
                                            │                            │                                            
                                            │                            │                                            
                                            │                            │                                            
                                            │                            │                                            
                             ┌──────────────▼────────────┐ ┌─────────────▼─────────────┐                              
                             │                           │ │                           │                              
                             │                           │ │                           │                              
                             │  SEND REQUEST TO GOOGLE   │ │  SERVE FROM CACHE (200)   │                              
                             │                           │ │                           │                              
                             │                           │ │                           │                              
                             └──────────────┬────────────┘ └───────────────────────────┘                              
                                            │                                                                         
                                            │                                                                         
                             ┌──────────────▼────────────┐                                                            
                             │                           │                                                            
                             │                           │                                                            
                             │  FORWARD RESPONSE (200)   │                                                            
                             │                           │                                                            
                             │                           │                                                            
                             └───────────────────────────┘                                                            
```

### Another Tradeoff

Another current tradeoff I am making while I look to get a KV Store implementation is I am storing the "massive" 5.1MB list of blocked URLs in memory during each spin up of of the application. Obviously this is inefficient but the list was too large for the normal storage options (and honestly that would be a pretty big abuse of things like config store).

The current plan is to use a KV store that is accessible from every instance, and if needed utilize a [k-anonymous](https://en.wikipedia.org/wiki/K-anonymity) key to reliable get back a set of values that will contain the result but be able to pack more values into each KV pair if it is too large still. I have left this for a future implementation/optimization for now.

Another thing I would like to do is make a script that I can easily add to the list (or remove if necessary). It will probably be a separate script to do so but it will be nice since a lot of the editors struggle with the size of this file and doing so with the [Fastly CLI](https://www.fastly.com/documentation/reference/cli/kv-store-entry/) appears easy.

### Fun Optimization

A very neat optimization of using an edge network (especially Fastly's unique architecture) is the ability to store results in cache effectively skipping the lookup to Google's APIs all together and _more than halving_ (from about 90,000μs to about 40,000μs (about .1 -> 0.04 seconds)) the response time from a full lookup, that's a pretty incredible speedup for just a few lines of code!

This is implemented to only store the URL in cache after the result is returned to the user so I never store anything that is blocked (and a blocked URL exits immediately after the positive lookup to the array). This is a pretty neat feature but does have a slight hiccup that at each Point of Presence (POP) has an individual cache, so for example if I am traveling I won't get the same immediate benefits of the quick lookups on the first response. However any subsequent lookups will be stored an easily revisited. The other issue is cache is used for everyone so anything we access here if unpopular can be overridden, so the more I use it the faster it would get!

### Logging and Observability

In order to try out a new observability platform during this project I started using [honeycomb](https://honeycomb.io). This leads us into a warning:

{{< admonition type=danger title="WARNING" >}}Please do not use my service if you care anything about privacy as since this is a personal project I can see all of the requests and where they come from.

It is a (far) future optimization for me to make this more privacy conscious.
{{< /admonition >}}

That being said I have some really good observability in place with support for dynamically logging any additional fields. Using the following option in the Honeycomb UI:

![Image on Honeycomb's site showing an unpack nested JSON toggle and the depth to look for](/images/honeycomb.png "Image on Honeycomb's site showing an unpack nested JSON toggle and the depth to look for")

One trip-up I hit while getting logging setup was I _had_ to send a specific shape to the Honeycomb service for it to be parsed properly, it wasn't explicitly said in any of the documentation I saw and it's fairly simple but it has to look like this:

```rust
struct LogFormat {
    time: String,
    data: LogData,
}
```

The `LogData` there can be anything (I use 2 levels of nested structures) that can be deserialized into JSON but it must take that overall shape to be parsed correctly. For example here is the shape of my `LogData`:

```rust
struct LogData {
    id: String,
    level: String,
    fastly_version: u32,
    message: String,
    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    additional_info: HashMap<String, String>,
}
```

This gives me a good way (since the ID is a Fastly `$ENVAR` constant per compute request) to find the flow of a single request and know which version of my application this processed on (helpful if I am looking at an update and waiting for the new version to deploy).

## Results

So onto the results! I have not started using this as a majority daily driver yet (I am using Firefox developer edition to test this alongside vanilla Firefox). I need to get a verified `plist` entry for my personal devices to use this system-wide and not just through the browser to be more useful. The short of the results are I am seeing the average page load to be about _twice as fast_, when they are served from cache about _4 times as fast_ for some of the worse offenders. With no "we see you have an adblocker" interstitial or pop ups!

I won't get into what I think that does to the web but I am glad I have yet-another-way™️ to block advertisements that I do not want to see and ads no daily value to my web experience.

Overall, this was a really fun project. I learned a ton about DNS, working on an edge networking, OS configuration, and observability.

## Next Steps

I am looking forward to making this a bit more secure and setting up some `plist`s to run this with signing on MacOS and iOS so I can use this all the time. I'll throw some updates on the repository when that's done, stay tuned or file an issue if you notice anything that could be improved!
