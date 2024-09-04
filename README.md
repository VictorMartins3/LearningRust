
# Quake Log Parser

Quake Log Parser is a Rust application designed to parse log files from Quake III Arena. The application reads the log file, extracts game statistics, and provides various reports, including total kills, player statistics, and the means of death.

## Table of Contents

- [Installation](#installation)
- [Usage](#usage)
- [Project Structure](#project-structure)
- [Features](#features)
- [Contact](#Contact)
- [Author](#Author)

## Installation

## Linux

To use this project, you need to have Rust installed on your machine. If you do not have Rust installed, you can install it using the following command:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

## Windows


**Install Rust**:
   Download and install Rust from [rustup.rs](https://rustup.rs/).

## Usage

1. **Clone the repository**:

   ```bash
   git clone https://github.com/VictorMartins3/projeto_cw.git
   cd projeto_cw
   ```

2. **Ensure the log file `qgames.log` is in the project directory**.

3. **Run the application**:

   ```bash
   cargo run
   ```

4. **Select an option** from the menu to generate the desired report.

      ![](src/utils/gifs/toptermica.gif)

## Project Structure

The project is organized as follows:

```plaintext
quake_log_parser/
├── bin/
│   └── main.rs
├── display/
│   └── mod.rs
├── game/
│   └── mod.rs
├── log/
│   └── qgames.log
├── log_parser/
│   └── mod.rs
├── report/
│   └── mod.rs
├── lib.rs
├── Cargo.toml
```

### `bin/main.rs`

The main entry point of the application. It provides a menu for selecting various reports.

### `display/mod.rs`

Contains functions to display different types of game statistics.

### `game/mod.rs`

Defines the `Game` struct and its associated methods for storing and manipulating game data.

### `log_parser/mod.rs`

Handles reading the log file and parsing its contents into `Game` objects.

### `report/mod.rs`

Generates detailed reports of the parsed game data.

### `lib.rs`

The library root file that includes the modules.

### `Cargo.toml`

Specifies the project dependencies and metadata.

### `log/qgames.log`

Sample log file from Quake III Arena (ensure this file is in the project directory).

## Features

- **Full Report**: Displays a comprehensive report of all games in the log file.
- **Total Kills**: Displays the total kills for each game.
- **Player Statistics**: Lists all players and their statistics.
- **Kill Statistics**: Displays the number of kills per player, sorted by the number of kills.
- **Means of Death**: Displays statistics on the means of death, sorted by frequency.
- **Games with Kills**: Displays games that have kills.
- **Games without Kills**: Displays games that do not have kills.

## Contact

For any questions or inquiries, you can reach me on Slack or via email at martins.victor212004@gmail.com.

## Author

[<img loading="lazy" src="https://avatars.githubusercontent.com/u/106573420?v=4" width=115><br><sub>Victor Martins</sub>](https://github.com/VictorMartins3) 
