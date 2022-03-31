const spawn = require('child_process').spawnSync;

try {
  spawn('yarn', ['--cwd', '..', 'wasm-pack', 'build'], {
    stdio: 'inherit',
  });

  spawn('yarn', ['next', 'build'], {
    stdio: 'inherit',
  });
} catch (e) {
  console.error(e);
  process.exit(1);
}
