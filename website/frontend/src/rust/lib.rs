use std::f64;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
fn start() {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    web_sys::request_animation_frame(window);
    let canvas = document.get_element_by_id("c").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

}
const data = JSON.parse(jQuery.ajax({
            url: "/static/systems.json",
            async: false
        }).responseText)
        const canvas = document.getElementById("c");
        const ctx = canvas.getContext("2d");
        ctx.fillStyle = "black"; // make the current viewport black
        ctx.fillRect(0, 0, canvas.width, canvas.height);
        const mouse = {
            x: 0,
            y: 0,
            w: 0,
            alt: false,
            shift: false,
            ctrl: false,
            buttonLastRaw: 0, // user modified value
            buttonRaw: 0,
            over: false,
            buttons: [1, 2, 4, 6, 5, 3], // masks for setting and clearing button raw bits;
        };

        function mouseMove(event) {
            mouse.x = event.offsetX;
            mouse.y = event.offsetY;
            if (mouse.x === undefined) {
                mouse.x = event.clientX;
                mouse.y = event.clientY;
            }
            mouse.alt = event.altKey;
            mouse.shift = event.shiftKey;
            mouse.ctrl = event.ctrlKey;
            if (event.type === "mousedown") {
                event.preventDefault()
                mouse.buttonRaw |= mouse.buttons[event.which-1];
            } else if (event.type === "mouseup") {
                mouse.buttonRaw &= mouse.buttons[event.which + 2];
            } else if (event.type === "mouseout") {
                mouse.buttonRaw = 0;
                mouse.over = false;
            } else if (event.type === "mouseover") {
                mouse.over = true;
            } else if (event.type === "mousewheel") {
                event.preventDefault()
                mouse.w = event.wheelDelta;
            } else if (event.type === "DOMMouseScroll") { // firefox
               mouse.w = -event.detail;
            }


        }

        function setupMouse(e) {
            e.addEventListener('mousemove', mouseMove);
            e.addEventListener('mousedown', mouseMove);
            e.addEventListener('mouseup', mouseMove);
            e.addEventListener('mouseout', mouseMove);
            e.addEventListener('mouseover', mouseMove);
            e.addEventListener('mousewheel', mouseMove);
            e.addEventListener('DOMMouseScroll', mouseMove); //firefox

            e.addEventListener("contextmenu", function (e) {
                e.preventDefault();
            }, false);
        }
        setupMouse(canvas);



struct DisplayTransform {
    x: f64,
    y: f64,
    ox: f64,
    oy: f64,
    scale: f64,
    rotate: f64,
    cx: f64,
    cy: f64,
    cox: f64,
    coy: f64,
    cscale: f64,
    crotate: f64,
    dx: f64,
    dy: f64,
    dox: f64,
    doy: f64,
    dscale: f64,
    drotate: f64,
    drag: f64,
    accel: f64,
    matrix: [f64; 6],
    inv_matrix: [f64; 6],
    mouse_x: f64,
    mouse_y: f64,
    ctx: web_sys::CanvasRenderingContext2d
}

impl DisplayTransform {
    pub fn set_transform(&self) {
        self.ctx;
    }
}


            setTransform: function () {
                let m = this.matrix;
                let i = 0;
                this.ctx.setTransform(m[i++], m[i++], m[i++], m[i++], m[i++], m[i++]);
            },
            setHome: function () {
                this.ctx.setTransform(1, 0, 0, 1, 0, 0);

            },
            update: function () {
                // smooth all movement out. drag and accel control how this moves
                // acceleration
                this.dx += (this.x - this.cx) * this.accel;
                this.dy += (this.y - this.cy) * this.accel;
                this.dox += (this.ox - this.cox) * this.accel;
                this.doy += (this.oy - this.coy) * this.accel;
                this.dscale += (this.scale - this.cscale) * this.accel;
                this.drotate += (this.rotate - this.crotate) * this.accel;
                // drag
                this.dx *= this.drag;
                this.dy *= this.drag;
                this.dox *= this.drag;
                this.doy *= this.drag;
                this.dscale *= this.drag;
                this.drotate *= this.drag;
                // set the chase values. Chase chases the required values
                this.cx += this.dx;
                this.cy += this.dy;
                this.cox += this.dox;
                this.coy += this.doy;
                this.cscale += this.dscale;
                this.crotate += this.drotate;

                // create the display matrix
                this.matrix[0] = Math.cos(this.crotate) * this.cscale;
                this.matrix[1] = Math.sin(this.crotate) * this.cscale;
                this.matrix[2] = -this.matrix[1];
                this.matrix[3] = this.matrix[0];

                // set the coords relative to the origin
                this.matrix[4] = -(this.cx * this.matrix[0] + this.cy * this.matrix[2]) + this.cox;
                this.matrix[5] = -(this.cx * this.matrix[1] + this.cy * this.matrix[3]) + this.coy;


                // create inverse matrix
                var det = (this.matrix[0] * this.matrix[3] - this.matrix[1] * this.matrix[2]);
                this.invMatrix[0] = this.matrix[3] / det;
                this.invMatrix[1] = -this.matrix[1] / det;
                this.invMatrix[2] = -this.matrix[2] / det;
                this.invMatrix[3] = this.matrix[0] / det;

                // check for mouse. Do controls and get real position of mouse.
                if (mouse !== undefined) {  // if there is a mouse get the real canvas coordinates of the mouse
                    if (mouse.oldX !== undefined && (mouse.buttonRaw & 1) === 1) { // check if panning (middle button)
                        var mdx = mouse.x - mouse.oldX; // get the mouse movement
                        var mdy = mouse.y - mouse.oldY;
                        // get the movement in real space
                        var mrx = (mdx * this.invMatrix[0] + mdy * this.invMatrix[2]);
                        var mry = (mdx * this.invMatrix[1] + mdy * this.invMatrix[3]);
                        this.x -= mrx;
                        this.y -= mry;
                    }
                    // do the zoom with mouse wheel
                    if (mouse.w !== undefined && mouse.w !== 0) {
                        this.ox = mouse.x;
                        this.oy = mouse.y;
                        this.x = this.mouseX;
                        this.y = this.mouseY;
                        /* Special note from answer */
                        // comment out the following if you change drag and accel
                        // and the zoom does not feel right (lagging and not
                        // zooming around the mouse
                        /*
                        this.cox = mouse.x;
                        this.coy = mouse.y;
                        this.cx = this.mouseX;
                        this.cy = this.mouseY;
                        */
                        if (mouse.w > 0) { // zoom in
                            this.scale *= 1.1;
                            mouse.w -= 20;
                            if (mouse.w < 0) {
                                mouse.w = 0;
                            }
                        }
                        if (mouse.w < 0) { // zoom out
                            this.scale *= 1 / 1.1;
                            mouse.w += 20;
                            if (mouse.w > 0) {
                                mouse.w = 0;
                            }
                        }

                    }
                    // get the real mouse position
                    let screenX = (mouse.x - this.cox);
                    let screenY = (mouse.y - this.coy);
                    this.mouseX = this.cx + (screenX * this.invMatrix[0] + screenY * this.invMatrix[2]);
                    this.mouseY = this.cy + (screenX * this.invMatrix[1] + screenY * this.invMatrix[3]);
                    mouse.rx = this.mouseX;  // add the coordinates to the mouse. r is for real
                    mouse.ry = this.mouseY;
                    // save old mouse position
                    mouse.oldX = mouse.x;
                    mouse.oldY = mouse.y;
                }
            }
        };

        const colorMap = { // TODO: Switch to rgb
            "RED_STAR": "red",
            "ORANGE_STAR": "orange",
            "WHITE_DWARF": "white",
            "YOUNG_STAR": "yellow",
            "BLACK_HOLE": "grey",
            "BLUE_STAR": "blue",
            "HYPERGIANT": "red", // repeated
            "NEUTRON_STAR": "white", // repeated
            "UNSTABLE": "blue" // repeated
        }


        function draw() {
            let zoom = 16/displayTransform.cscale;
            ctx.font = zoom + "px serif";
            for (const system of Object.keys(data)) {
                if (-displayTransform.matrix[4]-canvas.width/displayTransform.matrix[0] < data[system].x * 10 && -displayTransform.matrix[4]+canvas.width/displayTransform.matrix[0] > data[system].x * 10) { // x coord (5 is y)
                    if (-displayTransform.matrix[5]-canvas.height/displayTransform.matrix[0] < data[system].y * 10 && -displayTransform.matrix[5]+canvas.height/displayTransform.matrix[0] > data[system].y * 10) {
                        ctx.fillStyle = colorMap[data[system].type] || "green"
                        ctx.beginPath();
                        if (displayTransform.scale < 1) {
                            ctx.arc(data[system].x * 10, data[system].y * 10, 18 + 2 / displayTransform.cscale, 0, 2 * Math.PI);
                        } else {
                            ctx.arc(data[system].x * 10, data[system].y * 10, 20, 0, 2 * Math.PI);
                        }
                        ctx.fill();
                        if (displayTransform.scale > 0.5) {
                            ctx.globalAlpha = 0.8;
                            ctx.fillStyle = "green";
                            ctx.fillText(system + " (" + data[system].num_waypoints + " waypoints)", data[system].x * 10 + 20, data[system].y * 10 - 10);
                        }
                    }
                }
            }
        }

        function update(_time) {
            // update the transform
            displayTransform.update();
            // set home transform to clear the screen
            displayTransform.setHome();
            ctx.fillStyle = "black"; // make the current viewport black
            ctx.fillRect(0, 0, canvas.width, canvas.height);
            // display stuff
            displayTransform.setTransform();
            if (mouse.buttonRaw === 4) { // right click to return to home
                displayTransform.x = 0;
                displayTransform.y = 0;
                displayTransform.scale = 1;
                displayTransform.rotate = 0;
                displayTransform.ox = 0;
                displayTransform.oy = 0;
            }
            draw();
            ctx.restore();
            // register update as the function to call to request the next frame
            requestAnimationFrame(update);
        }
        displayTransform.update();
        displayTransform.setHome();
        displayTransform.setTransform();
        update(); // start canvas updates