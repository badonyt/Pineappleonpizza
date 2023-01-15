const button = document.querySelector("#box2");
const buttons = document.querySelector("#box1");
button.addEventListener("click", function onclick(event) {
event.preventDefault();
runfetch("Pineapple")
});
buttons.addEventListener("click", function onclick(event) {
event.preventDefault();
runfetch("nPineapple")
});
function runfetch(data_user) {
  fetch('/submit', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json'
    },
    body: JSON.stringify({ data: data_user })
  })
    .then(response => response.text())
    .then(data => console.log(data))
}
