// import from .js file because im lazy to configure ts
import init, { RollingAverage, Vec2, World } from "./pkg/boids.js";

let canvas = document.getElementById("canvas") as HTMLCanvasElement;
let ctx = canvas.getContext("2d"); //TODO: set alpha to false

let playButton = document.getElementById("play") as HTMLButtonElement;

let debugButton = document.getElementById("show-debug") as HTMLButtonElement;
let debug = false;
debugButton.addEventListener("click", () => {
    debug = !debug;
    debugButton.innerText = debug ? "Hide Debug" : "Show Debug";
});

let tickMsText = document.getElementById("tick-ms") as HTMLParagraphElement;

const BOIDS_SIZE = 12;
const INTERVAL_MS = 10;
const AVG_WINDOW = 100;

class Duration {
    start: number;
    constructor() {
        this.start = performance.now();
    }
    public elapsed_ms(): number {
        return performance.now() - this.start;
    }
}

function randRange(min: number, max: number) {
    return Math.random() * (max - min) + min;
}

function drawBoids(world: World) {
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    ctx.fillStyle = "blue";
    for (let i = 0; i < world.num_boids(); i++) {
        let boid = world.get_boid(i);
        let vel = boid.vel.normalize().mul_num(BOIDS_SIZE);
        let bot_left = vel.rotate((Math.PI * 11) / 12).add_vec(boid.pos);
        let bot_right = vel.rotate((Math.PI * 13) / 12).add_vec(boid.pos);

        ctx.beginPath();
        ctx.moveTo(boid.pos.x, boid.pos.y);
        ctx.lineTo(bot_left.x, bot_left.y);
        ctx.lineTo(bot_right.x, bot_right.y);
        ctx.fill();
    }

    if (debug) {
        // draw protected range
        ctx.strokeStyle = "red";
        ctx.lineWidth = 1;
        for (let i = 0; i < world.num_boids(); i++) {
            let boid = world.get_boid(i);
            ctx.beginPath();
            ctx.arc(
                boid.pos.x,
                boid.pos.y,
                world.protect_range,
                0,
                Math.PI * 2
            );
            ctx.stroke();
        }

        // draw visible range
        ctx.strokeStyle = "green";
        for (let i = 0; i < world.num_boids(); i++) {
            let boid = world.get_boid(i);
            ctx.beginPath();
            ctx.arc(
                boid.pos.x,
                boid.pos.y,
                world.visible_range,
                0,
                Math.PI * 2
            );
            ctx.stroke();
        }

        // draw margins
        ctx.strokeStyle = "black";
        ctx.setLineDash([5, 5]);
        ctx.beginPath();
        ctx.rect(
            world.margin,
            world.margin,
            canvas.width - world.margin * 2,
            canvas.height - world.margin * 2
        );
        ctx.stroke();
    }
}

init().then(() => {
    // wasm initialized
    canvas.width = window.innerWidth;
    canvas.height = window.innerHeight;
    let dims = Vec2.new(canvas.width, canvas.height);
    let num_boids = 5000;

    // world settings
    let protect_range = 8;
    let visible_range = 32;
    let avoid_factor = 0.05;
    let align_factor = 0.05;
    let cohesion_factor = 0.0005;
    let margin = 40;
    let turn_factor = 0.2;
    let max_speed = 6;
    let min_speed = 2;

    let world = World.new(
        dims,
        visible_range,
        protect_range,
        avoid_factor,
        align_factor,
        cohesion_factor,
        margin,
        turn_factor,
        max_speed,
        min_speed
    );

    for (let i = 0; i < num_boids; i++) {
        world.add_boid(
            Vec2.rand_01().mul_vec(dims),
            Vec2.rand_01()
                .mul_num(2)
                .sub_num(1)
                .normalize()
                .mul_num(randRange(min_speed, max_speed))
        );
    }
    drawBoids(world);

    let interval: number = undefined;
    let avgFps = RollingAverage.new(AVG_WINDOW);
    playButton.addEventListener("click", () => {
        if (interval) {
            clearInterval(interval);
            interval = undefined;
            playButton.innerText = "Play";
        } else {
            interval = setInterval(() => {
                let start = new Duration();
                world.tick();
                // world.tick_brute_force();
                avgFps.push(start.elapsed_ms());
                tickMsText.innerText = `Tick ms: ${avgFps.query().toFixed(1)}`;
                drawBoids(world);
            }, INTERVAL_MS);
            playButton.innerText = "Pause";
        }
    });
});
