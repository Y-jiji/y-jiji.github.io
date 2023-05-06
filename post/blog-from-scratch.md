---
topic: "Software Engineering"
tag: ["Web Development", "Web Dev"]
abstr: "I tried to construct a blog site without frontend stack, only using browser-native css, html, and javascript. "
date: "2023-05-06"
title: "A Static Site Generator from Scratch"
---

# A Static Site Generator from Scratch

## Current State of Frontend Stack

The frontend stack is too heavy. Although there are some 

## What is a static site?

What is a static site? Basically static sites act just like a read-only file explorer on PC. What's different is the requested files/pages are stored on some internet location, and they can be redirected to somewhere in this site that the site maintainer wants. 

However, this blog site will be as simple as possible and try to be accessible to blog posters, which means I will not go beyond what a file system + page renderer does and deployment is fast and simple. 

## Project Struture

I start from modularization, adopting the classic frontend-backend paradigm. 

In order to act as a page renderer, there should be a page where the content of posts can be plugged in. 

Here I have two options: 

1. render the page on deployment, directly serve from the page
2. render the page on access, use a parser-renderer script to parse markdown into html

In this project, option 2 is chosen, because in option 1, I have to perform complicated redirections on our internal links, since the pages are named into html. Instead, in option 2, redirecting pages are simple and unified. An extra bonus is I also have fast build time performance. 

These code are maintained [here](https://github.com/Y-jiji/blog/blob/main/site/post.html). We use node package `marked@latest` and `mathjax@2.6` to render markdown. 

As I have mentioned in previous sections, a static site should also act like a file explorer, which means I have to maintain some meta-info about our server-side file layout. Therefore I wrote a directory scanner, which will scan the post directory on each deployment and generate JSON meta information based on YAML frontmatter and meta data of local file system. ([scanner script](https://github.com/Y-jiji/blog/tree/main/blog-scan)). 

More specifically, to support topic displayment on the home page, the scanner will generate a topic-list. Later I also add a archive searcher, and the related meta data are `index-*.json` , which can presumably accelerate a query. (WIP)

## Local Testing

To perform local testing before deployment, I created a `site/config.js` file, which maintains the root site and resource location. A typical testing configuration can be: 

```javascript
const root = "http://localhost:3000"
```

Then start local server with

```shell
npx browser-sync start --server
```

## Deployment

To diable the default jeykll CI, an empty `.nojekyll` is needed in the root directory and a redirecting `index.html` to `site/index.html`

```
Mode                 LastWriteTime         Length Name
----                 -------------         ------ ----
d-----          2023/5/6      2:22                .github
d-----          2023/5/5     16:47                blog-scan
d-----          2023/5/6      0:10                info
d-----          2023/5/6      9:15                post
d-----          2023/5/6      1:22                site
-a----          2023/5/6      2:10              0 .nojekyll
-a----          2023/5/6      2:07            174 index.html
-a----          2023/5/6      1:59           1087 LICENSE
```

Then a simple git push will make it work. 

## Future Work

Currently this blog site is already a minimal viable prototype, but it still needs more style sheet and a more friendly archive seach page. 
