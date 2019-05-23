import "./style.css";

function main() {
  document.body.appendChild(document.createTextNode("Hello world"));

  import("../runtime/pkg").then(module => {
    module.run();
  });
}

main();
