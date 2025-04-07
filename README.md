# Bevy Platformer Game

A 2D platformer game built with the [Bevy Engine](https://bevyengine.org/) in Rust. This project appears to replicate mechanics found in classic platformer games, featuring entities like players and goombas, tile-based levels, physics, and animations.

## Project Structure

The project follows a modular structure within the `src/` directory:

-   `main.rs`: The main entry point of the application, sets up Bevy plugins, system sets (`AppSet`), the main camera, and initial state.
-   `config.rs`: Contains configuration constants like window dimensions (`WINDOW_WIDTH`, `WINDOW_HEIGHT`), tile sizes (`TILE_SIZE`), physics constants (`GRAVITY`), and sprite frame data (`FRAMES_RECT_PLAYER`, `FRAMES_RECT_GOOMBA_BROWN`).
-   `dev_tools/`: Contains plugins and systems useful for development, such as debug modes (`custom.rs`), egui integration (`mario_egui.rs`), and tilemap tools (`tilemap.rs`). Enabled via the `dev` feature flag.
-   `game/`: Core game logic.
    -   `animations/`: Handles sprite animations for different entities (Player, Goomba). Defines an `Animate` trait.
    -   `assets/`: Manages loading and handling of game assets (textures, levels). Defines an `AssetKey` trait and `HandleMap`.
    -   `entities/`: Defines game entities like the Player (`player.rs`) and Goomba (`goomba.rs`), including their spawning logic.
    -   `movement.rs`: Defines components like `MovementController` for handling entity movement input and systems for recording input and camera following.
    -   `physics/`: Implements the physics engine, including components (`components.rs` - `Pos`, `Vel`, `Aabb`, `BoxCollider`, etc.), resources (`resources.rs` - `Gravity`, `Contacts`), and systems (`systems.rs` - collision detection, resolution).
    -   `spawn/`: Logic for spawning levels (`level.rs`), maps (`map.rs`), and entities (`entities.rs`). Uses triggers like `SpawnLevel`.
    -   `tiles/`: Defines tile properties (`components.rs` - `Tile`, `Behaviour`), collision handling (`systems.rs`, `resources.rs`), and utility functions (`utils.rs`).
    -   `traits/`: Defines reusable character traits like movement (`go.rs`), jumping (`jump.rs`), and solid interactions (`solid.rs`).
-   `level.rs`: Handles loading and parsing of level data files (`LevelFile`).
-   `screen/`: Manages different game screens or states (e.g., `Loading`, `Playing`) using Bevy's state machine.
-   `ui/`: Contains user interface elements, interactions (`interaction.rs`), palettes (`palette.rs`), and custom widgets (`widgets.rs`).

## Running Locally

### Prerequisites

-   [Rust](https://www.rust-lang.org/tools/install) (includes Cargo, the Rust package manager)

### Build & Run

1.  **Clone the repository:**
    ```bash
    git clone <your-repository-url>
    cd <repository-directory>
    ```
2.  **Build and run the project:**
    ```bash
    cargo run
    ```

    This command will compile the project and launch the game. The first build might take some time to download and compile dependencies. Subsequent builds will be faster.

3.  **Build and run with developer tools (if needed):**
    ```bash
    cargo run --features dev
    ```
