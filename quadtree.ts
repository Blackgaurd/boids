// must import from .js file, not .ts file
import init, { Vec2, WasmQuadTree, WasmVec2Array } from "./pkg/boids.js";

let canvas = document.getElementById("canvas") as HTMLCanvasElement;
let ctx = canvas.getContext("2d");

let inputText = document.getElementById("input-text") as HTMLParagraphElement;
let inputCache = 0;

let totalPointsText = document.getElementById(
    "total-points"
) as HTMLParagraphElement;
let clearPointsButton = document.getElementById(
    "clear-points"
) as HTMLButtonElement;

const POINT_RADIUS = 3;
const DRAG_DIST = 20;

function isDigit(key: string) {
    return key.length == 1 && "0" <= key && key <= "9";
}
function drawTreeRecur(
    tree: WasmQuadTree,
    nodeIdx: number,
    tlCorner: Vec2,
    dims: Vec2
) {
    // draw the node's borders
    ctx.strokeStyle = "black";
    ctx.lineWidth = 2;
    ctx.beginPath();
    ctx.rect(tlCorner.x, tlCorner.y, dims.x, dims.y);
    ctx.stroke();

    // draw the node's points
    ctx.fillStyle = "red";
    ctx.strokeStyle = "red";
    for (let i = 0; i < tree.node_len(nodeIdx); i++) {
        let pos = tree.node_item_pos(nodeIdx, i);
        ctx.beginPath();
        ctx.arc(pos.x, pos.y, POINT_RADIUS, 0, Math.PI * 2);
        ctx.stroke();
        ctx.fill();
    }

    let shift = dims.div_num(2);
    let childIdx = tree.node_children(nodeIdx);
    let shiftDims = [Vec2.zero(), Vec2.new(1, 0), Vec2.new(0, 1), Vec2.from(1)];
    childIdx.forEach((idx, i) => {
        if (idx === 0) return;
        drawTreeRecur(
            tree,
            idx,
            tlCorner.add_vec(shift.mul_vec(shiftDims[i])),
            shift
        );
    });
}
function drawTree(tree: WasmQuadTree) {
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    drawTreeRecur(tree, 0, Vec2.zero(), tree.dims());
}
function drawCirclePoints(center: Vec2, radius: number, points: WasmVec2Array) {
    ctx.fillStyle = "green";
    ctx.strokeStyle = "green";
    for (let i = 0; i < points.len(); i++) {
        let pos = points.get(i);
        ctx.beginPath();
        ctx.arc(pos.x, pos.y, POINT_RADIUS, 0, Math.PI * 2);
        ctx.stroke();
        ctx.fill();
    }

    ctx.fillStyle = "blue";
    ctx.strokeStyle = "blue";
    ctx.beginPath();
    ctx.arc(center.x, center.y, POINT_RADIUS, 0, Math.PI * 2);
    ctx.stroke();
    ctx.fill();
    ctx.beginPath();
    ctx.arc(center.x, center.y, radius, 0, Math.PI * 2);
    ctx.stroke();
}

init().then(() => {
    // wasm initiated
    canvas.width = window.innerWidth;
    canvas.height = window.innerHeight;
    let dims = Vec2.new(canvas.width, canvas.height);
    let tree = WasmQuadTree.new(dims);
    let mouseDownPos = Vec2.zero();

    function windowClick(pos: Vec2) {
        tree.push(pos);
        drawTree(tree);
        totalPointsText.innerText = `Total points: ${tree.len()}`;
    }
    function windowDrag(startPos: Vec2, endPos: Vec2) {
        let radius = startPos.distance(endPos);
        let points = tree.query_circle(startPos, radius);
        drawTree(tree);
        drawCirclePoints(startPos, radius, points);
    }

    window.addEventListener("mousedown", (event) => {
        mouseDownPos = Vec2.new(event.clientX, event.clientY);
    });
    window.addEventListener("mouseup", (event) => {
        let mouseUpPos = Vec2.new(event.clientX, event.clientY);
        if (mouseDownPos.distance(mouseUpPos) < DRAG_DIST) {
            windowClick(mouseUpPos);
        } else {
            windowDrag(mouseDownPos, mouseUpPos);
        }
    });
    window.addEventListener("keydown", (event) => {
        if (isDigit(event.key)) {
            inputCache = inputCache * 10 + parseInt(event.key);
            inputText.innerText = `Add points: ${inputCache}`;
        } else if (event.key == "Backspace") {
            inputCache = Math.max(0, Math.floor(inputCache / 10));
            inputText.innerText = `Add points: ${inputCache}`;
        } else if (event.key == "Enter") {
            for (let i = 0; i < inputCache; i++) {
                tree.push(
                    Vec2.new(Math.random(), Math.random()).mul_vec(tree.dims())
                );
            }
            drawTree(tree);
            inputCache = 0;
            inputText.innerText = `Add points: ${inputCache}`;
            totalPointsText.innerText = `Total points: ${tree.len()}`;
        }
    });
    clearPointsButton.onclick = () => {
        tree.clear();
        totalPointsText.innerText = `Total points: ${tree.len()}`;
        inputCache = 0;
        inputText.innerText = `Add points: ${inputCache}`;
        drawTree(tree);
    };
});
