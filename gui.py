import tkinter as tk
from tkinter import font
from PIL import ImageTk, Image
from quarto import *
import quarto_agents

BLANK_TILE = 16

class QuartoGUI(tk.Tk):
    def __init__(self):
        super().__init__()
        self.title("Quarto Game")
        self.player1IsHuman = False
        self.player2IsHuman = True
        self._game = QuartoGame(
            quarto_agents.HumanPlayer(), quarto_agents.HumanPlayer(), gui_mode=True, bin_mode=False
        )

        self._loadPhotos()
        self._createMenu()
        self._createBoardDisplay()
        self._createBoardGrid()
        self._createPieceGrid()

    def _loadPhotos(self):
        self._photos = []
        imagePaths = [f"images/{i}.png" for i in range(17)]
        for imagePath in imagePaths:
            img = Image.open(imagePath)
            img = img.resize((int(0.75 * img.width), int(0.75 * img.height)))
            photo = ImageTk.PhotoImage(img)
            self._photos.append(photo)

    def _createMenu(self):
        menu_bar = tk.Menu(master=self)
        self.config(menu=menu_bar)
        file_menu = tk.Menu(master=menu_bar)
        file_menu.add_command(label="Exit", command=quit)
        menu_bar.add_cascade(label="File", menu=file_menu)

    def _createBoardDisplay(self):
        display_frame = tk.Frame(master=self)
        display_frame.pack(fill=tk.X, padx=100)
        self.display = tk.Label(
            master=display_frame,
            text="Ready?",
            font=font.Font(size=28, weight="bold"),
        )
        self.display2 = tk.Label(
            master=display_frame, text="Player 1's turn", font=font.Font(size=18, weight="bold")
        )
        self.currentButton = tk.Button(
            master=display_frame,
            fg="black",
            width=100,
            height=100,
            text="Current",
            font=("Helvetica 12 bold"),
            image=self._photos[BLANK_TILE],
            compound="center",
        )
        self.startButton = tk.Button(
            master=display_frame,
            fg="black",
            width=10,
            height=5,
            text="Start",
            font=("Helvetica 12 bold"),
        )
        self.display.pack()
        self.display2.pack()
        self.currentButton.pack(side=tk.LEFT)
        self.startButton.pack(side=tk.RIGHT)
        self.startButton.bind("<ButtonPress-1>", self.play)

    def _createBoardGrid(self):
        self._cells = {}
        self._takenCells = set()
        grid_frame = tk.Frame(master=self, background="black", padx=20, pady=20)
        grid_frame.pack(padx=50, pady=50, side=tk.LEFT)
        for row in range(4):
            for col in range(4):
                button = tk.Button(
                    master=grid_frame,
                    fg="black",
                    width=100,
                    height=100,
                    text=4 * row + col,
                    font=("Helvetica 12 bold"),
                    image=self._photos[BLANK_TILE],
                    compound="center",
                    bd=0,
                )
                self._cells[(row, col)] = button
                button.grid(row=row, column=col, padx=3, pady=3, sticky="nsew")

    def _createPieceGrid(self):
        self._pieces = {}
        self._takenPieces = set()
        grid_frame = tk.Frame(master=self, background="#876c3e")
        grid_frame.pack(padx=50, pady=20, side=tk.LEFT)

        for row in range(4):
            self.rowconfigure(row, weight=1, minsize=50)
            self.columnconfigure(row, weight=1, minsize=75)
            for col in range(4):
                button = tk.Button(
                    master=grid_frame,
                    text=str(4 * row + col),
                    font=("Helvetica 15 bold"),
                    image=self._photos[4 * row + col],
                    compound=tk.RIGHT,
                    relief=tk.FLAT,
                    bd=0,
                )
                self._pieces[(row, col)] = button
                button.grid(row=row, column=col, padx=5, pady=5)

    def _resetGrids(self):
        for row in range(4):
            for col in range(4):
                self._cells[(row, col)].configure(image=self._photos[BLANK_TILE], text=4 * row + col)
                self._pieces[(row, col)].configure(state=tk.ACTIVE)
        self.currentButton.configure(image=self._photos[BLANK_TILE], text="current")

    def _resetBoard(self, event):
        self._game.resetGame()
        self._takenCells.clear()
        self._takenPieces.clear()
        self._resetGrids()
        self._update_display(msg="Ready?")
        self.play(event)

    def isValidMove(self, position, nextPiece):
        if position not in range(16):
            self.display2["text"] = "This position does not exist"
            return False
        if nextPiece not in range(16):
            self.display2["text"] = "This piece does not exist"
            return False
        if position not in self._game.availablePositions:
            self.display2["text"] = "This cell is unavailable"
            return False
        if nextPiece not in self._game.availablePieces:
            self.display2["text"] = "This piece is not available"
            return False
        return True

    def makeMove(self, position, nextPiece):
        # validation checks
        if not self.isValidMove(position, nextPiece):
            return False

        # add to move history
        self._game.moveHistory.append((position, nextPiece))

        # place piece on board and update game state
        self.updateCell(position)
        row, col = qutil.get2dCoords(position)
        self._game.board[row][col] = self._game.currentPiece
        self._game.availablePositions.remove(position)

        # set next player's piece
        self.updateCurrent(nextPiece)
        self._game.currentPiece = nextPiece
        self._game.availablePieces.remove(self._game.currentPiece)

        if self._game.gui_mode:
            print("Move successful")
        return True

    def getPlayerMove(self, isPlayerOneTurn, isFirstMove=False):
        if isPlayerOneTurn:
            if self.player1IsHuman:
                pass
            else:
                if isFirstMove:
                    return self._game.pickRandomAvailablePiece()
                return self._game.player1.makeMove(self._game.getGameState(), gui_mode=self._game.gui_mode)
        else:
            if self.player2IsHuman:
                pass
            else:
                if isFirstMove:
                    return self._game.pickRandomAvailablePiece()
                return self._game.player2.makeMove(self._game.getGameState(), gui_mode=self._game.gui_mode)

    def makeAgentFirstMove(self):
        firstMove = self._game.pickRandomAvailablePiece()
        self._game.makeFirstMove(firstMove)
        self.updateCurrent(firstMove)
        

    def play(self, event):
        isPlayerOneTurn = True
        self.display["text"] = "Player 1's turn"
        
        # first move
        if not self.player1IsHuman:
            self.toggleGridFreeze()
            self.display2["text"] = "Agent is picking the first piece"
            self.after(2000, self.makeAgentFirstMove)
            self.toggleGridFreeze(toggleFreezeOn=False)
            isPlayerOneTurn = False

        self.startButton.bind("<ButtonPress-1>", self._resetBoard)
        self.startButton.configure(text="Reset")

        if self._game.gui_mode:
            self._game.showGameState()

        # subsequent moves
        for i in range(len(self._game.availablePositions) - 1):
            if self._game.gui_mode:
                self._game._showPlayerName(isPlayerOneTurn)

            # player 1
            if isPlayerOneTurn:
                self.display["text"] = "Player 1's turn"
                self.display2["text"] = "Agent is playing..."

                for i in range(3):
                    position, nextPiece = self._game.player1.makeMove(
                        self._game.getGameState(), gui_mode=self._game.gui_mode
                    )
                    validMove = self.makeMove(position, nextPiece)
                    if validMove:
                        break
                    elif i == 2:
                        print("Three invalid moves made - game ended")
                        return
            # player 2
            else:
                self.display["text"] = "Player 2's turn"
                self.display2["text"] = "Agent is playing..."

                for i in range(3):
                    position, nextPiece = self._game.player2.makeMove(
                        self._game.getGameState(), gui_mode=self._game.gui_mode
                    )
                    validMove = self.makeMove(position, nextPiece)
                    if validMove:
                        break
                    elif i == 2:
                        print("Three invalid moves made - game ended")
                        return

            if self._game.gui_mode:
                self._game.showGameState()

            if qutil.isGameOver(self._game.board):
                if isPlayerOneTurn:
                    self.display["text"] = "Player 1 Won!"
                    self.display2["text"] = ""
                else:
                    self.display["text"] = "Player 2 Won!"
                    self.display2["text"] = ""
                return
            isPlayerOneTurn = not isPlayerOneTurn

        # Place last piece and set nextPiece to nothing
        if self._game.gui_mode:
            self._game._showPlayerName(isPlayerOneTurn)
        self._game.makeLastMove()

        if self._game.gui_mode:
            self._game.showGameState()

        if qutil.isGameOver(self._game.board):
            if isPlayerOneTurn:
                self.display["text"] = "Player 1 Won!"
                self.display2["text"] = ""
            else:
                self.display["text"] = "Player 2 Won!"
                self.display2["text"] = ""
            return
        else:
            self.display["text"] = "DRAW!"
            self.display2["text"] = ""

    def updateCurrent(self, piece):
        target = self.currentButton
        target.configure(image=self._photos[piece], bg="#876c3e", text="")
        piece = self._pieces[qutil.get2dCoords(piece)]
        self._takenPieces.add(piece)
        piece.configure(state=tk.DISABLED)

    def updateCell(self, position):
        piece = self._game.currentPiece
        target = self._cells[qutil.get2dCoords(position)]
        target.configure(image=self._photos[piece], bg="#876c3e", text="")
        self._takenCells.add(target)
        self.currentButton.configure(image=self._photos[BLANK_TILE], text="current")

    def toggleGridFreeze(self, toggleFreezeOn=True):
        if toggleFreezeOn:
            for row in range(4):
                for col in range(4):
                    self._cells[(row, col)].configure(state=tk.DISABLED)
                    self._pieces[(row, col)].configure(state=tk.DISABLED)
        else:
            for row in range(4):
                for col in range(4):
                    if self._cells[(row, col)] not in self._takenCells:
                        self._cells[(row, col)].configure(state=tk.ACTIVE)
                    if self._pieces[(row, col)] not in self._takenPieces:
                        self._pieces[(row, col)].configure(state=tk.ACTIVE)

    def _update_display(self, msg, color="black"):
        self.display["text"] = msg
        self.display["fg"] = color

def main():
    gui = QuartoGUI()
    gui.mainloop()


if __name__ == "__main__":
    main()
