import init, { Vec2, WasmQuadTree } from "./pkg/boids.js";
var canvas = document.getElementById("canvas");
var ctx = canvas.getContext("2d");
var inputText = document.getElementById("input-text");
var inputCache = 0;
var totalPointsText = document.getElementById("total-points");
var clearPointsButton = document.getElementById("clear-points");
var POINT_RADIUS = 3;
var DRAG_DIST = 20;
function isDigit(key) {
    return key.length == 1 && "0" <= key && key <= "9";
}
function drawTreeRecur(tree, nodeIdx, tlCorner, dims) {
    ctx.strokeStyle = "black";
    ctx.lineWidth = 2;
    ctx.beginPath();
    ctx.rect(tlCorner.x, tlCorner.y, dims.x, dims.y);
    ctx.stroke();
    ctx.fillStyle = "red";
    ctx.strokeStyle = "red";
    for (var i = 0; i < tree.node_len(nodeIdx); i++) {
        var pos = tree.node_item_pos(nodeIdx, i);
        ctx.beginPath();
        ctx.arc(pos.x, pos.y, POINT_RADIUS, 0, Math.PI * 2);
        ctx.stroke();
        ctx.fill();
    }
    var shift = dims.div_num(2);
    var childIdx = tree.node_children(nodeIdx);
    var shiftDims = [Vec2.zero(), Vec2.new(1, 0), Vec2.new(0, 1), Vec2.from(1)];
    childIdx.forEach(function (idx, i) {
        if (idx === 0)
            return;
        drawTreeRecur(tree, idx, tlCorner.add_vec(shift.mul_vec(shiftDims[i])), shift);
    });
}
function drawTree(tree) {
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    drawTreeRecur(tree, 0, Vec2.zero(), tree.dims());
}
function drawCirclePoints(center, radius, points) {
    ctx.fillStyle = "green";
    ctx.strokeStyle = "green";
    for (var i = 0; i < points.len(); i++) {
        var pos = points.get(i);
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
init().then(function () {
    canvas.width = window.innerWidth;
    canvas.height = window.innerHeight;
    var dims = Vec2.new(canvas.width, canvas.height);
    var tree = WasmQuadTree.new(dims);
    var mouseDownPos = Vec2.zero();
    function windowClick(pos) {
        tree.push(pos);
        drawTree(tree);
        totalPointsText.innerText = "Total points: ".concat(tree.len());
    }
    function windowDrag(startPos, endPos) {
        var radius = startPos.distance(endPos);
        var points = tree.query_circle(startPos, radius);
        drawTree(tree);
        drawCirclePoints(startPos, radius, points);
    }
    window.addEventListener("mousedown", function (event) {
        mouseDownPos = Vec2.new(event.clientX, event.clientY);
    });
    window.addEventListener("mouseup", function (event) {
        var mouseUpPos = Vec2.new(event.clientX, event.clientY);
        if (mouseDownPos.distance(mouseUpPos) < DRAG_DIST) {
            windowClick(mouseUpPos);
        }
        else {
            windowDrag(mouseDownPos, mouseUpPos);
        }
    });
    window.addEventListener("keydown", function (event) {
        if (isDigit(event.key)) {
            inputCache = inputCache * 10 + parseInt(event.key);
            inputText.innerText = "Add points: ".concat(inputCache);
        }
        else if (event.key == "Backspace") {
            inputCache = Math.max(0, Math.floor(inputCache / 10));
            inputText.innerText = "Add points: ".concat(inputCache);
        }
        else if (event.key == "Enter") {
            for (var i = 0; i < inputCache; i++) {
                tree.push(Vec2.new(Math.random(), Math.random()).mul_vec(tree.dims()));
            }
            drawTree(tree);
            inputCache = 0;
            inputText.innerText = "Add points: ".concat(inputCache);
            totalPointsText.innerText = "Total points: ".concat(tree.len());
        }
    });
    clearPointsButton.onclick = function () {
        tree.clear();
        totalPointsText.innerText = "Total points: ".concat(tree.len());
        inputCache = 0;
        inputText.innerText = "Add points: ".concat(inputCache);
        drawTree(tree);
    };
});
