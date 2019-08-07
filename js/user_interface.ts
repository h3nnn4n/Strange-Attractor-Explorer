import * as $ from 'jquery';
import * as AColorPicker from 'a-color-picker';
const JSColorPicker = require('jscolor-picker');

function bindEvents(render_callback: any, random_callback: any) {
    $('#render').click(render_callback);
    $('#random').click(() => { setRandomParams(random_callback()) });
    $('#download').click(downloadCanvas);
}

function downloadCanvas() {
    var dataURL = getCanvas().toDataURL('image/png');
    $('#download').attr('href', dataURL);
    $('#download').attr('download', 'image.png');
}

function setRandomParams(params: any) {
    $('#a_value_start').val(params.a);
    $('#b_value_start').val(params.b);
    $('#c_value_start').val(params.c);
    $('#d_value_start').val(params.d);

    $('#a_value_end').val(params.a + Math.random() * 0.1);
    $('#b_value_end').val(params.b + Math.random() * 0.1);
    $('#c_value_end').val(params.c + Math.random() * 0.1);
    $('#d_value_end').val(params.d + Math.random() * 0.1);

    $('#render').click();

    console.log('found params with lyapunov: ' + params.lyapunov);
}

function readValue(name: string): number {
    let value = $('#' + name).val();

    return Number(value);
}

function readColor(name: string): number[] {
    let value = <string>$('#' + name).val();

    let parsed = <number[]><unknown>AColorPicker.parseColor(value, "rgb");

    return parsed;
}

function getCanvas() {
    let j_canvas = $('#canvas');

    let canvas = <HTMLCanvasElement>j_canvas[0];

    return canvas;
}

function updateProgressBar(value: number) {
    let bar = $('#progress_bar');

    bar.css('width', value + '%');
}

function resetProgressBar() {
    $('#progress_bar').hide();
    $('#progress_bar').width(0 + '%');
    $('#progress_bar').show();
}

export {
    bindEvents,
    readValue,
    readColor,
    getCanvas,
    updateProgressBar,
    resetProgressBar
}
