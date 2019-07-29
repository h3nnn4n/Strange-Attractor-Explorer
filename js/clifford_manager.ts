import { readValue, readColor, getCanvas, updateProgressBar, resetProgressBar } from './user_interface';
import { Clifford } from '../pkg/strange_attractor_explorer';
import { StateControl } from './state_control';

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

    last_bar_value: number = 0;

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

    update_config() {
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

        this.state_control.frames = this.frames;
    }

    start() {
        resetProgressBar();
        this.last_bar_value = 0;

        this.state_control.start();
    }

    finished_running(): boolean {
        return this.state_control.is_finished();
    }

    interpolate_and_render_step() {
        this.update_progress_bar();

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

    update_progress_bar() {
        let value = this.state_control.get_progress();

        if (value - this.last_bar_value < 5 && value < 100) {
            return;
        }

        this.last_bar_value = value;

        updateProgressBar(value);
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
