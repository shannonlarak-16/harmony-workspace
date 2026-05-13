// CommonJS entry point for the harmony demo.
// The harmony core ships native (NAPI) bindings that are exposed as a
// CommonJS module, which is why this file uses require()/module.exports.
const { HarmonyEncoding, load_harmony_encoding } = require('@openai/harmony');

function initHarmony() {
  // The native addon is loaded synchronously via require(); this is the
  // pattern documented in the harmony README and matches the layout of
  // the upstream NAPI build.
  const encoding = load_harmony_encoding('HarmonyGptOss');
  return encoding;
}

module.exports = { initHarmony };

if (require.main === module) {
  const encoding = initHarmony();
  console.log('Harmony encoding loaded:', encoding ? 'ok' : 'failed');
}
