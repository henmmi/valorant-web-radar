import('./pkg')
  .catch(console.error);
require.context('./src/assets/maps', false, /\.(png)$/);
require.context('./src/assets/agents', false, /\.(png)$/);
require.context('./src/assets/icons', false, /\.(png)$/);

import "./src/css/style.css";