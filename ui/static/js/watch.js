let player = document.getElementById("player");
var spinner = document.getElementById("loading-spinner");

let episode_cookie = getCookieEpisode();
if (episode_cookie == undefined) {
  episode_cookie = "1";

  setCookie("episode", "1", 30);
}
setPlayer(episode_cookie);

function setPlayer(episode) {
  let currentUrl = new URL(document.location);
  if (getCookieEpisode() != episode) {
    setCookie("episode", episode, 30);

    currentUrl.searchParams.set("ep", episode);
    document.location = currentUrl.href;
  }
  if (!currentUrl.searchParams.get("ep")) {
    currentUrl.searchParams.set("ep", episode);
    document.location = currentUrl.href;
  }
  spinner.style.display = "block";

  player.src = "/player/" + id + "/" + episode;
}

player.onload = function () {
  // Hide the spinner and show the iframe once it has loaded
  spinner.style.display = "none";
  player.style.display = "block";
};

function getCookieEpisode() {
  return getCookie("episode");
}

function getCookie(name) {
  var nameEQ = name + "=";
  var ca = document.cookie.split(";");
  for (var i = 0; i < ca.length; i++) {
    var c = ca[i];
    while (c.charAt(0) === " ") c = c.substring(1, c.length);
    if (c.indexOf(nameEQ) === 0) return c.substring(nameEQ.length, c.length);
  }
  return null;
}

function setCookie(name, value, days) {
  var expires = "";
  if (days) {
    var date = new Date();
    date.setTime(date.getTime() + days * 24 * 60 * 60 * 1000);
    expires = "; expires=" + date.toUTCString();
  }

  document.cookie =
    name +
    "=" +
    (value || "") +
    expires +
    "; path=/watch/" +
    window.location.href.split("/watch/")[1].split("?")[0];
}
