import numpy as np
import pandas as pd


def convertIntMoveToStr(move: int):
    if move <= 9:
        return "0" + str(move)
    else:
        return str(move)


def encodeBoard(boardArray, currentPiece):
    encoding = ""
    for i in range(4):
        for j in range(4):
            encoding += convertIntMoveToStr(boardArray[i][j])

    encoding += convertIntMoveToStr(currentPiece)
    return encoding


def decodeBoard(encoding):
    board_array = [int(encoding[i] + encoding[i + 1]) for i in range(0, len(encoding) - 2, 2)]
    board_array = np.reshape(board_array, (4, 4))

    current_piece = int(encoding[-2:])

    return board_array, current_piece


def getEncodingAfterMove(currentEncoding: str, nextPosition: int, nextPiece: int):
    return (
        currentEncoding[: 2 * nextPosition]
        + currentEncoding[-2:]
        + currentEncoding[2 * nextPosition + 2 : -2]
        + convertIntMoveToStr(nextPiece)
    )


def get2dCoords(ind):
    assert ind >= 0 and ind < 16, "Invalid linear index. Should be from 0 - 15 inclusive."
    row = ind // 4
    col = ind % 4

    return (row, col)


def getLinearCoords(row, col):
    return 4 * row + col


# Determines if there is a matching column of bits for a list of integers between 0 (inclusive) and 16 (exclusive)
def matchingPropertyExists(line):
    # bitwiseAnd - checks if there is a column of 1s by getting the conjunction
    # bitwiseNot - checks if there is a column of 0s after negating all integers, masking by 15 (1111) and then getting the conjuction
    bitwiseAnd = line[0]
    bitwiseNot = ~line[0] & 15
    for i in range(1, len(line)):
        bitwiseAnd &= line[i]
        bitwiseNot &= ~line[i] & 15

    result = bitwiseAnd | bitwiseNot
    return result > 0


def isGameOver(board):
    for i in range(4):
        # check horizontal lines
        if np.count_nonzero(board[i] == 16) == 0:
            if matchingPropertyExists(board[i]):
                return True

        # check vertical lines
        if np.count_nonzero(board[:, i] == 16) == 0:
            if matchingPropertyExists(board[:, i]):
                return True

    # check obtuse diagonal line
    if np.count_nonzero(np.diag(board) == 16) == 0:
        if matchingPropertyExists(np.diag(board)):
            return True

    # check acute diagonal line:
    if np.count_nonzero(np.diag(board[::-1]) == 16) == 0:
        if matchingPropertyExists(np.diag(board[::-1])):
            return True

    # no winning line found
    return False


def isGameOverEncoding(encoding):
    board = [int(encoding[i] + encoding[i + 1]) for i in range(0, len(encoding) - 2, 2)]

    for i in range(4):
        # check horizontal lines
        horizontal = board[4 * i : 4 * (i + 1)]
        if 16 not in horizontal:
            if matchingPropertyExists(horizontal):
                return True

        # check vertical lines
        vertical = board[i : len(board) : 4]
        if 16 not in vertical:
            if matchingPropertyExists(vertical):
                return True

    # check obtuse diagonal line
    diagonal1 = board[0 : len(board) : 5]
    if 16 not in diagonal1:
        if matchingPropertyExists(diagonal1):
            return True

    # check acute diagonal line:
    diagonal2 = board[3:-1:3]
    if 16 not in diagonal2:
        if matchingPropertyExists(diagonal2):
            return True

    return False


# transposition table functions
def createTable(file_name: str):
    df = pd.DataFrame(columns=["encoding", "evaluation", "movePos", "movePiece"])

    df["encoding"] = df["encoding"].astype("str")
    df["evaluation"] = df["evaluation"].astype("int8")
    df["movePos"] = df["movePos"].astype("int8")
    df["movePiece"] = df["movePiece"].astype("int8")

    df.to_pickle(f"tables/{file_name}.pkl")


# Used to create a pandas dataframe to store results for agents - linked to the name of an agent
def createAgentStatsTable(tableName: str):
    df = pd.DataFrame(
        columns=[
            "agentName",
            "labelName",
            "searchDepth",
            "wins",
            "losses",
            "draws",
            "cumulativeGameTime",
            "cumulativeAvgMoveTime",
            "numGamesPlayed",
        ]
    )
    df.to_pickle(f"{tableName}.pkl")


# Update an agent's stats after a game has been played - updatedRecord is in the form of a triplet (win, loss, draw) where 1 denotes the update for that and zero everywhere else
def updateAgentStats(tableName: str, agentName: str, updatedRecord):
    df = pd.read_pickle(f"{tableName}.pkl")
    agentNameSplit = agentName.split("-")
    searchDepth = agentNameSplit[1]
    labelName = None
    if len(agentNameSplit) < 3:
        labelName = agentNameSplit[0]
    else:
        labelName = agentNameSplit[0] + "-" + agentNameSplit[2]

    if agentName in df.agentName.values:
        df.loc[
            df["agentName"] == agentName,
            [
                "wins",
                "losses",
                "draws",
                "cumulativeGameTime",
                "cumulativeAvgMoveTime",
                "numGamesPlayed",
            ],
        ] += updatedRecord

        df["searchDepth"] = df["searchDepth"].astype("int8")
        df["wins"] = df["wins"].astype("int32")
        df["losses"] = df["losses"].astype("int32")
        df["draws"] = df["draws"].astype("int32")
        df["numGamesPlayed"] = df["numGamesPlayed"].astype("int32")
    else:
        record = {
            "agentName": agentName,
            "labelName": labelName,
            "searchDepth": searchDepth,
            "wins": updatedRecord[0],
            "losses": updatedRecord[1],
            "draws": updatedRecord[2],
            "cumulativeGameTime": updatedRecord[3],
            "cumulativeAvgMoveTime": updatedRecord[4],
            "numGamesPlayed": updatedRecord[5],
        }
        df = df.append(record, ignore_index=True)

        df["searchDepth"] = df["searchDepth"].astype("int8")
        df["wins"] = df["wins"].astype("int32")
        df["losses"] = df["losses"].astype("int32")
        df["draws"] = df["draws"].astype("int32")
        df["numGamesPlayed"] = df["numGamesPlayed"].astype("int32")

    df.to_pickle(f"{tableName}.pkl")


# Extracts data from detailed log file
def readLogFile(filename):
    log = open(filename)

    agent1, agent2, num_runs = log.readline().split(",")
    num_runs = int(num_runs)
    data = []
    game_results = []

    for i in range(num_runs):
        _ = log.readline()
        line = log.readline().strip()
        temp_data = []

        while line not in ["0", "1", "2"]:
            temp_data.append(line.split(","))
            line = log.readline().strip()

        data.append(temp_data)
        game_results.append(line)

    log.close()

    return (agent1, agent2), data, game_results


# Extracts data from summative log file
def readRunFile(filename):
    log = open(filename)
    agent1, agent2, _ = log.readline().split(",")
    log.close()

    df = pd.read_csv(filename, skiprows=1)
    return (agent1, agent2), df
