import { readValue, readColor, getCanvas } from './user_interface';
import { Clifford } from '../pkg/strange_attractor_explorer';

enum Mode {
    START,
    BOUND_CHECK,
    RENDER,
    DRAWING,
    FINISHED,
}

class StateControl {
    current_frame = 0;
    frames: number;

    current_mode: Mode;

    changed_state: boolean;

    constructor(frames: number) {
        this.frames = frames
        this.current_mode = Mode.START;

        this.changed_state = false;
    }

    get_p(): number {
        return this.current_frame / this.frames;
    }

    tick() {
        this.changed_state = false;

        if (this.if_starting()) {
            this.current_frame = 0;
            this.current_mode = Mode.BOUND_CHECK;
            this.changed_state = true;
        } else if (this.is_bound_checking()) {
            this.current_frame++;

            if (this.current_frame == this.frames - 1) {
                this.current_frame = 0;
                this.current_mode = Mode.RENDER;
                this.changed_state = true;
            }
        } else if (this.is_rendering()) {
            this.current_frame++;

            if (this.current_frame == this.frames - 1) {
                this.current_frame = 0;
                this.current_mode = Mode.DRAWING;
                this.changed_state = true;
            }
        } else if (this.is_drawing()) {
            this.current_mode = Mode.FINISHED;
            this.changed_state = true;
        } else if (this.is_finished()) {
            //
        }
    }

    start() {
        this.current_mode = Mode.START;
    }

    if_starting(): boolean {
        return this.current_mode == Mode.START;
    }

    is_bound_checking(): boolean {
        return this.current_mode == Mode.BOUND_CHECK;
    }

    is_rendering(): boolean {
        return this.current_mode == Mode.RENDER;
    }

    is_drawing(): boolean {
        return this.current_mode == Mode.DRAWING;
    }

    is_finished(): boolean {
        return this.current_mode == Mode.FINISHED;
    }

    did_state_change(): boolean {
        return this.changed_state;
    }
}

class CliffordManager {
    parameters_start: number[];
    parameters_end: number[];

    color_start: number[];
    color_end: number[];

    gamma: number;
    iters: number;
    frames: number;

    width: number;
    height: number;

    attractor: Clifford;
    image_data: any;

    rust: any;
    state_control: StateControl;

    constructor(rust: any) {
        this.rust = rust;

        let canvas = getCanvas();
        this.width = canvas.width;
        this.height = canvas.height;

        this.attractor = this.rust.init_attractor();
        this.image_data = this.rust.init_image_data(this.width, this.height);

        this.iters = readValue('iters_value');
        this.frames = readValue('frames_value');
        this.gamma = readValue('gamma_value');

        this.parameters_start = [
            readValue('a_value_start'),
            readValue('b_value_start'),
            readValue('c_value_start'),
            readValue('d_value_start'),
        ];

        this.parameters_end = [
            readValue('a_value_end'),
            readValue('b_value_end'),
            readValue('c_value_end'),
            readValue('d_value_end'),
        ];

        this.color_start = readColor('color_value_start');
        this.color_end = readColor('color_value_end');

        this.attractor.set_iters(this.iters);

        this.state_control = new StateControl(this.frames);
    }

    start() {
        this.state_control.start();
    }

    finished_running(): boolean {
        return this.state_control.is_finished();
    }

    interpolate_and_render_step() {
        this.state_control.tick();

        if (this.state_control.is_bound_checking()) {
            if (this.state_control.did_state_change()) {
                this.image_data.reset();
                this.attractor.find_bounding_box(true);
            }

            let p = this.state_control.get_p();
            this.set_attractor_parameters_and_color(p);
            this.attractor.find_bounding_box(false);
        } else if (this.state_control.is_rendering()) {
            if (this.state_control.did_state_change()) {
                this.attractor.bump_bounding_box();
            }

            let p = this.state_control.get_p();
            this.set_attractor_parameters_and_color(p);
            this.attractor.iterate(this.image_data);
        } else if (this.state_control.is_drawing()) {
            this.draw_to_canvas();
        } else if (this.state_control.is_finished()) {
            //
        }
    }

    interpolate_and_render() {
        for (let mode = 0; mode < 2; mode++) {
            for (let frame = 0; frame < this.frames; frame++) {
                let p = frame / this.frames;

                let parameters = this.interpolate_parameters(p)
                this.set_attractor_params(parameters);

                let color = this.interpolate_colors(p);
                this.attractor.set_color(
                    color[0],
                    color[1],
                    color[2]
                );

                if (mode == 0) {
                    this.attractor.find_bounding_box(false);
                } else if (mode == 1) {
                    this.attractor.iterate(this.image_data);
                }
            }

            if (mode == 0) {
                this.attractor.bump_bounding_box();
            }
        }
    }

    set_attractor_parameters_and_color(p: number) {
        let parameters = this.interpolate_parameters(p)
        this.set_attractor_params(parameters);

        let color = this.interpolate_colors(p);
        this.attractor.set_color(
            color[0],
            color[1],
            color[2]
        );
    }

    interpolate_parameters(p: number): number[] {
        return this.interpolate_vectors(
            this.parameters_start,
            this.parameters_end,
            p
        );
    }

    interpolate_colors(p: number): number[] {
        return this.interpolate_vectors(
            this.color_start,
            this.color_end,
            p
        )
    }

    interpolate_vectors(a: number[], b: number[], p: number): number[] {
        let parameters = [];

        for (let index = 0; index < a.length; index++) {
            parameters.push(
                (b[index] - a[index]) * p + a[index]
            );
        }

        return parameters;
    }

    set_attractor_params(parameters: number[]) {
        this.attractor.set_parameters(
            parameters[0],
            parameters[1],
            parameters[2],
            parameters[3],
        )
    }

    draw_to_canvas() {
        this.image_data.normalize_image();
        this.image_data.invert_colors();
        this.image_data.gamma_correction(this.gamma);

        let image_pixels = this.image_data.as_u8();

        let canvas = getCanvas();
        let canvas_context = canvas.getContext('2d');

        if (canvas_context != null) {
            let clamped_array = new Uint8ClampedArray(image_pixels);
            let image_data = new ImageData(clamped_array, this.width, this.height);

            canvas_context.putImageData(image_data, 0, 0);
        }
    }
}

export {
    CliffordManager
}
