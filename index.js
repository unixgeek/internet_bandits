const rust = import("./pkg");
const canvas = document.getElementById("canvas");

// Update this to use the time passed in by animationFrame. Target 30fps?
// Use math from sandbox project to count frames.
// Animate sprite with movement keys.

// Animate terminology.
// Separate thread that does calculation outside of animation/render loop?

rust.then(m => {
    const client = new m.Client();

    const fps = 30;
    const interval = 1000 / fps;
    console.log(interval);
    let lastTime = 0;

    // function render(time) {
    //     requestAnimationFrame(render);
    //
    //     if (time - lastTime >= interval) {
    //         // console.log(lastTime + "," + time);
    //         client.render(time, lastTime);
    //         lastTime = time;
    //     }
    // }

    const FPS_THROTTLE = 1000.0 / 30.0;
    const initialTime = Date.now();
    let lastDrawTime = -1;

    function render() {
        window.requestAnimationFrame(render);
        const currentTime = Date.now();

        if (currentTime >= lastDrawTime + FPS_THROTTLE) {
            client.render(currentTime, lastDrawTime);
            lastDrawTime = currentTime;

            if (window.innerHeight !== canvas.height || window.innerWidth !== canvas.width) {
                canvas.height = window.innerHeight;
                canvas.clientHeight = window.innerHeight;
                // canvas.style.height = window.innerHeight.toString();

                canvas.width = window.innerWidth;
                canvas.clientWidth = window.innerWidth;
                // canvas.style.width = window.innerWidth.toString();
            }

            // let elapsedTime = currentTime - initialTime;
            // client.update(elapsedTime, window.innerHeight, window.innerWidth);
        }
    }

    document.addEventListener("keydown", event => {
        if (event.key === "ArrowLeft") {
            client.move_player_left();
        } else if (event.key === "ArrowRight") {
            client.move_player_right();
        } else if (event.key === "ArrowDown") {
            client.move_player_down()
        } else if (event.key === "ArrowUp") {
            client.move_player_up();
        }
    });

    render();
}).catch(console.error);
