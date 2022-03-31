const spawn = require('child_process').spawnSync;

spawn('yarn', ['--cwd', '..', 'wasm-pack', 'build'], {
  stdio: 'inherit',
});

spawn('yarn', ['next', 'build'], {
  stdio: 'inherit',
});
