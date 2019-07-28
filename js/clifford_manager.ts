import { readValue, getCanvas } from './user_interface';
import { Clifford } from '../pkg/strange_attractor_explorer';

class CliffordManager {
    parameters_start: number[];
    parameters_end: number[];

    gamma: number;
    iters: number;
    frames: number;

    width: number;
    height: number;

    attractor: Clifford;
    image_data: any;

    rust: any;

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

        this.attractor.set_iters(this.iters);
    }

    interpolate_and_render() {
        for (let mode = 0; mode < 2; mode++) {
            for (let frame = 0; frame < this.frames; frame++) {
                let p = frame / this.frames;

                let parameters = this.interpolate_parameters(p)
                this.set_attractor_params(parameters);

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

    interpolate_parameters(p: number): number[] {
        let parameters = [
            0, 0, 0, 0
        ];

        for (let index = 0; index < this.parameters_start.length; index++) {
            parameters[index] =
                (this.parameters_end[index] - this.parameters_start[index]) * p + this.parameters_start[index];
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
