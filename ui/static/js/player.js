function setCookie(name, value, days) {
  var expires = "";
  if (days) {
    var date = new Date();
    date.setTime(date.getTime() + days * 24 * 60 * 60 * 1000);
    expires = "; expires=" + date.toUTCString();
  }
  console.log(window.location.href);
  document.cookie =
    name +
    "=" +
    (value || "") +
    expires +
    "; path=/player/" +
    window.location.href.split("/player/")[1];
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

url = url.replace(/&#x2F;/g, "/");

var player = videojs("player", {
  controls: true,
  autoplay: false,
  preload: "auto",
  sources: [
    {
      src: url,
      type: "application/x-mpegURL",
    },
  ],
});
player.qualityLevels();

player.httpSourceSelector();

player.on("loadedmetadata", function () {
  var button = player.controlBar.getChild("HttpSourceSelector");
  if (button) {
    button.el().firstChild.firstChild.innerHTML = "Quality";
  }
});

var savedTime = getCookie("videoCurrentTime");
if (savedTime) {
  player.currentTime(parseFloat(savedTime));
}

player.on("timeupdate", function () {
  setCookie("videoCurrentTime", player.currentTime(), 7); // Saves for 7 days
});

player.on("ended", function () {
  setCookie("videoCurrentTime", "", -1); // Deletes the cookie
});
