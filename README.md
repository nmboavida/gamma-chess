# gamma-go

This projeect creates a machine learning model for chess that is inspired by Google's AlphaGo. To achieve this, we intergrate a Convolutional Neural Network (CNN) for next move prediction and Monte Carlo Tree Search (MCTS) for search the optional move. MCTS is a probablistic algorithm that is more suitable for Go as it is able to handle combinatorial explosion as opposed to deterministic algorithms that do exhaustive search. Nevertheless we will stick with MCTS as an opportunity to explore its intricacies.

**Convolutional Neural Networks (CNNs)**: Powerful for pattern recognition and can effectively process the spatial structure of a chessboard. CNNs can be trained to understand chess positions and predict outcomes.

**Monte Carlo Tree Search (MCTS)**: A heuristic search algorithm for decision-making processes. It's used for exploring the most promising moves, balancing between exploring new moves and deepening the search in promising areas.

### Data Collection and Processing

Data: Gather a large dataset of chess games, including varied levels of play. Publicly available datasets like those from online chess websites could be useful.

Processing: Convert the chess games into a format understandable by your model. This usually involves representing the chessboard and pieces in a numerical format.

### Designing the CNN Architecture
Input Layer: The input should represent the chessboard. One common approach is a 8x8 grid with multiple channels representing different pieces.

Hidden Layers: Use several convolutional layers. These layers will help the model learn to recognize complex patterns on the chessboard.

Output Layer: This could be a policy output, predicting the next move, and a value output, estimating the probability of winning from the current position.

### Implementing Monte Carlo Tree Search
Integration with CNN: Use the CNN to evaluate board positions and guide the MCTS in selecting moves during the tree search.

Exploration vs. Exploitation: Fine-tune the balance in MCTS between exploring new parts of the tree and exploiting known promising paths.

### Training the Model
Supervised Learning: Initially, you might train the CNN with historical game data, teaching it to predict moves and game outcomes.
Reinforcement Learning: Further train the model using self-play, where the model plays against itself, learning from its own experiences.

### Testing and Refinement
Evaluation: Test the model against various levels of chess engines and human players to evaluate its performance.
Refinement: Continuously refine the model based on testing outcomes, adjusting the architecture, and tuning hyperparameters as needed.

### Optimization and Scaling
Efficiency: Optimize the model for computational efficiency, especially if you plan to run it on limited hardware.
Scaling Up: If resources allow, consider scaling the model to increase its learning capacity and performance.

### Tools and Libraries
TensorFlow or PyTorch: For building and training the CNN.
Libraries for Chess Logic: Libraries like python-chess can help manage chess game rules and moves.
Ethical and Fair Use Considerations
Ensure that any data used is sourced ethically and consider the impact of your model on the chess community.
Remember, this is a complex task that may require significant computational resources and expertise in both machine learning and chess. It's also a field of active research, so staying updated with the latest developments is crucial.


## Dependencies

1. Install pipx: `python3 -m pip install --user pipx`
2. Install poetry: `pipx install poetry`
3. Packages:
   1. `poetry add python-chess`
   2. `poetry add jupyter lab`

## Dataset

1. Download compressed dataset [here](https://database.lichess.org/standard/lichess_db_standard_rated_2016-05.pgn.zst) and add it to the dataset folder
2. `cd dataset` and run `zstd -d lichess_db_standard_rated_2016-05.pgn.zst -o lichess_db_standard_rated_2016-05.pgn`


## Run Notebooks

1. Run poetry shell: `poetry shell`


### Data Model

The lichness file represent an array of gamges with two tables: 

#### `_tag_roster` Fields

| Field Name | Type | Description |
|------------|------|-------------|
| Event      | String | The name of the event or tournament in which the game was played. |
| Site       | URL | The URL where the game was played or can be viewed. |
| Date       | String | The date when the game was played. '????.??.??' indicates unknown date. |
| Round      | String | The round number in the tournament or event. '?' means not provided. |
| White      | String | The username or the name of the player with the white pieces. |
| Black      | String | The username or the name of the player with the black pieces. |
| Result     | String | The outcome of the game ('0-1' for Black win, '1-0' for White win, '1/2-1/2' for draw). |

#### `_others` Fields

| Field Name      | Type   | Description |
|-----------------|--------|-------------|
| UTCDate         | String | The date of the game in Coordinated Universal Time (UTC). |
| UTCTime         | String | The time the game started in UTC. |
| WhiteElo        | Integer | The Elo rating of the white player at the time of the game. |
| BlackElo        | Integer | The Elo rating of the black player at the time of the game. |
| WhiteRatingDiff | Integer | The change in Elo rating for the white player as a result of this game. |
| BlackRatingDiff | Integer | The change in Elo rating for the black player as a result of this game. |
| ECO             | String | The Encyclopaedia of Chess Openings code for the opening played. |
| Opening         | String | The name of the opening played. |
| TimeControl     | String | The time control settings for the game. |
| Termination     | String | How the game ended (e.g., 'Time forfeit', 'Checkmate'). |

These tables provide a structured and detailed view of the various fields in a PGN game record, making it easier to understand and utilize the data.