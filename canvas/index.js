// For more comments about what's going on here, check out the `hello_world`
// example.
import('./pkg')
  .catch(console.error);
import { display_image } from './pkg';

const mapContext = require.context('./src/assets', false, /\.(png)$/);

//Populate dropdown with map options
const mapDropdown = document.getElementById("mapSelect");
mapContext.keys().forEach((mapKey) => {
    const opt = document.createElement('option');
    opt.value = mapContext(mapKey);
    opt.innerText = mapKey;
    mapDropdown.appendChild(opt);
});

// Function to display selected map on canvas
window.showMap = function() {
    const selectedMap = mapDropdown.value;
    display_image(selectedMap);
}

// Display player positions on canvas
display_player_position(250,250);
