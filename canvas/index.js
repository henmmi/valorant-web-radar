// For more comments about what's going on here, check out the `hello_world`
// example.
import('./pkg')
  .catch(console.error);
import { display_image } from './pkg';
import imagePath from './src/assets/Fracture.png';
display_image(imagePath);
