import tkinter as tk
from tkinter import font
from PIL import ImageTk, Image
from quarto import *
from enum import Enum
import quarto_agents

BLANK_TILE = 16
AGENT_DELAY_MS = 1500


class MoveType(Enum):
    PLACE_PIECE = 1
    PICK_PIECE = 2


class QuartoGUI(tk.Tk):
    def __init__(self):
        super().__init__()
        self.title("Quarto Game")
        self.player1IsHuman = True
        self.player2IsHuman = False
        self.player1 = quarto_agents.RandomAgent()
        self.player2 = quarto_agents.NegamaxAgent(depth=2, searchWindow=64)
        self._game = QuartoGame(self.player1, self.player2, gui_mode=True, bin_mode=False)
        self.moveCounter = 0
        self.isPlayerOneTurn = True
        self.moveType = MoveType.PICK_PIECE

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
            master=display_frame, text="PRESS START", font=font.Font(size=18, weight="bold")
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

    def _checkIfGameOver(self):
        if qutil.isGameOver(self._game.board):
            self._toggleGridFreeze()
            if self.isPlayerOneTurn:
                self.display["text"] = "Player 1 Won!"
                self.display2["text"] = ""
            else:
                self.display["text"] = "Player 2 Won!"
                self.display2["text"] = ""
            return True
        elif self.moveCounter == 15:
            self._toggleGridFreeze()
            self.display["text"] = "DRAW!"
            self.display2["text"] = ""
            return True
        return False

    def _updateState(self):
        self.moveCounter += 1
        self.isPlayerOneTurn = not self.isPlayerOneTurn
        self.moveType = MoveType.PLACE_PIECE
        self.display["text"] = f"Player {1 if self.isPlayerOneTurn else 2}'s Turn"

        isHumanTurn = (self.isPlayerOneTurn and self.player1IsHuman) or (
            not self.isPlayerOneTurn and self.player2IsHuman
        )
        if isHumanTurn:
            self.display2["text"] = "Place your piece"
            self._toggleGridFreeze(toggleFreezeOn=False)
        else:
            self._toggleGridFreeze()
            self.display2["text"] = "Agent is placing current piece"

        return isHumanTurn

    def _handleMoveEnd(self):
        if self._checkIfGameOver():
            return
        isHumanTurn = self._updateState()
        if not isHumanTurn:
            self.after(AGENT_DELAY_MS, self.makeAgentMove)

    def _placePieceEvent(self, position):
        if self.moveType == MoveType.PICK_PIECE:
            self.display2["text"] = "Please pick a piece"
            return

        self.placePiece(position)
        if self._checkIfGameOver():
            return
        self.moveType = MoveType.PICK_PIECE
        self.display2["text"] = "Pick your opponent's next piece"

    def _pickPieceEvent(self, nextPiece):
        if self.moveType == MoveType.PLACE_PIECE:
            self.display2["text"] = "Please place your current piece first"
            return

        self.pickNextPiece(nextPiece)
        self._handleMoveEnd()

    def _createBoardGrid(self):
        self._cells = {}
        self._takenCells = set()
        grid_frame = tk.Frame(master=self, background="black", padx=20, pady=20)
        grid_frame.pack(padx=50, pady=50, side=tk.LEFT)
        for row in range(4):
            for col in range(4):
                linearCoords = 4 * row + col
                button = tk.Button(
                    master=grid_frame,
                    fg="black",
                    width=100,
                    height=100,
                    text=linearCoords,
                    font=("Helvetica 12 bold"),
                    image=self._photos[BLANK_TILE],
                    compound="center",
                    bd=0,
                    command=lambda position=linearCoords: self._placePieceEvent(position),
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
                linearCoords = 4 * row + col
                button = tk.Button(
                    master=grid_frame,
                    text=str(linearCoords),
                    font=("Helvetica 15 bold"),
                    image=self._photos[4 * row + col],
                    compound=tk.RIGHT,
                    relief=tk.FLAT,
                    bd=0,
                    command=lambda position=linearCoords: self._pickPieceEvent(position),
                )
                self._pieces[(row, col)] = button
                button.grid(row=row, column=col, padx=5, pady=5)

    def _resetGrids(self):
        for row in range(4):
            for col in range(4):
                self._cells[(row, col)].configure(
                    image=self._photos[BLANK_TILE], text=4 * row + col, state=tk.ACTIVE
                )
                self._pieces[(row, col)].configure(state=tk.ACTIVE)
        self.currentButton.configure(image=self._photos[BLANK_TILE], text="current")

    def _resetState(self, event):
        self._game.resetGame()
        self.moveCounter = 0
        self.isPlayerOneTurn = True
        self.moveType = MoveType.PICK_PIECE
        self._takenCells.clear()
        self._takenPieces.clear()
        self._resetGrids()
        self._update_display(msg="Ready?")
        self.play(event)

    def placePiece(self, position):
        self._updateCell(position)
        row, col = qutil.get2dCoords(position)
        self._game.board[row][col] = self._game.currentPiece
        self._game.availablePositions.remove(position)

    def pickNextPiece(self, nextPiece):
        self._updateCurrent(nextPiece)
        self._game.currentPiece = nextPiece
        self._game.availablePieces.remove(self._game.currentPiece)
        if self._game.gui_mode:
            print("Move successful")
            self._game.showGameState()

    def makeAgentFirstMove(self):
        firstMove = self._game.pickRandomAvailablePiece()
        self._game.makeFirstMove(firstMove)
        self._updateCurrent(firstMove)

        if self._game.gui_mode:
            self._game.showGameState()
        self._handleMoveEnd()

    def makeAgentMove(self):
        position, nextPiece = None, None
        if self.isPlayerOneTurn:
            position, nextPiece = self.player1.makeMove(
                self._game.getGameState(), gui_mode=self._game.gui_mode
            )
        else:
            position, nextPiece = self.player2.makeMove(
                self._game.getGameState(), gui_mode=self._game.gui_mode
            )

        self.placePiece(position)
        if self._checkIfGameOver():
            return

        self.pickNextPiece(nextPiece)
        if self._game.gui_mode:
            self._game.showGameState()
        self._handleMoveEnd()

    def play(self, event):
        self.display["text"] = "Player 1's turn"

        if self.player1IsHuman:
            self.display2["text"] = "Pick your opponent's first piece"
        else:
            self._toggleGridFreeze()
            self.display2["text"] = "Agent is picking the first piece"
            self.after(AGENT_DELAY_MS, self.makeAgentFirstMove)

        self.startButton.bind("<ButtonPress-1>", self._resetState)
        self.startButton.configure(text="Reset")

    def _updateCurrent(self, piece):
        target = self.currentButton
        target.configure(image=self._photos[piece], bg="#876c3e", text="")
        piece = self._pieces[qutil.get2dCoords(piece)]
        self._takenPieces.add(piece)
        piece.configure(state=tk.DISABLED)

    def _updateCell(self, position):
        piece = self._game.currentPiece
        target = self._cells[qutil.get2dCoords(position)]
        target.configure(image=self._photos[piece], bg="#876c3e", text="")
        self._takenCells.add(target)
        self.currentButton.configure(image=self._photos[BLANK_TILE], text="current")

    def _toggleGridFreeze(self, toggleFreezeOn=True):
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
