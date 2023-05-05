// To apply this script, we assume:
//      there is a container/wrapper div element with id=navi
//      config.js is loaded, so we can use the configuration information inside it
// Personally, I think this design is disgusting (somewhat sticking components together, bad modularity).  
fetch(site+"navigation.html", {"credentials": "include"})
.then(async (value) => await value.text())
.then((text) => {
    // add the navigation bar elements
    let navi = document.getElementById("navi");
    navi.innerHTML = text;
})
.then((_any) => {
    // bind the click listeners pointing to absolute urls
    document.getElementById("home-but").addEventListener("click", (_ev) => {
        window.location.assign(site + "index.html");
    });
    document.getElementById("port-but").addEventListener("click", (_ev) => {
        window.location.assign(site + "portfolio.html");
    });
    document.getElementById("arch-but").addEventListener("click", (_ev) => {
        window.location.assign(site + "archive.html");
    });
})