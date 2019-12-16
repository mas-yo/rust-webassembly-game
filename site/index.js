const js = import("./node_modules/@mas-yo/client/client.js");
js.then(js => {
  js.greet("WebAssembly");
});