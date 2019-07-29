import * as $ from 'jquery';
import * as AColorPicker from 'a-color-picker';
const JSColorPicker = require('jscolor-picker');

function bindEvents(callback: any) {
    $('#render').click(callback);
    $('#download').click(downloadCanvas);
}

function downloadCanvas() {
    var dataURL = getCanvas().toDataURL('image/png');
    $('#download').attr('href', dataURL);
    $('#download').attr('download', 'image.png');
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
