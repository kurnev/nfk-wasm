Need for kill

Rust-wasm is responsible for calculating positions of all objects on the map, or simply physics.  

Javascript part renders game state in rAF in canvases. To improve perf there are two canvases. One is for map.  
And second one is for dynamic objects. 

Rust javascript interface is done via one array, so we have to consider how we store data in array. Lets try this:

First 4000 bytes are for map terrain: [object_position_x] [object_position_y] [object_size_width] [object_size_width]: 4 bytes for each cell.
Supported 1000 cells.

4001 and so on are for heroes: [object_position_x] [object_position_y] [object_size_width] [object_size_width] 4 bytes for cell

TODO: Add cursor position for heroes

I do not calculate delta, as there should be enough calculation power to do all universe physics calculations.

FAQ: 

Question: How to pass events to rust code, so it correctly applies changes

Answer: Seems like you can directly call Rust functions inside your JS code. Where you have to pass user input.

```
player.passEvent('press-left');
player.passEvent('press-left-mouse-click');
```


