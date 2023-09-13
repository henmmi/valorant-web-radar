import('./pkg')
  .catch(console.error);

const mapContext = require.context('./src/assets', false, /\.(png)$/);
