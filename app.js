import init, { Vec2, World } from './pkg/boids.js'

let debugButton = document.getElementById('show-debug')
let debugPanel = document.getElementById('debug')
let debugToggle = false
debugButton.innerText = 'Show debug'
debugPanel.style.display = 'none'
debugButton.addEventListener('click', () => {
    if (!debugToggle) {
        debugPanel.style.display = 'block'
        debugButton.innerText = 'Hide debug'
        debugToggle = true
    } else {
        debugPanel.style.display = 'none'
        debugButton.innerText = 'Show debug'
        debugToggle = false
    }
})

function updateDebugPanel(world) {
    if (!debugToggle) {
        return
    }
    let maxSpeed = world.max_boid_speed()
    let minSpeed = world.min_boid_speed()
    let numBoids = world.num_boids()
    debugPanel.innerText = `Boids: ${numBoids}
    Param max speed: ${world.max_speed}
    Max speed: ${maxSpeed.toFixed(2)}
    Min speed: ${minSpeed.toFixed(2)}`
}

function drawBoids(ctx, world) {
    ctx.fillStyle = 'blue'
    ctx.clearRect(0, 0, ctx.canvas.width, ctx.canvas.height)
    for (let i = 0; i < world.num_boids(); i++) {
        let boid = world.get_boid(i)
        let pos = boid.pos
        ctx.beginPath()
        ctx.arc(pos.x, pos.y, 5, 0, 2 * Math.PI)
        ctx.fill()
    }

    if (debugToggle) {
        ctx.strokeStyle = 'red'
        for (let i = 0; i < world.num_boids(); i++) {
            let boid = world.get_boid(i)
            let pos = boid.pos
            let vel = boid.vel
            ctx.beginPath()
            ctx.moveTo(pos.x, pos.y)
            ctx.lineTo(pos.x + vel.x * 10, pos.y + vel.y * 10)
            ctx.stroke()
        }
    }
}

init().then(() => {
    let canvas = document.getElementById('canvas')
    let ctx = canvas.getContext('2d')
    canvas.width = window.innerWidth
    canvas.height = window.innerHeight

    let params = {
        protect_range: 8,
        avoid_factor: 0.05,
        visible_range: 40,
        align_factor: 0.05,
        cohesion_factor: 0.0005,
        margin: 100,
        turn_factor: 0.5,
        max_speed: 6,
        min_speed: 2,
    }
    let world = World.new(
        Vec2.new(canvas.width, canvas.height),
        params.protect_range,
        params.avoid_factor,
        params.visible_range,
        params.align_factor,
        params.cohesion_factor,
        params.margin,
        params.turn_factor,
        params.max_speed,
        params.min_speed
    )

    for (let i = 0; i < 10; i++) {
        world.add_boid(
            Vec2.new(
                Math.random() * canvas.width,
                Math.random() * canvas.height
            ),
            Vec2.new(Math.random() * 2 - 1, Math.random() * 2 - 1)
                .normalize()
                .mult_num(3)
        )
    }

    // draw once to spawn boids
    drawBoids(ctx, world)
    updateDebugPanel(world)
    let interval = null

    let playButton = document.getElementById('play')
    playButton.innerText = interval ? 'Pause' : 'Play'
    const playPause = () => {
        if (interval) {
            clearInterval(interval)
            interval = null
            playButton.innerText = 'Play'
        } else {
            interval = setInterval(() => {
                world.tick()
                drawBoids(ctx, world)
                updateDebugPanel(world)
            }, 30)
            playButton.innerText = 'Pause'
        }
    }
    playButton.addEventListener('click', playPause)
})
