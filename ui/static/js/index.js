const matches = document.querySelectorAll("#release_time");

for (let i = 0; i < matches.length; i++) {
  let el = matches[i];

  const utcTimestamp = Number(el.innerText) * 1000;

  const utcDate = new Date(utcTimestamp);

  const localTime = utcDate
    .toLocaleTimeString(undefined, { hour12: false })
    .split(":");

  el.innerText = localTime[0] + " : " + localTime[1];
}
