---
topic: "Software Engineering"
tag: ["Web Development", "Web Dev"]
abstr: "I tried to construct a blog site without frontend stack, only using browser-native CSS, HTML, and javascript. "
date: "2023-05-06"
title: "A Static Site Generator from Scratch"
---

# A Static Site Generator from Scratch

## Frontend Stack v.s. Browser Native

The frontend stack is too heavy. Although there are some deficiencies in the browser-native tools speaking of production-level frontend engineering, generating some static sites is just within its capability zone. For simplicity, browser native tools are enough. 

## What is a static site?

What is a static site? Static sites act just like a read-only file explorer on a PC. What's different is the requested files/pages are stored on some internet location, and they can be redirected to somewhere on this site that the site maintainer wants. 

However, this blog site will be as simple as possible and try to be accessible to blog posters, which means I will not go beyond what a file system + page renderer does and deployment is fast and simple. 

## Project Structure

I start with modularization, adopting the classic frontend-backend paradigm. 

To act as a page renderer, there should be a page where the content of posts can be plugged in. 

Here I have two options: 

1. render the page on deployment, directly serve from the page
2. render the page on access, use a parser-renderer script to parse markdown into HTML

In this project, option 2 is chosen, because, in option 1, I have to perform complicated redirections on our internal links, since the pages are named in HTML. Instead, in option 2, redirecting pages are simple and unified. A bonus is I also have fast build-time performance. 

These codes are maintained [here](https://github.com/Y-jiji/blog/blob/main/site/post.html). We use node packages `marked@latest` and `mathjax@2.6` to render markdown. 

As I have mentioned in previous sections, a static site should also act like a file explorer, which means I have to maintain some meta-info about our server-side file layout. Therefore I wrote a directory scanner, which will scan the post directory on each deployment and generate JSON meta information based on YAML frontmatter and metadata of the local file system. ([scanner script](https://github.com/Y-jiji/blog/tree/main/blog-scan)). 

More specifically, the scanner will generate a `topic-list.json` to support topic displayment on the home page. Later I also add an archive searcher and the related metadata are `index-*.json`, which can presumably accelerate a query. (WIP)

## Local Testing

To perform local testing before deployment, I created a `site/config.js` file, which maintains the root site and resource location. A typical testing configuration can be: 

```javascript
const root = "http://localhost:3000"
```

Then start the local server with

```shell
npx browser-sync start --server
```

## Deployment

To disable the default Jekyll CI, an empty `.nojekyll` is needed in the root directory and a redirecting `index.html` to `site/index.html`

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

Generate essential meta information with: 

```
cargo -Z unstable-options -C blog-scan run --release -- -D ..
```

Then a simple git push will make it work. 

## Optimization

Loading big javascript files can be potentially slow. Fortunately, in html5, we can add `defer` attribute to scripts to fetch them in parallel. 

In practice, I only marked external utilities as `defer`, and add spinning locks to wait for some objects. 

## Prettify the Page

I add some interactive functionalities comparable to modern frontend enigneering. For example, these code appear in `site/post.html`. 

I don't know if several event listeners may run in parallel, so I add some brainless locks. As all variables in javascript is volatile. 

A long scroll or moving mouse to the top will reveal the navigation bar. 

Correspondently, leaving the mouse from the navigation bar wiil hide it away, and scrolling down will do the same job. 

More style sheet stuff is added and make the page look better. 

```javascript
// a scrolling var
var scrollingLock = false;
var lastScrollTop = 0;
// hide navigation bar
function hideNavigation() {
    let navi = document.querySelector("#navi-wrapper");
    let naviClass = navi.getAttribute("class") || "";
    navi.setAttribute("class", naviClass + " hidding");
}
// reveal navigation bar
function revealNavigation() {
    let navi = document.querySelector("#navi-wrapper");
    let naviClass = navi.getAttribute("class") || "";
    navi.setAttribute("class", (naviClass + " ").replaceAll("hidding ", ""));
}
// reveal navigation bar when scrolling up
// hide navigation bar when scrolling down
window.addEventListener("scroll", (_ev) => {
    if(scrollingLock) { return; }
    scrollingLock = true;
    var st = window.pageYOffset || document.documentElement.scrollTop;
    if (st > lastScrollTop) {
        hideNavigation();
    } else if (st < lastScrollTop - 20) {
        revealNavigation();
    }
    lastScrollTop = st <= 0 ? 0 : st;
    scrollingLock = false;
})
// reveal navigation bar when mouse is moved to the top
var mouseMoveLock = false;
var lastMouseY = 0;
window.addEventListener("mousemove", (ev) => {
    if (mouseMoveLock) { return; }
    mouseMoveLock = true;
    let event = ev || window.event;
    let mousePos = { x: event.clientX, y: event.clientY };
    let height = (document.getElementById("navi-wrapper").clientHeight) * 0.9;
    if (mousePos.y < height) {
        revealNavigation();
    } else if (mousePos.y > lastMouseY) {
        hideNavigation();
    }
    lastMouseY = mousePos.y;
    mouseMoveLock = false;
});
```