import init, { Vec2, QuadTree } from "./pkg/boids.js";

let canvas = document.getElementById("canvas");
let ctx = canvas.getContext("2d");

canvas.width = window.innerWidth;
canvas.height = window.innerHeight;

let inputText = document.getElementById("input-text");
let inputCache = 0;

let totalPointsText = document.getElementById("total-points");

let clearPointsButton = document.getElementById("clear-points");

const POINT_RADIUS = 3;

/**
 *
 * @param {QuadTree} tree
 * @param {number} nodeIdx
 * @param {Vec2} tlCorner
 * @param {Vec2} dims
 */
function drawTreeRecur(tree, nodeIdx, tlCorner, dims) {
    // draw the node borders
    ctx.strokeStyle = "black";
    ctx.lineWidth = 2;
    ctx.beginPath();
    ctx.rect(tlCorner.x, tlCorner.y, dims.x, dims.y);
    ctx.stroke();

    // draw the node points
    ctx.fillStyle = "red";
    ctx.strokeStyle = "red";
    let num_points = tree.node_num_points(nodeIdx);
    for (let i = 0; i < num_points; i++) {
        let point = tree.get_node_point(nodeIdx, i);
        ctx.beginPath();
        ctx.arc(point.x, point.y, POINT_RADIUS, 0, Math.PI * 2);
        ctx.stroke();
        ctx.fill();
    }

    let shift = dims.mul_num(0.5);
    [
        { nextIdx: tree.node_top_left(nodeIdx), shiftDims: Vec2.new(0, 0) },
        { nextIdx: tree.node_top_right(nodeIdx), shiftDims: Vec2.new(1, 0) },
        { nextIdx: tree.node_bot_left(nodeIdx), shiftDims: Vec2.new(0, 1) },
        { nextIdx: tree.node_bot_right(nodeIdx), shiftDims: Vec2.new(1, 1) },
    ].forEach((element) => {
        if (element.nextIdx === 0) return;
        drawTreeRecur(
            tree,
            element.nextIdx,
            tlCorner.add_vec(shift.mul_vec(element.shiftDims)),
            shift
        );
    });
}

/**
 *
 * @param {QuadTree} tree
 */
function drawTree(tree) {
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    drawTreeRecur(tree, 0, Vec2.zero(), tree.dims);
}

/**
 *
 * @param {Vec2} center
 * @param {number} radius
 * @param {Vec2[]} points
 */
function drawCirclePoints(center, radius, points) {
    ctx.fillStyle = "green";
    ctx.strokeStyle = "green";
    points.forEach((element) => {
        ctx.beginPath();
        ctx.arc(element.x, element.y, POINT_RADIUS, 0, Math.PI * 2);
        ctx.stroke();
        ctx.fill();
    });

    ctx.fillStyle = "blue";
    ctx.strokeStyle = "blue";
    ctx.beginPath();
    ctx.arc(center.x, center.y, 2, 0, Math.PI * 2);
    ctx.stroke();
    ctx.fill();
    ctx.beginPath();
    ctx.arc(center.x, center.y, radius, 0, Math.PI * 2);
    ctx.stroke();
}

/**
 *
 * @param {QuadTree} tree
 * @param {Vec2} center
 * @param {number} radius
 * @returns {Vec2[]}
 */
function queryCircle(tree, center, radius) {
    let points = tree.wasm_query_circle(center, radius);
    let ret = [];
    for (let i = 0; i < points.len(); i++) {
        ret.push(points.get(i));
    }
    return ret;
}

/**
 *
 * @param {string} str
 * @returns {boolean}
 */
export function isDigit(str) {
    if (str.length !== 1) return false;
    let code = str.charCodeAt(0);
    return 48 <= code && code <= 57;
}

init().then(() => {
    let tree = QuadTree.new(Vec2.new(canvas.width, canvas.height));
    const DRAG_DIST = 20;
    let mouseDownPos = Vec2.zero();

    /**
     *
     * @param {Vec2} pos
     */
    const windowClick = (pos) => {
        tree.add_point(pos);
        drawTree(tree);
        totalPointsText.innerText = `Total points: ${tree.num_points}`;
    };

    /**
     *
     * @param {Vec2} startPos
     * @param {Vec2} endPos
     */
    const windowDrag = (startPos, endPos) => {
        let radius = startPos.distance(endPos);
        let points = queryCircle(tree, startPos, radius);
        drawTree(tree);
        drawCirclePoints(startPos, radius, points);
    };

    window.addEventListener("mousedown", (event) => {
        mouseDownPos = Vec2.new(event.clientX, event.clientY);
    });
    window.addEventListener("mouseup", (event) => {
        let mouseUpPos = Vec2.new(event.clientX, event.clientY);
        let dist = mouseDownPos.distance(mouseUpPos);
        if (dist < DRAG_DIST) {
            // register click
            windowClick(mouseUpPos);
        } else {
            // register drag
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
                tree.add_point(
                    Vec2.new(Math.random(), Math.random()).mul_vec(tree.dims)
                );
            }
            drawTree(tree);
            inputCache = 0;
            inputText.innerText = `Add points: ${inputCache}`;
            totalPointsText.innerText = `Total points: ${tree.num_points}`;
        }
    });
    clearPointsButton.onclick = () => {
        tree.clear();
        totalPointsText.innerText = `Total points: ${tree.num_points}`;
        inputCache = 0;
        inputText.innerText = `Add points: ${inputCache}`;
        drawTree();
    };
});
