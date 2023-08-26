import init, { RollingAverage, Vec2, World } from "./pkg/boids.js";
var canvas = document.getElementById("canvas");
var ctx = canvas.getContext("2d");
var playButton = document.getElementById("play");
var debugButton = document.getElementById("show-debug");
var debug = false;
debugButton.addEventListener("click", function () {
    debug = !debug;
    debugButton.innerText = debug ? "Hide Debug" : "Show Debug";
});
var tickMsText = document.getElementById("tick-ms");
var BOIDS_SIZE = 12;
var INTERVAL_MS = 10;
var AVG_WINDOW = 100;
var Duration = (function () {
    function Duration() {
        this.start = performance.now();
    }
    Duration.prototype.elapsed_ms = function () {
        return performance.now() - this.start;
    };
    return Duration;
}());
function randRange(min, max) {
    return Math.random() * (max - min) + min;
}
function drawBoids(world) {
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    ctx.fillStyle = "blue";
    for (var i = 0; i < world.num_boids(); i++) {
        var boid = world.get_boid(i);
        var vel = boid.vel.normalize().mul_num(BOIDS_SIZE);
        var bot_left = vel.rotate((Math.PI * 11) / 12).add_vec(boid.pos);
        var bot_right = vel.rotate((Math.PI * 13) / 12).add_vec(boid.pos);
        ctx.beginPath();
        ctx.moveTo(boid.pos.x, boid.pos.y);
        ctx.lineTo(bot_left.x, bot_left.y);
        ctx.lineTo(bot_right.x, bot_right.y);
        ctx.fill();
    }
    if (debug) {
        ctx.strokeStyle = "red";
        ctx.lineWidth = 1;
        for (var i = 0; i < world.num_boids(); i++) {
            var boid = world.get_boid(i);
            ctx.beginPath();
            ctx.arc(boid.pos.x, boid.pos.y, world.protect_range, 0, Math.PI * 2);
            ctx.stroke();
        }
        ctx.strokeStyle = "green";
        for (var i = 0; i < world.num_boids(); i++) {
            var boid = world.get_boid(i);
            ctx.beginPath();
            ctx.arc(boid.pos.x, boid.pos.y, world.visible_range, 0, Math.PI * 2);
            ctx.stroke();
        }
        ctx.strokeStyle = "black";
        ctx.setLineDash([5, 5]);
        ctx.beginPath();
        ctx.rect(world.margin, world.margin, canvas.width - world.margin * 2, canvas.height - world.margin * 2);
        ctx.stroke();
    }
}
init().then(function () {
    canvas.width = window.innerWidth;
    canvas.height = window.innerHeight;
    var dims = Vec2.new(canvas.width, canvas.height);
    var num_boids = 5000;
    var protect_range = 8;
    var visible_range = 32;
    var avoid_factor = 0.05;
    var align_factor = 0.05;
    var cohesion_factor = 0.0005;
    var margin = 40;
    var turn_factor = 0.2;
    var max_speed = 6;
    var min_speed = 2;
    var world = World.new(dims, visible_range, protect_range, avoid_factor, align_factor, cohesion_factor, margin, turn_factor, max_speed, min_speed);
    for (var i = 0; i < num_boids; i++) {
        world.add_boid(Vec2.rand_01().mul_vec(dims), Vec2.rand_01()
            .mul_num(2)
            .sub_num(1)
            .normalize()
            .mul_num(randRange(min_speed, max_speed)));
    }
    drawBoids(world);
    var interval = undefined;
    var avgFps = RollingAverage.new(AVG_WINDOW);
    playButton.addEventListener("click", function () {
        if (interval) {
            clearInterval(interval);
            interval = undefined;
            playButton.innerText = "Play";
        }
        else {
            interval = setInterval(function () {
                var start = new Duration();
                world.tick();
                avgFps.push(start.elapsed_ms());
                tickMsText.innerText = "Tick ms: ".concat(avgFps.query().toFixed(1));
                drawBoids(world);
            }, INTERVAL_MS);
            playButton.innerText = "Pause";
        }
    });
});
