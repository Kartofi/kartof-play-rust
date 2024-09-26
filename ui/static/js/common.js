const maxTitleLength = 30;

const titles = document.querySelectorAll("#anime-title");
for (let i = 0; i < titles.length; i++) {
  let el = titles[i];
  if (el.innerHTML.length > maxTitleLength) {
    el.innerHTML = el.innerHTML.slice(0, maxTitleLength - 1);
    el.innerHTML += "...";
  }
}
