import { Wall, Function } from "wall";
import { InteractiveCanvas } from "interactive_canvas";

var wall = Wall.new(Function.Rueppel);

var modulus = 2n;

var colorTable = [
    "#FFFFFF",
    "#000000",
]

// canvas setup
const canvas = document.getElementById("canvas");
canvas.height = 500;
canvas.width = 500;
canvas.style.outlineColor = "black";
canvas.style.outlineWidth = "3px";
canvas.style.outlineStyle = "solid";
const GRID_SIZE = 20;
var GRID_LINES = false;

// selector setup
const func_select = document.getElementById("sequence");
const functions = {
    "square": Function.Square,
    "debruijn": Function.DeBruijn,
    "rueppel": Function.Rueppel,
    "rook": Function.Rook,
    "knight": Function.Knight,
};
func_select.addEventListener("change", (event) => {
    var val = event.target.value;
    wall = Wall.new(functions[val]);
});

for (let name in functions) {
    var option = document.createElement("option");
    option.innerText = name;
    option.value = name;
    if (name == 'rueppel') {
        option.selected = true;
    }
    func_select.appendChild(option);
}

// color picker setup
function create_color_row(index, default_color) {
    var row = document.createElement('tr');
    var cell = document.createElement('td');
    var pick = document.createElement('input');
    pick.type = "color";
    pick.value = default_color;
    pick.onchange = (event) => {
        colorTable[index] = event.target.value;
    }
    cell.appendChild(pick);
    row.appendChild(cell);
    color_table.appendChild(row);
}
const color_table = document.getElementById("colorpicker");
const add_color = document.getElementById("add_color");
const remove_color = document.getElementById("remove_color");
// add 0 selection
create_color_row(0, "#FFFFFF");
// add 1 selection
create_color_row(1, "#000000");

add_color.addEventListener("click", (event) => {
    var color = "#000000";
    colorTable.push(color);
    create_color_row(colorTable.length - 1, color);
    modulus += 1n;
});

remove_color.addEventListener("click", (event) => {
    if (colorTable.length > 2) {
        colorTable.pop();
        modulus -= 1n;
        color_table.removeChild(color_table.lastChild);
    }
});

function mod(x, y) {
    return ((x % y) + y) % y;
}

function draw(ctx) {
    // draw grid between p1 and p2
    // grid should line up at 0,0
    // render just off screen, so round down on p1 and round up on p2

    var p1 = ctx.transformedPoint(0,0);
    var p2 = ctx.transformedPoint(canvas.width,canvas.height);
    var start_row = Math.ceil(p1.x/GRID_SIZE) - 1;
    var end_row = Math.ceil(p2.x/GRID_SIZE) + 1;
    var start_col = Math.ceil(p1.y/GRID_SIZE) - 1;
    var end_col = Math.ceil(p2.y/GRID_SIZE) + 1;

    var low_x = start_row*GRID_SIZE;
    var low_y = start_col*GRID_SIZE;
    var high_x = end_row*GRID_SIZE;
    var high_y = end_col*GRID_SIZE;

    if (GRID_LINES) {
        for (var x=low_x; x<=high_x; x += GRID_SIZE) {
            ctx.beginPath();
            ctx.moveTo(x, low_y);
            ctx.lineTo(x, high_y);
            ctx.stroke();
        }

        for (var y=low_y; y<=high_y; y += GRID_SIZE) {
            ctx.beginPath();
            ctx.moveTo(low_x, y);
            ctx.lineTo(high_x, y);
            ctx.stroke();
        }
    }
    for (var row=start_row; row<=end_row; row++) {
        for (var col=start_col; col<=end_col; col++) {
            const item = mod(wall.get(col, row), modulus);
            var color = colorTable[item];
            if (color != "white") {
                ctx.beginPath();
                ctx.fillStyle = color;
                ctx.rect(row*GRID_SIZE, col*GRID_SIZE, GRID_SIZE, GRID_SIZE);
                ctx.fill();
            }
        }
    }

}

var IC = new InteractiveCanvas(canvas, draw);

IC.ctx.translate(0,2*GRID_SIZE);
IC.start();

