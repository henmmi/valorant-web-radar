import('./pkg')
  .catch(console.error);
require.context('./src/assets', false, /\.(png)$/);
