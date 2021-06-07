const helloPluginRid = Deno.openPlugin("target/debug/deno_example_plugin.dll");

console.log(`opened plugin: hello_plugin, rid = ${helloPluginRid}`);


const result = Deno.core.opSync(
  "deno_example_plugin",
  { val: "1" },
  new Uint8Array([]),
);

console.log(`return value from hello_plugin: ${result}`);