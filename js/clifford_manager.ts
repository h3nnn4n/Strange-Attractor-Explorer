import { readValue, readColor, getCanvas } from './user_interface';
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

    interpolate_parameters(p: number): number[] {
        return this.interpolate_vectors(
            this.parameters_start,
            this.parameters_end,
            p
        );
    }

    interpolate_colors(p: number): number[] {
        let color_start = readColor('color_value_start');
        let color_end = readColor('color_value_end');

        return this.interpolate_vectors(
            color_start,
            color_end,
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
