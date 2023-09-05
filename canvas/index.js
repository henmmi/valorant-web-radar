import('./pkg')
  .catch(console.error);
import { change_map } from './pkg';
import { display_player_position } from './pkg';
// import { websocket } from './pkg';
// websocket ("ws://localhost:27017");

const mapContext = require.context('./src/assets', false, /\.(png)$/);

// Draw map options and change if new map selected.
const mapDropdown = document.getElementById("mapSelect");

// Add map options to dropdown
mapContext.keys().forEach((mapKey) => {
    const opt = document.createElement('option');
    opt.value = mapContext(mapKey);
    opt.innerText = extractMapNameFromContext(mapKey);
    mapDropdown.appendChild(opt);
    console.log(opt.value);
});

let image_src = change_map(mapDropdown.value);

mapDropdown.addEventListener('change', handleMapChange);
function handleMapChange() {
    const currentMap = extractMapNameFromContext(mapDropdown.value);
    const selectedMap = extractMapNameFromURL(image_src);
    if (currentMap !== selectedMap) {
        change_map(mapDropdown.value);
        image_src = mapDropdown.value;
    }
}
function extractMapNameFromContext (context) {
    const regex = /\/([^\/]+)\.png$/;
    const match = context.match(regex);
    return match ? match[1] : null;
}
function extractMapNameFromURL(url) {
    const regex = /\/([^\/]+)-\w+\.png$/;
    const match = url.match(regex);
    return match ? match[1] : null;
}
// Display player positions on canvas
window.showPlayer = function() {
    display_player_position(1, 250, 250);
}
