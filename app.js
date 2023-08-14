import init, { Vec2, World } from './pkg/boids.js'

let debugButton = document.getElementById('show-debug')
let debugToggle = false
debugButton.innerText = 'Show debug'
debugButton.addEventListener('click', () => {
    if (!debugToggle) {
        debugButton.innerText = 'Hide debug'
        debugToggle = true
    } else {
        debugButton.innerText = 'Show debug'
        debugToggle = false
    }
})

function rotateVec2(vec, radians) {
    const COS = Math.cos(radians),
        SIN = Math.sin(radians)
    return Vec2.new(vec.x * COS - vec.y * SIN, vec.x * SIN + vec.y * COS)
}

function drawBoids(ctx, world) {
    ctx.fillStyle = 'blue'
    ctx.clearRect(0, 0, ctx.canvas.width, ctx.canvas.height)
    for (let i = 0; i < world.num_boids(); i++) {
        let boid = world.get_boid(i)
        let vel = boid.vel.normalize()
        let bot_left = rotateVec2(vel, (Math.PI * 11) / 12)
            .mult_num(12)
            .add_vec(boid.pos)
        let bot_right = rotateVec2(vel, (Math.PI * 13) / 12)
            .mult_num(12)
            .add_vec(boid.pos)

        ctx.beginPath()
        ctx.moveTo(boid.pos.x, boid.pos.y)
        ctx.lineTo(bot_left.x, bot_left.y)
        ctx.lineTo(bot_right.x, bot_right.y)
        ctx.fill()
    }

    if (debugToggle) {
        // draw protect range
        ctx.strokeStyle = 'red'
        ctx.lineWidth = 1
        ctx.setLineDash([])
        for (let i = 0; i < world.num_boids(); i++) {
            let boid = world.get_boid(i)
            ctx.beginPath()
            ctx.arc(boid.pos.x, boid.pos.y, world.protect_range, 0, Math.PI * 2)
            ctx.stroke()
        }

        // draw visible range
        ctx.strokeStyle = 'green'
        ctx.lineWidth = 1
        ctx.setLineDash([])
        for (let i = 0; i < world.num_boids(); i++) {
            let boid = world.get_boid(i)
            ctx.beginPath()
            ctx.arc(boid.pos.x, boid.pos.y, world.visible_range, 0, Math.PI * 2)
            ctx.stroke()
        }

        // draw margin
        ctx.strokeStyle = 'black'
        ctx.lineWidth = 1
        ctx.setLineDash([5, 5])
        let top_left = Vec2.new(world.margin, world.margin)
        let top_right = Vec2.new(ctx.canvas.width - world.margin, world.margin)
        let bot_left = Vec2.new(world.margin, ctx.canvas.height - world.margin)
        let bot_right = Vec2.new(
            ctx.canvas.width - world.margin,
            ctx.canvas.height - world.margin
        )
        ctx.beginPath()
        ctx.moveTo(top_left.x, top_left.y)
        ctx.lineTo(top_right.x, top_right.y)
        ctx.lineTo(bot_right.x, bot_right.y)
        ctx.lineTo(bot_left.x, bot_left.y)
        ctx.lineTo(top_left.x, top_left.y)
        ctx.stroke()
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
        margin: 40,
        turn_factor: 0.2,
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

    for (let i = 0; i < 100; i++) {
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
            }, 30)
            playButton.innerText = 'Pause'
        }
    }
    playButton.addEventListener('click', playPause)

    let protectSlider = document.getElementById('protect-range')
    protectSlider.value = params.protect_range
    protectSlider.addEventListener('change', () => {
        params.protect_range = protectSlider.value
        world.protect_range = params.protect_range
    })

    let visibleSlider = document.getElementById('visible-range')
    visibleSlider.value = params.visible_range
    visibleSlider.addEventListener('change', () => {
        params.visible_range = visibleSlider.value
        world.visible_range = params.visible_range
    })

    let marginSlider = document.getElementById('world-margin')
    marginSlider.value = params.margin
    marginSlider.addEventListener("change", () => {
        params.margin = marginSlider.value
        world.margin = params.margin
    })
})
