const rust = import("./pkg");

// Update this to use the time passed in by animationFrame. Target 30fps?
// Use math from sandbox project to count frames.
// Animate sprite with movement keys.

// Animate terminology.
// Separate thread that does calculation outside of animation/render loop?

rust.then(m => {
    m.say_hello();

    const FPS = 1000 / 30;
    console.log(FPS);
    const client = new m.Client();
    let initialTime = Date.now();
    let frameCount = 0;

    let otherTime = Date.now();

    function render() {
        window.requestAnimationFrame(render);
        const currentTime = Date.now();

        frameCount++;
        if (currentTime - initialTime >= FPS) {
            if ((currentTime - otherTime) >= 1000) {
                client.animate(frameCount);
                otherTime = Date.now();
            }
            frameCount = 0;
            initialTime = Date.now();
        }
    }

    render();
}).catch(console.error);
