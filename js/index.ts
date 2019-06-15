import { Board } from "../rs/pkg/tetris";

Promise.all([import("../rs/pkg/tetris"), import("../rs/pkg/tetris_bg")]).then(
  ([{ Board, Cell }, { memory }]) => {
    class Game {
      context: CanvasRenderingContext2D | null;
      tickDelay: number;
      tickTimerId?: number;
      board: Board;
      width: number;
      height: number;
      cellSize: number;
      borderSize: number;
      constructor(width: number, height: number, cellSize: number) {
        const canvas = <HTMLCanvasElement>document.getElementById("game-board");
        this.context = canvas.getContext("2d");

        this.board = Board.new(width, height);
        this.width = this.board.get_width();
        this.height = this.board.get_height();

        this.borderSize = 1;
        this.cellSize = cellSize;
        canvas.height =
          this.height * this.cellSize + (this.height + this.borderSize);
        canvas.width =
          this.width * this.cellSize + (this.width + this.borderSize);

        this.tickDelay = 1000;
        this.tick();
      }
      tick() {
        this.tickTimerId = window.setTimeout(() => {
          this.drawGrid();
          this.drawBoard();
          this.tick();
        }, this.tickDelay);
      }
      drawGrid() {
        if (this.context == null) return;

        this.context.beginPath();
        this.context.strokeStyle = "#000";

        for (let row = 0; row <= this.width; row += 1) {
          const x = row * (this.cellSize + this.borderSize) + 0.5;
          this.context.moveTo(x, 0);
          this.context.lineTo(
            x,
            (this.cellSize + this.borderSize) * this.height + 1
          );
        }

        for (let column = 0; column <= this.height; column += 1) {
          const y = column * (this.cellSize + this.borderSize) + 0.5;
          this.context.moveTo(0, y);
          this.context.lineTo((this.cellSize + 1) * this.width + 1, y);
        }

        this.context.stroke();
      }
      drawBoard() {
        if (this.context == null) return;

        const cellsPtr = this.board.cells();
        const cells = new Uint8Array(
          memory.buffer,
          cellsPtr,
          this.width * this.height
        );

        this.context.beginPath();

        for (let row = 0; row < this.width; row += 1) {
          for (let column = 0; column < this.height; column += 1) {
            const index = row * this.width + column;
            const cell = cells[index];
            const x = row * (this.cellSize + this.borderSize) + 1;
            const y = column * (this.cellSize + this.borderSize) + 1;
            this.context.moveTo(x, y);
            if (cell !== Cell.None) {
              switch (cell) {
                case Cell.IBlock:
                  this.context.fillStyle = "#00ffff";
                  break;
                case Cell.JBlock:
                  this.context.fillStyle = "#0000ff";
                  break;
                case Cell.LBlock:
                  this.context.fillStyle = "#ffaa00";
                  break;
                case Cell.OBlock:
                  this.context.fillStyle = "#ffff00";
                  break;
                case Cell.SBlock:
                  this.context.fillStyle = "#00ff00";
                  break;
                case Cell.TBlock:
                  this.context.fillStyle = "#9900ff";
                  break;
                case Cell.ZBlock:
                  this.context.fillStyle = "#ff0000";
                  break;
                default:
                  this.context.fillStyle = "#ffffff";
                  break;
              }
              this.context.fillRect(x, y, this.cellSize, this.cellSize);
            }
          }
        }

        this.context.stroke();
      }
    }

    const game = new Game(10, 25, 25);

    console.log(memory);
  }
);
