from quarto_agents.generic_quarto_agent import GenericQuartoAgent
import numpy as np
import itertools
import pandas as pd
import quarto_util as qutil

# NegaMax with Alpha-Beta pruning   
class NegamaxAgent(GenericQuartoAgent):

    def __init__(self, depth, transposition=None, searchWindow=256) -> None:
        super().__init__()
        self.depth = depth
        self.searchWindow = searchWindow

        self.hit = 0
        self.total = 0
        self.tableFileName = transposition
        if transposition is not None:
            self.table = pd.read_pickle(f'tables/{transposition}.pkl')

    # Only used in debugging
    def makeFirstMove(self, quartoGameState):
        nextPiece = int(input("Pick your opponent's first piece: "))
        return nextPiece
    
    def makeMove(self, quartoGameState):
        #print("\nSTATE RECEIVED: ", quartoGameState)
        maxScore, (position, nextPiece) = self.alphaBeta(quartoGameState, self.depth, -1000, 1000)
        print(f"Negamax agent placed piece at cell {position} and nextPiece is {nextPiece}")
        print("maxEval: ",maxScore)
        return position, nextPiece
    
    def alphaBeta(self, quartoGameState, depth, alpha, beta):
        board, currentPiece, availableNextPieces, availablePositions = quartoGameState
        encoding = qutil.encodeBoard(board, currentPiece)
        self.total += 1
        
        if qutil.isGameOver(board):
            return -np.inf, (16,16)
        if depth == 0 or len(availablePositions) == 0:
            return self.evaluation(board), (16,16)
        #check transposition table
        if self.tableFileName is not None:
            if encoding in self.table.encoding.values:
                self.hit += 1
                row = self.table[self.table["encoding"] == encoding].values[0]
                return row[1], (row[2],row[3])
            
        if len(availableNextPieces) == 0:
            availableNextPieces.add(16)

        maxScore = -np.inf
        bestMove = (16,16)

        '''
        The move ordering for the search window is as follows - We cycle through all available positions for a single next piece before considering
        another next piece. This way all positions are prioritized and explored first over exploring all next pieces for a single position.
        '''
        # search window counter
        counter = 0

        for move in itertools.product(availableNextPieces, availablePositions):
            #search window increment and termination
            counter += 1
            if counter > self.searchWindow:
                break

            #switch move indices to match format: (position, nextPiece)
            move = (move[1],move[0])

            #print("\ndepth:", depth,"move:", move)
            # simulate move
            row, col = qutil.get2dCoords(move[0])
            board[row][col] = currentPiece
            availablePositions.remove(move[0])
            availableNextPieces.remove(move[1])
            nextGameState = (board, move[1], availableNextPieces, availablePositions)

            # call for next turn
            cur = -self.alphaBeta(nextGameState, depth-1, -beta, -alpha)[0]
            if cur >= maxScore:
                maxScore = cur
                bestMove = move
            alpha = max(alpha, maxScore)
            
            # undo simulated move
            board[row][col] = 16
            availablePositions.add(move[0])
            availableNextPieces.add(move[1])

            #print(f"score: {cur}  a: {alpha}  b: {beta}")
            if alpha > beta:
                if self.tableFileName is not None and depth==self.depth:
                    self.updateTable([encoding,maxScore,bestMove[0],bestMove[1]])
                availableNextPieces.discard(16)
                return alpha, bestMove

        if self.tableFileName is not None and depth==self.depth:
            self.updateTable([encoding,maxScore,bestMove[0],bestMove[1]])
        availableNextPieces.discard(16)        
        return maxScore, bestMove
    
    # Counts how many lines of three pieces with an identical property
    def evaluation(self, board):
        tempLine = None
        numLines = 0

        for i in range(4):
            # check horizontal lines
            tempLine = list(board[i])
            if np.count_nonzero(board[i] == 16) == 1:
                tempLine.remove(16)
                if qutil.matchingPropertyExists(tempLine):
                    numLines += 1
            
            tempLine = list(board[:,i])
            # check vertical lines
            if np.count_nonzero(board[:,i] == 16) == 1:
                tempLine.remove(16)
                if qutil.matchingPropertyExists(tempLine):
                    numLines += 1

        # check obtuse diagonal line
        tempLine = list(np.diag(board))
        if np.count_nonzero(np.diag(board) == 16) == 1:
            tempLine.remove(16)
            if qutil.matchingPropertyExists(tempLine):
                    numLines += 1
            
        # check acute diagonal line:
        tempLine = list(np.diag(board[::-1]))
        if np.count_nonzero(np.diag(board[::-1]) == 16) == 1:
            tempLine.remove(16)
            if qutil.matchingPropertyExists(tempLine):
                    numLines += 1
        
        # no winning line found
        return numLines
    
    def updateTable(self, record):
        if record[1] == np.inf: record[1] = 10
        if record[1] == -np.inf: record[1] = -10
        row = {
            'encoding': record[0],
            'evaluation': record[1],
            'movePos': record[2],
            'movePiece': record[3]
        }
        self.table = self.table.append(row, ignore_index=True)

        self.table['encoding'] = self.table['encoding'].astype('str')
        self.table['evaluation'] = self.table['evaluation'].astype('int8')
        self.table['movePos'] = self.table['movePos'].astype('int8')
        self.table['movePiece'] = self.table['movePiece'].astype('int8')
    
    def saveTable(self):
        self.table.to_pickle(f'tables/{self.tableFileName}.pkl')

    def displayTranspositionMetrics(self):
        if self.tableFileName is None:
            print("No transposition table was loaded. Cannot display any transposition metrics.")
        else:
            print("\nnum hits: ", self.hit, "")
            print("num total: ", self.total, "")
            if self.total != 0:
                print(f"hit rate: {round((self.hit/self.total)*100,2)} %\n")
            print(self.table.info())
            print("\n")