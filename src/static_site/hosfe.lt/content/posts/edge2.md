---
title: "This Blog is on the Edge Part 2: The Journey Continues"
date: 2024-01-22T16:16:18-05:00
draft: false
toc: true
images:
tags: 
  - untagged
---

# Part 2: The Journey Continues

This is part two of how I got my blog on the edge using Fastly's Compute network. In this part I'll go over DNS, SSL, and what I use to build and deploy the blog in more detail. 

{{< admonition type=note title="Note" >}}
This post will be pretty Fastly specific when it comes to DNS and managing the certificates
{{< /admonition >}}

In case you missed it the [first part is here](https://hosfe.lt/posts/edge/). 

## It's Always DNS

I will admit I am not an expert here and what I have works for me and I had to run through a few trail and error runs to get what I wanted working.

With that out of the way the first thing to do is take your domain name and in the [Fastly Admin Portal](https://manage.fastly.com) on your compute service from part 1 add them to the  `Domains` section (this is the exact URL users will input when they want to visit your site.).

Here I have both (hosfe.lt and www.hosfe.lt). In the Fastly world it is better to have a subdomain (`www` or the like, I am planning on using `ricky` in the near future). which I'll explain why later. 

Once this is done move onto the `Hosts`, here is the specific host you want to use (this can be just your root level domain e.g. hosfe.lt) as it will be the one the certificate covers. Normally this would be your origin but since this site is completely on the edge we are just using it for certificate security at this point. 

## TLS

Now to secure the site we need to use the `Secure` feature of the dashboard. Here we can generate certificates for our host and any subdomains (I used the wildcard domain since I'll probably be moving from the root to `ricky.hosfe.lt` in the future). If you can I would recommend doing your root plus whatever subdomains you want (or wildcard).

This step will also required you to put in an `_acme-challenge` to verify you actually own your domain. I use Digital Ocean for my domain networking and throwing that `CNAME` in along with it's value was easy. The first certificate took about 10 minutes to verify and propagate after that they have been less than a minute. 

## DNS Part 2: There's Always a Part 2 

Once, you have your TLS certificate setup you should be good to go! In order to take full advantage of the edge you need to use a subdomain. This is because root domains are not allows to have a `CNAME` entry for them and must point to an `A` or `AAAA` record. You _can_ use Compute with a root domain and I'll show you how to find that here.

After we have created our TLS under the secure tab we need to know _which_ type of certificate we created. Going into the `Secure` App and `TLS Management` area we can click to view the certificate to see if we have a `t` or `s` type. 

![Which certificate type is active](/images/edge-post-2_certificate.png) 

Once we know that we can give our subdomain a `CNAME` that Fastly will utilize over the whole edge network. If you need more help there's a lot more info on the [developer docs](https://docs.fastly.com/en/guides/working-with-cname-records-and-your-dns-provider#tls-enabled-hostnames)

While we are here we can also give the root domain an IP address to hit if you are keen on using that. This won't take advantage of the whole edge at first so this is why I mentioned previously that it was better to use a subdomain. There's a lot more info [here](https://docs.fastly.com/en/guides/using-fastly-with-apex-domains#when-you-have-tls-configured).

That's it, you should now have a working secured website that is completely on the Fastly Compute network! Honestly, it's more work than GeoCities was but given the flexibility I am exited for the [internet to get weird again](https://www.rollingstone.com/culture/culture-commentary/internet-future-about-to-get-weird-1234938403/).

## Hugo 

In case you're wondering the static site generator I am currently using is [Hugo](https://gohugo.io). The Theme is [hermit-v2](https://themes.gohugo.io/themes/hermit-v2/). Realistically I wanted a simple yet customizable theme and went with the first I found. I can also change it out prety easily. Top that with an incredibly simple build system (literally compiling the site by just `hugo`) and I was sold. 


## How to Deploy and GitHub Actions

Lastly, how I currently deploy the blog. You can see my [GitHub Actions YAML here](https://github.com/deg4uss3r/hosfe.lt/blob/main/.github/workflows/test.yml). The important part is that I split this in two stages. A test and check on each commit/PR and a deploy only on a tag. This helps me control when I actually push content to the blog. 

```yaml
deploy:
    if: |
      startsWith(github.ref, 'refs/tags/v') && needs.test.result == 'success'
```

I love that little `if` statement. It will only deploy if the test suite was successful on a tagged run. 

Further one thing I found confusing at first was I needed to use the Fastly `fastly/compute-actions@v5` action not just the `deploy` action as it still needs to compile and ship the WASM binary. After that it was pretty easy to get working, it will use the `fastly.toml` at the root of the project and off it goes just like doing it via the CLI.

{{< admonition type=danger title="WARNING" >}}
Don't forget to store your Fastly API token in GitHub Secrets so you don't expose it!
{{< /admonition >}}

```yaml
      env:
        FASTLY_API_TOKEN: ${{ secrets.FASTLY_API_TOKEN }}
```

## What's next?

Well for the website I got some great suggestions and probably the first one will be compressing the included HTML text and shrinking the size of the WASM binary that gets uploaded (right now sitting at 1.78Mb).

Next, I would like to add much more logging and statistics to understand traffic flow and be able to play with observability and what's possible with the compute platform (with the added benefit of knowing how little people have read the blog). 

 Slowly but surely I'll get to those but I also want to sprinkle in writing on some topics I want to chat about, so see you soon and I hope this helps!
