import { Board } from "../rs/pkg/tetris";

Promise.all([import("../rs/pkg/tetris"), import("../rs/pkg/tetris_bg")]).then(
  ([{ Board }, { memory }]) => {
    class Game {
      context: CanvasRenderingContext2D | null;
      moveDelay: number;
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

        this.moveDelay = 1000;
        this.drawGrid();
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
    }

    const game = new Game(10, 25, 25);
    console.log(memory);
  }
);
