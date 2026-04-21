// import from .js file because im lazy to configure ts
import init, { Boid, RollingAverage, Vec2, World } from "./pkg/boids.js";

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

const BOIDS_SIZE = 6;
const INTERVAL_MS = 5;
const AVG_WINDOW = 100;

// world settings
let numBoids = 5000;
let protectedRange = 8;
let visibleRange = 32;
let avoidFactor = 0.05;
let alignFactor = 0.05;
let cohesionFactor = 0.0005;
let margin = 25;
let turnFactor = 0.2;
let maxSpeed = 4;
let minSpeed = 1;

let intervalId: number = undefined;

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

// https://stackoverflow.com/questions/2353211/
function hueToRgb(p: number, q: number, t: number): number {
    if (t < 0) t += 1;
    if (t > 1) t -= 1;
    if (t < 1 / 6) return p + (q - p) * 6 * t;
    if (t < 1 / 2) return q;
    if (t < 2 / 3) return p + (q - p) * (2 / 3 - t) * 6;
    return p;
}

function hslToRgb(h: number, s: number, l: number): [number, number, number] {
    if (s == 0) {
        return [l, l, l];
    }
    const q = l < 0.5 ? l * (1 + s) : l + s - l * s;
    const p = 2 * l - q;
    let r = hueToRgb(p, q, h + 1 / 3);
    let g = hueToRgb(p, q, h);
    let b = hueToRgb(p, q, h - 1 / 3);

    return [Math.round(r * 255), Math.round(g * 255), Math.round(b * 255)];
}

// https://stackoverflow.com/questions/17525215/calculate-color-values-from-green-to-red/17527156#17527156
function speedToColor(speed: Vec2): string {
    // slower boids are more red
    // faster boids are more green
    let speedMag = speed.length();
    let hue = ((speedMag - minSpeed) / (maxSpeed - minSpeed)) * 120;
    let [r, g, b] = hslToRgb(hue / 360, 1, 0.5);
    return `rgb(${r}, ${g}, ${b})`;
}

function drawBoids(world: World) {
    ctx.fillStyle = "black";
    ctx.fillRect(0, 0, canvas.width, canvas.height);
    for (let i = 0; i < numBoids; i++) {
        let boid = world.get_boid(i);
        // cache pos and vel to avoid repeated WASM allocations
        let pos = boid.pos;
        let vel = boid.vel;

        let vel_norm = vel.normalize();
        let vel_dir = vel_norm.mul_num(BOIDS_SIZE);
        vel_norm.free();

        let rot1 = vel_dir.rotate((Math.PI * 11) / 12);
        let bot_left = rot1.add_vec(pos);
        rot1.free();

        let rot2 = vel_dir.rotate((Math.PI * 13) / 12);
        let bot_right = rot2.add_vec(pos);
        rot2.free();
        vel_dir.free();

        ctx.fillStyle = speedToColor(vel);
        ctx.beginPath();
        ctx.moveTo(pos.x, pos.y);
        ctx.lineTo(bot_left.x, bot_left.y);
        ctx.lineTo(bot_right.x, bot_right.y);
        ctx.fill();

        bot_left.free();
        bot_right.free();
        vel.free();
        pos.free();
        boid.free();
    }

    if (debug) {
        // draw protected range
        ctx.strokeStyle = "red";
        ctx.lineWidth = 1;
        for (let i = 0; i < world.num_boids(); i++) {
            let boid = world.get_boid(i);
            let pos = boid.pos;
            ctx.beginPath();
            ctx.arc(pos.x, pos.y, world.protect_range, 0, Math.PI * 2);
            ctx.stroke();
            pos.free();
            boid.free();
        }

        // draw visible range
        ctx.strokeStyle = "blue";
        for (let i = 0; i < world.num_boids(); i++) {
            let boid = world.get_boid(i);
            let pos = boid.pos;
            ctx.beginPath();
            ctx.arc(pos.x, pos.y, world.visible_range, 0, Math.PI * 2);
            ctx.stroke();
            pos.free();
            boid.free();
        }

        // draw margins
        ctx.strokeStyle = "white";
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
    canvas.width = window.outerWidth;
    canvas.height = window.outerHeight;
    let dims = Vec2.new(canvas.width, canvas.height);

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
            Vec2.js_rand_01().mul_vec(dims),
            Vec2.js_rand_01()
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

    let avgTick = RollingAverage.new(AVG_WINDOW);
    let avgRender = RollingAverage.new(AVG_WINDOW);
    playButton.addEventListener("click", () => {
        if (intervalId) {
            clearInterval(intervalId);
            intervalId = undefined;
            playButton.innerText = "Play";
        } else {
            intervalId = setInterval(() => {
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
