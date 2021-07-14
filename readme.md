# Rust Client Example

This repo is for exploring rust and learning more about it. 

It is based around a client-server model. The client will aim to be WASM compatible to allow for a web deployment.

The Server will aim to be an API router for handling server calls (REST + Websocket) and possibly some WebRTC stuff for video/audio

Additionally will try to act a simple boilerplate for spinning up quick apps.

---

### Note on State Management

This example is using **egui** which is an immediate GUI amd because of this - there needs to be a persistent state management layer if you want to implement any stateful updates.

The actual binding is very performant because of this, but it also means that you have to think in terms of state reflow and where those variables will live before you wish to paint the changes (this mainly is when a user interacts with an input field).

	ex. You may want to capture the value of a text field, but not necessary apply it until the save button is clicked. This requires we hold both the old and new state in memory while we are painting the gui but then once we have determined the final value we can discard the temporary storage (since this needs to live outside of the GUI's immediate flow which gets called many times per second)


For these reasons, the client is using a simple central store - 'state' and a temp store 'new_state' which can be used to temporarily mutate and make adjustments before writing back to the state.

Anything which doesn't need to live beyond the life of the component can also be placed inside of a local reference via its own struct.

** Note: This may change - as I am still getting used to the flows of rust and immediate gui use.

---

### Acknowledgements

The GUI is using the awesome [Egui Library](https://github.com/emilk/egui) which is being used for the graphical ui components within the client.