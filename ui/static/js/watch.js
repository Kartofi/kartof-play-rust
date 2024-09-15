let player = document.getElementById("player");
var spinner = document.getElementById("loading-spinner");

setPlayer(1);

function setPlayer(episode) {
  spinner.style.display = "block";

  player.src = "/player/" + id + "/" + episode;

  player.contentWindow.location.reload();
}

player.onload = function () {
  // Hide the spinner and show the iframe once it has loaded
  spinner.style.display = "none";
  player.style.display = "block";
};
