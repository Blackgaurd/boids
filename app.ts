// import from .js file because im lazy to configure ts
import init, { RollingAverage, Vec2, World } from "./pkg/boids.js";

let canvas = document.getElementById("canvas") as HTMLCanvasElement;
let ctx = canvas.getContext("2d");

let playButton = document.getElementById("play") as HTMLButtonElement;

let debugButton = document.getElementById("show-debug") as HTMLButtonElement;
let debug = false;
debugButton.addEventListener("click", () => {
    debug = !debug;
    debugButton.innerText = debug ? "Hide Debug" : "Show Debug";
});

let tickMsText = document.getElementById("tick-ms") as HTMLParagraphElement;
let renderMsText = document.getElementById("render-ms") as HTMLParagraphElement;

let protectRangeSlider = document.getElementById(
    "protect-range"
) as HTMLInputElement;
let visibleRangeSlider = document.getElementById(
    "visible-range"
) as HTMLInputElement;
let avoidFactorSlider = document.getElementById(
    "avoid-factor"
) as HTMLInputElement;
let alignFactorSlider = document.getElementById(
    "align-factor"
) as HTMLInputElement;
let cohesionFactorSlider = document.getElementById(
    "cohesion-factor"
) as HTMLInputElement;
let marginSlider = document.getElementById("margin") as HTMLInputElement;
let turnFactorSlider = document.getElementById(
    "turn-factor"
) as HTMLInputElement;
let boidsSizeSlider = document.getElementById("boids-size") as HTMLInputElement;

let boidsSize = 5;
const INTERVAL_MS = 5;
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
        let vel = boid.vel.normalize().mul_num(boidsSize);
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
    let numBoids = 15000;

    // world settings
    let protectedRange = 8;
    let visibleRange = 32;
    let avoidFactor = 0.05;
    let alignFactor = 0.05;
    let cohesionFactor = 0.0005;
    let margin = 25;
    let turnFactor = 0.2;
    let maxSpeed = 6;
    let minSpeed = 2;

    let world = World.new(
        dims,
        visibleRange,
        protectedRange,
        avoidFactor,
        alignFactor,
        cohesionFactor,
        margin,
        turnFactor,
        maxSpeed,
        minSpeed
    );

    for (let i = 0; i < numBoids; i++) {
        world.add_boid(
            Vec2.rand_01().mul_vec(dims),
            Vec2.rand_01()
                .mul_num(2)
                .sub_num(1)
                .normalize()
                .mul_num(randRange(minSpeed, maxSpeed))
        );
    }
    drawBoids(world);

    // param event listeners
    protectRangeSlider.value = protectedRange.toString();
    protectRangeSlider.addEventListener("input", () => {
        protectedRange = parseInt(protectRangeSlider.value);
        world.protect_range = protectedRange;
    });
    visibleRangeSlider.value = visibleRange.toString();
    visibleRangeSlider.addEventListener("input", () => {
        visibleRange = parseInt(visibleRangeSlider.value);
        world.visible_range = visibleRange;
    });
    avoidFactorSlider.value = (avoidFactor * 100).toString();
    avoidFactorSlider.addEventListener("input", () => {
        avoidFactor = parseFloat(avoidFactorSlider.value) / 100;
        world.avoid_factor = avoidFactor;
    });
    alignFactorSlider.value = (alignFactor * 100).toString();
    alignFactorSlider.addEventListener("input", () => {
        alignFactor = parseFloat(alignFactorSlider.value) / 100;
        world.align_factor = alignFactor;
    });
    cohesionFactorSlider.value = (cohesionFactor * 100_000).toString();
    cohesionFactorSlider.addEventListener("input", () => {
        cohesionFactor = parseFloat(cohesionFactorSlider.value) / 100_000;
        world.cohesion_factor = cohesionFactor;
    });
    marginSlider.value = margin.toString();
    marginSlider.addEventListener("input", () => {
        margin = parseInt(marginSlider.value);
        world.margin = margin;
    });
    turnFactorSlider.value = (turnFactor * 10).toString();
    turnFactorSlider.addEventListener("input", () => {
        turnFactor = parseFloat(turnFactorSlider.value) / 10;
        world.turn_factor = turnFactor;
    });
    boidsSizeSlider.value = boidsSize.toString();
    boidsSizeSlider.addEventListener("input", () => {
        boidsSize = parseInt(boidsSizeSlider.value);
    });

    let interval: number = undefined;
    let avgTick = RollingAverage.new(AVG_WINDOW);
    let avgRender = RollingAverage.new(AVG_WINDOW);
    playButton.addEventListener("click", () => {
        if (interval) {
            clearInterval(interval);
            interval = undefined;
            playButton.innerText = "Play";
        } else {
            interval = setInterval(() => {
                let start = new Duration();
                world.tick();
                avgTick.push(start.elapsed_ms());
                tickMsText.innerText = `Tick ms: ${avgTick.query().toFixed(1)}`;

                start = new Duration();
                drawBoids(world);
                avgRender.push(start.elapsed_ms());
                renderMsText.innerText = `Render ms: ${avgRender
                    .query()
                    .toFixed(1)}`;
            }, INTERVAL_MS);
            playButton.innerText = "Pause";
        }
    });
});
