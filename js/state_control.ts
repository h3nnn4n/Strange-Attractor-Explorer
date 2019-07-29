enum Mode {
    START,
    BOUND_CHECK,
    RENDER,
    DRAWING,
    FINISHED,
}

export class StateControl {
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

    get_progress(): number {
        if (this.is_starting()) {
            return 0;
        }

        if (this.is_finished() || this.is_drawing()) {
            return 100;
        }

        let value = this.get_p() * 0.5;

        if (this.is_rendering()) {
            value += 0.5;
        }

        return value * 100;
    }

    tick() {
        this.changed_state = false;

        if (this.is_starting()) {
            this.current_frame = 0;
            this.current_mode = Mode.BOUND_CHECK;
            this.changed_state = true;
        } else if (this.is_bound_checking()) {
            this.current_frame++;

            if (this.current_frame == this.frames) {
                this.current_frame = 0;
                this.current_mode = Mode.RENDER;
                this.changed_state = true;
            }
        } else if (this.is_rendering()) {
            this.current_frame++;

            if (this.current_frame == this.frames) {
                this.current_frame = 0;
                this.current_mode = Mode.DRAWING;
                this.changed_state = true;
            }
        } else if (this.is_drawing()) {
            this.changed_state = true;
            this.current_mode = Mode.FINISHED;
        } else if (this.is_finished()) {
            //
        }
    }

    start() {
        this.current_mode = Mode.START;
    }

    is_starting(): boolean {
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